


#[derive(Debug, Copy,Clone, PartialEq, Eq, Hash)]
pub enum TokenKind{
    FLOAT,
    INTEGER,
    PLUS,
    STAR,
    EQUAL,
    MODULO,
    SLASH,
    DASH,
    AMPER,

    GREATER,
    LESS,
    NEGATION,
    VERTICAL_BAR,

    SEMICOLON,

    IF,
    ELSE,
    FOR,
    WHILE,
    FUNCTION,
    RETURN,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    COMMA,
    COLON,
    IDENTIFIER,
    LITERALSTRING,
}
