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

use pest::error::Error;
use pest::{pratt_parser::PrattParser, Parser};
use pest_derive::Parser;

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
            .op(Op::infix(equal, Left) | Op::infix(not_equal, Left) | Op::infix(unknown, Left))
            .op(Op::infix(comparison, Left) | Op::infix(less, Left) | Op::infix(greater, Left) | Op::infix(less_equal, Left) | Op::infix(greater_equal, Left))
            .op(Op::infix(addition, Left) | Op::infix(subtract, Left))
            .op(Op::infix(mul, Left) | Op::infix(div, Left) | Op::infix(modulo, Left))
            .op(Op::prefix(unary_expression))
    };
}

#[derive(Debug, Default)]
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
            panic!("Input {} could not be parsed as entitlement", s)
        }
    }
}

#[derive(Debug, Default)]
pub enum CombiningAlgorithm {
    #[default]
    DenyOverrides,
    PermitOverrides,
    FirstApplicable,
    OnlyOneApplicable,
    DenyUnlessPermit,
    PermitUnlessDeny,
}

impl CombiningAlgorithm {
    fn new(s: &str) -> Self {
        if s.eq("deny-overrides") {
            CombiningAlgorithm::DenyOverrides
        } else if s.eq("permit-overrides") {
            CombiningAlgorithm::PermitOverrides
        } else if s.eq("first-applicable") {
            CombiningAlgorithm::FirstApplicable
        } else if s.eq("only-one-applicable") {
            CombiningAlgorithm::OnlyOneApplicable
        } else if s.eq("deny-unless-permit") {
            CombiningAlgorithm::DenyUnlessPermit
        } else if s.eq("permit-unless-deny") {
            CombiningAlgorithm::PermitUnlessDeny
        } else {
            panic!("Input {} could not be parsed as combining algorithm", s)
        }
    }
}

#[derive(Debug)]
pub enum Import {
    Function(String),
    Library { name: String, alias: String },
    Wildcard(String),
}

impl Import {
    fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        fn parse_import_statement_sapl_ids(pair: pest::iterators::Pair<Rule>) -> String {
            match pair.as_rule() {
                Rule::sapl_id => pair.as_str().to_string(),
                rule => unreachable!(
                    "parse_import_statement_sapl_ids expected sapl_id, found {:?}",
                    rule
                ),
            }
        }

        match pair.as_rule() {
            Rule::function_import => {
                let sapl_ids: Vec<String> = pair
                    .into_inner()
                    .map(parse_import_statement_sapl_ids)
                    .collect();
                Import::Function(sapl_ids.join(".").to_string())
            }
            Rule::library_import => {
                let mut sapl_ids: Vec<String> = pair
                    .into_inner()
                    .map(parse_import_statement_sapl_ids)
                    .collect();
                let alias = sapl_ids.pop().unwrap();
                Import::Library {
                    name: sapl_ids.join("."),
                    alias,
                }
            }
            Rule::wildcard_import => {
                let sapl_ids: Vec<String> = pair
                    .into_inner()
                    .map(parse_import_statement_sapl_ids)
                    .collect();
                let mut import = sapl_ids.join(".").to_string();
                import.push_str(".*");
                Import::Wildcard(import)
            }
            rule => unreachable!(
            "Sapl::parse expected function_import, library_import or wildcard_import, found {:?}",
            rule
        ),
        }
    }
}

#[derive(Debug, Default)]
pub struct PolicySet {
    imports: Option<Vec<Import>>,
    name: String,
    combining_algorithm: CombiningAlgorithm,
    policies: Vec<Policy>,
}

impl PolicySet {
    fn new(pairs: pest::iterators::Pairs<Rule>, imports: Option<Vec<Import>>) -> Self {
        let mut policy_set = PolicySet {
            imports,
            ..Default::default()
        };

        for pair in pairs {
            let mut imports = None;
            match pair.as_rule() {
                Rule::policy_set_name => {
                    let mut name = pair.as_str().to_string();
                    name.retain(|c| c != '\"');
                    policy_set.name = name;
                }
                Rule::combining_algorithm => {
                    policy_set.combining_algorithm = CombiningAlgorithm::new(pair.as_str())
                }
                Rule::policy => policy_set.policies.push(Policy::new(pair.into_inner(), imports)),
                rule => unreachable!(
                    "Sapl::parse expected policy_set_name, combining_algorithm or policy, found {:?}",
                    rule
                ),
            }
        }

        policy_set
    }
}

