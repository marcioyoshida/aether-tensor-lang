# AetherLang Roadmap

**Tensor-first compiled language for AI**  
Zero Python layers • LoRAs & weights as syntax • Direct MLIR → CUDA/TPU  

**Current Status**: Phase 0 (Vision & Specification) — **COMPLETE** (May 2026)

---

## Phase 0: Vision & Specification (Completed May 2026)
**Goal**: Solidify the language design and create a compelling open-source foundation.

**Milestones**:
- ✅ Full syntax specification (this repo)
- ✅ Complete CAPABILITIES.md (10 categories, 50+ primitives)
- ✅ 12+ working example scripts (`examples/*.aether`)
- ✅ README.md, ROADMAP.md, LICENSE, CONTRIBUTING.md
- ✅ GitHub repo structure and issue templates
- ✅ Living design history (conversation traces with Grok)

**Done**: The entire AI workflow (tensors, LoRAs, PDF RAG, specialist chaining, ethics tuning, one-line training) is now expressed as clean, readable syntax.

---

## Phase 1: Parser & Compiler Frontend (Q2–Q3 2026)
**Goal**: Turn `.aether` files into valid MLIR.

**Milestones**:
- Rust-based parser (using `lalrpop` or `tree-sitter`)
- Abstract Syntax Tree (AST) with full type/shape inference
- Static analysis: shape checking, LoRA compatibility, ethics guard insertion
- MLIR lowering for core tensor ops (`@`, `embed`, `parallel_for`, `infer`)
- First working compiler binary: `aether build model.aether → model.mlir`
- CI pipeline with syntax tests and example validation
- Documentation generator (`aether doc`)

**Target Release**: v0.1 “Alpha Syntax”  
**Success Metric**: All examples in `/examples` compile to valid MLIR (no runtime yet)

---

## Phase 2: Runtime & CUDA Backend (Q3–Q4 2026)
**Goal**: Execute AetherLang programs on real GPUs with zero layers.

**Milestones**:
- MLIR → PTX/CUDA codegen (via MLIR’s GPU dialect + custom passes)
- Built-in kernel library (FlashAttention, RMSNorm, RoPE, etc.)
- LoRA merge/unmerge/swap as native operations
- Inference runtime with composable samplers
- Weight versioning and surgical patching
- `aether run` CLI for single-binary execution

**Target Release**: v0.2 “Metal Alpha”  
**Success Metric**: End-to-end PDF → embedding → inference pipeline runs on H100 at > 200 tokens/s

---

## Phase 3: Advanced Primitives & Ecosystem (Q1 2027)
**Goal**: Deliver the full AetherLang vision.

**Milestones**:
- Specialist orchestration + zero-copy hidden-state chaining
- Native PDF parser + RAG index (`pdf.load`, `build_index`, `search`)
- One-line training loop with FSDP / ethics guard
- Ethics scoring and auto-tuning primitives
- Multi-GPU / multi-node support (NCCL collectives)
- VS Code extension + syntax highlighting
- Hugging Face / Safetensors import/export
- Official examples gallery + community showcase

**Target Release**: v0.5 “Beta”  
**Success Metric**: Reproduce all 10 capability categories from CAPABILITIES.md on hardware

---

## Phase 4: Production & Community (Q2 2027 onward)
**Goal**: Make AetherLang the default language for serious AI engineering.

**Milestones**:
- v1.0 Stable release
- Package manager (`aether pkg`)
- Cloud-native deployment (Kubernetes operator)
- Mojo / TensaLang interop layer
- Benchmark suite vs. PyTorch + vLLM
- Academic paper + conference talks
- Contributor guide + governance model

**Stretch Goals**:
- TPU / AMD / Apple Silicon backends
- Multimodal (vision + text) primitives
- Formal verification of shape safety
- Self-hosting compiler (AetherLang compiles itself)

---

## Timeline (Aspirational)

| Phase       | Target Quarter | Release Tag     |
|-------------|----------------|-----------------|
| 0 (Vision)  | May 2026       | —               |
| 1 (Parser)  | Q2–Q3 2026     | v0.1 Alpha      |
| 2 (Runtime) | Q3–Q4 2026     | v0.2 Metal      |
| 3 (Advanced)| Q1 2027        | v0.5 Beta       |
| 4 (Prod)    | Q2 2027+       | v1.0 Stable     |

---

## How to Contribute
1. Star the repo and join Discussions
2. Pick an issue labeled `good first issue` or `help wanted