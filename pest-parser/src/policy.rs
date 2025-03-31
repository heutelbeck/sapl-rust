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

use std::rc::Rc;

use crate::advice::Advice;
use crate::authorization_subscription::AuthorizationSubscription;
use crate::expr::Expr;
use crate::transformation::Transformation;
use crate::where_statement::WhereStatement;
use crate::Decision;
use crate::Entitlement;
use crate::Rule;

#[derive(Debug, Default)]
pub struct Policy {
    pub name: String,
    entitlement: Entitlement,
    target_exp: Option<Rc<Expr>>,
    where_statements: Option<Vec<WhereStatement>>,
    obligations: Option<Box<Expr>>,
    advice: Option<Vec<Advice>>,
    transformation: Option<Vec<Transformation>>,
}

impl Policy {
    pub fn new(pairs: pest::iterators::Pairs<Rule>) -> Self {
        let mut policy = Policy::default();

        for pair in pairs {
            match pair.as_rule() {
                Rule::policy_name => {
                    let mut name = pair.as_str().to_string();
                    name.retain(|c| c != '\"');
                    policy.name = name;
                }
                Rule::entitlement => policy.entitlement = Entitlement::new(pair.as_str()),
                Rule::target_expression => {
                    policy.target_exp = Some(Rc::new(Expr::parse(pair.clone().into_inner())))
                }
                Rule::where_statement => {
                    policy.where_statements = Some(
                        pair.clone()
                            .into_inner()
                            .map(WhereStatement::parse)
                            .collect(),
                    );
                }
                Rule::obligation => {
                    policy.obligations = Some(Box::new(Expr::parse(pair.clone().into_inner())));
                }
                Rule::advice => {
                    policy.advice = Some(pair.clone().into_inner().map(Advice::parse).collect());
                }
                Rule::transformation => {
                    policy.transformation = Some(
                        pair.clone()
                            .into_inner()
                            .map(Transformation::parse)
                            .collect(),
                    );
                }
                rule => unreachable!(
                    "Sapl::parse expected policy_name or entitlement, found {:?}",
                    rule
                ),
            }
        }

        policy
    }

    pub fn validate(&self) -> Result<(), String> {
        match &self.target_exp {
            Some(exp) => match exp.validate_target_expr() {
                Some(err) => {
                    let mut joined = String::new();
                    joined.push_str(&format!("The validation of target expression in the policy {} was not successful for the following reasons:\n", &self.name));
                    for e in err {
                        joined.push_str(&format!("* {}\n", e));
                    }
                    Err(joined)
                }
                None => Ok(()),
            },
            None => Ok(()),
        }
    }

    pub fn evaluate(&self, auth_subscription: &AuthorizationSubscription) -> Decision {
        // Target Expression    |   Condition   |   Decision
        //---------------------------------------------------------
        // false (not matching) |   don’t care  |   NOT_APPLICABLE
        // true (matching)      |   false       |   NOT_APPLICABLE
        // Error                |   don’t care  |   INDETERMINATE
        // true (matching)      |   Error       |   INDETERMINATE
        // true (matching)      |   true        |   (PERMIT or DENY)
        //
        // https://sapl.io/docs/3.0.0-SNAPSHOT/6_2_Policy/

        // TODO umbau mit einem stream aus target und where_statement?

        let target = self
            .target_exp
            .as_ref()
            .unwrap_or(&Rc::new(Expr::Boolean(true)))
            .evaluate(auth_subscription);

        match target {
            Err(_) => Decision::Indeterminate,
            Ok(t) => match t {
                false => Decision::NotApplicable,
                true => match self.evaluate_where_statement(auth_subscription) {
                    Err(_) => Decision::Indeterminate,
                    Ok(c) => match c {
                        false => Decision::NotApplicable,
                        true => Decision::entitlement(&self.entitlement),
                    },
                },
            },
        }
    }

    pub fn evaluate_where_statement(
        &self,
        _auth_subscription: &AuthorizationSubscription,
    ) -> Result<bool, String> {
        if let None = self.where_statements {
            return Ok(true);
        }

        //TODO evaluate every statement and combine the result
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_sapl_file;

    #[test]
    fn policy_parse_element_of() {
        let element_of = parse_sapl_file("policy \"doctor and nurse access to patient data\" permit action.java.name == \"findById\" where \"ROLE_DOCTOR\" in subject..authority || \"ROLE_NURSE\" in subject..authority;");
        assert!(element_of.is_ok());
    }
}
