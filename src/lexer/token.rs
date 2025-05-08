#[derive(Debug)]
pub enum Token {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Int,
    Void,
    Return,
    Integer(u64),
    Identifier(String),
    Negate,
    BitFlip,
    Decrement,
}
