use crate::{expr::Ast, Eval, Expression};

#[derive(Debug)]
pub struct ConstantIntegerExpr {
    pub v: i32,
}

impl Expression for ConstantIntegerExpr {}
impl Eval for ConstantIntegerExpr {
    fn eval(&self) -> Ast {
        Ast::Finished
    }
}
