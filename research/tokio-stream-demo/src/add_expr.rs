use crate::expr::Ast;
use crate::Eval;
use crate::Expression;

use std::sync::Arc;

#[derive(Debug)]
pub struct AddExpr {
    pub lhs: Arc<dyn Expression>,
    pub rhs: Arc<dyn Expression>,
}

impl Expression for AddExpr {}
impl Eval for AddExpr {
    fn eval(&self) -> Ast {
        use crate::expr::Ast::*;
        // match (self.lhs.eval(), self.rhs.eval()) {
        //     (Integer(l), Integer(r)) => Integer(l + r),
        //     _ => todo!(),
        // }
        Integer(crate::ConstantIntegerExpr { v: 42 })
    }
}
