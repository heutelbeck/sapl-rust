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

use crate::authorization_subscription::AuthorizationSubscription;
use crate::{AuthorizationDecision, Entitlement, Policy, Val};
use futures::Stream;
use pin_project_lite::pin_project;
use serde::Serialize;
use std::fmt::Display;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Decision {
    #[serde(rename = "PERMIT")]
    Permit,
    #[serde(rename = "DENY")]
    Deny,
    #[serde(rename = "INDETERMINATE")]
    Indeterminate,
    #[serde(rename = "NOT_APPLICABLE")]
    NotApplicable,
}

impl Decision {
    pub fn entitlement(e: &Entitlement) -> Self {
        match e {
            Entitlement::Deny => Decision::Deny,
            Entitlement::Permit => Decision::Permit,
        }
    }
}

impl Display for Decision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let decision_str = match self {
            Decision::Permit => "PERMIT",
            Decision::Deny => "DENY",
            Decision::Indeterminate => "INDETERMINATE",
            Decision::NotApplicable => "NOT_APPLICABLE",
        };
        write!(f, "{decision_str}")
    }
}

pin_project! {
    /// Stream returned by the [`eval_decision`] method.
    pub struct DecisionStream<T> {
        #[pin]
        a: T,
        policy: Policy,
        auth_subscription: Arc<AuthorizationSubscription>,
    }
}

impl<T> DecisionStream<T> {
    pub(super) fn new(
        a: T,
        policy: Policy,
        auth_subscription: Arc<AuthorizationSubscription>,
    ) -> DecisionStream<T>
    where
        T: Stream<Item = Result<Val, String>>,
    {
        DecisionStream {
            a,
            policy,
            auth_subscription,
        }
    }
}

impl<T> Stream for DecisionStream<T>
where
    T: Stream<Item = Result<Val, String>>,
{
    type Item = AuthorizationDecision;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Condition   |   Decision
        //--------------------------------
        // true        |   (PERMIT or DENY)
        // false       |   NOT_APPLICABLE
        // others      |   INDETERMINATE
        // Error       |   INDETERMINATE
        //
        // https://sapl.io/docs/3.0.0-SNAPSHOT/6_2_Policy/
        use Poll::*;

        match self.as_mut().project().a.poll_next(cx) {
            Ready(Some(val)) => match val {
                Ok(Val::Boolean(true)) | Ok(Val::CompInteger(true, _)) => {
                    match &self.policy.entitlement {
                        Entitlement::Permit => Ready(Some(AuthorizationDecision {
                            decision: Decision::entitlement(&self.policy.entitlement),
                            resource: self.policy.evaluate_transformation(&self.auth_subscription),
                            obligation: self.policy.evaluate_obligation(&self.auth_subscription),
                            advice: self.policy.evaluate_advice(&self.auth_subscription),
                        })),
                        Entitlement::Deny => Ready(Some(AuthorizationDecision {
                            decision: Decision::entitlement(&self.policy.entitlement),
                            resource: None,
                            obligation: self.policy.evaluate_obligation(&self.auth_subscription),
                            advice: self.policy.evaluate_advice(&self.auth_subscription),
                        })),
                    }
                }
                Ok(Val::Boolean(false)) | Ok(Val::CompInteger(false, _)) => {
                    Ready(Some(AuthorizationDecision::new(Decision::NotApplicable)))
                }
                Err(e) => {
                    println!("Err evaluate where statement: {e:#?}");
                    Ready(Some(AuthorizationDecision::new(Decision::Indeterminate)))
                }
                _ => {
                    println!("Err evaluate where statement: {val:#?}");
                    Ready(Some(AuthorizationDecision::new(Decision::Indeterminate)))
                }
            },
            Pending => Pending,
            Ready(None) => Ready(None),
        }
    }
}
