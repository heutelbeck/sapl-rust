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
use serde_json::Value;
use std::sync::Arc;

pub(crate) fn evaluate(keys: &Arc<[Ast]>, src: &Value) -> Value {
    let results: Vec<Value> = keys
        .iter()
        .filter_map(|entry| {
            if let Ast::String(key) = entry {
                match src.get(key) {
                    Some(Value::Array(arr)) => Some(Value::Array(arr.clone())),
                    Some(Value::String(s)) => Some(Value::String(s.clone())),
                    Some(Value::Number(n)) => Some(Value::Number(n.clone())),
                    Some(Value::Bool(b)) => Some(Value::Bool(*b)),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect();

    Value::Array(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn get_data() -> Value {
        json!({
            "key" : "value1",
            "key2": {
                "key": "value4"
            },
            "array1" : [
                { "key" : "value2" },
                { "key" : "value3" }
            ],
            "array2" : [
                1, 2, 3, 4, 5
            ],
            "key3": true,
            "key4": 42,
        })
    }

    #[test]
    fn evaluate_to_null() {
        assert_eq!(
            json!([]),
            evaluate(
                &Arc::from(vec![Ast::String("demo".to_string())]),
                &get_data()
            )
        );
    }

    #[test]
    fn evaluate_key_array_obj() {
        assert_eq!(
            json!(["value1", [1, 2, 3, 4, 5]]),
            evaluate(
                &Arc::from(vec![
                    Ast::String("key".to_string()),
                    Ast::String("array2".to_string())
                ]),
                &get_data()
            )
        );
    }

    #[test]
    fn evaluate_key_boolean_number() {
        assert_eq!(
            json!([true, 42]),
            evaluate(
                &Arc::from(vec![
                    Ast::String("key3".to_string()),
                    Ast::String("key4".to_string())
                ]),
                &get_data()
            )
        );
    }
}
