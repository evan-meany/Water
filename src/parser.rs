use crate::token::Token;
use crate::token::Literal;
use crate::token::Type;
use crate::token::Operator;

pub struct NodeProgram {
   statements: Vec<Box<NodeStatement>>
}

impl NodeProgram {
   fn new() ->Self {
      return NodeProgram{statements: Vec::new()};
   }

   fn push(&mut self, statement: Box<NodeStatement>) {
      self.statements.push(statement);
   }
}

enum NodeStatement {
   Assign(NodeAssign),
   Return(NodeReturn)
}

struct NodeAssign {
   identifier: String,
   expression: Box<NodeExpression>
}

impl NodeAssign {
   fn new(identifier: String, expression: Box<NodeExpression>) -> Self {
      return NodeAssign { identifier: identifier, expression: expression};
   }
}

struct NodeReturn {
   expression: Box<NodeExpression>
}

impl NodeReturn {
   fn new(expression: Box<NodeExpression>) -> Self {
      return NodeReturn{expression: expression}
   }
}

enum LiteralOrExpr {
   Literal(Literal),
   NodeExpression(Box<NodeExpression>)
}

struct NodeExpression {
   expression_type: Type,
   lhs: LiteralOrExpr,
   operator: Option<Operator>,
   rhs: Option<LiteralOrExpr>
}

impl NodeExpression {
   fn new(expression_type: Type, lhs: LiteralOrExpr, 
          operator: Option<Operator>, rhs: Option<LiteralOrExpr>) -> Self {
      return NodeExpression {expression_type: expression_type, 
                             lhs: lhs, operator: operator, rhs: rhs }
   }
}

pub struct Parser {
   tokens: Vec<Token>
}

impl Parser {
   pub fn new(tokens: Vec<Token>) -> Self {
      return Parser{tokens: tokens}
   }

   pub fn parse(&mut self) -> Result<NodeProgram, String> {
      let mut program = NodeProgram::new();
      Ok(program)
   }
}