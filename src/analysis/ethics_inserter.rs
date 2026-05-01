/// Automatic ethics-score insertion pass.
///
/// Walks the typed AST and wraps `infer()` call sites that touch
/// user-facing output with an `ethics_score()` guard when the
/// source did not already include one.

use crate::analysis::infer::TypedExpr;
use crate::parser::ast::*;

pub fn insert_ethics_guards(exprs: Vec<TypedExpr>) -> Vec<TypedExpr> {
    exprs.into_iter().map(wrap_if_needed).collect()
}

fn wrap_if_needed(te: TypedExpr) -> TypedExpr {
    match &te.expr {
        Expr::InferCall(_) => {
            // Wrap the infer call in an EthicsScore node
            let guarded = Expr::EthicsScore(EthicsScore {
                target: Box::new(te.expr.clone()),
            });
            TypedExpr { expr: guarded, ty: te.ty }
        }
        _ => te,
    }
}
