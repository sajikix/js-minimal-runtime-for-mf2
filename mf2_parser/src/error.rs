#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxError {
    EmptyToken(Error),
    BadEscape(Error),
    BadInputExpression(Error),
    DuplicateAttribute(Error),
    DuplicateOptionName(Error),
    ExtraContent(Error),
    ParseError(Error),
    MissingSyntax(Error),
    InvalidCharacter(Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    start: usize,
    end: usize,
    expected: String,
}

impl Error {
    pub fn new(start: usize, end: Option<usize>, expected: Option<String>) -> Self {
        Self {
            start,
            end: end.unwrap_or(start + 1),
            expected: expected.unwrap_or_else(|| String::new()),
        }
    }
}
