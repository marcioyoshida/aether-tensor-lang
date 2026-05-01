pub mod ast;
pub mod lexer;

use ast::Expr;

pub fn parse(src: &str) -> Result<Vec<Expr>, String> {
    let _lexer = lexer::Lexer::new(src);
    // TODO: wire up LALRPOP-generated parser
    // grammar::ProgramParser::new().parse(lexer)
    todo!("wire up LALRPOP parser")
}
