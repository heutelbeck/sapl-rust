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
use crate::evaluate::{
    attribute_union_step, expression_step, index_union_step, key_step, recursive_index_step,
    recursive_key_step,
};
use serde_json::Value;

pub(crate) fn evaluate(key: i64, keys: &[Ast], src: &Value) -> Value {
    let num: i64 = if key.is_negative() && src.is_array() {
        src.as_array().unwrap().len() as i64 + key
    } else {
        key
    };

    if let Ok(index) = usize::try_from(num) {
        match src.get(index) {
            Some(data) => match keys.first() {
                Some(Ast::KeyStep(s)) | Some(Ast::EscapedKeyStep(s)) => {
                    key_step::evaluate(s, keys.get(1..).unwrap_or(&[]), data)
                }
                Some(Ast::IndexStep(i)) => evaluate(*i, keys.get(1..).unwrap_or(&[]), data),
                Some(Ast::ExpressionStep(s)) => {
                    expression_step::evaluate(s, keys.get(1..).unwrap_or(&[]), data)
                }
                Some(Ast::RecursiveKeyStep(s)) => recursive_key_step::evaluate(s, data),
                Some(Ast::RecursiveIndexStep(i)) => recursive_index_step::evaluate(*i, data),
                Some(Ast::AttributeUnionStep(k)) => attribute_union_step::evaluate(k, data),
                Some(Ast::IndexUnionStep(k)) => index_union_step::evaluate(k, data),
                None => data.clone(),
                _ => Value::Null,
            },
            None => Value::Null,
        }
    } else {
        Value::Null
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn get_array1() -> Value {
        json!(
            [
                { "key" : "value2" },
                { "key" : "value3" }
            ]
        )
    }

    fn get_array2() -> Value {
        json!([1, 2, 3, 4, 5])
    }

    #[test]
    fn evaluate_positive_index_step() {
        assert_eq!(json!({"key": "value2"}), evaluate(0, &[], &get_array1()));
        assert_eq!(json!(4), evaluate(3, &[], &get_array2()));
    }

    #[test]
    fn evaluate_negativ_index_step() {
        assert_eq!(json!({"key": "value3"}), evaluate(-1, &[], &get_array1()));
        assert_eq!(json!(2), evaluate(-4, &[], &get_array2()));
    }
}
