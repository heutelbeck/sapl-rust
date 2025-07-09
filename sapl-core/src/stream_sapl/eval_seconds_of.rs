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
use chrono::Timelike;
use futures::{Stream, StreamExt, stream::Fuse};
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project! {
    /// Stream returned by the [`eval_seconds_of`] method.
    pub struct EvalSecondsOf<T> {
        #[pin]
        a: Fuse<T>,
    }
}

impl<T> EvalSecondsOf<T> {
    pub(super) fn new(a: T) -> EvalSecondsOf<T>
    where
        T: Stream<Item = Result<Val, String>>,
    {
        EvalSecondsOf { a: a.fuse() }
    }
}

impl<T> Stream for EvalSecondsOf<T>
where
    T: Stream<Item = Result<Val, String>>,
{
    type Item = T::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        use Poll::*;
        match self.as_mut().project().a.poll_next(cx) {
            Ready(Some(val)) => match val {
                Ok(Val::DateTime(dt)) => {
                    println!("eval_seconds_of debugging: {dt}");
                    Ready(Some(Ok(Val::Integer(dt.second().try_into().unwrap_or(0)))))
                }
                Err(e) => Ready(Some(Err(e))),
                others => Ready(Some(Err(format!(
                    "stream sapl seconds of for {others:#?} is not implemented"
                )))),
            },
            Pending => Pending,
            Ready(None) => Ready(None),
        }
    }
}
