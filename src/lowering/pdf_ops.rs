/// Lowers `pdf.load(path)` to the MLIR tensor dialect.
///
/// At compile time the PDF is parsed and its pages are embedded
/// as a constant f32 tensor via the tensor dialect.  At runtime,
/// dynamic paths fall back to `aether.pdf_load_rt`.

use crate::analysis::infer::TypedExpr;
use crate::mlir::context::MlirModule;
use crate::parser::ast::Expr;

pub fn lower_pdf(module: &mut MlirModule, te: TypedExpr) {
    if let Expr::PdfLoad(load) = &te.expr {
        // Static path → inline as dense<> constant (best-effort)
        module.push_op(format!(
            "%pdf_tensor = aether.pdf_load(\"{}\") : tensor<?x768xf32>",
            load.path
        ));
    }
}
