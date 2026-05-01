/// Validates and plans specialist-chain execution order.
///
/// A specialist chain is a sequence of LoRA adapters applied in turn,
/// with optional zero-copy hidden-state hand-offs between steps.

use crate::parser::ast::{ReasonChain, SpecialistStep};

#[derive(Debug)]
pub struct ChainPlan {
    pub steps: Vec<PlannedStep>,
}

#[derive(Debug)]
pub struct PlannedStep {
    pub adapter: String,
    /// If Some, the previous step's KV cache is reused directly.
    pub reuse_hidden: Option<String>,
}

pub fn plan(chain: &ReasonChain) -> Result<ChainPlan, String> {
    validate_chain(&chain.steps)?;
    let steps = chain
        .steps
        .iter()
        .map(|s| PlannedStep {
            adapter: s.adapter.0.clone(),
            reuse_hidden: s.hidden_in.as_ref().map(|h| h.0.clone()),
        })
        .collect();
    Ok(ChainPlan { steps })
}

fn validate_chain(steps: &[SpecialistStep]) -> Result<(), String> {
    if steps.is_empty() {
        return Err("specialist chain must have at least one step".into());
    }
    // Ensure hand-off identifiers refer to a previous step's adapter
    let mut known: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for step in steps {
        if let Some(ref src) = step.hidden_in {
            if !known.contains(src.0.as_str()) {
                return Err(format!(
                    "hidden_in '{}' does not refer to a preceding adapter",
                    src.0
                ));
            }
        }
        known.insert(step.adapter.0.as_str());
    }
    Ok(())
}
