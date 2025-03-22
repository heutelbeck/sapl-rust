/*
    Copyright 2025 Stefan Weng

    Licensed under the Apache License, Version 2.0 (the "License"); you may not
    use this file except in compliance with the License. You may obtain a copy
    of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
    WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
    License for the specific language governing permissions and limitations
    under the License.
*/

use crate::{basic_identifier_expression::BasicIdentifierExpression, Expr, Rule};
use std::rc::Rc;

#[derive(Debug)]
pub struct Schema {
    sapl_id: String,
    expr: Rc<Expr>,
}

impl Schema {
    pub fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut inner_rules = pair.into_inner();
        let sapl_id = inner_rules.next().unwrap();

        Schema {
            sapl_id: match sapl_id.as_rule() {
                Rule::id => sapl_id.as_str().to_string(),
                Rule::basic_identifier_expression => {
                    BasicIdentifierExpression::new(sapl_id.as_str()).to_string()
                }
                rule => panic!(
                    "Schema expect id or basic_identifier_expression, found {:?}",
                    rule
                ),
            },
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
