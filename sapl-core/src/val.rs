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
    pub fn is_number(&self) -> bool {
        matches!(self, Val::Integer(_) | Val::Float(_))
    }
    pub fn is_integer(&self) -> bool {
        matches!(self, Val::Integer(_))
    }
    pub fn is_float(&self) -> bool {
        matches!(self, Val::Float(_))
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
