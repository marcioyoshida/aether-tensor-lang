// ── Atoms ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum DType {
    F32,
    F16,
    BF16,
    I32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DimExpr {
    Lit(i64),
    Symbolic(Ident),
    Dynamic,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShapeExpr {
    pub dims: Vec<DimExpr>,
}

#[derive(Debug, Clone)]
pub enum BinOp {
    MatMul,
    Add,
    Sub,
    Mul,
}

// ── AST nodes ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TensorDecl {
    pub name: Ident,
    pub dtype: DType,
    pub shape: ShapeExpr,        // compile-time or symbolic
    pub init: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub struct InferCall {
    pub model: Ident,
    pub input: Box<Expr>,
    pub out_shape: ShapeExpr,
}

#[derive(Debug, Clone)]
pub struct PdfLoad {
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct ReasonChain {
    pub topic: Box<Expr>,
    pub steps: Vec<SpecialistStep>,   // e.g. GaN → Econ → Supply
}

#[derive(Debug, Clone)]
pub struct SpecialistStep {
    pub adapter: Ident,
    pub hidden_in: Option<Ident>,     // zero-copy hand-off
}

#[derive(Debug, Clone)]
pub struct EthicsScore {
    pub target: Box<Expr>,
}

// ── Top-level expression enum ─────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Expr {
    TensorDecl(TensorDecl),
    BinaryOp(Box<Expr>, BinOp, Box<Expr>),
    LoraMerge(Box<Expr>, Box<Expr>),           // base + lora
    LoraSwap(Ident, Box<Expr>),
    InferCall(InferCall),
    PdfLoad(PdfLoad),
    ReasonChain(ReasonChain),
    EthicsScore(EthicsScore),
}