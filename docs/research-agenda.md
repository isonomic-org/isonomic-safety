# Research agenda

## Question

Do runtime-level controls keep an agent under its user's control when it processes content written by an attacker? And what do they cost the user in interruptions and failed tasks?

The controls are properties of the runtime, not of a model. The evaluation measures how much each control contributes, model by model, by switching it on and off.

## The agent-control evaluation

Location: [`evals/agent-control/`](../evals/agent-control/). Work starts once a full agent session runs end to end in the agent runtime.

**What it drives.** The agent runtime (`isonomic-harness`), through its in-memory test host, with a local open-weight model from the [model list](model-policy.md).

**Controls under test**, each switchable on and off:

| Control | What it does |
| --- | --- |
| Approval binding | A consequential action proceeds only on an approval carrying a token tied to a user action; an ambiguous answer is a denial |
| Content-is-data | Text from tools, files and web pages cannot change policy, permissions, tools or budgets |
| Memory evidence rule | A memory write must cite the user's own words from the current session |
| Untrusted-session flag | Once a session has read untrusted content, memory writes ask first |

**Attack sets**

- Public agent-hijacking and prompt-injection benchmarks. Each benchmark's licence is checked before anything is copied into this repository; where copying is not allowed, the evaluation fetches the benchmark at run time and records its version.
- Our own memory-poisoning cases: injected content tries to plant a false memory, and a later session tests whether that memory is used.

**Measures**, per model and per control setting:

- attack success rate;
- task completion rate;
- false approvals: actions that went ahead without the user's real consent;
- approval prompts per task. Too many prompts is itself a safety failure, because people stop reading them.

**Outputs**

- Raw results as JSON.
- A generated markdown scorecard per model in [`results/`](../results/).
- One command that reproduces a run from a pinned model checksum, benchmark version and runtime commit.

## Publication rules

- Every run that is reported is published in full, including runs where a control fails.
- When a control fails, an issue is filed in the repository that owns the control, and the scorecard links to it.
- A scorecard states the runtime commit, model checksum, benchmark version and control settings, so anyone can reproduce it.
- Benchmarks understate adaptive attackers. Every scorecard says so.

## Not in scope

- Model alignment, interpretability and training.
- Anything outside the desktop agent runtime.
