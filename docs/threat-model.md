# Threat model

This document covers the agent runtime of Isonomic Desktop: the engine that runs every agent session, the window and its first-party apps, third-party apps (Views) and agent machines (Cells). It states what is protected, from whom, by which mechanism, and how each claim is tested.

Sources are the decision records in the product repositories, cited as *record, decision number*. A mechanism marked *untested* has no test yet that shows it holds against the adversary named; the [agent-control evaluation](research-agenda.md) is how those tests get written.

## The system in one paragraph

Isonomic Desktop is two programs: a window and a headless engine, which talk over one local socket. Apps never see a connection string, a port, a key or the network. Every agent session runs in the engine, never inside an app (Agent stack 1). Apps, including the chat app, drive and watch sessions through one interface. First-party apps are trusted code compiled into the window; third-party apps are Views, never native code (Desktop 2). Models run on the user's machine by default (Desktop 9).

## What is protected

| Asset | Where it lives |
| --- | --- |
| The user's prompts and conversation history | The session log: an append-only table in the engine's database (Agent stack 11) |
| The user's files | The user's disk and the engine's blob store |
| The user's memory | Engine-owned tables, reached only through the host's memory tools and the SDK (Memory 5) |
| The user's authority over agents | Approval decisions, which only the user can make (Agent stack 13) |

## Adversaries

### 1. Injected content in web pages, files and tool results

An attacker controls text that an agent reads: a web page, a document, an email, or the output of a tool. The text tries to make the agent act for the attacker.

**What holds**

- **Injected text cannot approve an action.** A consequential action needs an approval decision carrying a token tied to a user action. Text in the model's context cannot produce that token. An ambiguous answer is treated as a denial. (Agent stack 13)
- **Injected text cannot plant a memory.** A memory write must point to a span of at least two words in the text of the user's own message in that session. Attachments, file excerpts, tool results and context an app attached are not evidence. (Memory 12)
- **Memory is data, never authority.** A memory reaches the model as notes about the user. It cannot change policy, approvals, budgets, tools or presets. (Memory 2)
- **A session that has read untrusted content asks first.** Once the session's untrusted flag is set, a memory write asks the user before it proceeds. (Memory 16)
- **Every memory write is announced.** A notice with Undo follows every tool write; no app, preset or View can turn it off. (Memory 13)

**What does not hold**

- The model can still be persuaded. The controls above bound what persuasion can achieve; they do not prevent it.
- Text the user pastes into the message box counts as the user's own words. The mandatory notice is the only check on that path.

**How it is tested**

- The memory specification's gate requires that evidence pointing at attachment text, app-attached context or a tool result is refused, and that planted text in a tool result never becomes memory. These gate tests belong to the memory work, which is not yet built.
- Approval binding against injected content: untested. Covered by the agent-control evaluation.

### 2. A malicious third-party View

A third-party app, written as a View, tries to read the user's data or act beyond what the user granted.

**What holds**

- **Third-party apps are Views, never native code.** (Desktop 2)
- **Views can request approvals but never decide them.** (Agent stack 13)
- **Views cannot read the user's memory.** A third-party app gets its own memory scope at most. (Memory 3)
- **Confinement is claimed for Views.** Mechanical confinement is claimed only for Views and Cells; first-party apps are trusted code with least-privilege tokens, not sandboxed. (Desktop 12)

**How it is tested**

- The memory gate requires that Views cannot read user scope. Not yet built.
- View confinement: the mechanism and its tests are specified in the Views repository, not in a decision record this document can cite. Listed as an open item.

### 3. A malicious Cell

A Cell is one agent's machine: a Linux microVM with its own kernel, started and destroyed by the engine. The adversary is the code running inside it, including a third-party agent harness, with root in the guest.

**What holds**

- **The boundary is the hypervisor.** OS sandboxes are inner layers only, and there is no silent downgrade when virtualization is missing. (Cells 2)
- **No secrets enter a Cell.** Gateways outside the Cell attach credentials. (Cells 4)
- **One wire, and egress denied by default.** vsock to the engine is the only connection. Egress is denied unless allowed by host, method and path, and the gateway replaces credentials rather than forwarding the agent's. (Cells 5)
- **Authority only narrows.** An agent holds a signed capability set; delegation only narrows it; Cells never hold the engine's keys. (Cells 6)
- **The guest is assumed hostile.** Every control sits outside it. (Cells 14)

**How it is tested**

- Cells Milestone 0 gate on the Intel test machine: 20 of 20 escape items passed; the machine is gone 160 ms after the host process is killed; 50 start-stop cycles with no leaks; the host's frame decoder was fuzzed with 7.8 million inputs in ten minutes without a crash.

**What does not hold**

- Prompt injection is not solved; damage is bounded to the agent's grants.
- The hypervisor is trusted.
- Actions taken on the user's real desktop are outside this model.

Cells arrive at desktop Milestone 6 (Desktop record, milestones). Their Milestone 0 code exists; nothing above is shipped.

### 4. A compromised user device

Malware with the user's privileges on the user's computer. **Out of scope.** Such an attacker can read what the user can read and act as the user. Nothing in this document defends against it.

### 5. A misaligned model

The model itself pursues a goal other than the user's.

**What bounds it**

- **Approvals.** It cannot approve its own consequential actions. (Agent stack 13)
- **Confinement.** Inside a Cell it holds no secrets, has no network beyond what the user allowed, and its authority only narrows. (Cells 2, 4, 5, 6)
- **Crash isolation.** A panic or hang in one session never affects the engine or other sessions. (Agent stack 15)
- **A record.** Every session is kept in an append-only log. (Agent stack 11)
- **A model server only the engine can reach.** The local model server listens on loopback only, with a fresh port and a random secret at every launch, and starts with an environment built from an allow-list. (Inference 2, 3; Desktop 7)

Nothing else bounds a misaligned model. Isonomic does not make models aligned.

## Open items

- View confinement mechanism and tests: not yet covered by a citable decision record.
- Whether tool results and web content are "data, never authority" in general, beyond memory: no decision record states it for the whole runtime. The agent-control evaluation tests it as a control (content-is-data).
- The untrusted-session flag beyond memory writes: specified for Cell tools as the Rule of Two (Cells 7), at Cells Milestone 4.
