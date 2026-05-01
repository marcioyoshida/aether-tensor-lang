/// Hand-written lexer; feeds tokens to the LALRPOP-generated parser.

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'input> {
    // Literals
    IntLit(i64),
    FloatLit(f64),
    StringLit(&'input str),
    Ident(&'input str),

    // Keywords
    Tensor,
    Lora,
    Infer,
    Pdf,
    Reason,
    Ethics,
    Parallel,
    Embed,
    Swap,
    Merge,

    // Symbols
    At,       // @  (matmul)
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
    Arrow,    // ->
    Colon,
    Comma,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    // Dtypes
    F32,
    F16,
    BF16,
    I32,
}

pub struct Lexer<'input> {
    input: &'input str,
    pos: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self { input, pos: 0 }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token<'input>, usize), String>;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip whitespace
        while self.pos < self.input.len()
            && self.input.as_bytes()[self.pos].is_ascii_whitespace()
        {
            self.pos += 1;
        }
        if self.pos >= self.input.len() {
            return None;
        }
        // TODO: lex actual tokens
        Some(Err(format!("unexpected char at {}", self.pos)))
    }
}
