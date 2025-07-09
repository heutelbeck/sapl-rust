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

use futures::Stream;
use pin_project_lite::pin_project;
use sapl_core::Decision;
use serde_json::Value;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pin_project! {
    pub struct PermitUnlessDeny {
        #[pin]
        streams: Vec<Pin<Box<dyn Stream<Item = Decision> + Send>>>,
        decisions: Vec<Poll<Option<Decision>>>,
        first_decision_done: bool,
    }
}

impl PermitUnlessDeny {
    pub fn new(streams: Vec<Pin<Box<dyn Stream<Item = Decision> + Send>>>) -> Self {
        PermitUnlessDeny {
            decisions: (0..streams.len()).map(|_| Poll::Pending).collect(),
            streams,
            first_decision_done: false,
        }
    }

    fn is_first_decision_done(&mut self) -> bool {
        if !self.first_decision_done {
            for d in &self.decisions {
                if d.is_pending() {
                    return false;
                }
            }
            self.first_decision_done = true;
            true
        } else {
            true
        }
    }

    fn finished(&self) -> bool {
        let mut finished = true;
        for r in &self.decisions {
            match r {
                Poll::Ready(None) => {}
                _ => {
                    finished = false;
                    break;
                }
            }
        }

        finished
    }

    fn evaluate(&mut self, results: Vec<Poll<Option<Decision>>>) -> Poll<Option<Decision>> {
        let mut evaluation_needed = true;
        for (i, result) in results.into_iter().enumerate() {
            if result.is_pending() {
                evaluation_needed = false;
            }
            if self.decisions[i] != result && !result.is_pending() {
                self.decisions[i] = result;
            }
        }

        if !evaluation_needed {
            return if self.finished() {
                Poll::Ready(None)
            } else {
                Poll::Pending
            };
        }

        for d in &self.decisions {
            if *d == Poll::Ready(Some(Decision::Deny)) {
                return Poll::Ready(Some(Decision::Deny));
            }
        }
        Poll::Ready(Some(Decision::Permit))
    }
}

impl Stream for PermitUnlessDeny {
    type Item = Value;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        //Check all streams, emit the first result after all decisions emit the first
        //result and after that update the decision on every new result
        use Poll::*;

        let mut this = self.as_mut().project();
        let len = this.streams.as_ref().len();
        let mut results: Vec<Poll<Option<Decision>>> = Vec::with_capacity(this.decisions.len());

        for i in 0..len {
            let s = this.streams.as_mut().get_mut()[i].as_mut();

            results.push(if this.decisions[i] != Ready(None) {
                s.poll_next(cx)
            } else {
                Ready(None)
            });
        }

        if !self.is_first_decision_done() {
            for (i, result) in results.clone().into_iter().enumerate() {
                if self.as_mut().project().decisions[i] != result && !result.is_pending() {
                    self.as_mut().project().decisions[i] = result;
                }
            }
        }

        if self.is_first_decision_done() {
            match self.evaluate(results) {
                Ready(Some(Decision::Deny)) => Ready(Some(
                    serde_json::from_str(r#"{"decision": "DENY"}"#).unwrap(),
                )),
                Ready(Some(Decision::Permit)) => Ready(Some(
                    serde_json::from_str(r#"{"decision": "PERMIT"}"#).unwrap(),
                )),
                Ready(Some(Decision::NotApplicable)) => Ready(Some(
                    serde_json::from_str(r#"{"decision": "NOT_APPLICABLE"}"#).unwrap(),
                )),
                Ready(Some(Decision::Indeterminate)) => Ready(Some(
                    serde_json::from_str(r#"{"decision": "INDETERMINATE"}"#).unwrap(),
                )),
                Ready(None) => Ready(None),
                Pending => Pending,
            }
        } else {
            Pending
        }
    }
}
