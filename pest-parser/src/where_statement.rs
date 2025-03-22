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

use crate::{Expr, Rule};
#[derive(Debug)]
pub enum WhereStatement {
    Expression(Expr),
    SaplPairs(Vec<WhereStatement>),
    SaplPair(Vec<WhereStatement>),
    VariableAssignment(Vec<WhereStatement>),
    UnaryPlus(Box<WhereStatement>),
    UnaryMinus(Box<WhereStatement>),
    LogicalNot(Box<WhereStatement>),
    Boolean(bool),
    Integer(i32),
    Float(f32),
    SaplId(String),
    String(String),
    Filter(String),
    Id(String),
}

impl WhereStatement {
    pub fn parse(pair: pest::iterators::Pair<Rule>) -> WhereStatement {
        use WhereStatement::*;

        match pair.as_rule() {
        Rule::condition => Expression(Expr::parse(pair.into_inner())),
        Rule::variable_assignment => VariableAssignment(pair.into_inner().map(WhereStatement::parse).collect()),
        Rule::string => WhereStatement::new_string(pair.as_str()),
        Rule::boolean_literal => WhereStatement::Boolean(pair.as_str().parse().unwrap()),
        Rule::integer => WhereStatement::Integer(pair.as_str().trim().parse().unwrap()),
        Rule::float => WhereStatement::Float(pair.as_str().trim().parse().unwrap()),
        Rule::id => Id(pair.as_str().to_string()),
        Rule::pairs => SaplPairs(pair.into_inner().map(WhereStatement::parse).collect()),
        Rule::pair => SaplPair(pair.into_inner().map(WhereStatement::parse).collect()),
        rule => unreachable!(
            "parse_where_statement expected conditon, variable_assignment, id, string, integer, floar, boolean_literal, pairs or pair, found {:?}",
            rule
        ),
    }
    }
    fn new_string(src: &str) -> Self {
        let mut s = src.to_string();
        s.retain(|c| c != '\"');
        Self::String(s)
    }
}
