/// Lowers `infer(model, input): [shape]` to an AetherOps dialect call.
///
/// The model identifier is resolved to a registered adapter path and
/// the call is emitted as `aether.infer` which the runtime links
/// against the loaded GGUF / SafeTensors weights.

use crate::analysis::infer::TypedExpr;
use crate::mlir::context::MlirModule;
use crate::parser::ast::Expr;

pub fn lower_infer(module: &mut MlirModule, te: TypedExpr) {
    if let Expr::InferCall(call) = &te.expr {
        let out_shape = call
            .out_shape
            .dims
            .iter()
            .map(|d| format!("{d:?}"))
            .collect::<Vec<_>>()
            .join(", ");
        module.push_op(format!(
            "%out = aether.infer @{model}(%input) : tensor<{out_shape}xf32>",
            model = call.model.0,
            out_shape = out_shape,
        ));
    }
}
