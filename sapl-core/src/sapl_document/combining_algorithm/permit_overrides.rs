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

use crate::{AuthorizationDecision, Decision};
use futures::Stream;
use pin_project_lite::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pin_project! {
    pub struct PermitOverrides {
        #[pin]
        streams: Vec<Pin<Box<dyn Stream<Item = AuthorizationDecision> + Send>>>,
        decisions: Vec<Poll<Option<AuthorizationDecision>>>,
        first_decision_done: bool,
    }
}

impl PermitOverrides {
    pub fn new(streams: Vec<Pin<Box<dyn Stream<Item = AuthorizationDecision> + Send>>>) -> Self {
        PermitOverrides {
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

    fn evaluate(
        &mut self,
        results: Vec<Poll<Option<AuthorizationDecision>>>,
    ) -> Poll<Option<AuthorizationDecision>> {
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

        let mut indeterminate = false;
        let mut deny = false;
        let mut auth_decision = AuthorizationDecision::new(Decision::NotApplicable);
        for d in &self.decisions {
            use Poll::*;
            if let Ready(Some(r)) = d {
                match r.decision {
                    Decision::Permit => return d.clone(),
                    Decision::Indeterminate => {
                        indeterminate = true;
                    }
                    Decision::Deny => {
                        deny = true;
                        auth_decision.collect(r.clone());
                    }
                    _ => {}
                }
            }
        }

        if indeterminate {
            auth_decision.set_decision(Decision::Indeterminate);
            return Poll::Ready(Some(auth_decision));
        }

        if deny {
            auth_decision.set_decision(Decision::Deny);
            return Poll::Ready(Some(auth_decision));
        }

        Poll::Ready(Some(auth_decision))
    }
}

impl Stream for PermitOverrides {
    type Item = AuthorizationDecision;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        //Check all streams, emit the first result after all decisions emit the first
        //result and after that update the decision on every new result
        use Poll::*;

        let mut this = self.as_mut().project();
        let len = this.streams.as_ref().len();
        let mut results: Vec<Poll<Option<AuthorizationDecision>>> =
            Vec::with_capacity(this.decisions.len());

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
                Ready(Some(decision)) => Ready(Some(decision)),
                Ready(None) => Ready(None),
                Pending => Pending,
            }
        } else {
            Pending
        }
    }
}
