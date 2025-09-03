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

mod ast;
mod authorization_decision;
pub mod authorization_subscription;
mod basic_identifier_expression;
mod decision;
mod evaluate;
mod functions;
mod import;
pub mod pip;
mod sapl_document;
mod schema;
mod simplify;
pub mod stream_sapl;
mod val;

pub use crate::ast::{Ast, Op};
pub use crate::authorization_decision::AuthorizationDecision;
pub use crate::decision::Decision;
pub use crate::import::Import;
pub use crate::sapl_document::CombiningAlgorithm;
pub use crate::sapl_document::DocumentBody;
pub use crate::sapl_document::Policy;
pub use crate::sapl_document::PolicySet;
pub use crate::sapl_document::SaplDocument;
pub use crate::sapl_document::combining_algorithm::{
    deny_overrides, deny_unless_permit, only_one_applicable, permit_overrides, permit_unless_deny,
};
pub use crate::schema::Schema;
pub use crate::val::Val;
pub use stream_sapl::{DecisionCombinedStream, ValueStream};

use authorization_subscription::AuthorizationSubscription;
use pest::error::Error;
use pest::{Parser, pratt_parser::PrattParser};
use pest_derive::Parser;
use serde_json::Value;
use std::{
    pin::Pin,
    sync::{Arc, RwLock},
};
use stream_sapl::StreamSapl;
use stream_sapl::{once_decision, once_val};
use tokio_stream::Stream;

type BoxedValStream = Pin<Box<dyn Stream<Item = Result<Val, String>> + Send>>;

#[derive(Parser)]
#[grammar = "grammar/sapl.pest"]
pub struct SaplParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            .op(Op::infix(lazy_or, Left))
            .op(Op::infix(lazy_and, Left))
            .op(Op::infix(eager_or, Left))
            .op(Op::infix(exclusive_or, Left))
            .op(Op::infix(eager_and, Left))
            .op(Op::infix(equal, Left) | Op::infix(not_equal, Left) | Op::infix(regex, Left))
            .op(Op::infix(comparison, Left) | Op::infix(less, Left) | Op::infix(greater, Left) | Op::infix(less_equal, Left) | Op::infix(greater_equal, Left))
            .op(Op::infix(addition, Left) | Op::infix(subtract, Left))
            .op(Op::infix(mul, Left) | Op::infix(div, Left) | Op::infix(modulo, Left) | Op::infix(FILTER, Left))
            .op(Op::prefix(unary_expression))
    };
}

#[derive(Clone, Debug, Default)]
pub enum Entitlement {
    Permit,
    #[default]
    Deny,
}

impl Entitlement {
    fn new(s: &str) -> Self {
        if s.eq("permit") {
            Entitlement::Permit
        } else if s.eq("deny") {
            Entitlement::Deny
        } else {
            panic!("Input {s} could not be parsed as entitlement")
        }
    }
}

pub fn parse_sapl_file(file: &str) -> Result<SaplDocument, Box<Error<Rule>>> {
    let pairs = SaplParser::parse(Rule::sapl_document, file)?;

    //parse imports
    let mut imports: Option<Vec<Import>> = None;
    for p in pairs.clone() {
        if p.as_rule() == Rule::import_statement {
            if let Some(ref mut i) = imports {
                i.push(Import::parse(p.into_inner().next().unwrap()));
            } else {
                imports = Some(vec![Import::parse(p.into_inner().next().unwrap())]);
            }
        }
    }

    //parse schemas
    let mut schemas: Option<Vec<Schema>> = None;
    for p in pairs.clone() {
        if p.as_rule() == Rule::schema {
            if let Some(ref mut i) = schemas {
                i.push(Schema::parse(p));
            } else {
                schemas = Some(vec![Schema::parse(p)]);
            }
        }
    }

    //parse document body
    fn parse(pairs: pest::iterators::Pairs<Rule>) -> DocumentBody {
        for pair in pairs {
            match pair.as_rule() {
                Rule::policy => {
                    return DocumentBody::Policy(Policy::new(pair.into_inner()));
                }
                Rule::policy_set => {
                    return DocumentBody::PolicySet(PolicySet::new(pair.into_inner()));
                }
                _rule => {}
            }
        }
        unreachable!("Sapl::parse expected policy_set or policy not found");
    }

    //return parsed SaplDocument
    Ok(SaplDocument {
        imports,
        schemas,
        body: parse(pairs),
    })
}

pub trait Eval {
    fn eval(
        &self,
        auth_subscription: Arc<RwLock<Value>>,
    ) -> Pin<Box<dyn Stream<Item = Result<Val, String>> + Send>>;
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn parse_simple_policy() {
        let policy = parse_sapl_file("policy \"policy 1\" deny");
        assert!(policy.is_ok());
    }

    #[test]
    fn parse_policy_set() {
        let policy_set = parse_sapl_file(
            "set \"classified documents\" first-applicable policy \"Clearance (1/3)\" permit",
        );
        assert!(policy_set.is_ok());
    }

    #[test]
    fn parse_library_import() {
        let import = parse_sapl_file("import filter as filter policy \"policy\" permit");
        assert!(import.is_ok());
    }

    #[test]
    fn parse_function_import() {
        let import = parse_sapl_file("import sapl.pip.http.get policy \"policy\" permit");
        assert!(import.is_ok());
    }

    #[test]
    fn parse_wildcard_import() {
        let import = parse_sapl_file("import filter.* policy \"policy\" permit");
        assert!(import.is_ok());
    }

    #[test]
    fn parse_schema() {
        let schema = parse_sapl_file("subject schema aSubjectSchema policy \"policy schema\" deny");
        assert!(schema.is_ok());
    }

    #[test]
    fn parse_where_statement() {
        let where_statement = parse_sapl_file(
            "policy \"test_policy\" permit where var variable = \"anAttribute\"; subject.attribute == variable; var foo = true schema {\"type\": \"boolean\"}",
        );
        assert!(where_statement.is_ok());
    }

    #[test]
    fn parse_obligation() {
        let obligation =
            parse_sapl_file("policy \"test\" permit obligation \"logging:log_access\"");
        assert!(obligation.is_ok());
    }

    #[test]
    fn parse_advice() {
        let advice = parse_sapl_file("policy \"policy 1\" deny advice \"logging:inform_admin\"");
        assert!(advice.is_ok());
    }

    #[test]
    fn parse_advice_with_pairs() {
        let advice = parse_sapl_file(
            "policy \"policy 1\" deny advice { \"type\": \"logAccess\", \"message\": (\"Administrator \" + subject.name + \" has manipulated patient: \" + action.http.requestedURI) }",
        );
        assert!(advice.is_ok());
    }
    #[test]
    fn parse_transformation() {
        let transformation =
            parse_sapl_file("policy \"test\" permit transform resource.content |- filter.blacken");
        assert!(transformation.is_ok());
    }

    #[test]
    fn parse_target_expression() {
        let target_exp = parse_sapl_file(
            "policy \"test_policy\" permit subject.id == \"anId\" | action == \"anAction\"",
        );
        assert!(target_exp.is_ok());
    }

    #[test]
    fn parse_all_options() {
        let policy = parse_sapl_file(
            "import filter as filter subject schema aSubjectSchema policy \"test_policy\" permit subject.id == \"anId\" | action == \"anAction\" where var variable = \"anAttribute\"; subject.attribute == variable; var foo = true schema {\"type\": \"boolean\" } obligation \"logging:log_access\" advice \"logging:inform_admin\" transform resource.content |- filter.blacken",
        );
        assert!(policy.is_ok());
    }
}
