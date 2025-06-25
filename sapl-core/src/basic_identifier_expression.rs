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
use std::fmt::Display;

use crate::authorization_subscription::AuthorizationSubscription;

#[derive(PartialEq, Debug)]
pub enum BasicIdentifierExpression {
    Subject,
    Action,
    Resource,
    Environment,
}

impl BasicIdentifierExpression {
    pub fn new(s: &str) -> Self {
        if s.eq("subject") {
            BasicIdentifierExpression::Subject
        } else if s.eq("action") {
            BasicIdentifierExpression::Action
        } else if s.eq("resource") {
            BasicIdentifierExpression::Resource
        } else if s.eq("environment") {
            BasicIdentifierExpression::Environment
        } else {
            panic!(
                "Input {} could not be parsed as basic identifier expression",
                s
            )
        }
    }

    pub fn evaluate(
        &self,
        keys: &mut VecDeque<String>,
        auth_subscription: &AuthorizationSubscription,
    ) -> Value {
        auth_subscription.get_value(self, keys)
    }
}

impl Display for BasicIdentifierExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BasicIdentifierExpression::*;

        write!(
            f,
            "{}",
            match &self {
                Subject => "subject",
                Action => "action",
                Resource => "resource",
                Environment => "environment",
            }
        )
    }
}
