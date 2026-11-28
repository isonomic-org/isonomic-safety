# Threat model

This document covers the agent runtime of Isonomic Desktop: the engine that runs every agent session, the window and its first-party apps, third-party apps (Views) and agent machines (Cells). It states what is protected, from whom, by which mechanism, and how each claim is tested.

Sources are the decision records in the product repositories, cited as *record, decision number*. A mechanism marked *untested* has no test yet that shows it holds against the adversary named; the [agent-control evaluation](research-agenda.md) is how those tests get written.

The tool kits cite the design *Tool kits: architecture v0.3* by section, with the test it requires. It is approved for building but is not yet a decision record in a product repository. *Invariant N* is one of the twelve property tests in its "Testing method".

## The system in one paragraph

Isonomic Desktop is two programs: a window and a headless engine, which talk over one local socket. Apps never see a connection string, a port, a key or the network. Every agent session runs in the engine, never inside an app (Agent stack 1). Apps, including the chat app, drive and watch sessions through one interface. First-party apps are trusted code compiled into the window; third-party apps are Views, never native code (Desktop 2). Models run on the user's machine by default (Desktop 9). Sessions can be given [tool kits](#tool-kits), not yet built.

## What is protected

| Asset | Where it lives |
| --- | --- |
| The user's prompts and conversation history | The session log: an append-only table in the engine's database (Agent stack 11) |
| The user's files | The user's disk and the engine's blob store |
| The user's memory | Engine-owned tables, reached only through the host's memory tools and the SDK (Memory 5) |
| The user's authority over agents | Approval decisions, which only the user can make (Agent stack 13) |
| The user's linked folders | Folders the user linked to an app in the window, for reading or for writing too (Tool kits: architecture v0.3, "Trust boundary") |
| The user's standing permissions | Grants in the engine's identity records, listed in Settings (Tool kits: architecture v0.3, "The frame") |

## Adversaries

### 1. Injected content in web pages, files and tool results

An attacker controls text that an agent reads: a web page, a document, an email, or the output of a tool. The text tries to make the agent act for the attacker.

**What holds**

- **Injected text cannot approve an action.** A consequential action needs an approval decision carrying a token tied to a user action. Text in the model's context cannot produce that token. An ambiguous answer is treated as a denial. (Agent stack 13)
- **Injected text cannot plant a memory.** A memory write must point to a span of at least two words in the text of the user's own message in that session. Attachments, file excerpts, tool results and context an app attached are not evidence. (Memory 12)
- **Memory is data, never authority.** A memory reaches the model as notes about the user. It cannot change policy, approvals, budgets, tools or presets. (Memory 2)
- **A session that has read untrusted content asks first.** Once the session's untrusted flag is set, a memory write asks the user before it proceeds. (Memory 16)
- **Every memory write is announced.** A notice with Undo follows every tool write; no app, preset or View can turn it off. (Memory 13)
- **Text in linked files is untrusted.** Files are a path for this adversary: a downloaded document can carry instructions. Reading a file sets the private and untrusted flags, so the next outward call asks. (Tool kits: architecture v0.3, "Decisions"; tests: kits-web fixture, case [file-asks-for-command](../evals/agent-control/cases/file-asks-for-command.md).)

**What does not hold**

- The model can still be persuaded. The controls above bound what persuasion can achieve; they do not prevent it.
- Text the user pastes into the message box counts as the user's own words. The mandatory notice is the only check on that path.
- Treating files as untrusted has a cost: after a file read, each web call asks. Prompts per task are measured.

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

