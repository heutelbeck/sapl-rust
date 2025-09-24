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

pub(crate) fn evaluate(key: &str, src: &Value) -> Value {
    let mut results = Vec::new();

    match src {
        Value::Object(obj) => {
            for (k, v) in obj {
                if k == key {
                    results.push(v.clone());
                } else if v.is_object() || v.is_array() {
                    let sub_results = evaluate(key, v);
                    if let Value::Array(sub_arr) = sub_results {
                        results.extend(sub_arr);
                    } else {
                        results.push(sub_results);
                    }
                }
            }
        }
        Value::Array(arr) => {
            for item in arr {
                let sub_results = evaluate(key, item);
                if let Value::Array(sub_arr) = sub_results {
                    results.extend(sub_arr);
                } else {
                    results.push(sub_results);
                }
            }
        }
        _ => {}
    }

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
            ]
        })
    }

    #[test]
    fn evaluate_to_null() {
        assert_eq!(json!([]), evaluate("\"demo\"", &get_data()));
    }

    #[test]
    fn evaluate_key() {
        assert_eq!(
            json!(["value1", "value4", "value2", "value3"]),
            evaluate("key", &get_data())
        );
    }
}
