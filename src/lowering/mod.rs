pub mod infer_to_mlir;
pub mod lora_ops;
pub mod pdf_ops;
pub mod tensor_ops;

use crate::analysis::infer::TypedExpr;
use crate::mlir::context::MlirModule;
use crate::parser::ast::Expr;

pub fn lower(exprs: Vec<TypedExpr>) -> MlirModule {
    let mut module = MlirModule::new();
    for te in exprs {
        lower_expr(&mut module, te);
    }
    module
}

fn lower_expr(module: &mut MlirModule, te: TypedExpr) {
    match te.expr {
        Expr::TensorDecl(_) | Expr::BinaryOp(_, _, _) => {
            tensor_ops::lower_tensor(module, te);
        }
        Expr::LoraMerge(_, _) | Expr::LoraSwap(_, _) => {
            lora_ops::lower_lora(module, te);
        }
        Expr::PdfLoad(_) => {
            pdf_ops::lower_pdf(module, te);
        }
        Expr::InferCall(_) => {
            infer_to_mlir::lower_infer(module, te);
        }
        _ => {
            // ReasonChain, EthicsScore, etc. — handled by analysis passes
        }
    }
}
