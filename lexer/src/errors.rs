use owo_colors::OwoColorize;
use std::fmt;

#[derive(Debug)]
pub enum LexerError<'a> {
    // invalid character, line, position (col_start, col end)
    InvalidCharacter {
        context: &'a str,
        filename: &'a str,
        character: char,
        line: usize,
        col: usize,
    },

    // line, position
    UnexpectedEOF {
        context: &'a str,
        filename: &'a str,
        line: usize,
        col: usize,
    },

    UnterminatedString {
        context: &'a str,
        filename: &'a str,
        line: usize,
        col: usize,
    },
}

impl<'a> fmt::Display for LexerError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCharacter {
                context,
                filename,
                character,
                line,
                col,
            } => {
                writeln!(f, "{} {}", "invalid character".red(), character)?;
                writeln!(f, "{}:{}:{}", filename.blue(), line.green(), col.white())?;
                writeln!(f, "|\n{:>3} | {}", line.green(), context)?;
                writeln!(
                    f,
                    "| {:>width$}^ {}",
                    "",
                    "invalid character".red(),
                    width = *col + 3
                )
            }

            Self::UnexpectedEOF {
                context,
                filename,
                line,
                col,
            } => {
                writeln!(f, "{}", "unexpected EOF".red())?;
                writeln!(f, "--> {}:{}:{}", filename, line, col)?;
                writeln!(f, "  |\n{:>3} | {}", line.green(), context)?;
                writeln!(
                    f,
                    "  | {:>width$}^ {}",
                    "",
                    "unexpected EOF".red(),
                    width = *col
                )
            }

            Self::UnterminatedString {
                context,
                filename,
                line,
                col,
            } => {
                writeln!(f, "{}", "Unterminated String literal".red())?;
                writeln!(f, " --> {}:{}:{}", filename.blue(), line.green(), col)?;
                writeln!(f, "  |\n{:>3} | {}", line.green(), context)?;
                writeln!(
                    f,
                    "  | {:>width$}^ {}",
                    "",
                    "unterminated string".red(),
                    width = *col
                )
            }
        }
    }
}
