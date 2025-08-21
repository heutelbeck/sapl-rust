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

use crate::Decision;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AuthorizationDecision {
    pub decision: Decision,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obligation: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advice: Option<Value>,
}

impl AuthorizationDecision {
    pub fn new(decision: Decision) -> Self {
        Self {
            decision,
            resource: None,
            obligation: None,
            advice: None,
        }
    }

    pub fn collect(&mut self, auth_decision: AuthorizationDecision) {
        self.add_obligation(auth_decision.obligation);
        self.add_advice(auth_decision.advice);
        self.add_resource(auth_decision.resource);
    }

    fn add_obligation(&mut self, obligation: Option<Value>) {
        if let Some(mut obligation) = obligation {
            if self.obligation.is_none() {
                self.obligation = Some(obligation);
                return;
            }

            if let Some(oblig) = self.obligation.as_mut().unwrap().as_array_mut()
                && let Some(new_obligation) = obligation.as_array_mut()
            {
                oblig.extend(new_obligation.iter().cloned());
            }
        }
    }

    fn add_advice(&mut self, advice: Option<Value>) {
        if let Some(mut advice) = advice {
            if self.advice.is_none() {
                self.advice = Some(advice);
                return;
            }

            if let Some(a) = self.advice.as_mut().unwrap().as_array_mut()
                && let Some(new_advice) = advice.as_array_mut()
            {
                a.extend(new_advice.iter().cloned());
            }
        }
    }

    fn add_resource(&mut self, resource: Option<Value>) {
        if resource.is_some() {
            match self.resource {
                Some(_) => {
                    self.resource = None;
                    self.decision = Decision::Deny;
                }
                None => {
                    self.resource = resource;
                }
            }
        }
    }

    pub fn set_decision(&mut self, decision: Decision) {
        self.decision = decision
    }
}

impl Default for AuthorizationDecision {
    fn default() -> Self {
        Self {
            decision: Decision::Deny,
            resource: None,
            obligation: None,
            advice: None,
        }
    }
}
