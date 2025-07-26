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
use crate::delay::Delay;
use chrono::Local;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};
use tokio_stream::Stream;

pub struct Time {
    duration: Duration,
    delay: Delay,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(1),
            delay: Delay {
                when: Instant::now(),
            },
        }
    }
}

impl Time {
    pub fn new(update_interval_in_millis: u64) -> Self {
        Self {
            duration: Duration::from_millis(update_interval_in_millis),
            delay: Delay {
                when: Instant::now(),
            },
        }
    }

    pub fn now() -> Result<Val, String> {
        Ok(Val::String(Local::now().to_rfc3339()))
    }
}

impl Stream for Time {
    type Item = Result<Val, String>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Val, String>>> {
        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                let when = self.delay.when + self.duration;
                self.delay = Delay { when };
                //why rfc3339
                //https://docs.rs/chrono/0.4.19/chrono/struct.DateTime.html#method.parse_from_rfc3339
                Poll::Ready(Some(Ok(Val::String(Local::now().to_rfc3339()))))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
