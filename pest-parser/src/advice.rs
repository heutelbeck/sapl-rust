use crate::{Expr, Rule};
use std::rc::Rc;

#[derive(Debug)]
pub struct Advice {
    expr: Rc<Expr>,
}

impl Advice {
    pub fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        Advice {
            expr: Rc::new(Expr::parse(pair.into_inner())),
        }
    }
}
