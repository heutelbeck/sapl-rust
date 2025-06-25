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
    pub struct DenyUnlessPermit {
        #[pin]
        streams: Vec<Pin<Box<dyn Stream<Item = Decision> + Send>>>,
        decisions: Vec<Option<Decision>>,
        first_decision_done: bool,
    }
}

impl DenyUnlessPermit {
    pub fn new(streams: Vec<Pin<Box<dyn Stream<Item = Decision> + Send>>>) -> Self {
        DenyUnlessPermit {
            decisions: (0..streams.len()).map(|_| None::<Decision>).collect(),
            streams,
            first_decision_done: false,
        }
    }

    fn is_first_decision_done(&mut self) -> bool {
        if !self.first_decision_done {
            for d in &self.decisions {
                if d.is_none() {
                    self.first_decision_done = false;
                    return false;
                }
            }
            true
        } else {
            true
        }
    }

    fn get_decision(&self) -> &str {
        for d in &self.decisions {
            if *d == Some(Decision::Permit) {
                return r#"{"decision": "PERMIT"}"#;
            }
        }
        r#"{"decision": "DENY"}"#
    }
}

impl Stream for DenyUnlessPermit {
    type Item = Value;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        //Check all streams, emit the first result after all decisions emit the first
        //result and after that update the decision on every new result
        use Poll::*;

        let mut this = self.as_mut().project();
        let len = this.streams.as_ref().len();

        for i in 0..len {
            let s = this.streams.as_mut().get_mut()[i].as_mut();
            match s.poll_next(cx) {
                Ready(Some(decision)) => {
                    this.decisions[i] = Some(decision);
                }
                Ready(None) => panic!(),
                Pending => panic!(),
            }
        }

        if self.is_first_decision_done() {
            Ready(Some(serde_json::from_str(self.get_decision()).unwrap()))
        } else {
            Ready(None) //Pending
        }
    }
}
