use crate::{Expr, Rule};
use std::rc::Rc;

#[derive(Debug)]
pub struct Schema {
    sapl_id: String,
    expr: Rc<Expr>,
}

impl Schema {
    pub fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut inner_rules = pair.into_inner();

        Schema {
            sapl_id: inner_rules.next().unwrap().as_str().to_string(),
            expr: Rc::new(Expr::parse(inner_rules)),
        }
    }
}
