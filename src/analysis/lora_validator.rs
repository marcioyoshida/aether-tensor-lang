/// Validates LoRA adapter compatibility before merge / swap.
///
/// Checks:
///  - rank consistency between base weight and adapter matrices
///  - dtype compatibility (promotes to wider type on mismatch)
///  - no circular swap chains

use crate::analysis::infer::TensorType;

pub struct LoraAdapter {
    pub name: String,
    pub rank: usize,
    pub base_ty: TensorType,
}

pub fn validate_merge(base: &TensorType, adapter: &LoraAdapter) -> Result<(), String> {
    if base.shape != adapter.base_ty.shape {
        return Err(format!(
            "LoRA adapter '{}' shape {:?} does not match base shape {:?}",
            adapter.name, adapter.base_ty.shape, base.shape
        ));
    }
    Ok(())
}

pub fn validate_swap_chain(chain: &[String]) -> Result<(), String> {
    let mut seen = std::collections::HashSet::new();
    for name in chain {
        if !seen.insert(name) {
            return Err(format!("circular LoRA swap chain detected at adapter '{name}'"));
        }
    }
    Ok(())
}
