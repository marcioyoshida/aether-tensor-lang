/// AetherOps dialect definition.
///
/// Ops registered here are lowered to LLVM IR by the standard
/// MLIR lowering pipeline once the AetherOps passes have run.

pub const DIALECT_NAMESPACE: &str = "aether";

/// All ops in the AetherOps dialect.
pub enum AetherOp {
    /// Tensor declaration with optional initializer.
    TensorDecl,
    /// Matrix multiply (alias for linalg.matmul for shape tracking).
    MatMul,
    /// LoRA adapter merge: base + A·B^T * scale.
    LoraMerge,
    /// Hot-swap a LoRA adapter on a named model handle.
    LoraSwap,
    /// Runtime PDF ingestion → dense tensor.
    PdfLoad,
    /// Model inference call dispatched to the runtime engine.
    Infer,
    /// Ethics score gate (NOP in release builds, asserts in debug).
    EthicsScore,
}

impl AetherOp {
    pub fn mnemonic(&self) -> &'static str {
        match self {
            AetherOp::TensorDecl  => "aether.tensor.decl",
            AetherOp::MatMul      => "aether.matmul",
            AetherOp::LoraMerge   => "aether.lora_merge",
            AetherOp::LoraSwap    => "aether.lora_swap",
            AetherOp::PdfLoad     => "aether.pdf_load",
            AetherOp::Infer       => "aether.infer",
            AetherOp::EthicsScore => "aether.ethics_score",
        }
    }
}
