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

use crate::AuthorizationDecision;
use futures::{Stream, StreamExt, stream::Fuse};
use pin_project_lite::pin_project;
use serde_json::Value;
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project! {
    /// Stream returned by the [`to_json_value`] method.
    pub struct ValueStream<T> {
        #[pin]
        a: Fuse<T>,
    }
}

impl<T> ValueStream<T> {
    pub(super) fn new(a: T) -> ValueStream<T>
    where
        T: Stream<Item = AuthorizationDecision>,
    {
        ValueStream { a: a.fuse() }
    }
}

impl<T> Stream for ValueStream<T>
where
    T: Stream<Item = AuthorizationDecision>,
{
    type Item = Value;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        use Poll::*;
        match self.as_mut().project().a.poll_next(cx) {
            Ready(Some(decision)) => Ready(Some(
                serde_json::to_value(decision)
                    .expect("Failed to serialize AuthorizationDecision to JSON"),
            )),
            Pending => Pending,
            Ready(None) => Ready(None),
        }
    }
}
