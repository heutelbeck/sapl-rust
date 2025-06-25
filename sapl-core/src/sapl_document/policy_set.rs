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

use crate::{Ast, AuthorizationSubscription, CombiningAlgorithm, Decision, Policy, Rule};
use async_stream::stream;
use futures::future;
use std::sync::Arc;
use tokio_stream::Stream;

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
                    "Sapl::parse expected policy_set_name, combining_algorithm or policy, found {:?}",
                    rule
                ),
            }
        }

        policy_set
    }

    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }

    pub fn evaluate(&self, _auth_subscription: &AuthorizationSubscription) -> Decision {
        Decision::NotApplicable
    }

    pub async fn evaluate_as_stream(
        &self,
        _auth_subscription: &AuthorizationSubscription,
        //) -> Box<dyn Stream<Item = Decision> + '_> {
    ) -> impl Stream<Item = Decision> + '_ {
        stream! {
            yield Decision::NotApplicable;
            future::pending::<()>().await;
        }
    }
}
