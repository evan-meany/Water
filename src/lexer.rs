use crate::token::Keyword;
use crate::token::Type;
use crate::token::Literal;
use crate::token::Token;

fn is_number(word: &str) -> bool {
   word.parse::<f64>().is_ok()
}

pub struct Lexer {
   file_contents: String,
}

impl Lexer {
   pub fn new(file_contents: String) -> Self {
      return Lexer{file_contents: file_contents};
   }

   pub fn get_tokens(&mut self) -> Result<Vec<Token>, String> {
      let mut tokens: Vec<Token> = Vec::new();
      let words: Vec<&str> = self.file_contents.split_whitespace().collect();
      for word in words {
         let mut sub_word = String::new();
         for c in word.chars() {
            if c == ';' || c == '(' || c == ')' || c == '=' {
                  match self.create_token(&sub_word) {
                     Ok(Some(token)) => tokens.push(token),
                     Ok(None) => {}
                     Err(err) => return Err(err),
                  }
                  match self.create_char_token(c) {
                     Ok(Some(token)) => tokens.push(token),
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
                  Ok(Some(token)) => tokens.push(token),
                  Ok(None) => {}
                  Err(err) => return Err(err),
            }
         }
      }
      Ok(tokens)
   }

   fn create_char_token(&self, c: char) -> Result<Option<Token>, String> {
      if c == ';' {
          return Ok(Some(Token::Semicolon));
      }
      else if c == '=' {
         return Ok(Some(Token::Equals));
      }
      else {
         return Ok(Some(Token::Identifier(String::from(c))));
      }
  }

   fn create_token(&self, word: &str) -> Result<Option<Token>, String> {
      if word.len() == 0 {
         return Ok(None);
      }
      else if word == "return" {
          return Ok(Some(Token::Keyword(Keyword::Return)));
      }
      else if word == "Int64" {
          return Ok(Some(Token::Type(Type::Integer64)));
      }
      else if word == "String" {
         return Ok(Some(Token::Type(Type::String)));
      }
      else if is_number(word) {
          return Ok(Some(Token::Literal(Literal::Number(word.parse().unwrap()))));
      }
      else if word.len() == 1 {
          return self.create_char_token(word.chars().next().unwrap());
      }
      else if word.len() > 1 && word.chars().next().unwrap() == '"' && word.chars().last().unwrap() == '"' {
         return Ok(Some(Token::Literal(Literal::String(String::from("Word")))));
      }
      else {
          return Ok(Some(Token::Identifier(String::from(word))));
      }
  }   
}