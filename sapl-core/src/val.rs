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

use serde_json::{Value, json};

#[derive(Debug, PartialEq)]
pub enum Val {
    Boolean(bool),
    Integer(i32),
    Float(f32),
    String(String),
    CompInteger(bool, i32),
    CompFloat(bool, f32),
    Json(Value),
    None,
}

impl Val {
    pub fn to_value(&self) -> Value {
        let val = match self {
            Val::Float(f) => serde_json::to_value(*f),
            Val::None => Ok(json!(null)),
            Val::String(s) => serde_json::to_value(s),
            Val::Boolean(b) => serde_json::to_value(*b),
            Val::Integer(i) => serde_json::to_value(*i),
            Val::CompFloat(b, _) => serde_json::to_value(*b),
            Val::CompInteger(b, _) => serde_json::to_value(*b),
            Val::Json(j) => Ok(j.clone()),
        };

        val.expect("Failed to serialze Val to JSON")
    }
}

impl Clone for Val {
    fn clone(&self) -> Self {
        match self {
            Val::Float(f) => Val::Float(*f),
            Val::None => Val::None,
            Val::String(s) => Val::String(s.clone()),
            Val::Boolean(b) => Val::Boolean(*b),
            Val::Integer(i) => Val::Integer(*i),
            Val::CompFloat(b, f) => Val::CompFloat(*b, *f),
            Val::CompInteger(b, i) => Val::CompInteger(*b, *i),
            Val::Json(j) => Val::Json(j.clone()),
        }
    }
}
