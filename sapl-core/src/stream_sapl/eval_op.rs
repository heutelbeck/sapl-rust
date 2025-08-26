/*
    Copyright 2025 Stefan Weng

    Licensed under the Apache License, Version 2.0 (the "License"); you may not
    use self.as_mut().project() file except in compliance with the License. You may obtain a copy
    of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
    WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
    License for the specific language governing permissions eager_and limitations
    under the License.
*/

use crate::Val;
use futures::{Stream, StreamExt, stream::Fuse};
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::result::Result;
use std::task::{Context, Poll};

pin_project! {
    /// Stream returned by the [`eval_op`] method.
    pub struct EvalOp<T, U, F>
    where
        F: Fn(&Result<Val, String>, &Result<Val, String>) -> Result<Val, String>,
    {
        #[pin]
        a: Fuse<T>,
        #[pin]
        b: Fuse<U>,
        // save the last stream results
        lhs: Result<Val, String>,
        rhs: Result<Val, String>,
        op_fn: F,
    }
}

impl<T, U, F> EvalOp<T, U, F>
where
    F: Fn(&Result<Val, String>, &Result<Val, String>) -> Result<Val, String>,
{
    pub(super) fn new(a: T, b: U, op_fn: F) -> EvalOp<T, U, F>
    where
        T: Stream<Item = Result<Val, String>>,
        U: Stream<Item = T::Item>,
        F: Fn(&Result<Val, String>, &Result<Val, String>) -> Result<Val, String>,
    {
        EvalOp {
            a: a.fuse(),
            b: b.fuse(),
            lhs: Ok(Val::None),
            rhs: Ok(Val::None),
            op_fn,
        }
    }
}

impl<T, U, F> Stream for EvalOp<T, U, F>
where
    F: Fn(&Result<Val, String>, &Result<Val, String>) -> Result<Val, String>,
    T: Stream<Item = Result<Val, String>>,
    U: Stream<Item = T::Item>,
{
    type Item = T::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        use Poll::*;

        match (
            self.as_mut().project().a.poll_next(cx),
            self.as_mut().project().b.poll_next(cx),
        ) {
            (Ready(Some(val1)), Ready(Some(val2))) => {
                *self.as_mut().project().lhs = val1;
                *self.as_mut().project().rhs = val2;
            }
            (Ready(Some(val1)), Pending) => {
                *self.as_mut().project().lhs = val1;
            }
            (Pending, Ready(Some(val2))) => {
                *self.as_mut().project().rhs = val2;
            }
            (Ready(Some(val1)), Ready(None)) => {
                *self.as_mut().project().lhs = val1;
            }
            (Ready(None), Ready(Some(val2))) => {
                *self.as_mut().project().rhs = val2;
            }
            (Ready(None), Ready(None)) => return Ready(None),
            (_, _) => return Pending,
        }

        if *self.as_ref().project_ref().lhs == Ok(Val::None)
            || *self.as_ref().project_ref().rhs == Ok(Val::None)
        {
            return Pending;
        }

        Ready(Some((self.op_fn)(
            self.as_ref().project_ref().lhs,
            self.as_ref().project_ref().rhs,
        )))
    }
}
