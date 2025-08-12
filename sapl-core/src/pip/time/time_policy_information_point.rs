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
use chrono::Local;
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::time::{Interval, MissedTickBehavior, interval};
use tokio_stream::Stream;

pub struct Time {
    interval: Interval,
}

impl Default for Time {
    fn default() -> Self {
        let mut interval = interval(Duration::from_millis(1));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        Self { interval }
    }
}

impl Time {
    pub fn new(update_interval_in_millis: u64) -> Self {
        let mut interval = interval(Duration::from_millis(update_interval_in_millis));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        Self { interval }
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
        match self.interval.poll_tick(cx) {
            //why rfc3339
            //https://docs.rs/chrono/0.4.19/chrono/struct.DateTime.html#method.parse_from_rfc3339
            Poll::Ready(_) => Poll::Ready(Some(Ok(Val::String(Local::now().to_rfc3339())))),
            Poll::Pending => Poll::Pending,
        }
    }
}
