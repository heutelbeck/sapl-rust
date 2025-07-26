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

use crate::{AuthorizationDecision, Decision, Val};

use std::pin::Pin;
use std::task::{Context, Poll};
use tokio_stream::Stream;

// Function that creates a one-shot stream from a given Val
pub fn once_val(value: Val) -> impl Stream<Item = Result<Val, String>> {
    pub struct OnceValStream {
        value: Option<Result<Val, String>>,
    }

    impl Stream for OnceValStream {
        type Item = Result<Val, String>;

        fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Poll::Ready(self.value.take())
        }
    }

    OnceValStream {
        value: Some(Ok(value)),
    }
}

// Function that creates a one-shot stream from a given Decision
pub fn once_decision(value: Decision) -> impl Stream<Item = AuthorizationDecision> {
    pub struct OnceDecisionStream {
        value: Decision,
    }

    impl Stream for OnceDecisionStream {
        type Item = AuthorizationDecision;

        fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Poll::Ready(Some(AuthorizationDecision::new(self.value.clone())))
        }
    }

    OnceDecisionStream { value }
}
