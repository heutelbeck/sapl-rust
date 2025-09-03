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

use crate::{AuthorizationDecision, Entitlement, Policy, Val};
use futures::Stream;
use log::error;
use pin_project_lite::pin_project;
use serde::Serialize;
use serde_json::Value;
use std::{
    fmt::Display,
    pin::Pin,
    sync::{Arc, RwLock},
    task::{Context, Poll},
};

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
        variable_context: Arc<RwLock<Value>>,
    }
}

impl<T> DecisionStream<T> {
    pub(super) fn new(
        a: T,
        policy: Policy,
        variable_context: Arc<RwLock<Value>>,
    ) -> DecisionStream<T>
    where
        T: Stream<Item = Result<Val, String>>,
    {
        DecisionStream {
            a,
            policy,
            variable_context,
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
                        Entitlement::Permit => Ready(Some(AuthorizationDecision::new(
                            Decision::entitlement(&self.policy.entitlement),
                            self.policy
                                .evaluate_transformation(self.variable_context.clone()),
                            self.policy
                                .evaluate_obligation(self.variable_context.clone()),
                            self.policy.evaluate_advice(self.variable_context.clone()),
                        ))),
                        Entitlement::Deny => Ready(Some(AuthorizationDecision::new(
                            Decision::entitlement(&self.policy.entitlement),
                            None,
                            self.policy
                                .evaluate_obligation(self.variable_context.clone()),
                            self.policy.evaluate_advice(self.variable_context.clone()),
                        ))),
                    }
                }
                Ok(Val::Boolean(false)) | Ok(Val::CompInteger(false, _)) => {
                    Ready(Some(Decision::NotApplicable.into()))
                }
                Err(e) => {
                    error!("Err evaluate where statement: {e:#?}");
                    Ready(Some(Decision::Indeterminate.into()))
                }
                _ => {
                    error!("Err evaluate where statement: {val:#?}");
                    Ready(Some(Decision::Indeterminate.into()))
                }
            },
            Pending => Pending,
            Ready(None) => Ready(None),
        }
    }
}
