/// Wrapper around the MLIR context and pass manager.

pub struct MlirModule {
    ops: Vec<String>,
}

impl MlirModule {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }

    pub fn push_op(&mut self, op: String) {
        self.ops.push(op);
    }

    pub fn ops(&self) -> &[String] {
        &self.ops
    }
}

impl Default for MlirModule {
    fn default() -> Self {
        Self::new()
    }
}
