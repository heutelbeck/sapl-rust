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
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/sapl.pest"]
pub struct SaplParser;

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
pub enum ReservedId {
    Subject,
    Action,
    Resource,
    Environment,
}

impl ReservedId {
    fn new(s: &str) -> Self {
        if s.eq("subject") {
            ReservedId::Subject
        } else if s.eq("action") {
            ReservedId::Action
        } else if s.eq("resource") {
            ReservedId::Resource
        } else if s.eq("environment") {
            ReservedId::Environment
        } else {
            panic!("Input {} could not be parsed as reserved id", s)
        }
    }
}

#[derive(Debug)]
pub enum Import {
    Function(Vec<String>),
    Library { lhs: Vec<String>, rhs: Vec<String> },
    Wildcard(String),
}

impl Import {
    fn new(pair: pest::iterators::Pair<Rule>) -> Option<Self> {
        fn parse(pair: pest::iterators::Pair<Rule>) -> Import {
            match pair.as_rule() {
                Rule::function_import => todo!(),
                Rule::library_import => todo!(),
                Rule::wildcard_import => todo!(),
                rule => unreachable!(
                    "Sapl::parse expected function_import, library_import or wildcard_import, found {:?}",
                    rule
                ),
            }
        }

        match pair.as_rule() {
            Rule::import_statement => Some(Import::Wildcard("Das ist ein Test".to_string())),
            _ => None,
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
pub enum NewSaplDocument {
    Policy(Policy),
    PolicySet(PolicySet),
    Import(Import),
}

pub fn parse_sapl_file_new(file: &str) -> Result<NewSaplDocument, Error<Rule>> {
    let pair = SaplParser::parse(Rule::sapl_document, file)?
        .next()
        .unwrap();

    use pest::iterators::Pair;
    println!("{:?}", pair);

    let mut imports: Option<Vec<Import>> = None;

    // for pair in pairs.clone() {
    //     if pair.as_rule() == Rule::import_statement {
    //         if let Some(ref mut i) = imports {
    //             i.push(Import::Wildcard("follow".to_string()));
    //         } else {
    //             imports = Some(vec![Import::Wildcard("init".to_string())]);
    //         }
    //     } else {
    //         break;
    //     }
    // }

    // while pair.as_rule() == Rule::import_statement {
    //     println!("import loop reached");
    //     if let Some(ref mut i) = imports {
    //         i.push(Import::Wildcard("follow".to_string()));
    //     } else {
    //         imports = Some(vec![Import::Wildcard("init".to_string())]);
    //     }
    //     pair.into_inner().next().unwrap();
    // }

    // fn parse_value(pair: Pair<Rule>) -> NewSaplDocument {
    //     let mut imports = None;
    //
    //     match pair.as_rule() {
    //         Rule::policy => NewSaplDocument::Policy(Policy::new(pair.into_inner(), imports)),
    //         Rule::policy_set => {
    //             NewSaplDocument::PolicySet(PolicySet::new(pair.into_inner(), imports))
    //         }
    //         // Rule::import_statement => {
    //         //     match imports {
    //         //         None => {
    //         //             let import: Vec<Import> =
    //         //                 vec![Import::Wildcard("Das ist ein Test 123".to_string())];
    //         //             imports = Some(import);
    //         //         }
    //         //         Some(mut i) => i.push(Import::Wildcard("Zu einfach".to_string())),
    //         //     }
    //         //     println!("Import is coming");
    //         // }
    //         rule => unreachable!(
    //             "Sapl::parse expected import_statement, schema, policy_set or policy, found {:?}",
    //             rule
    //         ),
    //     }
    // }
    //
    // Ok(parse_value(pair))

    println!("{:?}", pair);
    match pair.as_rule() {
        Rule::policy => Ok(NewSaplDocument::Policy(Policy::new(
            pair.into_inner(),
            imports,
        ))),
        Rule::policy_set => Ok(NewSaplDocument::PolicySet(PolicySet::new(
            pair.into_inner(),
            imports,
        ))),
        rule => unreachable!(
            "Sapl::parse expected policy_set or policy, found {:?}",
            rule
        ),
    }
}

#[derive(Debug)]
pub enum SaplDocument<'a> {
    Document(Vec<SaplDocument<'a>>),
    LibraryImport(Vec<SaplDocument<'a>>),
    Schema(Vec<SaplDocument<'a>>),
    PolicySet(Vec<SaplDocument<'a>>),
    Policy(Vec<SaplDocument<'a>>),
    PolicyName(&'a str),
    PolicySetName(&'a str),
    Entitlement(Entitlement),
    CombiningAlgorithm(CombiningAlgorithm),
    Transformation(Vec<SaplDocument<'a>>),
    Advice(Vec<SaplDocument<'a>>),
    Obligation(Vec<SaplDocument<'a>>),
    Statement(Vec<SaplDocument<'a>>),
    Where(Vec<SaplDocument<'a>>),
    TargetExpression(Vec<SaplDocument<'a>>),
    ReserveId(ReservedId),
    SaplId(&'a str),
    Id(&'a str),
    Filter(&'a str),
    Value(&'a str),
}

pub fn parse_sapl_file(file: &str) -> Result<SaplDocument, Error<Rule>> {
    let sapl = SaplParser::parse(Rule::sapl_document, file)?
        .next()
        .unwrap();

    use pest::iterators::Pair;

    fn parse_value(pair: Pair<Rule>) -> SaplDocument {
        match pair.as_rule() {
            Rule::document => SaplDocument::Document(pair.into_inner().map(parse_value).collect()),
            Rule::library_import => {
                SaplDocument::LibraryImport(pair.into_inner().map(parse_value).collect())
            }
            Rule::policy_set => {
                SaplDocument::PolicySet(pair.into_inner().map(parse_value).collect())
            }
            Rule::policy => SaplDocument::Policy(pair.into_inner().map(parse_value).collect()),
            Rule::policy_name => SaplDocument::PolicyName(pair.as_str()),
            Rule::policy_set_name => SaplDocument::PolicySetName(pair.as_str()),
            Rule::entitlement => SaplDocument::Entitlement(Entitlement::new(pair.as_str())),
            Rule::combining_algorithm => {
                SaplDocument::CombiningAlgorithm(CombiningAlgorithm::new(pair.as_str()))
            }
            Rule::sapl_id => SaplDocument::SaplId(pair.as_str()),
            Rule::id => SaplDocument::SaplId(pair.as_str()),
            Rule::schema => SaplDocument::Schema(pair.into_inner().map(parse_value).collect()),
            Rule::reserved_id => SaplDocument::ReserveId(ReservedId::new(pair.as_str())),
            Rule::transformation => {
                SaplDocument::Transformation(pair.into_inner().map(parse_value).collect())
            }
            Rule::advice => SaplDocument::Advice(pair.into_inner().map(parse_value).collect()),
            Rule::obligation => {
                SaplDocument::Obligation(pair.into_inner().map(parse_value).collect())
            }
            Rule::statement => {
                SaplDocument::Statement(pair.into_inner().map(parse_value).collect())
            }
            Rule::where_statement => {
                SaplDocument::Where(pair.into_inner().map(parse_value).collect())
            }
            Rule::target_expression => {
                SaplDocument::TargetExpression(pair.into_inner().map(parse_value).collect())
            }
            Rule::value => SaplDocument::Value(pair.as_str()),
            Rule::FILTER => SaplDocument::Filter(pair.as_str()),
            rule => unreachable!(
                "Sapl::parse expected import_statement, schema, policy_set or policy, found {:?}",
                rule
            ),
        }
    }

    Ok(parse_value(sapl))
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
        let policy = parse_sapl_file_new("policy \"policy 1\" deny");
        assert!(policy.is_ok());
    }

    #[test]
    fn parse_policy_set() {
        let policy_set = parse_sapl_file_new(
            "set \"classified documents\" first-applicable policy \"Clearance (1/3)\" permit",
        );
        assert!(policy_set.is_ok());
    }

    #[test]
    fn parse_library_import() {
        let import = parse_sapl_file_new("import filter as filter policy \"policy\" permit");
        assert!(import.is_ok());
    }

    #[test]
    fn parse_function_import() {
        let import = parse_sapl_file_new("import sapl.pip.http.get policy \"policy\" permit");
        assert!(import.is_ok());
    }

    #[test]
    fn parse_wildcard_import() {
        let import = parse_sapl_file_new("import filter.* policy \"policy\" permit");
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
