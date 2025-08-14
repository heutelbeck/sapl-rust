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
use futures::Stream;
use pin_project_lite::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pin_project! {
    pub struct DecisionCombinedStream<F>
    where
        F: Fn(&[Option<AuthorizationDecision>]) -> AuthorizationDecision,
    {
        #[pin]
        streams: Vec<Pin<Box<dyn Stream<Item = AuthorizationDecision> + Send>>>,
        decisions: Vec<Option<AuthorizationDecision>>,
        combine_fn: F,
        has_initial_emission: bool,
    }
}

impl<F> DecisionCombinedStream<F>
where
    F: Fn(&[Option<AuthorizationDecision>]) -> AuthorizationDecision + Send,
{
    pub fn new(
        streams: Vec<Pin<Box<dyn Stream<Item = AuthorizationDecision> + Send>>>,
        combine_fn: F,
    ) -> Self {
        let len = streams.len();
        Self {
            streams,
            decisions: vec![None; len],
            combine_fn,
            has_initial_emission: false,
        }
    }
}

impl<F> Stream for DecisionCombinedStream<F>
where
    F: Fn(&[Option<AuthorizationDecision>]) -> AuthorizationDecision + Send,
{
    type Item = AuthorizationDecision;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.as_mut().project();
        let len = this.streams.len();
        let mut any_stream_changed = false;
        let mut all_streams_ended;

        // 🔄 STEP 1: Always poll all streams in a loop to consume ALL available data
        loop {
            let mut made_progress = false;
            all_streams_ended = true;

            for i in 0..len {
                // Keep polling each stream until it returns Pending
                loop {
                    match this.streams[i].as_mut().poll_next(cx) {
                        Poll::Ready(Some(new_decision)) => {
                            all_streams_ended = false;
                            made_progress = true;

                            // Check if this is actually a change
                            let is_change = match &this.decisions[i] {
                                Some(old_decision) => *old_decision != new_decision,
                                None => true, // First value is always a change
                            };

                            if is_change {
                                println!("📥 Stream {i} changed: {new_decision:?}");
                                this.decisions[i] = Some(new_decision);
                                any_stream_changed = true;
                                break;
                            } else {
                                println!("📥 Stream {i} same value: {new_decision:?}");
                            }
                            continue; // Keep polling this stream
                        }
                        Poll::Ready(None) => {
                            // Stream ended
                            if this.decisions[i].is_some() {
                                println!("❌ Stream {i} ended");
                                made_progress = true;
                                this.decisions[i] = None;
                            }
                            break; // This stream is done
                        }
                        Poll::Pending => {
                            all_streams_ended = false;
                            break; // This stream has no more data right now
                        }
                    }
                }
            }

            if !made_progress || any_stream_changed || all_streams_ended {
                break;
            }
        }

        // 🎯 STEP 2: Decide if we should emit
        let should_emit = if !*this.has_initial_emission {
            // For initial emission: emit as soon as we have ANY data
            let has_data = this.decisions.iter().any(|d| d.is_some());
            if has_data {
                println!("🎉 Initial emission condition met");
            }
            has_data
        } else {
            // After initial: emit only on actual changes
            if any_stream_changed {
                println!("🔄 Change detected, should emit");
            }
            any_stream_changed
        };

        if should_emit {
            if !*this.has_initial_emission {
                *this.has_initial_emission = true;
                println!(
                    "🎉 Initial emission with {} decisions",
                    this.decisions.iter().filter(|d| d.is_some()).count()
                );
            }

            // 🔧 STEP 3: Combine the decisions
            let combined_decision = (this.combine_fn)(this.decisions);
            println!("📤 Emitting combined decision: {combined_decision:?}");
            return Poll::Ready(Some(combined_decision));
        }

        // 🏁 STEP 4: Check if all streams are done
        if all_streams_ended {
            println!("❌ All streams ended");
            return Poll::Ready(None);
        }

        // ⏳ No changes right now, but streams are still active
        println!("⏳ No changes, returning Pending (streams still active)");
        Poll::Pending
    }
}
