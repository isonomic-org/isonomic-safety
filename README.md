# isonomic-safety

The public record of Isonomic's safety work: what the system protects, against whom, how each protection is enforced, and tests that show whether it holds.

Isonomic is an open-source desktop app for running AI models and agents on your own computer. Its product code lives in other repositories. This repository holds the safety material those repositories are measured against.

## Ground rule

Everything here must be useful to someone who does not use Isonomic. The threat model, the model admission bar and the evaluation are written so that another agent runtime can reuse them. Product code stays in the product repositories.

## Layout

| Path | What it holds |
| --- | --- |
| [docs/threat-model.md](docs/threat-model.md) | What is protected, the adversaries, and for each one what holds, by which mechanism, and how it is tested |
| [docs/commitments.md](docs/commitments.md) | Each safety commitment, marked shipped, in the build, or planned with a date, plus what we do not claim |
| [docs/model-policy.md](docs/model-policy.md) | The bar a model must clear before Isonomic ships it, and the current model list |
| [docs/research-agenda.md](docs/research-agenda.md) | What the agent-control evaluation tests and what gets published |
| `evals/agent-control/` | The agent-control evaluation (not yet written; see the research agenda) |
| `results/` | Generated scorecards, one per model |
| `tools/claims-check/` | A check, run in CI, that every planned commitment carries a date |
| [SECURITY.md](SECURITY.md) | How to report a vulnerability privately |

## How claims are made

A commitment appears in `docs/commitments.md` only if a decision record in the product repositories supports it. Each one names its source. Anything planned carries a date; anything without a date is left out. Negative results are published alongside positive ones.

## Checks

```sh
cargo test --workspace
cargo run -p claims-check -- docs/commitments.md
```

## Licence

Apache-2.0. See [LICENSE](LICENSE) and [NOTICE](NOTICE). Copyright 2026 Anud Inc.
