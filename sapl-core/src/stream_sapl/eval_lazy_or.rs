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
use crate::evaluate::eager_or;
use futures::{Stream, StreamExt, stream::Fuse};
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::result::Result;
use std::task::{Context, Poll};

pin_project! {
    /// Stream returned by the [`eval_lazy_or`] method.
    pub struct EvalLazyOr<T, U>
    {
        #[pin]
        a: Fuse<T>,
        #[pin]
        b: Fuse<U>,
        // save the last stream results
        lhs: Result<Val, String>,
        rhs: Result<Val, String>,
    }
}

impl<T, U> EvalLazyOr<T, U> {
    pub(super) fn new(a: T, b: U) -> EvalLazyOr<T, U>
    where
        T: Stream<Item = Result<Val, String>>,
        U: Stream<Item = T::Item>,
    {
        EvalLazyOr {
            a: a.fuse(),
            b: b.fuse(),
            lhs: Ok(Val::None),
            rhs: Ok(Val::None),
        }
    }
}

impl<T, U> Stream for EvalLazyOr<T, U>
where
    T: Stream<Item = Result<Val, String>>,
    U: Stream<Item = T::Item>,
{
    type Item = T::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        use Poll::*;

        let poll_lhs = self.as_mut().project().a.poll_next(cx);

        let need_rhs = poll_lhs == Poll::Ready(Some(Ok(Val::Boolean(false))));
        let poll_rhs = if need_rhs {
            self.as_mut().project().b.poll_next(cx)
        } else {
            Poll::Pending
        };

        match (poll_lhs, poll_rhs) {
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

        println!("lazy_or lhs: {:#?}", *self.as_ref().project_ref().lhs);
        println!("lazy_or rhs: {:#?}", *self.as_ref().project_ref().rhs);

        // if *self.as_ref().project_ref().lhs == Ok(Val::None)
        //     || *self.as_ref().project_ref().rhs == Ok(Val::None)
        // {
        //     return Pending;
        // }

        if *self.as_ref().project_ref().lhs == Ok(Val::Boolean(true)) {
            return Ready(Some(Ok(Val::Boolean(true))));
        }

        Ready(Some(eager_or(
            self.as_ref().project_ref().lhs,
            self.as_ref().project_ref().rhs,
        )))
    }
}
