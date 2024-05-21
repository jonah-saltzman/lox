use super::object::Object;
use ascii::AsciiStr;

#[derive(Debug)]
pub enum TokenType {
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
    Identifier,
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
    kind: TokenType,
    lexeme: &'a AsciiStr,
    object: Option<Object>,
    line: usize,
    start: usize,
    end: usize,
}

impl<'a> Token<'a> {
    pub fn new(
        kind: TokenType,
        lexeme: &'a AsciiStr,
        object: Option<Object>,
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

    pub fn into_object(self) -> Option<Object> {
        self.object
    }
}
