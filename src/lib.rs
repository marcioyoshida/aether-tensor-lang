pub mod analysis;
pub mod diagnostics;
pub mod lowering;
pub mod mlir;
pub mod parser;

use std::path::Path;

pub fn build(input: &Path, output: &Path) {
    let src = std::fs::read_to_string(input).expect("failed to read input");
    let ast = parser::parse(&src).unwrap_or_else(|e| {
        diagnostics::report_and_exit(e);
    });
    let typed = analysis::infer::infer(ast).unwrap_or_else(|e| {
        diagnostics::report_and_exit(e);
    });
    let mlir_module = lowering::lower(typed);
    mlir::emitter::emit(&mlir_module, output);
}

pub fn check(input: &Path) {
    let src = std::fs::read_to_string(input).expect("failed to read input");
    let ast = parser::parse(&src).unwrap_or_else(|e| {
        diagnostics::report_and_exit(e);
    });
    analysis::infer::infer(ast).unwrap_or_else(|e| {
        diagnostics::report_and_exit(e);
    });
    println!("ok");
}

pub fn doc(_input: &Path, _out_dir: &Path) {
    todo!("doc generation")
}
