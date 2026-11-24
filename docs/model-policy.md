# Model admission policy

The bar a model must clear before Isonomic ships it in its model list. It applies to every model the desktop app offers to download and run locally.

## The bar

A model is admitted only if all of these hold:

1. **Open weights.** The weights are published and can be run on the user's own hardware.
2. **A licence that allows commercial use.** The licence is recorded with a link to its text.
3. **A known source.** The download URL is recorded, along with who published that file. Where the file is a conversion or quantization by someone other than the model's developer, both are named.
4. **A pinned checksum.** The SHA-256 of the exact file is pinned in the manifest, and the download is refused if it does not match.
5. **The developer's published safety evaluations, where they exist.** Linked from the model's entry. Their absence is recorded, not hidden.
6. **A published result from our agent-control evaluation.** A scorecard in [`results/`](../results/) for that model, at the version and quantization we ship.

A model that clears items 1 to 4 can be listed for testing. It is admitted only when items 5 and 6 are also met.

## Current model list

Taken from the model manifest in `isonomic-inference` (`crates/library/models.json`). Every entry meets items 1 to 4. **No model meets item 6 yet,** because the agent-control evaluation has not run. Item 5 has not yet been recorded for any entry.

### Chat models

| Model | Licence | File published by | SHA-256 (first 12) | Status |
| --- | --- | --- | --- | --- |
| Qwen3.5 0.8B, Q4_K_M | Apache-2.0 | unsloth | `bd258782e35f` | Listed |
| Qwen3.5 2B, Q4_K_M | Apache-2.0 | unsloth | `aaf42c8b7c3c` | Listed |
| Qwen3.5 4B, Q4_K_M (default chat model) | Apache-2.0 | ggml-org | `217002fa8008` | Listed |
| Qwen3.5 9B, Q4_K_M | Apache-2.0 | unsloth | `03b74727a860` | Listed |
| Phi-3 mini 4k instruct, Q4 | MIT | Microsoft | `8a83c7fb9049` | Listed |
| SmolLM3 3B, Q4_K_M | Apache-2.0 | ggml-org | `8334b850b7bd` | Listed |
| Gemma 4 E4B instruct, QAT Q4_0 | Apache-2.0 (as recorded in the manifest) | Google | `676c35070db6` | Listed |
| Qwen3.6 35B-A3B, UD-Q4_K_XL | Apache-2.0 | unsloth | `707a55a8a439` | Draft: hash not yet confirmed by a real download |

### Embedding models

| Model | Licence | File published by | SHA-256 (first 12) | Status |
| --- | --- | --- | --- | --- |
| Nomic Embed Text v1.5, Q4_K_M (default) | Apache-2.0 | Nomic AI | `d4e388894e09` | Listed |
| Qwen3 Embedding 0.6B, Q8_0 | Apache-2.0 | Qwen | `06507c7b4268` | Listed |

Embedding models do not act as agents, so item 6 does not apply to them.

### Excluded

Kimi is excluded: server-class, with a custom licence (Inference record, model library).

## Changing the list

A model is added or removed by a change to the manifest in `isonomic-inference` and a matching change here. A model whose agent-control scorecard shows a control failing is not removed for that reason alone: the failure is published, and an issue is filed against the control.
