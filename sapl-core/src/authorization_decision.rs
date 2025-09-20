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
    pub obligations: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advice: Option<Vec<Value>>,
}

impl AuthorizationDecision {
    pub fn new(
        decision: Decision,
        resource: Option<Value>,
        obligations: Option<Value>,
        advice: Option<Value>,
    ) -> Self {
        Self {
            decision,
            resource,
            obligations: match obligations {
                Some(obj) => Some(vec![obj]),
                None => None,
            },
            advice: match advice {
                Some(obj) => Some(vec![obj]),
                None => None,
            },
        }
    }

    pub fn collect(&mut self, auth_decision: AuthorizationDecision) {
        self.add_obligations(auth_decision.obligations);
        self.add_advice(auth_decision.advice);
        self.add_resource(auth_decision.resource);
    }

    fn add_obligations(&mut self, obligations: Option<Vec<Value>>) {
        if let Some(obligations) = obligations {
            if self.obligations.is_none() {
                self.obligations = Some(obligations);
                return;
            }

            if let Some(oblig) = self.obligations.as_mut() {
                oblig.append(&mut obligations.clone());
            }
        }
    }

    fn add_advice(&mut self, advice: Option<Vec<Value>>) {
        if let Some(advice) = advice {
            if self.advice.is_none() {
                self.advice = Some(advice);
                return;
            }

            if let Some(a) = self.advice.as_mut() {
                a.append(&mut advice.clone());
            }
        }
    }

    fn add_resource(&mut self, resource: Option<Value>) {
        if resource.is_some() {
            match self.resource {
                Some(_) => {
                    self.resource = None;
                    self.decision = Decision::Indeterminate;
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
            obligations: None,
            advice: None,
        }
    }
}

impl From<Decision> for AuthorizationDecision {
    fn from(decision: Decision) -> Self {
        Self {
            decision,
            ..Self::default()
        }
    }
}

impl From<AuthorizationDecision> for Value {
    fn from(value: AuthorizationDecision) -> Self {
        serde_json::to_value(value).expect("Failed to serialize AuthorizationDecision to JSON")
    }
}

impl std::fmt::Display for AuthorizationDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Decision: {}", self.decision)?;

        if let Some(resource) = &self.resource {
            write!(f, ", Resource: {}", resource)?;
        }

        if let Some(obligations) = &self.obligations
            && !obligations.is_empty()
        {
            let obligations_str = obligations
                .iter()
                .map(|o| o.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            write!(f, ", Obligations: [{}]", obligations_str)?;
        }

        if let Some(advice) = &self.advice
            && !advice.is_empty()
        {
            let advice_str = advice
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            write!(f, ", Advice: [{}]", advice_str)?;
        }

        Ok(())
    }
}
