

#[derive(Debug, Clone, PartialEq)]
pub enum LookUpError {
    NotFound(String),
}

impl std::fmt::Display for LookUpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookUpError::NotFound(msg) => write!(f, "Lookup error: {}", msg),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    UnexpectedToken(String),
    MissingToken(String),
    InvalidExpression(String),
    TypeError(String),
    LookupError(LookUpError),
}

impl From<LookUpError> for ParserError {
    fn from(err: LookUpError) -> Self {
        ParserError::LookupError(err)
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken(msg) => write!(f, "Unexpected token: {}", msg),
            ParserError::MissingToken(msg) => write!(f, "Missing token: {}", msg),
            ParserError::InvalidExpression(msg) => write!(f, "Invalid expression: {}", msg),
            ParserError::TypeError(msg) => write!(f, "Type error: {}", msg),
            ParserError::LookupError(look_up_error) => {
                write!(f, "Lookup error: {}", look_up_error)
            }
        }
    }
}
