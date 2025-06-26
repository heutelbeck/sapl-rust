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

use crate::delay::Delay;
use crate::stream_sapl::StreamSapl;
use crate::val::Val;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};
use tokio_stream::Stream;

#[derive(PartialEq, Debug, Clone)]
pub struct BooleanIntervalNew {
    state: bool,
    duration: Duration,
    delay: Delay,
}

impl BooleanIntervalNew {
    pub fn new(duration: Duration) -> Self {
        Self {
            state: false,
            duration,
            delay: Delay {
                when: Instant::now(),
            },
        }
    }
}

impl Stream for BooleanIntervalNew {
    type Item = Val;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                let when = self.delay.when + self.duration;
                self.delay = Delay { when };
                self.state = !self.state;
                Poll::Ready(Some(Val::Boolean(self.state)))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
