#[allow(dead_code, unused_variables, unreachable_patterns)]

#[allow(dead_code, unused_variables, unreachable_patterns)]
mod token;
#[allow(dead_code, unused_variables, unreachable_patterns)]
mod lexer;
#[allow(dead_code, unused_variables, unreachable_patterns)]
mod parser;

use std::fs::File;
use std::io::Read;
use lexer::Lexer;
use parser::Parser;
use token::print_tokens;

fn load_file(file_path: &str) -> Result<String, String> {
    let mut file = File::open(file_path).map_err(|err| {
        format!("ERROR: Could not open '{file_path}'. {err}")
    })?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|err| {
        format!("ERROR: Could not read '{file_path}' to string. {err}")
    })?;

    Ok(contents)
}

fn main() {
    let file_path = "test.wtr";
    
    let file_contents = match load_file(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let mut lexer = Lexer::new(file_contents);
    let tokens = match lexer.get_tokens() {
        Ok(tokens_result) => {
            println!("Successful lex");
            print_tokens(&tokens_result);
            tokens_result
        }
        Err(err) => {
            eprintln!("ERROR: Could not lex tokens. {err}");
            return;
        }
    };


    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(program_result) => {
            println!("Successful parse");
            program_result
        }
        Err(err) => {
            eprintln!("ERROR: {err}");
            return;
        }
    };

    
}
