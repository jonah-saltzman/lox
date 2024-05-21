use super::object::Object;
use ascii::AsciiStr;
use derive_more::derive::Debug;

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
    #[debug(skip)]
    start: usize,
    #[debug(skip)]
    end: usize,
    col_start: usize,
    col_end: usize
}

impl<'a> Token<'a> {
    pub fn new(
        kind: TokenType<'a>,
        lexeme: &'a AsciiStr,
        object: Object,
        line: usize,
        start: usize,
        end: usize,
        col_start: usize,
        col_end: usize
    ) -> Self {
        Self {
            kind,
            lexeme,
            object,
            line,
            start,
            end,
            col_start,
            col_end
        }
    }

    pub fn into_object(self) -> Object {
        self.object
    }
}
