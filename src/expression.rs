use crate::{scanner::Token, visitors::Visitor};

pub enum Expression {
    Literal(Literal),
    Unary(Box<Unary>),
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
}

impl Expression {
    pub fn accept<R>(&self, visitor: Box<&dyn Visitor<R>>) -> R {
        match self {
            Expression::Literal(literal) => visitor.visit_literal(literal),
            Expression::Unary(unary) => visitor.visit_unary(unary),
            Expression::Binary(binary) => visitor.visit_binary(binary),
            Expression::Grouping(grouping) => visitor.visit_grouping(grouping),
        }
    }
}

pub struct Literal {
    pub value: Value,
}

pub struct Unary {
    pub operator: Token,
    pub right: Expression,
}

pub struct Binary {
    pub left: Expression,
    pub operator: Token,
    pub right: Expression,
}

pub struct Grouping {
    pub expresion: Expression,
}
pub enum Value {
    Number(f64),
    Strinng(String),
    True,
    False,
    Nil,
}
