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

use crate::basic_identifier_expression::BasicIdentifierExpression;
use crate::Rule;

#[derive(Debug)]
pub enum Transformation {
    SaplPairs(Vec<Transformation>),
    SaplPair(Vec<Transformation>),
    BasicIdent(Vec<Transformation>),
    BasicIdentExpr(BasicIdentifierExpression),
    String(String),
    KeyStep(String),
    Filter(String),
    FilterComponent(String),
}

impl Transformation {
    pub fn parse(pair: pest::iterators::Pair<Rule>) -> Transformation {
        use Transformation::*;

        match pair.as_rule() {
            Rule::string => Self::new_string(pair.as_str()),
            Rule::key_step => KeyStep(pair.as_str().to_string()),
            Rule::pairs => SaplPairs(pair.into_inner().map(Transformation::parse).collect()),
            Rule::pair => SaplPair(pair.into_inner().map(Transformation::parse).collect()),
            Rule::basic_identifier => BasicIdent(pair.into_inner().map(Transformation::parse).collect()),
            Rule::basic_identifier_expression => BasicIdentExpr(BasicIdentifierExpression::new(pair.as_str())),
            Rule::FILTER => Filter(pair.as_str().to_string()),
            Rule::filter_component => Transformation::FilterComponent(pair.as_str().to_string()),
            rule => unreachable!(
                "parse_transformation expected pairs, pair, string, key_step, id, basic_identifier, basic_identifier_expression, FILTER or filter_component, found {:?}",
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