#[derive(Debug, Default)]
pub struct Policy {
    imports: Option<Vec<Import>>,
    name: String,
    entitlement: Entitlement,
}

impl Policy {
    fn new(pairs: pest::iterators::Pairs<Rule>, imports: Option<Vec<Import>>) -> Self {
        let mut policy = Policy {
            imports,
            ..Default::default()
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::policy_name => {
                    let mut name = pair.as_str().to_string();
                    name.retain(|c| c != '\"');
                    policy.name = name;
                }
                Rule::entitlement => policy.entitlement = Entitlement::new(pair.as_str()),
                rule => unreachable!(
                    "Sapl::parse expected policy_name or entitlement, found {:?}",
                    rule
                ),
            }
        }

        policy
    }
}

#[derive(Debug)]
pub enum PolicyType {
    Policy(Policy),
    PolicySet(PolicySet),
}

#[derive(Debug)]
pub struct SaplDocument {
    imports: Option<Vec<Import>>,
    
}

pub fn parse_sapl_file(file: &str) -> Result<SaplDocument, Box<Error<Rule>>> {
    let mut pairs = SaplParser::parse(Rule::sapl_document, file)?;

    use pest::iterators::Pair;
    //println!("{:?}", pair);

    let mut imports: Option<Vec<Import>> = None;

    for p in pairs.clone() {
        //println!("Iteration... {:#?}", p.as_rule());
        if p.as_rule() == Rule::import_statement {
            if let Some(ref mut i) = imports {
                i.push(Import::parse(p.into_inner().next().unwrap()));
            } else {
                imports = Some(vec![Import::parse(p.into_inner().next().unwrap())]);
            }
        }
    }

    //println!("imports: {:#?}", imports);

    while pairs.next().unwrap().as_rule() == Rule::import_statement {
        println!("Getroffen")
        pairs.next().unwrap();
    }

    fn parse_value(pair: Pair<Rule>, imports: Option<Vec<Import>>) -> SaplDocument {
        match pair.as_rule() {
            Rule::policy => SaplDocument::Policy(Policy::new(pair.into_inner(), imports)),
            Rule::policy_set => {
                SaplDocument::PolicySet(PolicySet::new(pair.into_inner(), imports))
            }
            rule => unreachable!(
                "Sapl::parse expected import_statement, schema, policy_set or policy, found {:?}",
                rule
            ),
        }
    }

    Ok(parse_value(pairs.next().unwrap(), imports))

    // println!("{:?}", pair);
    // match pair.as_rule() {
    //     Rule::policy => Ok(NewSaplDocument::Policy(Policy::new(
    //         pair.into_inner(),
    //         imports,
    //     ))),
    //     Rule::policy_set => Ok(NewSaplDocument::PolicySet(PolicySet::new(
    //         pair.into_inner(),
    //         imports,
    //     ))),
    //     rule => unreachable!(
    //         "Sapl::parse expected policy_set or policy, found {:?}",
    //         rule
    //     ),
    // }
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
        let where_statement = parse_sapl_file("policy \"test_policy\" permit where var variable = \"anAttribute\"; subject.attribute == variable; var foo = true schema {\"type\": \"boolean\"}");
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
        let policy = parse_sapl_file("import filter as filter subject schema aSubjectSchema policy \"test_policy\" permit subject.id == \"anId\" | action == \"anAction\" where var variable = \"anAttribute\"; subject.attribute == variable; var foo = true schema {\"type\": \"boolean\" } obligation \"logging:log_access\" advice \"logging:inform_admin\" transform resource.content |- filter.blacken");
        assert!(policy.is_ok());
    }
}
