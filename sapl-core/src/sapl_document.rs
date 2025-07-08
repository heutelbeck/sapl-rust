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

use crate::{AuthorizationSubscription, Decision, Import, Schema};
use std::pin::Pin;
use tokio_stream::Stream;

mod combining_algorithm;
pub use combining_algorithm::CombiningAlgorithm;

mod policy;
pub use policy::Policy;

mod policy_set;
pub use policy_set::PolicySet;

#[derive(Debug)]
pub struct SaplDocument {
    pub imports: Option<Vec<Import>>,
    pub schemas: Option<Vec<Schema>>,
    pub body: DocumentBody,
}

impl SaplDocument {
    pub fn evaluate(&self, auth_subscription: &AuthorizationSubscription) -> Decision {
        use DocumentBody::*;
        match &self.body {
            Policy(p) => p.evaluate(auth_subscription),
            PolicySet(ps) => ps.evaluate(auth_subscription),
        }
    }

    pub fn evaluate_as_stream(
        &self,
        auth_subscription: &AuthorizationSubscription,
    ) -> Pin<Box<dyn Stream<Item = Decision> + Send>> {
        use DocumentBody::*;
        match &self.body {
            Policy(p) => Box::pin(p.evaluate_as_stream(auth_subscription)),
            PolicySet(_) => panic!("not implemented"),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        let mut result_schema_validation: Result<(), String> = Ok(());
        if let Some(schemas) = &self.schemas {
            let schema_results: Vec<_> = schemas
                .iter()
                .filter_map(|s| s.validate(self.body.name()))
                .collect();
            if !schema_results.is_empty() {
                result_schema_validation = Err(schema_results.concat());
            }
        }

        match (result_schema_validation, &self.body.validate()) {
            (Ok(_), Ok(_)) => Ok(()),
            (Ok(_), Err(b)) => Err(b.to_string()),
            (Err(s), Ok(_)) => Err(s),
            (Err(s), Err(b)) => Err(s + b),
        }
    }

    pub fn name(&self) -> &str {
        match &self.body {
            DocumentBody::Policy(p) => &p.name,
            DocumentBody::PolicySet(ps) => &ps.name,
        }
    }
}

#[derive(Debug)]
pub enum DocumentBody {
    Policy(Policy),
    PolicySet(PolicySet),
}

impl DocumentBody {
    fn validate(&self) -> Result<(), String> {
        match &self {
            DocumentBody::Policy(p) => p.validate(),
            DocumentBody::PolicySet(ps) => ps.validate(),
        }
    }

    fn name(&self) -> &str {
        match &self {
            DocumentBody::Policy(p) => &p.name,
            DocumentBody::PolicySet(ps) => &ps.name,
        }
    }
}
