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

use crate::Val;
use futures::{stream::Fuse, Stream, StreamExt};
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project! {
    /// Stream returned by the [`eval_and`] method.
    pub struct EvalAnd<T, U> {
        #[pin]
        a: Fuse<T>,
        #[pin]
        b: Fuse<U>,
        // save the last stream results
        lhs: Result<Val, String>,
        rhs: Result<Val, String>,

    }
}

impl<T, U> EvalAnd<T, U> {
    pub(super) fn new(a: T, b: U) -> EvalAnd<T, U>
    where
        T: Stream<Item = Result<Val, String>>,
        U: Stream<Item = T::Item>,
    {
        EvalAnd {
            a: a.fuse(),
            b: b.fuse(),
            lhs: Ok(Val::None),
            rhs: Ok(Val::None),
        }
    }
}

impl<T, U> Stream for EvalAnd<T, U>
where
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
                *self.as_mut().project().lhs = val1.clone();
                *self.as_mut().project().rhs = val2.clone();

                Ready(Some(equal_and(val1, val2)))
            }
            (Ready(Some(val1)), Pending) => {
                *self.as_mut().project().lhs = val1.clone();

                Ready(Some(equal_and(val1, self.as_mut().project().rhs.clone())))
            }
            (Pending, Ready(Some(val2))) => {
                *self.as_mut().project().rhs = val2.clone();

                Ready(Some(equal_and(self.as_mut().project().lhs.clone(), val2)))
            }
            (Ready(Some(val1)), Ready(None)) => {
                *self.as_mut().project().lhs = val1.clone();

                Ready(Some(equal_and(val1, self.as_mut().project().rhs.clone())))
            }
            (Ready(None), Ready(Some(val2))) => {
                *self.as_mut().project().rhs = val2.clone();

                Ready(Some(equal_and(self.as_mut().project().lhs.clone(), val2)))
            }
            (Ready(None), Ready(None)) => Ready(None),
            (_, _) => Pending,
        }
    }
}

fn equal_and(lhs: Result<Val, String>, rhs: Result<Val, String>) -> Result<Val, String> {
    use Val::*;
    match (lhs, rhs) {
        (Ok(Boolean(l)), Ok(Boolean(r))) => Ok(Boolean(l && r)),
        (Err(e), _) => Err(e),
        (_, Err(e)) => Err(e),
        (lhs, rhs) => Err(format!(
            "stream sapl EvalEq for {:#?} and {:#?} is not implemented",
            lhs, rhs,
        )),
    }
}
