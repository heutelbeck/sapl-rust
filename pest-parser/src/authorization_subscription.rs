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
use crate::basic_identifier_expression::BasicIdentifierExpression;
use serde::Deserialize;
use serde_json::Value;
use std::collections::VecDeque;

#[derive(Debug, Deserialize)]
pub struct AuthorizationSubscription {
    subject: Value,
    action: Value,
    resource: Value,
    environment: Option<Value>,
}

impl AuthorizationSubscription {
    pub fn new_example_subscription1() -> Self {
        AuthorizationSubscription {
            subject: serde_json::from_str(r#""WILLI""#).unwrap_or_default(),
            action: serde_json::from_str(r#""read""#).unwrap_or_default(),
            resource: serde_json::from_str(r#""something""#).unwrap_or_default(),
            environment: None,
        }
    }

    pub fn new_example_subscription2() -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_read_value_top_level() {
        let auth_sub = AuthorizationSubscription::new_example_subscription1();
        let mut keys = VecDeque::new();
        assert_eq!(
            Value::String("read".to_string()),
            auth_sub.get_value(&BasicIdentifierExpression::Action, &mut keys)
        );
    }

    #[test]
    fn action_read_nested_value() {
        let auth_sub = AuthorizationSubscription::new_example_subscription2();
        let mut keys = VecDeque::from(["nutrients".to_string(), "calories".to_string()]);
        assert_eq!(
            Value::String("low".to_string()),
            auth_sub.get_value(&BasicIdentifierExpression::Action, &mut keys)
        );
    }

    #[test]
    fn subject_read_value_top_level() {
        let auth_sub = AuthorizationSubscription::new_example_subscription1();
        let mut keys = VecDeque::new();
        assert_eq!(
            Value::String("WILLI".to_string()),
            auth_sub.get_value(&BasicIdentifierExpression::Subject, &mut keys)
        );
    }

    #[test]
    fn resource_read_value_top_level() {
        let auth_sub = AuthorizationSubscription::new_example_subscription1();
        let mut keys = VecDeque::new();
        assert_eq!(
            Value::String("something".to_string()),
            auth_sub.get_value(&BasicIdentifierExpression::Resource, &mut keys)
        );
    }

    #[test]
    fn environment_read_option_none() {
        let auth_sub = AuthorizationSubscription::new_example_subscription1();
        let mut keys = VecDeque::new();
        assert_eq!(
            Value::Null,
            auth_sub.get_value(&BasicIdentifierExpression::Environment, &mut keys)
        );
    }
}
