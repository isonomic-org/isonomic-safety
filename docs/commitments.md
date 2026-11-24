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

| Commitment | Source |
| --- | --- |
| Memory is data, never authority: it cannot change policy, approvals, budgets, tools or presets. | Memory 2 |
| Evidence rule: a memory write or forget must point to at least two words of the user's own message text in that session. Attachments, tool results and app-attached context are not evidence. | Memory 12 |
| Untrusted-session flag: once a session has read untrusted content, a memory write asks first. An ambiguous answer is a denial. | Memory 16 |
| Every memory write is announced with Undo, and no app, preset or View can turn the notice off. | Memory 13 |
| Memory is never sent to a remote model unless the user turns that on. | Memory 10 |
| Secrets, payment and ID numbers, raw tool or web content, and the model's inferences about the user are never stored as memory. | Memory 14 |

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
- **Untested is not proven.** Until the agent-control evaluation publishes results, the controls above are designs and code, not measured protection.
