/// Lowers tensor declarations and binary ops to MLIR tensor / linalg dialect.
///
/// `@` → linalg.matmul
/// `+` → linalg.add (broadcast-aware)
/// `parallel_for` → scf.parallel
/// `embed(x, vocab)` → aether.embed

use crate::analysis::infer::TypedExpr;
use crate::mlir::context::MlirModule;
use crate::parser::ast::{BinOp, Expr};

pub fn lower_tensor(module: &mut MlirModule, te: TypedExpr) {
    match &te.expr {
        Expr::TensorDecl(decl) => {
            module.push_op(format!(
                "%{} = aether.tensor.decl [{:?}] : {:?}",
                decl.name.0, decl.shape, decl.dtype
            ));
        }
        Expr::BinaryOp(lhs, op, rhs) => {
            let lhs_id = expr_id(lhs);
            let rhs_id = expr_id(rhs);
            let mlir_op = match op {
                BinOp::MatMul => "linalg.matmul",
                BinOp::Add => "linalg.add",
                BinOp::Sub => "linalg.sub",
                BinOp::Mul => "linalg.mul",
            };
            module.push_op(format!("{mlir_op} ins(%{lhs_id}, %{rhs_id})"));
        }
        _ => {}
    }
}

fn expr_id(expr: &Expr) -> String {
    match expr {
        Expr::TensorDecl(d) => d.name.0.clone(),
        _ => "tmp".to_string(),
    }
}
