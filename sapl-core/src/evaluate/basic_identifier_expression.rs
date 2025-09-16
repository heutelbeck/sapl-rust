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
use std::{
    fmt::Display,
    sync::{Arc, RwLock},
};

use crate::evaluate::key_step;
use crate::{Ast, evaluate::wildcard_step};

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
            panic!("Input {s} could not be parsed as basic identifier expression")
        }
    }

    pub fn evaluate(&self, keys: &[Ast], variable_context: Arc<RwLock<Value>>) -> Value {
        let src;
        if let Ok(var) = variable_context.read() {
            src = var.get(self.to_string()).unwrap().clone();
        } else {
            return Value::Null;
        };

        match keys.first() {
            Some(Ast::KeyStep(s)) | Some(Ast::EscapedKeyStep(s)) => {
                key_step::evaluate(s, keys.get(1..).unwrap_or(&[]), &src)
            }
            Some(Ast::WildcardStep) => wildcard_step::evaluate(keys.get(1..).unwrap_or(&[]), &src),
            None => src.clone(),
            _ => Value::Null,
        }
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
