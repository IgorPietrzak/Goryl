pub mod expressions;
use expressions::Expr;

impl Expr {
    pub fn ast_printer(&self) {
        match self {
            Expr::Binary(_) => self.pretty_print(),
            Expr::Grouping(_) => self.pretty_print(),
            Expr::Literal(_) => self.pretty_print(),
            Expr::Unary(_) => self.pretty_print(),
        }
    }

    fn pretty_print(&self) {
        println!("{:?}", self);
    }
}

#[cfg(test)]
mod test {
    use self::expressions::Literal;

    use super::*;
    #[test]
    fn test_pretty_printer() {
        let expression = Expr::Literal(Literal {
            value: crate::syntax::token::Literal::String("Hello".to_string()),
        });
        expression.ast_printer();
    }
}
