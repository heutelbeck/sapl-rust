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
use crate::evaluate::index_step;
use serde_json::Value;

pub(crate) fn evaluate(key: &str, keys: &[Ast], src: &Value) -> Value {
    match src.get(key) {
        Some(data) => match keys.first() {
            Some(Ast::KeyStep(s)) | Some(Ast::EscapedKeyStep(s)) => {
                evaluate(s, keys.get(1..).unwrap_or(&[]), data)
            }
            Some(Ast::IndexStep(i)) => index_step::evaluate(*i, keys.get(1..).unwrap_or(&[]), data),
            None => data.clone(),
            _ => Value::Null,
        },
        None => Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn get_data() -> Value {
        json!({
            "key" : "value1",
            "key2": {
                "key": "value3"
            },
            "array1" : [
                { "key" : "value2" },
                { "key" : "value3" }
            ],
            "array2" : [
                1, 2, 3, 4, 5
            ]
        })
    }

    #[test]
    fn evaluate_to_null() {
        assert_eq!(Value::Null, evaluate("\"demo\"", &[], &get_data()));
    }

    #[test]
    fn evaluate_to_value1() {
        assert_eq!(json!("value1"), evaluate("key", &[], &get_data()));
    }

    #[test]
    fn evaluate_key_step_to_value2() {
        let keys = [Ast::KeyStep("key".to_string())];
        assert_eq!(json!("value3"), evaluate("key2", &keys, &get_data()));
    }

    #[test]
    fn evaluate_escaped_key_step_to_value2() {
        let keys = [Ast::EscapedKeyStep("key".to_string())];
        assert_eq!(json!("value3"), evaluate("key2", &keys, &get_data()));
    }
}
