use std::char;
use std::fs::File;
use std::io::Read;

fn load_file(file_path: &str) -> Result<String, String> {
    let file_result = File::open(file_path);
    match file_result {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    return Ok(format!("{contents}"));
                }
                Err(err) => {
                    return Err(format!("ERROR: Could not read '{}' to string. {}", file_path, err));
                }
            }
        }
        Err(err) => {
            return Err(format!("ERROR: Could not open '{}'. {}", file_path, err));
        }
    }
}

fn is_number(word: &str) -> bool {
    word.parse::<f64>().is_ok()
}

enum Keyword {
    Return,
    Integer
}

enum Token {
    Keyword(Keyword),
    Number(i64),
    Semicolon,
    Identifier(String),
}

trait Printable {
    fn print(&self);
}

impl Printable for Token {
    fn print(&self) {
        match self {
            Token::Keyword(keyword) => {
                let keyword_str = match keyword {
                    Keyword::Return => "Return",
                    Keyword::Integer => "Integer",
                };
                println!("Token: Keyword [{}]", keyword_str);
            }
            Token::Number(number) => println!("Token: Number [{}]", number),
            Token::Identifier(variable) => println!("Token: Identifier [{}]", variable),
            Token::Semicolon => println!("Token: Semicolon"),
        }
    }
}

struct Lexer {
    file_contents: String,
    tokens: Vec<Token>
}

impl Lexer {
    fn new(file_contents: String) -> Self {
        return Lexer{file_contents, tokens: Vec::new()};
    }

    fn create_char_token(&self, c: char) -> Result<Option<Token>, String> {
        if c == ';' {
            return Ok(Some(Token::Semicolon));
        }
        else {
            let mut error = String::from("Bad character: ");
            error.push(c);
            return Err(error);
        }
    }

    fn create_token(&self, word: &str) -> Result<Option<Token>, String> {
        if word == "return" {
            return Ok(Some(Token::Keyword(Keyword::Return)));
        }
        else if word == "int" {
            return Ok(Some(Token::Keyword(Keyword::Integer)));
        }
        else if is_number(word) {
            return Ok(Some(Token::Number(word.parse().unwrap())));
        }
        else if word.len() == 1 {
            return self.create_char_token(word.chars().next().unwrap());
        }
        else {
            return Ok(Some(Token::Identifier(String::from(word))));
        }
    }

    fn get_tokens(&mut self) -> Result<String, String> {
        let words: Vec<&str> = self.file_contents.split_whitespace().collect();
        for word in words {
            let mut sub_word = String::new();
            for c in word.chars() {
                if c == ';' || c == '(' || c == ')' {
                    match self.create_token(&sub_word) {
                        Ok(Some(token)) => self.tokens.push(token),
                        Ok(None) => {}
                        Err(err) => return Err(err),
                    }
                    match self.create_char_token(c) {
                        Ok(Some(token)) => self.tokens.push(token),
                        Ok(None) => {}
                        Err(err) => return Err(err),
                    }
                    sub_word.clear();
                    continue;
                }
                sub_word.push(c);
            }
            if !sub_word.is_empty() {
                match self.create_token(&sub_word) {
                    Ok(Some(token)) => self.tokens.push(token),
                    Ok(None) => {}
                    Err(err) => return Err(err),
                }
            }
        }
        Ok(String::from("Successful lex"))
    }

    fn print_tokens(&self) {
        for token in self.tokens.iter() {
            token.print();
        }
    }
}

fn main() {
    let file_path = "test.wtr";
    
    match load_file(file_path) {
        Ok(file_contents) => {
            let mut lexer = Lexer::new(file_contents);
            match lexer.get_tokens() {
                Ok(success) => {
                    println!("{success}");
                    lexer.print_tokens();
                }
                Err(err) => println!("ERROR: Could not lex tokens. {err}"),
            }
        }
        Err(err) => eprintln!("{err}"),
    }
}
