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

    pub fn validate(&self, name: &str) -> Option<String> {
        match &self.expr.validate_schema_expr() {
            Some(err) => {
                let mut joined = String::new();
                joined.push_str(
                    &format!("The validation of schema in policy {} was not successful for the following reasons:\n", name)
                );
                for e in err {
                    joined.push_str(&format!("* {}\n", e));
                }
                Some(joined)
            }
            None => None,
        }
    }
}
