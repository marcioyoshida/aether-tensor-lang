# Contributing to AetherLang

**Thank you for considering contributing to AetherLang!**  
We are building a **tensor-first compiled language** that eliminates Python boilerplate and fuses AI workflows (tensors, LoRAs, PDFs, specialist orchestration, ethics tuning, and one-line training) straight to metal via MLIR.

This document explains how you can help make AetherLang a reality.

---

## Code of Conduct

By participating in this project, you agree to follow our [Code of Conduct](CODE_OF_CONDUCT.md).  
We expect a welcoming, respectful, and inclusive environment for everyone — regardless of experience level, background, or identity.

---

## Ways to Contribute

You don’t need to be a compiler expert to help! Here are the main ways to contribute:

### 1. **Report Issues & Suggest Features**
- Open an issue for:
  - Bugs in the syntax specification
  - Missing capabilities
  - Ideas for new primitives (e.g., “add native vision encoder syntax”)
  - Real-world pain points from PyTorch/vLLM workflows
- Use the provided issue templates for **Bug Report**, **Feature Request**, or **Syntax Proposal**.

### 2. **Improve Documentation & Examples**
- Fix typos or clarify sections in `README.md`, `CAPABILITIES.md`, or `ROADMAP.md`
- Add or improve example scripts in the `examples/` folder
- Write tutorials or blog-post-style guides in `docs/`

### 3. **Design & Specification Work (Phase 0 & 1)**
- Review and comment on syntax proposals
- Help refine the CAPABILITIES list
- Propose new operations that fit the “one-file, one-compile, zero-layers” philosophy

### 4. **Code Contributions (starting Phase 1)**
- Rust parser & AST
- MLIR codegen passes
- CUDA backend kernels
- CLI (`aether build`, `aether run`, `aether debug`)
- VS Code syntax highlighter
- Test suite and CI

**Good first issues** will be labeled `good first issue` and `help wanted