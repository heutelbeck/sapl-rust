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

use futures::Stream;
use std::sync::Arc;

use crate::{
    AuthorizationDecision, Policy, Val, authorization_subscription::AuthorizationSubscription,
    decision::DecisionStream,
};

mod once;
pub use once::{once_decision, once_val};

mod decision_combine_stream;
pub use decision_combine_stream::DecisionCombinedStream;

mod value_stream;
pub use value_stream::ValueStream;

mod eval_op;
pub(crate) use eval_op::EvalOp;

mod eval_seconds_of;
pub use eval_seconds_of::EvalSecondsOf;

pub trait StreamSapl: Stream<Item = Result<Val, String>> + Send {
    fn eval_op<U, F>(self, other: U, op_fn: F) -> EvalOp<Self, U, F>
    where
        F: Fn(&Result<Val, String>, &Result<Val, String>) -> Result<Val, String>,
        U: Stream<Item = Result<Val, String>> + Send,
        Self: Sized,
    {
        EvalOp::new(self, other, op_fn)
    }

    fn eval_to_decision(
        self,
        policy: Policy,
        auth_subscription: &Arc<AuthorizationSubscription>,
    ) -> DecisionStream<Self>
    where
        Self: Sized,
    {
        DecisionStream::new(self, policy, auth_subscription.clone())
    }

    fn eval_seconds_of(self) -> EvalSecondsOf<Self>
    where
        Self: Sized,
    {
        EvalSecondsOf::new(self)
    }
}

impl<S> StreamSaplDecision for S where S: Stream<Item = AuthorizationDecision> + std::marker::Send {}

pub trait StreamSaplDecision: Stream<Item = AuthorizationDecision> + Send {
    fn to_json_value(self) -> ValueStream<Self>
    where
        Self: Sized,
    {
        ValueStream::new(self)
    }
}
