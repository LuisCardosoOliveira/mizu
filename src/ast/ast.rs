use crate::token::Token;

pub trait Node {
  fn token_literal(&self) -> String;
  fn string(&self) -> String;
}

pub trait Statement: Node {
  fn statement_node(&self);
}

pub trait Expression: Node {
  fn expression_node(&self);
}

pub struct Program {
  pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
  pub fn new() -> Self {
    Self {
      statements: Vec::new(),
    }
  }
}

impl Node for Program {
  fn token_literal(&self) -> String {
    if !self.statements.is_empty() {
      self.statements[0].token_literal()
    } else {
      String::from("")
    }
  }

  fn string(&self) -> String {
    let mut out = String::new();

    for s in &self.statements {
      out.push_str(&s.string());
    }

    out
  }
}

pub struct VarStatement {
  pub token: Token,
  pub name: Identifier,
  pub value: Option<Box<dyn Expression>>,
}

impl Statement for VarStatement {
  fn statement_node(&self) {}
}

impl Node for VarStatement {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }
  fn string(&self) -> String {
    let mut out = String::new();

    out.push_str(&self.token_literal());
    out.push_str(" ");
    out.push_str(&self.name.value);
    out.push_str(" = ");

    if let Some(value) = &self.value {
      out.push_str(&value.string());
    }

    out.push_str(";");

    out
  }
}

pub struct Identifier {
  pub token: Token,
  pub value: String,
}

impl Expression for Identifier {
  fn expression_node(&self) {}
}

impl Node for Identifier {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }
  fn string(&self) -> String {
    self.value.clone()
  }
}

pub struct RetStatement {
  pub token: Token,
  pub return_value: Option<Box<dyn Expression>>,
}

impl Statement for RetStatement {
  fn statement_node(&self) {}
}

impl Node for RetStatement {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }

  fn string(&self) -> String {
    let mut out = String::new();

    out.push_str(&self.token_literal());
    out.push_str(" ");

    if let Some(value) = &self.return_value {
      out.push_str(&value.string());
    }

    out.push_str(";");

    out
  }
}

pub struct ExpressionStatement {
  pub token: Token,
  pub expression: Option<Box<dyn Expression>>,
}

impl Statement for ExpressionStatement {
  fn statement_node(&self) {}
}

impl Node for ExpressionStatement {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }
  fn string(&self) -> String {
    if let Some(expr) = &self.expression {
      expr.string()
    } else {
      String::new()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::token::TokenType;

  #[test]
  fn test_string_value() {
    let program = Program {
      statements: vec![Box::new(VarStatement {
        token: Token {
          token_type: TokenType::Var,
          literal: String::from("var"),
        },
        name: Identifier {
          token: Token {
            token_type: TokenType::Ident,
            literal: String::from("myVar"),
          },
          value: String::from("myVar"),
        },
        value: Some(Box::new(Identifier {
          token: Token {
            token_type: TokenType::Ident,
            literal: String::from("anotherVar"),
          },
          value: String::from("anotherVar"),
        })),
      })],
    };

    assert_eq!(program.string(), "var myVar = anotherVar;");
  }
}
