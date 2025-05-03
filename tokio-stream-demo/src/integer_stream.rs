/*inter
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
pub struct IntegerInterval {
    value: i64,
    state: bool,
    duration: Duration,
    delay: Delay,
}

impl IntegerInterval {
    pub fn new(value: i64, duration: Duration) -> Self {
        Self {
            value,
            state: false,
            duration,
            delay: Delay {
                when: Instant::now(),
            },
        }
    }
}

impl Stream for IntegerInterval {
    type Item = Val;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                let when = self.delay.when + self.duration;
                self.delay = Delay { when };
                self.state = !self.state;
                self.value = self.value.wrapping_add(1);
                Poll::Ready(Some(Val::Integer(self.value)))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
