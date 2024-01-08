use crate::token::Literal;
use crate::token::Token;
use crate::token::Type;
use crate::token::Operator;
use crate::token::Keyword;
use crate::token::compare_operators;
use crate::token::Printable;

use std::collections::VecDeque;

enum LiteralOrExpression {
   Literal(Token),
   NodeExpression(Box<NodeExpression>)
}

pub struct NodeProgram {
   statements: Vec<NodeStatement>
}
impl NodeProgram {
   fn new() ->Self {
      return NodeProgram{statements: Vec::new()};
   }

   fn push(&mut self, statement: NodeStatement) {
      self.statements.push(statement);
   }

   fn print(&self, indentation: usize) {
      println!("{:indent$}Program", "", indent = indentation);
      for statement in &self.statements {
         match statement {
            NodeStatement::Assign(assign_node) => assign_node.print(indentation + 2),
            NodeStatement::Return(return_node) => return_node.print(indentation + 2)
         }
      }
   }
}

enum NodeStatement {
   Assign(NodeAssign),
   Return(NodeReturn)
}

struct NodeAssign {
   type_token: Token,
   identifier: Token,
   lit_or_expr: LiteralOrExpression
}
impl NodeAssign {
   fn new(type_token: Token, identifier: Token, lit_or_expr: LiteralOrExpression) -> Result<Self, String> {
      match &type_token {
         Token::Type(_) => {}
         _ => return Err(String::from("NodeAssign expects Token::Type")),
      }
      match &identifier {
         Token::Identifier(_) => {}
         _ => return Err(String::from("NodeAssign expects Token::Identifier")),
      } 
      match &lit_or_expr {
         LiteralOrExpression::Literal(literal_token) => {
            match literal_token {
               Token::Literal(_) => {}
               _ => return Err(String::from("NodeReturn expects LiteralOrExpression"))
            } 
         }
         LiteralOrExpression::NodeExpression(_) => {}
         _ => return Err(String::from("NodeReturn expects LiteralOrExpression"))
      }
      return Ok(NodeAssign { type_token: type_token, identifier: identifier, lit_or_expr: lit_or_expr})
   }

   fn print(&self, indentation: usize) {
      println!("{:indent$}Assign", "", indent = indentation);
      print!("{:indent$}", "", indent = indentation + 2);
      self.identifier.print();

      match &self.lit_or_expr {
         LiteralOrExpression::Literal(literal) => {
            print!("{:indent$}", "", indent = indentation + 2);
            literal.print();
         }
         LiteralOrExpression::NodeExpression(expression) => expression.print(indentation + 2)
      } 
   }
}

struct NodeReturn {
   lit_or_expr: LiteralOrExpression
}
impl NodeReturn {
   fn new(lit_or_expr: LiteralOrExpression) -> Result<Self, String> {
      match &lit_or_expr {
         LiteralOrExpression::Literal(literal_token) => {
            match literal_token {
               Token::Literal(_) => {}
               _ => return Err(String::from("NodeReturn expects LiteralOrExpression"))
            } 
         }
         LiteralOrExpression::NodeExpression(_) => {}
         _ => return Err(String::from("NodeReturn expects LiteralOrExpression"))
      }
      return Ok(NodeReturn{lit_or_expr: lit_or_expr})
   }

   fn print(&self, indentation: usize) {
      println!("{:indent$}Return", "", indent = indentation);
   
      match &self.lit_or_expr {
         LiteralOrExpression::Literal(literal) => { 
            print!("{:indent$}", "", indent = indentation + 2);
            literal.print();
         }
         LiteralOrExpression::NodeExpression(expression) => expression.print(indentation + 2)
      }
   }
}

struct NodeExpression {
   expression_type: Type,
   lhs: LiteralOrExpression,
   operator: Operator,
   rhs: LiteralOrExpression
}
impl NodeExpression {
   fn new(lhs: LiteralOrExpression, operator: Operator, rhs: LiteralOrExpression) -> Result<Self, String> {
      let lhs_type = match &lhs {
         LiteralOrExpression::Literal(literal_1) => {
            match literal_1 {
               Token::Literal(literal_2) => {
                  match literal_2 {
                     Literal::Number(_) => Type::Integer64,
                     Literal::String(_) => Type::String
                  }
               }
               _ => return Err(String::from("NodeExpression expects lhs to be Literal or Expression"))
            }
         }
         LiteralOrExpression::NodeExpression(expression) => expression.expression_type
      };

      let rhs_type = match &rhs {
         LiteralOrExpression::Literal(literal_1) => {
            match literal_1 {
               Token::Literal(literal_2) => {
                  match literal_2 {
                     Literal::Number(_) => Type::Integer64,
                     Literal::String(_) => Type::String
                  }
               }
               _ => return Err(String::from("NodeExpression expects lhs to be Literal or Expression"))
            }
         }
         LiteralOrExpression::NodeExpression(expression) => expression.expression_type
      };

      if lhs_type != rhs_type {
         return Err(String::from("Left and right hand types do not match"))
      }
      return Ok(NodeExpression {expression_type: lhs_type, lhs: lhs, operator: operator, rhs: rhs })
   }

   fn print(&self, indentation: usize) {
      println!("{:indent$}Expression", "", indent = indentation);
      match &self.lhs {
         LiteralOrExpression::Literal(literal) => {
            print!("{:indent$}", "", indent = indentation + 2);
            literal.print()
         }
         LiteralOrExpression::NodeExpression(expression) => expression.print(indentation + 2),
      }
      print!("{:indent$}", "", indent = indentation + 2);
      self.operator.print();
      match &self.rhs {
         LiteralOrExpression::Literal(literal) => {
            print!("{:indent$}", "", indent = indentation + 2);
            literal.print()
         }
         LiteralOrExpression::NodeExpression(expression) => expression.print(indentation + 2),
      }
   }
}

