# Commitments

Every commitment below names the decision record in the product repositories that supports it. A commitment is **shipped** when it is in a released build, **in the build** when it is being built into the product now, and **planned** only with a date. Nothing without a date is listed as planned.

Isonomic Desktop has not been released, so nothing is shipped yet.

## Shipped

Nothing yet.

| Commitment | Source |
| --- | --- |

## In the build

| Commitment | Source |
| --- | --- |
| Models run on the user's machine by default. Remote providers are opt-in, their keys sit in the vault, every remote call is logged, and one switch cuts all remote streams. | Desktop 9 |
| Model files are downloaded against a manifest that pins each file's SHA-256 checksum. | Inference record, model library |
| The local model server listens on loopback only, on a fresh port with a random 32-byte secret at every launch. | Inference 2 |
| Child processes get an environment built from an allow-list; nothing is inherited from the user's shell. | Desktop 7; Inference 3 |
| Every agent session runs in the engine. No app runs an agent loop. | Agent stack 1 |
| An approval decision needs a token tied to a user action. An ambiguous answer is a denial. A first-party app decides only for its own sessions; Views and third-party code can request approvals but never decide them. | Agent stack 13 |
| A panic or hang in one session never affects the engine or other sessions. | Agent stack 15 |
| Session logs are an append-only table in the engine's database. | Agent stack 11 |
| Mechanical confinement is claimed only for Views and Cells. First-party apps are trusted code with least-privilege tokens. | Desktop 12 |

## Decided, release not yet dated

These are decided in a record but not yet built, and the record leaves their release to the launch plan. They move to **in the build** when work starts, and to **planned** only when they have a date.

Rows citing *Tool kits: architecture v0.3* come from that design, which is approved for building but is not yet a decision record in a product repository. Isonomic Search is not listed: it waits on a decision record for the hosted service.

| Commitment | Source |
| --- | --- |
| Memory is data, never authority: it cannot change policy, approvals, budgets, tools or presets. | Memory 2 |
| Evidence rule: a memory write or forget must point to at least two words of the user's own message text in that session. Attachments, tool results and app-attached context are not evidence. | Memory 12 |
| Untrusted-session flag: once a session has read untrusted content, a memory write asks first. An ambiguous answer is a denial. | Memory 16 |
| Every memory write is announced with Undo, and no app, preset or View can turn the notice off. | Memory 13 |
| Memory is never sent to a remote model unless the user turns that on. | Memory 10 |
| Secrets, payment and ID numbers, raw tool or web content, and the model's inferences about the user are never stored as memory. | Memory 14 |
| Agents reach only folders the user links in the window. The Capsule directory, key stores, other apps' data, system folders and the whole home folder can never be linked. | Tool kits: architecture v0.3, Trust boundary |
| The model cannot cause a permission question. A tool with no grant is not offered. | Tool kits: architecture v0.3, Who can ask the person for a permission |
| File contents count as untrusted content. | Tool kits: architecture v0.3, Decisions |
| Every file change an agent makes can be undone, or it asks every time. | Tool kits: architecture v0.3, Files |
| Once a session has read private data and untrusted content, every outward call asks. Nothing is remembered for code, for such a call, or for a change that cannot be undone. | Tool kits: architecture v0.3, How a call is decided; Standing permissions |
| On the host, every command asks, shows the whole command and is never remembered. It runs only inside the system sandbox, which closes the engine's socket; if the sandbox cannot be applied, it does not run. Interactive sessions only. | Tool kits: architecture v0.3, Decisions; Shell and processes |
| Nothing leaves the device for web access until the user answers one question per app. Every web call is logged. | Tool kits: architecture v0.3, Decisions |
| The browser uses a throwaway profile with none of the user's cookies or sign-ins. An action on a site asks until the user allows that site for the session, and never for good. | Tool kits: architecture v0.3, Decisions; Browser |
| Computer control runs only inside a Cell, never on the user's real desktop. | Tool kits: architecture v0.3, Decisions |
| A skill is data, never authority. Its scripts run as commands, with the same approvals. Only a person installs or enables a skill. There is no public skill marketplace. | Tool kits: architecture v0.3, Decisions; Isonomic Skills |

## Planned

Nothing is planned until it has a date.

| Commitment | Date | Source |
| --- | --- | --- |

## What we do not claim

- **We do not make models aligned.** The controls bound what an agent can touch and do. They do not change what a model wants or says.
- **Prompt injection is not solved.** A model can still be persuaded by text it reads. The controls limit what that achieves.
- **First-party apps are not sandboxed.** They are trusted code with least-privilege tokens.
- **A compromised device is out of scope.** Malware running as the user can do what the user can do.
- **The hypervisor is trusted** for Cells, and actions on the user's real desktop are outside the Cell model.
- **Text the user pastes counts as the user's own words** for the memory evidence rule.
- **A host command is not contained.** The sandbox stops writes outside granted folders, closes the engine's socket and limits the network. It does not stop a command from reading most of what the user can read.
- **Web access sends data off the device.** Once the user turns it on, addresses and search text leave the device. Models stay local.
- **Search is a private relay, not private search.** The provider reads every query, the query text can identify the user, and the provider's promise not to keep queries is a contract Isonomic cannot verify.
- **Untested is not proven.** Until the agent-control evaluation publishes results, the controls above are designs and code, not measured protection.
