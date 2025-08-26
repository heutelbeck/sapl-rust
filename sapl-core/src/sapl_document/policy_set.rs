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

use crate::{
    Ast, AuthorizationDecision, AuthorizationSubscription, CombiningAlgorithm, Decision, Policy,
    Rule, once_decision,
    sapl_document::combining_algorithm::{
        deny_overrides, deny_unless_permit, first_applicable, only_one_applicable,
        permit_overrides, permit_unless_deny,
    },
    stream_sapl::DecisionCombinedStream,
};
use futures::Stream;
use log::error;
use std::{pin::Pin, sync::Arc};

#[derive(Debug, Default)]
pub struct PolicySet {
    pub name: String,
    combining_algorithm: CombiningAlgorithm,
    target_exp: Option<Arc<Ast>>,
    policies: Vec<Policy>,
}

impl PolicySet {
    pub fn new(pairs: pest::iterators::Pairs<Rule>) -> Self {
        let mut policy_set = PolicySet::default();

        for pair in pairs {
            match pair.as_rule() {
                Rule::policy_set_name => {
                    let mut name = pair.as_str().to_string();
                    name.retain(|c| c != '\"');
                    policy_set.name = name;
                }
                Rule::combining_algorithm => {
                    policy_set.combining_algorithm = CombiningAlgorithm::new(pair.as_str())
                }
                Rule::target_expression => {
                    policy_set.target_exp = Some(Arc::new(Ast::parse(pair.clone().into_inner())))
                }
                Rule::policy => policy_set.policies.push(Policy::new(pair.into_inner())),
                rule => unreachable!(
                    "Sapl::parse expected policy_set_name, combining_algorithm or policy, found {rule:?}"
                ),
            }
        }

        policy_set
    }

    pub fn validate(&self) -> Result<(), String> {
        let mut result = String::new();

        for p in &self.policies {
            if let Err(e) = p.validate() {
                result.push_str(&e);
            }
        }

        if result.is_empty() {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn evaluate(&self, auth_sub: &AuthorizationSubscription) -> AuthorizationDecision {
        use CombiningAlgorithm::*;

        match self.evaluate_target_expr(auth_sub) {
            Err(e) => {
                error!("Err evaluate target expression: {e:#?}");
                Decision::Indeterminate.into()
            }
            Ok(false) => Decision::NotApplicable.into(),
            Ok(true) => {
                let decisions = self
                    .policies
                    .iter()
                    .map(|p| Some(p.evaluate(auth_sub)))
                    .collect::<Box<[Option<AuthorizationDecision>]>>();

                match &self.combining_algorithm {
                    DENY_OVERRIDES => deny_overrides(&decisions),
                    DENY_UNLESS_PERMIT => deny_unless_permit(&decisions),
                    FIRST_APPLICABLE => first_applicable(&decisions),
                    ONLY_ONE_APPLICABLE => only_one_applicable(&decisions),
                    PERMIT_OVERRIDES => permit_overrides(&decisions),
                    PERMIT_UNLESS_DENY => permit_unless_deny(&decisions),
                }
            }
        }
    }

    pub fn evaluate_as_stream(
        &self,
        auth_sub: &Arc<AuthorizationSubscription>,
    ) -> Pin<Box<dyn Stream<Item = AuthorizationDecision> + std::marker::Send>> {
        use CombiningAlgorithm::*;

        match self.evaluate_target_expr(auth_sub) {
            Err(e) => {
                error!("Err evaluate target expression: {e:#?}");
                Box::pin(once_decision(Decision::Indeterminate.into()))
            }
            Ok(false) => Box::pin(once_decision(Decision::NotApplicable.into())),
            Ok(true) => {
                let policy_streams = self
                    .policies
                    .iter()
                    .map(|p| p.evaluate_as_stream(auth_sub))
                    .collect();

                match &self.combining_algorithm {
                    DENY_OVERRIDES => {
                        Box::pin(DecisionCombinedStream::new(policy_streams, deny_overrides))
                    }
                    DENY_UNLESS_PERMIT => Box::pin(DecisionCombinedStream::new(
                        policy_streams,
                        deny_unless_permit,
                    )),
                    FIRST_APPLICABLE => Box::pin(DecisionCombinedStream::new(
                        policy_streams,
                        first_applicable,
                    )),
                    ONLY_ONE_APPLICABLE => Box::pin(DecisionCombinedStream::new(
                        policy_streams,
                        only_one_applicable,
                    )),
                    PERMIT_OVERRIDES => Box::pin(DecisionCombinedStream::new(
                        policy_streams,
                        permit_overrides,
                    )),
                    PERMIT_UNLESS_DENY => Box::pin(DecisionCombinedStream::new(
                        policy_streams,
                        permit_unless_deny,
                    )),
                }
            }
        }
    }

    fn evaluate_target_expr(&self, auth_sub: &AuthorizationSubscription) -> Result<bool, String> {
        match self.target_exp.as_ref() {
            Some(exp) => exp.evaluate(auth_sub),
            None => Ok(true),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_sapl_file;

    #[test]
    fn policy_set_validate_ok() {
        let ps = parse_sapl_file(
            "set \"demo\" first-applicable for action.java.name == \"getDocuments\" policy \"first\" permit where time.secondOf(<time.now>) < 20; policy \"second\" permit where time.secondOf(<time.now>) < 40; policy \"third\" permit",
        );
        assert!(ps.expect("parsing failed").validate().is_ok());
    }

    #[test]
    fn policy_set_valiate_target_expr_lazy_and() {
        let ps = parse_sapl_file(
            "set \"demo\" first-applicable for action.java.name == \"getDocuments\" policy \"first\" permit 5 < 6 && 4 > 3 policy \"second\" permit where time.secondOf(<time.now>) < 40; policy \"third\" permit",
        );

        assert!(ps.expect("parsing failed").validate().is_err());
    }

    #[test]
    fn policy_set_valiate_target_expr_lazy_or() {
        let ps = parse_sapl_file(
            "set \"demo\" first-applicable for action.java.name == \"getDocuments\" policy \"first\" permit permit where time.secondOf(<time.now>) < 20; policy \"second\" permit 5 < 6 || 4 > 3 policy \"third\" permit",
        );

        assert!(ps.expect("parsing failed").validate().is_err());
    }
}
