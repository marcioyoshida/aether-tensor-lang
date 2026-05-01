# AetherLang Ω

**Tensor-first. Compiled. Zero layers.**

A purpose-built language where tensors, LoRAs, weights, PDFs, and specialist chaining are **syntax primitives**. The compiler fuses everything straight to CUDA/TPU via MLIR — no Python, no PyTorch, no boilerplate.

## Capabilities (see full list in [CAPABILITIES.md](CAPABILITIES.md))

- Native tensors with compile-time shape checking
- LoRA merge/unmerge/swap as operators (`model = base + lora`)
- One-line training, inference, and PDF → embedding pipelines
- Specialist orchestration with zero-copy hidden-state chaining
- Built-in ethics scoring and on-the-fly weight tuning

## Quick Start (coming soon)

```aether
lora = base.new_lora().train(data, epochs=3)
model = base + lora
answer = pdf.load("paper.pdf").chunk().embed().search(query).infer()