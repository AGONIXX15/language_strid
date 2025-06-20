use std::env;
use std::fs::File;
use std::io::Read;

use lexer::errors::LexerError;
use lexer::token::Token;

use parser::parser::engine::Parser;
use parser::parser::lookups::BindingPower;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return;
    }

    // try to open a File in that path
    let mut file: File = match File::open(&args[1]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("the path doesnt exists: {e}");
            return;
        }
    };
    // buffer to read the file
    let mut buffer: String = String::new();
    if let Err(e) = file.read_to_string(&mut buffer) {
        eprintln!("Error al leer el archivo: {e}");
    }

    // buffer ready to use

    let tokens: Vec<Token> = match lexer::tokenize(&buffer, &args[1]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Lexer error: {}", e);
            return;
        }
    };

    let mut parser: Parser = Parser::new(&tokens, 0);
    let v = Parser::parse_expr(&mut parser, BindingPower::Primary)
        .map_err(|e| {
            eprintln!("Parser error: {}", e);
        })
        .ok();
    for expr in v {
        println!("{:#?}", expr);
    }
}
