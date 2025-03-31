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
use serde_json::Value;
use std::collections::VecDeque;

use crate::basic_identifier_expression::BasicIdentifierExpression;

#[derive(Debug)]
pub struct AuthorizationSubscription {
    subject: Value,
    action: Value,
    resource: Value,
    environment: Option<Value>,
}

impl AuthorizationSubscription {
    pub fn new_example_subscription1() -> Self {
        AuthorizationSubscription {
            subject: serde_json::from_str(r#"{"subject": "WILLI"}"#).unwrap_or_default(),
            action: serde_json::from_str(r#"{"action": "read"}"#).unwrap_or_default(),
            resource: serde_json::from_str(r#"{"resource": "something"}"#).unwrap_or_default(),
            environment: None,
        }
    }

    pub fn new_example_subscription2() -> Self {
        AuthorizationSubscription {
            subject: serde_json::from_str(r#"{"subject": "WILLI"}"#).unwrap_or_default(),
            action: serde_json::from_str(r#"{"action": { "name": "Apple", "color": "Red", "nutrients": { "calories": "low" } } }"#).unwrap_or_default(),
            resource: serde_json::from_str(r#"{"resource": "something"}"#).unwrap_or_default(),
            environment: None,
        }
    }

    pub fn get_value(&self, bie: &BasicIdentifierExpression, keys: &mut VecDeque<String>) -> Value {
        use BasicIdentifierExpression::*;
        match bie {
            Subject => Self::search_value(&self.subject, keys),
            Action => Self::search_value(&self.action, keys),
            Resource => Self::search_value(&self.resource, keys),
            Environment => match &self.environment {
                Some(env) => Self::search_value(env, keys),
                None => Value::Null,
            },
        }
    }

    fn search_value(v: &Value, keys: &mut VecDeque<String>) -> Value {
        println!("search_value: v={:#?} und keys={:#?}", v, keys);
        match keys.pop_front() {
            Some(k) => match v.get(k) {
                None => Value::Null,
                Some(data) => Self::search_value(data, keys),
            },
            None => v.clone(),
        }
    }
}
