use ascii::AsciiString;

#[derive(Debug, PartialEq)]
pub enum Object {
    None,
    Str(AsciiString)
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Str(s) => f.write_fmt(format_args!("'{}'", s)),
            _ => Ok(())
        }
    }
}

impl Object {
    pub fn new() -> Self {
        Self::None
    }

    pub fn is_none(&self) -> bool {
        *self == Object::None
    }

    pub fn is_some(&self) -> bool {
        *self != Object::None
    }
}