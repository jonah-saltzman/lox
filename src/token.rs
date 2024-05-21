use super::object::Object;
use ascii::AsciiStr;

#[derive(Debug, Clone, Copy)]
pub enum TokenType<'a> {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(&'a AsciiStr),
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
pub struct Token<'a> {
    kind: TokenType<'a>,
    lexeme: &'a AsciiStr,
    object: Object,
    line: usize,
    start: usize,
    end: usize,
}

impl<'a> Token<'a> {
    pub fn new(
        kind: TokenType<'a>,
        lexeme: &'a AsciiStr,
        object: Object,
        line: usize,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            kind,
            lexeme,
            object,
            line,
            start,
            end,
        }
    }

    pub fn into_object(self) -> Object {
        self.object
    }
}
