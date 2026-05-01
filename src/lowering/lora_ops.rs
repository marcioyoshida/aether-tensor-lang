/// Lowers LoRA operations to the AetherOps MLIR dialect.
///
/// `base + lora`  → aether.lora_merge
/// `swap(id, a)`  → aether.lora_swap
/// `merge(a, b)`  → aether.lora_merge_inplace

use crate::analysis::infer::TypedExpr;
use crate::mlir::context::MlirModule;
use crate::parser::ast::Expr;

pub fn lower_lora(module: &mut MlirModule, te: TypedExpr) {
    match &te.expr {
        Expr::LoraMerge(base, adapter) => {
            let base_id = expr_id(base);
            let adapter_id = expr_id(adapter);
            module.push_op(format!(
                "%merged = aether.lora_merge(%{base_id}, %{adapter_id})"
            ));
        }
        Expr::LoraSwap(target, adapter) => {
            let adapter_id = expr_id(adapter);
            module.push_op(format!(
                "aether.lora_swap @{} with %{adapter_id}",
                target.0
            ));
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
