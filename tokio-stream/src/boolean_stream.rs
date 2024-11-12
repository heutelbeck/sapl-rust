/*
    Copyright 2024 Stefan Weng

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

use crate::{combine_eager::CombineEager, combine_lazy::CombineLazy, delay::Delay};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};
use tokio_stream::Stream;

pub struct BooleanInterval {
    state: bool,
    duration: Duration,
    delay: Delay,
}

impl BooleanInterval {
    pub fn new(duration: Duration) -> Self {
        Self {
            state: false,
            duration,
            delay: Delay {
                when: Instant::now(),
            },
        }
    }

    pub fn combine_eager<U>(self, other: U) -> CombineEager<Self, U>
    where
        U: Stream<Item = <BooleanInterval as Stream>::Item>,
    {
        CombineEager::new(self, other)
    }

    pub fn combine_lazy<U>(self, other: U) -> CombineLazy<Self, U>
    where
        U: Stream<Item = <BooleanInterval as Stream>::Item>,
    {
        CombineLazy::new(self, other)
    }
}

impl Stream for BooleanInterval {
    type Item = bool;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<bool>> {
        /*if self.state == false {
            // No more delays
            return Poll::Ready(None);
        }*/

        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                let when = self.delay.when + self.duration;
                self.delay = Delay { when };
                self.state = !self.state;
                Poll::Ready(Some(self.state))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
