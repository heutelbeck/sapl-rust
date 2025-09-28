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
    attribute_union_step, expression_step, index_step, index_union_step, key_step,
    recursive_index_step, recursive_key_step, wildcard_step,
};
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
                Some(Ast::WildcardStep) => wildcard_step::evaluate(data),
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
mod test {
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

    fn get_data_id() -> Value {
        json!({ "id": {
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
        }})
    }

    fn get_index_step(index: i64) -> Ast {
        Ast::IndexStep(index)
    }

    fn get_expr_key_step() -> Ast {
        let expr = Arc::new(Ast::Expr {
            lhs: Ast::Integer(3).into(),
            op: crate::Op::Addition,
            rhs: Ast::Integer(1).into(),
        });
        Ast::ExpressionStep(expr)
    }

    #[test]
    fn evaluate_key_step() {
        assert_eq!(
            json!("value4"),
            evaluate(
                "key2",
                &[Ast::KeyStep("key".to_string())],
                Arc::new(RwLock::new(get_data()))
            )
        );
    }

    #[test]
    fn evaluate_index_step() {
        assert_eq!(
            json!(1),
            evaluate(
                "array2",
                &[get_index_step(0)],
                Arc::new(RwLock::new(get_data()))
            )
        );
    }

    #[test]
    fn evaluate_negativ_index_step() {
        assert_eq!(
            json!(3),
            evaluate(
                "array2",
                &[get_index_step(-3)],
                Arc::new(RwLock::new(get_data()))
            )
        );
    }

    #[test]
    fn evaluate_expression_key_step() {
        assert_eq!(
            json!(5),
            evaluate(
                "array2",
                &[get_expr_key_step()],
                Arc::new(RwLock::new(get_data()))
            )
        );
    }

    #[test]
    fn evaluate_recursive_key_step() {
        assert_eq!(
            json!(["value1", "value4", "value2", "value3"]),
            evaluate(
                "id",
                &[Ast::RecursiveKeyStep("key".to_string())],
                Arc::new(RwLock::new(get_data_id()))
            )
        );
    }

    #[test]
    fn evaluate_recursive_index_step() {
        assert_eq!(
            json!([{"key": "value2"}, 1]),
            evaluate(
                "id",
                &[Ast::RecursiveIndexStep(0)],
                Arc::new(RwLock::new(get_data_id()))
            )
        );
    }

    #[test]
    fn evaluate_attribute_union_step() {
        assert_eq!(
            json!(["value1", [1, 2, 3, 4, 5]]),
            evaluate(
                "id",
                &[
                    Ast::AttributeUnionStep(Arc::from(vec![
                        Ast::String("key".to_string()),
                        Ast::String("array2".to_string())
                    ])),
                    get_expr_key_step()
                ],
                Arc::new(RwLock::new(get_data_id()))
            )
        );
    }

    #[test]
    fn evaluate_index_union_step() {
        assert_eq!(
            json!([3, 4]),
            evaluate(
                "id",
                &[
                    Ast::KeyStep("array2".to_string()),
                    Ast::IndexUnionStep(Arc::from(vec![Ast::Integer(2), Ast::Integer(3)])),
                    get_expr_key_step()
                ],
                Arc::new(RwLock::new(get_data_id()))
            )
        );
    }

    #[test]
    fn evaluate_wildcard_step() {
        assert_eq!(
            json!(["value1", "value4", [{"key": "value2"}, {"key": "value3"}], [1, 2, 3, 4, 5]]),
            evaluate(
                "id",
                &[Ast::WildcardStep, get_expr_key_step()],
                Arc::new(RwLock::new(get_data_id()))
            )
        );
    }
}
