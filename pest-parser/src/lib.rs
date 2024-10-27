/*
    Copyright 2024 Stefan Weng

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

mod decision;

pub use crate::decision::Decision;

use lazy_static::lazy_static;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/sapl.pest"]
pub struct SaplParser;

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        //Precedence is defined lowest to highest
        PrattParser::new()
            //Addition and subtract have equal precedence
            //.op(Op::infix(policy, Left))
    };
}

#[derive(Debug)]
pub enum SaplDocument<'a> {
    ImportStatment,
    Schema,
    PolicySet,
    Policy(Box<SaplDocument<'a>>),
    String(&'a str),
    Entitlement(Entitlement),
}

impl SaplDocument<'_> {
    pub fn get_decision(&self) -> Decision {
        Decision::Deny
    }
}

#[derive(Debug)]
pub enum Entitlement {
    Permit,
    Deny,
}

impl Entitlement {
    fn new(s: &str) -> Self {
        if s.eq("permit") {
            Entitlement::Permit
        } else {
            Entitlement::Deny
        }
    }
}

fn parse_sapl_document(pairs: Pairs<Rule>) -> SaplDocument {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::import_statement => SaplDocument::ImportStatment,
            Rule::schema => SaplDocument::Schema,
            Rule::policy_set => SaplDocument::PolicySet,
            Rule::entitlement => SaplDocument::Entitlement(Entitlement::new(primary.as_str())),
            Rule::policy => parse_sapl_document(primary.into_inner()),
            Rule::string => SaplDocument::String(primary.as_str()),
            rule => unreachable!(
                "Sapl::parse expected import_statement, schema, policy_set or policy, found {:?}",
                rule
            ),
        })
        .parse(pairs)
}

pub fn parse(document: &str) -> SaplDocument {
    match SaplParser::parse(Rule::sapl_document, document) {
        Ok(mut pairs) => {
            println!(
                "Parsed: {:#?}",
                parse_sapl_document(
                    //inner of expr
                    pairs.next().unwrap().into_inner()
                )
            );
        }
        Err(e) => {
            eprintln!("Parse failed {:?}", e);
        }
    }

    SaplDocument::Schema
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_failed() {
        let decision = parse("Das ist ein Test");
        assert_eq!(decision.get_decision(), Decision::Deny);
    }

    #[test]
    fn policy() {
        let pair = SaplParser::parse(Rule::policy, "policy \"policy 1\" deny")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(pair.as_rule(), Rule::policy);
    }

    #[test]
    fn entitlement_success() {
        let pair = SaplParser::parse(Rule::entitlement, "deny")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(pair.as_rule(), Rule::entitlement);

        let pair = SaplParser::parse(Rule::entitlement, "permit")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(pair.as_rule(), Rule::entitlement);
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: Error { variant: ParsingError { positives: [entitlement], negatives: [] }, location: Pos(0), line_col: Pos((1, 1)), path: None, line: \"policy\", continued_line: None, parse_attempts: None }"
    )]
    fn entitlement_fail() {
        let pair = SaplParser::parse(Rule::entitlement, "policy")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(pair.as_rule(), Rule::entitlement);
    }
}
