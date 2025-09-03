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

use crate::Ast;
use crate::evaluate::{index_step, key_step, wildcard_step};
use serde_json::Value;
use std::sync::{Arc, RwLock};

pub(crate) fn evaluate(key: &str, keys: &[Ast], src: Arc<RwLock<Value>>) -> Value {
    if let Ok(var) = src.read() {
        match var.get(key) {
            Some(data) => match keys.first() {
                Some(Ast::KeyStep(s)) | Some(Ast::EscapedKeyStep(s)) => {
                    key_step::evaluate(s, keys.get(1..).unwrap_or(&[]), data)
                }
                Some(Ast::IndexStep(i)) => {
                    index_step::evaluate(*i, keys.get(1..).unwrap_or(&[]), data)
                }
                Some(Ast::WildcardStep) => {
                    wildcard_step::evaluate(keys.get(1..).unwrap_or(&[]), data)
                }
                None => data.clone(),
                _ => Value::Null,
            },
            None => Value::Null,
        }
    } else {
        Value::Null
    }
}
