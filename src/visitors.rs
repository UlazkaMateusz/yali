use crate::expression::{Binary, Expression, Grouping, Literal, Unary};

pub trait Visitor<R> {
    fn visit_literal(&self, literal: &Literal) -> R;
    fn visit_unary(&self, unary: &Unary) -> R;
    fn visit_binary(&self, binary: &Binary) -> R;
    fn visit_grouping(&self, grouping: &Grouping) -> R;
}

pub struct AstPrinter {}

impl AstPrinter {
    fn parenthesize(&self, name: &str, exps: &[&Expression]) -> String {
        let middle: String = exps
            .iter()
            .map(|x| x.accept::<String>(Box::new(self)))
            .collect::<Vec<String>>()
            .join(" ");

        format!("({name} {middle})")
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_literal(&self, literal: &Literal) -> String {
        todo!()
    }

    fn visit_unary(&self, unary: &Unary) -> String {
        self.parenthesize(&unary.operator.lexeme, &[&unary.right])
    }

    fn visit_binary(&self, binary: &Binary) -> String {
        self.parenthesize(&binary.operator.lexeme, &[&binary.left, &binary.right])
    }

    fn visit_grouping(&self, grouping: &Grouping) -> String {
        self.parenthesize("group", &[&grouping.expresion])
    }
}
