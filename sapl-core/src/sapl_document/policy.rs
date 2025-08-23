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

use log::error;
use serde_json::Value;
use std::pin::Pin;
use std::sync::Arc;
use tokio_stream::Stream;

use crate::AuthorizationDecision;
use crate::BoxedValStream;
use crate::Decision;
use crate::Entitlement;
use crate::Eval;
use crate::Rule;
use crate::Val;
use crate::ast::Ast;
use crate::authorization_subscription::AuthorizationSubscription;
use crate::evaluate::eager_and;
use crate::stream_sapl::StreamSapl;
use crate::{once_decision, once_val};

#[derive(Debug, Default, Clone)]
pub struct Policy {
    pub(crate) name: String,
    pub(crate) entitlement: Entitlement,
    target_exp: Option<Arc<Ast>>,
    where_statements: Option<Arc<Vec<Ast>>>,
    obligations: Option<Arc<Ast>>,
    advice: Option<Arc<Ast>>,
    transformation: Option<Arc<Ast>>,
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
                    policy.target_exp = Some(Arc::new(Ast::parse(pair.clone().into_inner())))
                }
                Rule::where_statement => {
                    policy.where_statements = Some(Arc::new(
                        pair.clone()
                            .into_inner()
                            .map(Ast::parse_where_statement)
                            .collect(),
                    ));
                }
                Rule::obligation => {
                    policy.obligations = Some(Arc::new(Ast::parse(pair.clone().into_inner())));
                }
                Rule::advice => {
                    policy.advice = Some(Arc::new(Ast::parse(pair.clone().into_inner())));
                }
                Rule::transformation => {
                    policy.transformation = Some(Arc::new(Ast::parse(pair.clone().into_inner())));
                }
                rule => {
                    unreachable!("Sapl::parse expected policy_name or entitlement, found {rule:?}")
                }
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
                        joined.push_str(&format!("* {e}\n"));
                    }
                    Err(joined)
                }
                None => Ok(()),
            },
            None => Ok(()),
        }
    }

    pub fn evaluate(&self, auth_subscription: &AuthorizationSubscription) -> AuthorizationDecision {
        let result = self.evaluate_decison(auth_subscription);
        match result {
            Decision::Permit => AuthorizationDecision {
                decision: result,
                resource: self.evaluate_transformation(auth_subscription),
                obligation: self.evaluate_obligation(auth_subscription),
                advice: self.evaluate_advice(auth_subscription),
            },
            Decision::Deny => AuthorizationDecision {
                decision: result,
                resource: None,
                obligation: self.evaluate_obligation(auth_subscription),
                advice: self.evaluate_advice(auth_subscription),
            },
            _ => AuthorizationDecision::new(result),
        }
    }

    pub fn evaluate_decison(&self, auth_subscription: &AuthorizationSubscription) -> Decision {
        // Target Expression    |   Condition   |   Decision
        //---------------------------------------------------------
        // false (not matching) |   don’t care  |   NOT_APPLICABLE
        // true (matching)      |   false       |   NOT_APPLICABLE
        // Error                |   don’t care  |   INDETERMINATE
        // true (matching)      |   Error       |   INDETERMINATE
        // true (matching)      |   true        |   (PERMIT or DENY)
        //
        // https://sapl.io/docs/3.0.0-SNAPSHOT/6_2_Policy/
        let target = match self.target_exp.as_ref() {
            Some(exp) => exp.evaluate(auth_subscription),
            None => Ok(true),
        };

        match target {
            Err(e) => {
                error!("Evaluate target expression: {e:#?}");
                Decision::Indeterminate
            }
            Ok(false) => Decision::NotApplicable,
            Ok(true) => match self.evaluate_where_statement(auth_subscription) {
                Err(e) => {
                    error!("Evaluate where statement: {e:#?}");
                    Decision::Indeterminate
                }
                Ok(false) => Decision::NotApplicable,
                Ok(true) => Decision::entitlement(&self.entitlement),
            },
        }
    }

    pub fn evaluate_where_statement(
        &self,
        auth_subscription: &AuthorizationSubscription,
    ) -> Result<bool, String> {
        let result = Ok(true);

        if self.where_statements.is_none() {
            return result;
        }

        for condition in self.where_statements.as_ref().unwrap().iter() {
            match condition.evaluate(auth_subscription) {
                Ok(true) => {}
                Ok(false) => return Ok(false),
                Err(e) => {
                    error!("Evaluate_where_statement got {e:#?}");
                    return Ok(false);
                }
            }
        }

        result
    }

    pub fn evaluate_as_stream(
        &self,
        auth_subscription: &Arc<AuthorizationSubscription>,
    ) -> Pin<Box<dyn Stream<Item = AuthorizationDecision> + std::marker::Send>> {
        // Target Expression    |   Condition   |   Decision
        //---------------------------------------------------------
        // false (not matching) |   don’t care  |   NOT_APPLICABLE
        // true (matching)      |   false       |   NOT_APPLICABLE
        // Error                |   don’t care  |   INDETERMINATE
        // true (matching)      |   Error       |   INDETERMINATE
        // true (matching)      |   true        |   (PERMIT or DENY)
        //
        // https://sapl.io/docs/3.0.0-SNAPSHOT/6_2_Policy/

        let target = match self.target_exp.as_ref() {
            Some(exp) => exp.evaluate(auth_subscription),
            None => Ok(true),
        };

        match target {
            Err(e) => {
                error!("Evaluate target expression: {e:#?}");
                Box::pin(once_decision(AuthorizationDecision::new(
                    Decision::Indeterminate,
                )))
            }
            Ok(false) => Box::pin(once_decision(AuthorizationDecision::new(
                Decision::NotApplicable,
            ))),
            Ok(true) => Box::pin(
                self.evaluate_where_as_stream(auth_subscription)
                    .eval_to_decision(self.clone(), auth_subscription),
            ),
        }
    }

    fn evaluate_where_as_stream(
        &self,
        auth_subscription: &AuthorizationSubscription,
    ) -> BoxedValStream {
        if self.where_statements.is_none() {
            return Box::pin(once_val(Val::Boolean(true)));
        }

        let mut eval_streams: Vec<_> = self
            .where_statements
            .as_ref()
            .unwrap()
            .iter()
            .map(|s| s.eval(auth_subscription))
            .collect();

        fn combine(first: BoxedValStream, streams: &mut Vec<BoxedValStream>) -> BoxedValStream {
            match streams.pop() {
                Some(s) => Box::pin(first.eval_op(combine(s, streams), eager_and)),
                None => Box::pin(first),
            }
        }

        combine(
            eval_streams
                .pop()
                .expect("Error evaluating where statement"),
            &mut eval_streams,
        )
    }

    pub(crate) fn evaluate_obligation(
        &self,
        auth_subscription: &AuthorizationSubscription,
    ) -> Option<Value> {
        if let Some(obligation) = &self.obligations
            && let Ok(Val::Json(obj)) = obligation.evaluate_inner(auth_subscription)
        {
            return Some(obj);
        }

        None
    }

    pub(crate) fn evaluate_advice(
        &self,
        auth_subscription: &AuthorizationSubscription,
    ) -> Option<Value> {
        if let Some(advice) = &self.advice
            && let Ok(Val::Json(obj)) = advice.evaluate_inner(auth_subscription)
        {
            return Some(obj);
        }

        None
    }

    pub(crate) fn evaluate_transformation(
        &self,
        auth_subscription: &AuthorizationSubscription,
    ) -> Option<Value> {
        if let Some(transform) = &self.transformation
            && let Ok(result) = transform.evaluate_inner(auth_subscription)
        {
            return Some(result.to_value());
        };

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_sapl_file;

    #[test]
    fn policy_parse_element_of() {
        let element_of = parse_sapl_file(
            "policy \"doctor and nurse access to patient data\" permit action.java.name == \"findById\" where \"ROLE_DOCTOR\" in subject..authority || \"ROLE_NURSE\" in subject..authority;",
        );
        assert!(element_of.is_ok());
    }
}