fn extend_expression_lhs(mut expression: NodeExpression, literal: Token, operator: Operator) -> Result<NodeExpression, String> {
   match compare_operators(&operator, &expression.operator) {
      true => {
         match expression.lhs {
            LiteralOrExpression::Literal(lhs_literal) => {
               let new_lhs = match NodeExpression::new(LiteralOrExpression::Literal(literal), operator, LiteralOrExpression::Literal(lhs_literal)) {
                  Ok(new_expression) => new_expression,
                  Err(err) => return Err(err)
               };
               expression.lhs = LiteralOrExpression::NodeExpression(Box::new(new_lhs));
               return Ok(expression);
            }
            LiteralOrExpression::NodeExpression(lhs_expression) => {
               let unboxed_lhs_expression = *lhs_expression;
               let new_lhs = extend_expression_lhs(unboxed_lhs_expression, literal, operator)?;
               expression.lhs = LiteralOrExpression::NodeExpression(Box::new(new_lhs));
               return Ok(expression);
            }
         }
      }
      false =>  {
         match NodeExpression::new(LiteralOrExpression::Literal(literal), operator, LiteralOrExpression::NodeExpression(Box::new(expression))) {
            Ok(new_expression) => return Ok(new_expression),
            Err(err) => return Err(err)
         }
      }   
   }
}

pub struct Parser {
   tokens: VecDeque<Token>,
}
impl Parser {
   pub fn new(tokens: Vec<Token>) -> Self {
      let tokens_queue: VecDeque<Token> = tokens.into_iter().collect();
      return Parser{tokens: tokens_queue}
   }

   pub fn parse(&mut self) -> Result<NodeProgram, String> {
      let mut program = NodeProgram::new();

      while self.tokens.len() > 0 {
         let token = match self.dequeue_token() {
            Some(token_value) => token_value,
            None => return Err(String::from("No token at front of token queue")),
         };

         match token {
            Token::Keyword(keyword) => {
               match keyword {
                  Keyword::Return => {
                     let parse_result = match self.parse_return() {
                        Ok(result) => result,
                        Err(err) => return Err(err),
                     };
                     program.push(NodeStatement::Return(parse_result))
                  }
                  _ => { }
               }
            }
            Token::Type(_) => {
               let statement = match self.parse_assign(token) {
                  Ok(node_assign) => node_assign,
                  Err(err) => return Err(err),
               };
               program.push(NodeStatement::Assign(statement));
            }
            _ => {}
         }
      }

      Ok(program)
   }

   fn parse_return(&mut self) -> Result<NodeReturn, String> {
      match self.parse_expression() {
         Ok(expresion) => {
            match NodeReturn::new(expresion) {
               Ok(node_return) => return Ok(node_return),
               Err(err) => return Err(err)
            }
         }
         Err(err) => return Err(err)
      }
   }

   fn parse_assign(&mut self, type_token: Token) -> Result<NodeAssign, String> {
      let identifier_token = match self.dequeue_token() {
         Some(token_value) => token_value,
         None => return Err(String::from("Expected identifier")),
      };

      match self.dequeue_token() {
         Some(token_value) => {
            match token_value {
               Token::Equals => {},
               _ => return Err(String::from("Expected '='")),
            }
         }
         None => return Err(String::from("Expected '='")),
      }

      match self.parse_expression() {
         Ok(lit_or_expr) => {
            match NodeAssign::new(type_token, identifier_token, lit_or_expr) {
               Ok(node_assign) => return Ok(node_assign),
               Err(err) => return Err(err),
            }
         }
         Err(err) => Err(err),
      }
   }

   fn parse_expression(&mut self) -> Result<LiteralOrExpression, String> {
      let literal_token = match self.dequeue_token() {
         Some(token_value) => token_value,
         None => return Err(String::from("No literal found in expression"))
      };
      
      let next_token = match self.dequeue_token() {
         Some(token_value) => token_value,
         None => return Err(String::from("Expected operator or semicolon"))
      };

      match next_token {
         Token::Semicolon => return Ok(LiteralOrExpression::Literal(literal_token)),
         Token::Operator(operator) => {
            match self.parse_expression() {
               Ok(rhs_result) => {
                  match rhs_result {
                     LiteralOrExpression::Literal(rhs_literal) => {
                        let expression = NodeExpression::new(
                           LiteralOrExpression::Literal(literal_token), operator, LiteralOrExpression::Literal(rhs_literal))?;
                        return Ok(LiteralOrExpression::NodeExpression(Box::new(expression)));
                     }
                     LiteralOrExpression::NodeExpression(expression) => {
                        match extend_expression_lhs(*expression, literal_token, operator) {
                           Ok(new_expression) => return Ok(LiteralOrExpression::NodeExpression(Box::new(new_expression))),
                           Err(err) => return Err(err)
                        }
                        
                     }
                  }
               }
               Err(err) => return Err(err)
            }
         }
         _ => return Err(String::from("Expected operator or semicolon"))
      }
   }

   fn dequeue_token(&mut self) -> Option<Token> {
      self.tokens.pop_front()
   }
}

pub fn print_ast(program: &NodeProgram) {
   println!("");
   println!("_________________________________________________");
   println!("_______________ABSTRACT SYNTAX TREE______________");
   println!("");
   program.print(0);
   println!("_________________________________________________");
   println!("_________________________________________________");

}
