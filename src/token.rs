pub enum Keyword {
   Return,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Type {
   Integer64,
   String
}

pub enum Literal {
   Number(i64),
   String(String)
}

#[derive(Copy, Clone)]
pub enum Operator {
   Plus, Minus, Multiply, Divide
}

fn operator_precedence(operator: &Operator) -> usize {
   match operator {
      Operator::Plus => return 0,
      Operator::Minus => return 0,
      Operator::Multiply => return 1,
      Operator::Divide => return 1,
   }
}

pub fn compare_operators(operator_1: &Operator, operator_2: &Operator) -> bool {
   let precedence_1 = operator_precedence(operator_1);
   let precedence_2 = operator_precedence(operator_2);
   return precedence_1 > precedence_2;
}

pub enum Token {
   Keyword(Keyword),
   Type(Type),
   Literal(Literal),
   Equals,
   Semicolon,
   Identifier(String),
   Operator(Operator)
}

pub trait Printable {
   fn print(&self);
}

impl Printable for Token {
   fn print(&self) {
      match &self {
         Token::Keyword(keyword) => {
            let keyword_str = match keyword {
               Keyword::Return => "Return",
            };
            println!("Token: Keyword [{}]", keyword_str);
         }
         Token::Type(type_type) => {
            let type_str = match type_type {
               Type::Integer64 => "Int64",
               Type::String => "String",
            };
            println!("Token: Type [{type_str}]")
         }
         Token::Literal(literal) => {
            let literal_str = match literal {
               Literal::Number(num) => format!("Number: {}", num.to_string()),
               Literal::String(str) => format!("String: '{}'", str) ,
            };
            println!("Token: Literal [{literal_str}]");
         }
         Token::Equals => println!("Token: Equals"),
         Token::Semicolon => println!("Token: Semicolon"),
         Token::Identifier(variable) => println!("Token: Identifier [{variable}]"),
         Token::Operator(operator) => operator.print(),
         _ => println!("Token: unknown")
       }
   }
}

impl Printable for Operator {
   fn print(&self) {
      let operator_str = match &self {
         Operator::Plus => String::from("Plus"),
         Operator::Minus => String::from("Minus"),
         Operator::Multiply => String::from("Multiply"),
         Operator::Divide => String::from("Divide"),
         _ => String::from("unknown")
      };
      println!("Token: Operator [{operator_str}]");
   }
}

pub fn print_tokens(tokens: &Vec<Token>) {
   for token in tokens.iter() {
       token.print();
   }
}