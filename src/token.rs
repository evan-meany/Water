pub enum Keyword {
   Return,
}

pub enum Type {
   Integer64,
   String
}

pub enum Literal {
   Number(i64),
   String(String)
}

pub enum Operator {
   Plus, Minus, Multiply, Divide
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
      match self {
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
         Token::Operator(operator) => {
            let operator_str = match operator {
               Operator::Plus => String::from("Plus"),
               Operator::Minus => String::from("Minus"),
               Operator::Multiply => String::from("Multiply"),
               Operator::Divide => String::from("Divide"),
               _ => String::from("unknown")
            };
            println!("Token: Operator [{operator_str}]");
         }
         _ => println!("Token: unknown")
       }
   }
}

pub fn print_tokens(tokens: &Vec<Token>) {
   for token in tokens.iter() {
       token.print();
   }
}