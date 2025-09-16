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
use crate::evaluate::BasicIdentifierExpression;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::VecDeque;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizationSubscription {
    subject: Value,
    action: Value,
    resource: Value,
    environment: Option<Value>,
}

impl AuthorizationSubscription {
    #[cfg(test)]
    pub fn new_example_subscription1() -> Value {
        serde_json::from_str(r#"{ "subject": "WILLI", "action": "read", "resource": "something"}"#)
            .unwrap_or_default()
    }

    #[cfg(test)]
    pub fn new_example_subscription2() -> Value {
        serde_json::from_str(r#"{ "subject": "WILLI", "action": { "name": "Apple", "color": "Red", "nutrients": { "calories": "low" }}, "resource": "something"}"#)
            .unwrap_or_default()
    }

    #[cfg(test)]
    pub fn new_example_subscription3() -> Self {
        AuthorizationSubscription {
            subject: serde_json::from_str(r#""WILLI""#).unwrap_or_default(),
            action: serde_json::from_str(r#""read""#).unwrap_or_default(),
            resource: serde_json::from_str(r#""something""#).unwrap_or_default(),
            environment: None,
        }
    }

    #[cfg(test)]
    pub fn new_example_subscription4() -> Self {
        AuthorizationSubscription {
            subject: serde_json::from_str(r#""WILLI""#).unwrap_or_default(),
            action: serde_json::from_str(
                r#"{ "name": "Apple", "color": "Red", "nutrients": { "calories": "low" } }"#,
            )
            .unwrap_or_default(),
            resource: serde_json::from_str(r#""something""#).unwrap_or_default(),
            environment: None,
        }
    }

    #[cfg(test)]
    pub fn new_example_subscription5() -> Value {
        serde_json::from_str(r#"{ "subject": "WILLI", "action": { "key": "value1", "array1": [ { "key": "value2" }, { "key": "value3"} ], "array2": [1,2,3,4,5] }, "resource": "something"}"#)
            .unwrap_or_default()
    }

    pub fn get(&self, bie: &BasicIdentifierExpression) -> Value {
        use BasicIdentifierExpression::*;
        match bie {
            Subject => self.subject.clone(),
            Action => self.action.clone(),
            Resource => self.resource.clone(),
            Environment => match &self.environment {
                Some(env) => env.clone(),
                None => Value::Null,
            },
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
        match keys.pop_front() {
            Some(k) => match v.get(k) {
                None => Value::Null,
                Some(data) => Self::search_value(data, keys),
            },
            None => v.clone(),
        }
    }
}

impl From<AuthorizationSubscription> for Value {
    fn from(auth_sub: AuthorizationSubscription) -> Value {
        serde_json::to_value(auth_sub).expect("Failed to convert to Value")
    }
}

impl From<&AuthorizationSubscription> for Value {
    fn from(auth_sub: &AuthorizationSubscription) -> Value {
        serde_json::to_value(auth_sub).expect("Failed to convert to Value")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_read_value_top_level() {
        let auth_sub = AuthorizationSubscription::new_example_subscription3();
        let mut keys = VecDeque::new();
        assert_eq!(
            Value::String("read".to_string()),
            auth_sub.get_value(&BasicIdentifierExpression::Action, &mut keys)
        );
    }

    #[test]
    fn action_read_nested_value() {
        let auth_sub = AuthorizationSubscription::new_example_subscription4();
        let mut keys = VecDeque::from(["nutrients".to_string(), "calories".to_string()]);
        assert_eq!(
            Value::String("low".to_string()),
            auth_sub.get_value(&BasicIdentifierExpression::Action, &mut keys)
        );
    }

    #[test]
    fn subject_read_value_top_level() {
        let auth_sub = AuthorizationSubscription::new_example_subscription3();
        let mut keys = VecDeque::new();
        assert_eq!(
            Value::String("WILLI".to_string()),
            auth_sub.get_value(&BasicIdentifierExpression::Subject, &mut keys)
        );
    }

    #[test]
    fn resource_read_value_top_level() {
        let auth_sub = AuthorizationSubscription::new_example_subscription3();
        let mut keys = VecDeque::new();
        assert_eq!(
            Value::String("something".to_string()),
            auth_sub.get_value(&BasicIdentifierExpression::Resource, &mut keys)
        );
    }

    #[test]
    fn environment_read_option_none() {
        let auth_sub = AuthorizationSubscription::new_example_subscription3();
        let mut keys = VecDeque::new();
        assert_eq!(
            Value::Null,
            auth_sub.get_value(&BasicIdentifierExpression::Environment, &mut keys)
        );
    }
}
