


#[derive(Debug, Clone)]
pub enum TokenKind{
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
