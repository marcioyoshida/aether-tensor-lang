/// Shape and type inference pass.
///
/// Walks the AST, resolves symbolic dims, and annotates every node
/// with a concrete (or partially-symbolic) `TensorType`.

use crate::parser::ast::*;

#[derive(Debug, Clone)]
pub struct TensorType {
    pub dtype: DType,
    pub shape: Vec<DimExpr>,
}

#[derive(Debug)]
pub struct TypedExpr {
    pub expr: Expr,
    pub ty: Option<TensorType>,
}

pub fn infer(ast: Vec<Expr>) -> Result<Vec<TypedExpr>, String> {
    let mut ctx = InferCtx::default();
    ast.into_iter().map(|e| ctx.infer_expr(e)).collect()
}

#[derive(Default)]
struct InferCtx {
    env: std::collections::HashMap<String, TensorType>,
}

impl InferCtx {
    fn infer_expr(&mut self, expr: Expr) -> Result<TypedExpr, String> {
        match &expr {
            Expr::TensorDecl(decl) => {
                let ty = TensorType {
                    dtype: decl.dtype.clone(),
                    shape: decl.shape.dims.clone(),
                };
                self.env.insert(decl.name.0.clone(), ty.clone());
                Ok(TypedExpr { expr, ty: Some(ty) })
            }
            Expr::BinaryOp(_, op, _) => {
                // TODO: propagate shapes through matmul / add
                let _ = op;
                Ok(TypedExpr { expr, ty: None })
            }
            Expr::InferCall(call) => {
                let ty = TensorType {
                    dtype: DType::F32,
                    shape: call.out_shape.dims.clone(),
                };
                Ok(TypedExpr { expr, ty: Some(ty) })
            }
            _ => Ok(TypedExpr { expr, ty: None }),
        }
    }
}
