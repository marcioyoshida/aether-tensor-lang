# AetherLang Capabilities

**Complete Operation List** — Version 1.0 (May 2026)

AetherLang is a **tensor-native, compiled language** where **every AI operation is a first-class primitive**.  
No Python boilerplate. No framework layers. Full compile-time fusion to CUDA/TPU kernels via MLIR.  
Everything below is either a single keyword/expression or a one-line declaration.

---

### 1. Core Tensor Operations (native, shape-aware, zero boilerplate)
- `Tensor<dtype, [shape]> = load(...)` — declare + load with compile-time shape inference
- `@`, `.*`, `+`, `-`, `/`, `^` — fused matmul, elementwise ops, broadcasting
- `softmax`, `ReLU`, `SwiGLU`, `RMSNorm`, `RoPE` — built-in attention primitives
- `parallel_for [dims] { ... }` — GPU/TPU parallel loops (auto grid/block)
- `@shared`, `@register`, `@device(N)` — explicit memory spaces
- `async_copy`, `fence_async`, `warp_reduce_*` — CUDA async + warp primitives as functions
- `quantize(Int4/Int8/FP8, group_size)` — in-place or out-of-place quantization
- `clamp`, `broadcast`, `reshape`, `permute`, `concat` — tensor surgery
- `embed(vocab, rotary=RoPE, norm=RMSNorm)` — fused token → embedding pipeline

### 2. LoRA & Adapter Operations (syntax primitives)
- `base.new_lora(rank, alpha, target=["*.q_proj", ...])` — create LoRA
- `model = base + lora` / `model = base - lora` — zero-copy merge/unmerge
- `model.swap_adapter(lora)` — hot-swap in < 1 ms
- `save(lora, "file.lora")` / `load("file.lora")` — delta-only save/load
- `lora = lora + ethics_lora` — multi-adapter composition
- `model.swap_adapter([lora1, lora2])` — simultaneous multi-LoRA activation

### 3. Weight & Model Management Operations
- `w = model.weights` — reference entire weight store
- `w["layer.name"] = w["layer.name"].quantize(...)` — targeted weight editing
- `w.checkout("version")` / `w.commit("name")` — git-style weight versioning
- `w.merge(delta)` / `w.patch(safety_vector)` — surgical weight updates
- `model = Transformer(layers=32, d_model=8192, ...)` — declarative model definition
- `custom_block(t) = t |> attn() |> feedforward()` — custom block syntax

### 4. Embedding, Inference & Sampling Operations
- `tokens.embed(model, layer="embedding")` — fused embedding
- `model.infer(emb, max_new_tokens, sampler=TopP(0.95) + Temperature(0.8))` — single-call inference
- `sampler = TopK(50) + RepetitionPenalty(1.15) + EthicsBoost(0.4)` — composable samplers
- `output.hidden_states` — expose raw hidden states for chaining
- `flash_attn(q, k, v, causal=true)` — built-in optimized attention kernels
- `graph = capture { entire_pipeline }` — CUDA Graph capture

### 5. Data Ingestion & RAG Operations (PDF-first)
- `docs = pdf.load_dir("*.pdf")` or `pdf.load(url)` — native GPU-accelerated PDF parser
- `docs.extract(ocr=true, tables_as_json=true)` — text + table + image extraction
- `chunks = docs.chunk(strategy="semantic", max_tokens=512)` — smart chunking
- `embeddings = chunks.embed(model)` — fused embedding