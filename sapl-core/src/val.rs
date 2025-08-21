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
    Integer(i64),
    Float(f64),
    String(String),
    CompInteger(bool, i64),
    CompFloat(bool, f64),
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_to_value_float() {
        let val = Val::Float(1.14);
        let result = val.to_value();

        assert_eq!(result, json!(1.14));
        assert!(result.is_f64());
        assert_eq!(result.as_f64().unwrap(), 1.14);
    }

    #[test]
    fn test_to_value_none() {
        let val = Val::None;
        let result = val.to_value();

        assert_eq!(result, json!(null));
        assert!(result.is_null());
    }

    #[test]
    fn test_to_value_string() {
        let val = Val::String("hello world".to_string());
        let result = val.to_value();

        assert_eq!(result, json!("hello world"));
        assert!(result.is_string());
        assert_eq!(result.as_str().unwrap(), "hello world");
    }

    #[test]
    fn test_to_value_string_empty() {
        let val = Val::String(String::new());
        let result = val.to_value();

        assert_eq!(result, json!(""));
        assert!(result.is_string());
        assert_eq!(result.as_str().unwrap(), "");
    }

    #[test]
    fn test_to_value_string_special_chars() {
        let val = Val::String("hello\nworld\t\"quoted\"".to_string());
        let result = val.to_value();

        assert_eq!(result, json!("hello\nworld\t\"quoted\""));
        assert!(result.is_string());
        assert_eq!(result.as_str().unwrap(), "hello\nworld\t\"quoted\"");
    }

    #[test]
    fn test_to_value_boolean() {
        check_boolean(true);
        check_boolean(false);

        fn check_boolean(b: bool) {
            let val = Val::Boolean(b);
            let result = val.to_value();

            assert_eq!(result, json!(b));
            assert!(result.is_boolean());
            assert_eq!(result.as_bool().unwrap(), b);
        }
    }

    #[test]
    fn test_to_value_integer() {
        check_integer(42);
        check_integer(-123);
        check_integer(i64::MIN);
        check_integer(i64::MAX);

        fn check_integer(i: i64) {
            let val = Val::Integer(i);
            let result = val.to_value();

            assert_eq!(result, json!(i));
            assert!(result.is_i64());
            assert_eq!(result.as_i64().unwrap(), i);
        }
    }

    #[test]
    fn test_to_value_comp_float_true() {
        check_comp_float(true, 1.14);
        check_comp_float(false, 1.718);

        fn check_comp_float(b: bool, f: f64) {
            let val = Val::CompFloat(b, f);
            let result = val.to_value();

            assert_eq!(result, json!(b));
            assert!(result.is_boolean());
            assert_eq!(result.as_bool().unwrap(), b);
        }
    }

    #[test]
    fn test_to_value_comp_integer() {
        check_comp_integer(true, 42);
        check_comp_integer(false, -123);

        fn check_comp_integer(b: bool, i: i64) {
            let val = Val::CompInteger(b, i);
            let result = val.to_value();

            assert_eq!(result, json!(b));
            assert!(result.is_boolean());
            assert_eq!(result.as_bool().unwrap(), b);
        }
    }

    #[test]
    fn test_to_value_json_object() {
        let json_value = json!({
            "name": "test",
            "value": 42,
            "active": true
        });

        let val = Val::Json(json_value.clone());
        let result = val.to_value();

        assert_eq!(result, json_value);
        assert!(result.is_object());
        assert_eq!(result["name"], "test");
        assert_eq!(result["value"], 42);
        assert_eq!(result["active"], true);
    }

    #[test]
    fn test_to_value_json_array() {
        let json_value = json!([1, "two", true, null]);

        let val = Val::Json(json_value.clone());
        let result = val.to_value();

        assert_eq!(result, json_value);
        assert!(result.is_array());
        assert_eq!(result[0], 1);
        assert_eq!(result[1], "two");
        assert_eq!(result[2], true);
        assert_eq!(result[3], json!(null));
    }

    #[test]
    fn test_to_value_json_null() {
        let json_value = json!(null);

        let val = Val::Json(json_value);
        let result = val.to_value();

        assert_eq!(result, json!(null));
        assert!(result.is_null());
    }

    #[test]
    fn test_to_value_json_nested() {
        let json_value = json!({
            "users": [
                {"name": "Alice", "age": 30},
                {"name": "Bob", "age": 25}
            ],
            "meta": {
                "total": 2,
                "active": true
            }
        });

        let val = Val::Json(json_value.clone());
        let result = val.to_value();

        assert_eq!(result, json_value);
        assert_eq!(result["users"][0]["name"], "Alice");
        assert_eq!(result["meta"]["total"], 2);
    }

    #[test]
    fn test_clone_float() {
        let original = Val::Float(1.14);
        let cloned = original.clone();

        assert_eq!(original, cloned);

        // Extract values for direct comparison
        if let (Val::Float(orig_val), Val::Float(clone_val)) = (&original, &cloned) {
            assert_eq!(*orig_val, 1.14);
            assert_eq!(*clone_val, 1.14);
            assert_eq!(orig_val, clone_val);
        }
    }

    #[test]
    fn test_clone_none() {
        let original = Val::None;
        let cloned = original.clone();

        assert!(matches!(original, Val::None));
        assert!(matches!(cloned, Val::None));
    }

    #[test]
    fn test_clone_string() {
        let original = Val::String("hello world".to_string());
        let cloned = original.clone();

        assert_eq!(original, cloned);

        // Extract values for direct comparison
        if let (Val::String(orig_val), Val::String(clone_val)) = (&original, &cloned) {
            assert_eq!(orig_val, "hello world");
            assert_eq!(clone_val, "hello world");
            assert_eq!(orig_val, clone_val);
        }
    }

    #[test]
    fn test_clone_boolean() {
        check_boolean(true);
        check_boolean(false);

        fn check_boolean(b: bool) {
            let original = Val::Boolean(b);
            let cloned = original.clone();

            assert_eq!(original, cloned);

            if let (Val::Boolean(orig), Val::Boolean(clone)) = (&original, &cloned) {
                assert_eq!(orig, clone);
                assert_eq!(*orig, b);
                assert_eq!(*clone, b);
            }
        }
    }

    #[test]
    fn test_clone_integer() {
        let original = Val::Integer(42);
        let cloned = original.clone();

        assert_eq!(original, cloned);

        if let (Val::Integer(orig), Val::Integer(clone)) = (&original, &cloned) {
            assert_eq!(orig, clone);
            assert_eq!(*orig, 42);
            assert_eq!(*clone, 42);
        }
    }

    #[test]
    fn test_clone_comp_float() {
        let original = Val::CompFloat(true, 1.718);
        let cloned = original.clone();

        assert_eq!(original, cloned);

        // Extract values for direct comparision
        if let (Val::CompFloat(orig_b, orig_f), Val::CompFloat(clone_b, clone_f)) =
            (&original, &cloned)
        {
            assert_eq!(orig_b, clone_b);
            assert_eq!(orig_f, clone_f);
            assert!(*orig_b);
            assert_eq!(*orig_f, 1.718);
            assert!(*clone_b);
            assert_eq!(*clone_f, 1.718);
        }
    }

    #[test]
    fn test_clone_comp_integer() {
        let original = Val::CompInteger(false, 123);
        let cloned = original.clone();

        assert_eq!(original, cloned);

        // Extract values for direct comparision
        if let (Val::CompInteger(orig_b, orig_i), Val::CompInteger(clone_b, clone_i)) =
            (&original, &cloned)
        {
            assert_eq!(orig_b, clone_b);
            assert_eq!(orig_i, clone_i);
            assert!(!*orig_b);
            assert_eq!(*orig_i, 123);
            assert!(!*clone_b);
            assert_eq!(*clone_i, 123);
        }
    }

    #[test]
    fn test_clone_json() {
        use serde_json::json;

        let json_value = json!({
            "name": "test",
            "value": 42,
            "nested": {
                "array": [1, 2, 3],
                "boolean": true
            }
        });

        let original = Val::Json(json_value);
        let cloned = original.clone();

        assert_eq!(original, cloned);

        // Extract values for direct comparision
        if let (Val::Json(orig), Val::Json(clone)) = (&original, cloned) {
            // Verify content
            assert_eq!(orig["name"], "test");
            assert_eq!(orig["value"], 42);
            assert_eq!(orig["nested"]["array"][0], 1);
            assert_eq!(orig["nested"]["boolean"], true);

            assert_eq!(clone["name"], "test");
            assert_eq!(clone["value"], 42);
            assert_eq!(clone["nested"]["array"][0], 1);
            assert_eq!(clone["nested"]["boolean"], true);
        }
    }

    #[test]
    fn test_clone_empty_string() {
        let original = Val::String(String::new());
        let cloned = original.clone();

        assert_eq!(original, cloned);

        // Extract values for direct comparision
        if let (Val::String(orig), Val::String(clone)) = (&original, &cloned) {
            assert_eq!(orig, clone);
            assert!(orig.is_empty());
            assert!(clone.is_empty());
        }
    }

    #[test]
    fn test_clone_extreme_values() {
        // Test with extreme float values
        let original_inf = Val::Float(f64::INFINITY);
        let cloned_inf = original_inf.clone();

        if let (Val::Float(orig), Val::Float(clone)) = (&original_inf, &cloned_inf) {
            assert!(orig.is_infinite());
            assert!(clone.is_infinite());
            assert_eq!(orig.is_sign_positive(), clone.is_sign_positive());
        }

        // Test with NaN
        let original_nan = Val::Float(f64::NAN);
        let cloned_nan = original_nan.clone();

        if let (Val::Float(orig), Val::Float(clone)) = (&original_nan, &cloned_nan) {
            assert!(orig.is_nan());
            assert!(clone.is_nan());
        }

        // Test with extreme integer values
        let original_max = Val::Integer(i64::MAX);
        let cloned_max = original_max.clone();

        if let (Val::Integer(orig), Val::Integer(clone)) = (&original_max, &cloned_max) {
            assert_eq!(orig, clone);
            assert_eq!(*orig, i64::MAX);
            assert_eq!(*clone, i64::MAX);
        }
    }

    #[test]
    fn test_clone_independence() {
        // Test that modifying cloned String doesn't affect original
        let mut original = Val::String("original".to_string());
        let cloned = original.clone();

        // Modify original
        if let Val::String(ref mut s) = original {
            s.push_str(" modified");
        }

        // Check that cloned is unchanged
        if let (Val::String(orig), Val::String(clone)) = (&original, &cloned) {
            assert_eq!(orig, "original modified");
            assert_eq!(clone, "original");
            assert_ne!(orig, clone);
        }
    }
}
