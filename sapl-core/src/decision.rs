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

use crate::Entitlement;
use crate::Val;
use futures::Stream;
use pin_project_lite::pin_project;
use std::fmt::Display;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Clone, Debug, PartialEq)]
pub enum Decision {
    Permit,
    Deny,
    Indeterminate,
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
        entitlement: Entitlement,
    }
}

impl<T> DecisionStream<T> {
    pub(super) fn new(a: T, entitlement: Entitlement) -> DecisionStream<T>
    where
        T: Stream<Item = Result<Val, String>>,
    {
        DecisionStream { a, entitlement }
    }
}

impl<T> Stream for DecisionStream<T>
where
    T: Stream<Item = Result<Val, String>>,
{
    type Item = Decision;

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
                Ok(Val::Boolean(true)) => Ready(Some(Decision::entitlement(&self.entitlement))),
                Ok(Val::Boolean(false)) => Ready(Some(Decision::NotApplicable)),
                Err(e) => {
                    println!("Err evaluate where statement: {e:#?}");
                    Ready(Some(Decision::Indeterminate))
                }
                _ => {
                    println!("Err evaluate where statement: {val:#?}");
                    Ready(Some(Decision::Indeterminate))
                }
            },
            Pending => Pending,
            Ready(None) => Ready(None),
        }
    }
}
