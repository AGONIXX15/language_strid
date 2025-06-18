use std::fmt;

#[derive(Debug)]
pub enum LexerError {
    // invalid character, line, position (col_start, col end)
    InvalidCharacter(char, usize, usize),

    // line, position
    UnexpectedEOF(usize,usize),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCharacter(c,line,col) => {
                write!(f,"invalid character '{}' at line {}, column {}",c,line,col)
            }

            Self::UnexpectedEOF(line,pos ) => {
                write!(f,"unexpected EOF at line {}, position {}", line, pos)
            }
        }
    }
}
