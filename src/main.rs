#[allow(dead_code, unused_variables, unreachable_patterns)]
mod token;
#[allow(dead_code, unused_variables, unreachable_patterns)]
mod lexer;
#[allow(dead_code, unused_variables, unreachable_patterns)]
mod parser;
#[allow(dead_code, unused_variables, unreachable_patterns)]
mod generator;

use std::fs::File;
use std::io::Read;
use lexer::Lexer;
use parser::Parser;
use generator::Generator;

#[allow(dead_code, unused_imports, unused_variables, unreachable_patterns)]
use token::print_tokens;
use parser::print_ast;

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

fn assemble_file(file_path: &str) {
    
}

fn main() {
    let input_path = "files\\test.wtr";
    let output_path = "files\\test.asm";
    
    let file_contents = match load_file(input_path) {
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
            // print_tokens(&tokens_result);
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
            print_ast(&program_result);
            program_result
        }
        Err(err) => {
            eprintln!("ERROR: {err}");
            return;
        }
    };

    let mut generator = Generator::new(program);
    match generator.generate(output_path) {
        Ok(_) => println!("Successful generate"),
        Err(err) => {
            eprintln!("ERROR: {err}");
            return;
        }
    }


}
