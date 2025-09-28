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
            if let Ast::Integer(key) = entry {
                let num: i64 = if key.is_negative() && src.is_array() {
                    src.as_array().unwrap().len() as i64 + key
                } else {
                    *key
                };

                if let Ok(index) = usize::try_from(num) {
                    src.get(index).cloned()
                } else {
                    None
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

    fn get_array1() -> Value {
        json!([{"key1": "value2"}, {"key2": "value3"}, {"key": "value4"}])
    }

    fn get_array2() -> Value {
        json!([1, 2, 3, 4, 5])
    }

    fn get_array3() -> Value {
        json!(["key", "value", "sapl"])
    }

    #[test]
    fn evaluate_to_null() {
        assert_eq!(
            json!([]),
            evaluate(&Arc::from(vec![Ast::Integer(42)]), &get_array1())
        );
    }

    #[test]
    fn evaluate_array_numbers() {
        assert_eq!(
            json!([3, 4, 5]),
            evaluate(
                &Arc::from(vec![Ast::Integer(2), Ast::Integer(3), Ast::Integer(-1)]),
                &get_array2()
            )
        );
    }

    #[test]
    fn evaluate_array_strings() {
        assert_eq!(
            json!(["key", "sapl"]),
            evaluate(
                &Arc::from(vec![Ast::Integer(0), Ast::Integer(2)]),
                &get_array3()
            )
        );
    }

    #[test]
    fn evaluate_array_obj() {
        assert_eq!(
            json!([{"key1": "value2"}, {"key": "value4"}]),
            evaluate(
                &Arc::from(vec![Ast::Integer(0), Ast::Integer(2)]),
                &get_array1()
            )
        );
    }
}
