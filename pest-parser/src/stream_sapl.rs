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

use crate::{decision::DecisionStream, Entitlement, Val};

mod once;
pub use once::{once_decision, once_val};

mod eval_eq;
pub use eval_eq::EvalEq;

mod eval_le;
pub use eval_le::EvalLe;

mod eval_add;
pub use eval_add::EvalAdd;

mod eval_sub;
pub use eval_sub::EvalSub;

mod eval_and;
pub use eval_and::EvalAnd;

mod eval_ge;
pub use eval_ge::EvalGe;

mod eval_seconds_of;
pub use eval_seconds_of::EvalSecondsOf;

pub trait StreamSapl: Stream<Item = Result<Val, String>> + Send {
    fn eval_eq<U>(self, other: U) -> EvalEq<Self, U>
    where
        U: Stream<Item = Result<Val, String>> + Send,
        Self: Sized,
    {
        EvalEq::new(self, other)
    }

    fn eval_le<U>(self, other: U) -> EvalLe<Self, U>
    where
        U: Stream<Item = Result<Val, String>> + Send,
        Self: Sized,
    {
        EvalLe::new(self, other)
    }

    fn eval_add<U>(self, other: U) -> EvalAdd<Self, U>
    where
        U: Stream<Item = Result<Val, String>> + Send,
        Self: Sized,
    {
        EvalAdd::new(self, other)
    }

    fn eval_sub<U>(self, other: U) -> EvalSub<Self, U>
    where
        U: Stream<Item = Result<Val, String>> + Send,
        Self: Sized,
    {
        EvalSub::new(self, other)
    }

    fn eval_and<U>(self, other: U) -> EvalAnd<Self, U>
    where
        U: Stream<Item = Result<Val, String>> + Send,
        Self: Sized,
    {
        EvalAnd::new(self, other)
    }

    fn eval_ge<U>(self, other: U) -> EvalGe<Self, U>
    where
        U: Stream<Item = Result<Val, String>> + Send,
        Self: Sized,
    {
        EvalGe::new(self, other)
    }

    fn eval_to_decision(self, entitlement: Entitlement) -> DecisionStream<Self>
    where
        Self: Sized,
    {
        DecisionStream::new(self, entitlement)
    }

    fn eval_seconds_of(self) -> EvalSecondsOf<Self>
    where
        Self: Sized,
    {
        EvalSecondsOf::new(self)
    }
}