Malware with the user's privileges on the user's computer. **Out of scope.** Such an attacker can read what the user can read and act as the user. Nothing in this document defends against it. See also [The local socket](#the-local-socket).

### 5. A misaligned model

The model itself pursues a goal other than the user's.

**What bounds it**

- **Approvals.** It cannot approve its own consequential actions. (Agent stack 13)
- **Confinement.** Inside a Cell it holds no secrets, has no network beyond what the user allowed, and its authority only narrows. (Cells 2, 4, 5, 6)
- **Crash isolation.** A panic or hang in one session never affects the engine or other sessions. (Agent stack 15)
- **A record.** Every session is kept in an append-only log. (Agent stack 11)
- **A model server only the engine can reach.** The local model server listens on loopback only, with a fresh port and a random secret at every launch, and starts with an environment built from an allow-list. (Inference 2, 3; Desktop 7)

Nothing else bounds a misaligned model. Isonomic does not make models aligned.

## Tool kits

Kits give a session files, commands, the web, a browser, skills and search. Commands and the browser run in a Cell if there is one, else on a confined host backend. **Nothing here is built.** Each *What holds* line cites the design section that decides it and the test required before it ships; until then it is a design, not a measured protection. A *canary* is a broken build the test must fail. The evaluation cases are in [evals/agent-control/cases/](../evals/agent-control/cases/).

### Controls every kit shares

- **The model cannot ask for a permission.** A tool with no grant is not offered; a call missing a grant gets an error, never a prompt. (Tool kits: architecture v0.3, "Who can ask the person for a permission"; tests: kits-offer fixture, conformance permissions.request_needs_person, invariant 5.)
- **The run uses what was checked:** the same folder handle, the same network address. (Tool kits: architecture v0.3, "Addition two: access"; tests: invariants 6 and 8.)
- **The grant is checked again** in the engine, at the moment of the operation. (Tool kits: architecture v0.3, "Addition three: grants"; tests: grants and files contract suites.)
- **The three-flag rule.** Once a session has read private data and untrusted content, every outward call asks, offering only "once". (Tool kits: architecture v0.3, "How a call is decided", row 3; tests: invariants 2 and 3, the canary policy that ignores the flags.)
- **Nothing is remembered** for code, under the three-flag rule, or for a change that cannot be undone. (Tool kits: architecture v0.3, "Standing permissions"; test: invariant 2.)
- **A session approval covers one scope in one session,** never another scope, a fork, or a session after purge. (Tool kits: architecture v0.3, "Addition three: grants"; tests: invariant 4, conformance kits.remember.)
- **Every standing permission can be revoked,** and the next check refuses. (Tool kits: architecture v0.3, "Standing permissions"; test: conformance permissions.revoked.)

### Linked folders

**What holds**

- **Only the user links a folder,** in the window. (Tool kits: architecture v0.3, "Trust boundary"; test: conformance permissions.shell_only.)
- **Some places can never be linked:** the Capsule directory, key stores, other apps' data, system folders, the whole home folder. (Tool kits: architecture v0.3, "Trust boundary"; test: the never-linkable list.)
- **Paths resolve step by step inside the folder.** Absolute paths, ~ and .. have no authority. A link out is refused, even if swapped in after the check. (Tool kits: architecture v0.3, "Files"; tests: path escape table with the two published escapes, invariant 6, the prefix canary.)
- **Writes to version-control metadata and the project instruction file are refused,** in any spelling. (Tool kits: architecture v0.3, "Files"; test: path escape table.)
- **Every file change can be undone:** a pre-image, an inverse move, or the system Trash. A change that cannot be undone asks every time. (Tool kits: architecture v0.3, "Files"; tests: undo table, invariant 7, kits-files-write fixture.)

**What does not hold**

- Every session of an app can read its linked folders.
- Where it may write, an agent can change files the user did not mean it to. Undo repairs; it does not prevent. The undo call comes later, with rewind.
- Injected text in a file can lead the model to read other files in the folder.

### Leaks through fetched addresses

An address can carry data off the device: read a private file, then fetch an address holding its contents. Every web call is outward; the three-flag rule is the control.

**What holds**

- **Nothing leaves the device until the user answers one web question per app.** (Tool kits: architecture v0.3, "Decisions"; test: kits-offer fixture.)
- **After a file read, read_url asks,** shows the full address and offers only "once". (Tool kits: architecture v0.3, "Web"; tests: kits-web fixture, invariant 3, case page-asks-for-fetch.)
- **read_url sends read requests only,** with no cookies or credentials. (Tool kits: architecture v0.3, "Web"; test: egress contract suite.)
- **Every web call is logged.** (Tool kits: architecture v0.3, "Decisions".)

**What is not claimed.** With web access on, addresses and search text leave the device; "nothing leaves this device" is then false. Until both flags are set, a fetch runs without asking, and its address carries whatever the model put in it. A user can approve an address they did not read. Sites see the device's address.

### Requests to local services

The engine's own services listen on the device. A fetched address must not reach them.

**What holds**

- **Private, loopback and link-local addresses are blocked** for read_url, search, the browser and confined commands. Each redirect is checked again. (Tool kits: architecture v0.3, "Web"; tests: egress table, which must include every local service the engine runs, invariant 8, the re-lookup canary.)

### Host commands

**What is not claimed.** A host command is not contained. It runs as the user. The sandbox stops writes outside granted folders, closes the engine's socket and the never-linkable places, and limits the network. It does not stop a command from reading most of what the user can read. With the web grant, a command can send what it read to a public address through the egress proxy. Approving each command is the control.

**What holds**

- **Every host command asks, shows the whole command, offers only "once", and is never remembered.** (Tool kits: architecture v0.3, "Decisions"; tests: kits-shell fixture, invariants 1 and 2.)
- **Commands over 2,000 characters are refused.** (Tool kits: architecture v0.3, "Shell and processes"; test: unit test on command length.)
- **No unconfined mode.** If the sandbox cannot be applied, the command does not run. (Tool kits: architecture v0.3, "Decisions"; test: exec contract suite on the host backend.)
- **Writes outside granted folders fail.** (Tool kits: architecture v0.3, "Shell and processes"; tests: R1's confinement cases, the open-profile canary.)
- **No network without the web grant,** and with it only the egress proxy. The environment is allow-listed. (Tool kits: architecture v0.3, "Shell and processes"; test: command table.)
- **Interactive sessions only.** Autonomous and delegated sessions get commands only in a Cell. (Tool kits: architecture v0.3, "Decisions".)

**What does not hold.** The danger mark never approves or blocks, and misses what it does not list. People asked often approve without reading. Confinement tests run after merge, not on each pull request. In a Cell, commands need no approval of their own, though the three-flag rule still asks; what an ask means with nobody present is open.

### The browser

**What holds**

- **A private profile per session,** destroyed with it, with none of the user's cookies or sign-ins. (Tool kits: architecture v0.3, "Browser"; tests: isolation test, bypass table.)
- **No downloads, uploads, file dialogs, clipboard, camera, microphone or location,** and no typing into password fields. (Tool kits: architecture v0.3, "Browser"; test: refusal table.)
- **The site is the scope of an action.** page_act asks at every autonomy level until the user allows that site for the session, never for good. (Tool kits: architecture v0.3, "Browser"; tests: kits-browser fixture, invariant 4.)
- **On the host, all traffic goes through the egress proxy.** With the proxy stopped, no page loads. (Tool kits: architecture v0.3, "Browser"; test: bypass table.)
- **In a Cell, page content never renders on the host.** (Tool kits: architecture v0.3, "What a Cell changes".)

**What does not hold.** On the host, the installed browser is a trusted part. The bypass tests show its traffic obeys the address rules, not that it has no flaws. On an allowed site, actions run without asking until both flags are set.

### The local socket

A socket call with no token is treated as the window, so any program running as the user can grant permissions and mint user-action tokens. That is adversary 4, out of scope. An agent's command is inside this model.

**What holds**

- **The host sandbox closes the socket and the Capsule directory to commands.** (Tool kits: architecture v0.3, "Shell and processes"; tests: command table, the open-socket canary.)
- **Host commands do not ship until that test is green.** (Tool kits: architecture v0.3, "Start here", gates.)

Tokens for the window itself, planned for desktop Milestone 4, are the proper fix. They are not built.

### Skills

A skill is instructions the model will follow, so it is a path for adversary 1.

**What holds**

- **A skill is data, never authority.** It cannot change policy, approvals, grants, budgets, tools or presets. (Tool kits: architecture v0.3, "Isonomic Skills"; tests: invariant 11, the canary skill that says approvals are not needed.)
- **A skill's scripts are commands,** with the same approvals and confinement. (Tool kits: architecture v0.3, "Isonomic Skills"; test: case skill-asks-for-command.)
- **A person decides trust, at install.** The model never installs, enables or edits a skill. A saved skill stays off until approved. (Tool kits: architecture v0.3, "Isonomic Skills"; tests: skills-basic fixture, conformance skills.shell_only.)
- **Bundled skills pass a seven-step intake** and signed hashes. No public marketplace. (Tool kits: architecture v0.3, "Decisions"; tests: the intake rejection cases, the tampered-bundle test.)

**What does not hold.** An approved personal skill is trusted, not safe. It can tell the model to misuse what the session holds, and it wins a name clash with a bundled skill. Review and scanning can miss a malicious skill.

### Search

Search runs through a relay in Isonomic Cloud. It cannot start before the hosted service has a decision record and a provider agreement is signed.

| Party | Sees | Does not see |
| --- | --- | --- |
| The network | That the device talks to Isonomic Cloud | The query or the results |
| Anud, running the relay | Counts and timing | The query, the results, or which account asked, while the attestation holds |
| The search provider | The query text, coming from Isonomic | Who asked, their address or account, or their queries as a linked set |
| Sites in the results | Nothing until the device opens one; then its address | |

**What holds**

- **The relay forwards only the query, a count and a freshness value.** (Tool kits: architecture v0.3, "Isonomic Search"; tests: scrubber property test, the address-forwarding canary.)
- **No query is kept.** (Tool kits: architecture v0.3, "Isonomic Search"; test: planted words.)
- **A relay with the wrong measurement gets no query.** (Tool kits: architecture v0.3, "Isonomic Search"; test: attestation vectors.)
- **Under the three-flag rule, each search asks and shows the query.** (Tool kits: architecture v0.3, "Isonomic Search"; test: kits-web fixture.)

**What is not claimed.** This is a private relay, not private search: the provider reads every query. Three limits remain. The query text can itself identify the user. The provider's promise not to keep queries is a contract Isonomic cannot verify. Attestation proves only what it measures, which waits on the hosted service's record. Results can carry instructions; see case search-result-instruction.

### Computer control

Only inside a Cell, on the Cell's own display. Actions on the user's real desktop stay outside this model. (Tool kits: architecture v0.3, "Decisions"; test: screen_read and screen_act are never offered.)

## Open items

- View confinement mechanism and tests: not yet covered by a citable decision record.
- Whether tool results and web content are "data, never authority" in general, beyond memory: no decision record states it for the whole runtime. The agent-control evaluation tests it as a control (content-is-data).
- The untrusted-session flag beyond memory writes: specified for Cell tools as the Rule of Two (Cells 7), at Cells Milestone 4. Tool kits: architecture v0.3 applies the three-flag rule to every kit tool on both backends.
- What an ask means in a Cell session with nobody present: left to the Cells record.
- Search: waits on a decision record for the hosted service and on a signed provider agreement.
