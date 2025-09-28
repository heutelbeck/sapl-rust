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
use std::{
    fmt::Display,
    sync::{Arc, RwLock},
};

use crate::{
    Ast,
    evaluate::{
        attribute_union_step, expression_step, index_step, index_union_step, key_step,
        recursive_index_step, recursive_key_step, wildcard_step,
    },
};

#[derive(PartialEq, Debug)]
pub enum BasicIdentifierExpression {
    Subject,
    Action,
    Resource,
    Environment,
}

impl BasicIdentifierExpression {
    pub fn new(s: &str) -> Self {
        if s.eq("subject") {
            BasicIdentifierExpression::Subject
        } else if s.eq("action") {
            BasicIdentifierExpression::Action
        } else if s.eq("resource") {
            BasicIdentifierExpression::Resource
        } else if s.eq("environment") {
            BasicIdentifierExpression::Environment
        } else {
            panic!("Input {s} could not be parsed as basic identifier expression")
        }
    }

    pub fn evaluate(&self, keys: &[Ast], variable_context: Arc<RwLock<Value>>) -> Value {
        let src;
        if let Ok(var) = variable_context.read() {
            src = var.get(self.to_string()).unwrap().clone();
        } else {
            return Value::Null;
        };

        match keys.first() {
            Some(Ast::KeyStep(s)) | Some(Ast::EscapedKeyStep(s)) => {
                key_step::evaluate(s, keys.get(1..).unwrap_or(&[]), &src)
            }
            Some(Ast::IndexStep(i)) => index_step::evaluate(*i, keys.get(1..).unwrap_or(&[]), &src),
            Some(Ast::WildcardStep) => wildcard_step::evaluate(&src),
            Some(Ast::ExpressionStep(s)) => {
                expression_step::evaluate(s, keys.get(1..).unwrap_or(&[]), &src)
            }
            Some(Ast::RecursiveKeyStep(s)) => recursive_key_step::evaluate(s, &src),
            Some(Ast::RecursiveIndexStep(i)) => recursive_index_step::evaluate(*i, &src),
            Some(Ast::AttributeUnionStep(k)) => attribute_union_step::evaluate(k, &src),
            Some(Ast::IndexUnionStep(k)) => index_union_step::evaluate(k, &src),
            None => src.clone(),
            _ => Value::Null,
        }
    }
}

impl Display for BasicIdentifierExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BasicIdentifierExpression::*;

        write!(
            f,
            "{}",
            match &self {
                Subject => "subject",
                Action => "action",
                Resource => "resource",
                Environment => "environment",
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    fn get_data() -> Value {
        json!({"action": {
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
            json!("value1"),
            BasicIdentifierExpression::new("action").evaluate(
                &[Ast::KeyStep("key".to_string())],
                Arc::new(RwLock::new(get_data()))
            )
        );
    }

    #[test]
    fn evaluate_key_step2() {
        assert_eq!(
            json!("value4"),
            BasicIdentifierExpression::new("action").evaluate(
                &[
                    Ast::KeyStep("key2".to_string()),
                    Ast::KeyStep("key".to_string())
                ],
                Arc::new(RwLock::new(get_data()))
            )
        );
    }

    #[test]
    fn evaluate_escaped_key_step() {
        assert_eq!(
            json!("value4"),
            BasicIdentifierExpression::new("action").evaluate(
                &[
                    Ast::EscapedKeyStep("key2".to_string()),
                    Ast::EscapedKeyStep("key".to_string())
                ],
                Arc::new(RwLock::new(get_data()))
            )
        );
    }

    #[test]
    fn evaluate_index_step() {
        assert_eq!(
            json!(1),
            BasicIdentifierExpression::new("action").evaluate(
                &[Ast::KeyStep("array2".to_string()), get_index_step(0)],
                Arc::new(RwLock::new(get_data()))
            )
        );
    }

    #[test]
    fn evaluate_negativ_index_step() {
        assert_eq!(
            json!(3),
            BasicIdentifierExpression::new("action").evaluate(
                &[Ast::KeyStep("array2".to_string()), get_index_step(-3)],
                Arc::new(RwLock::new(get_data()))
            )
        );
    }

    #[test]
    fn evaluate_expression_key_step() {
        assert_eq!(
            json!(5),
            BasicIdentifierExpression::new("action").evaluate(
                &[Ast::KeyStep("array2".to_string()), get_expr_key_step()],
                Arc::new(RwLock::new(get_data()))
            )
        );
    }

    #[test]
    fn evaluate_recursive_key_step() {
        assert_eq!(
            json!(["value1", "value4", "value2", "value3"]),
            BasicIdentifierExpression::new("action").evaluate(
                &[
                    Ast::RecursiveKeyStep("key".to_string()),
                    get_expr_key_step()
                ],
                Arc::new(RwLock::new(get_data()))
            )
        );
    }

    #[test]
    fn evaluate_recursive_index_step() {
        assert_eq!(
            json!([{"key": "value2"}, 1]),
            BasicIdentifierExpression::new("action").evaluate(
                &[Ast::RecursiveIndexStep(0), get_expr_key_step()],
                Arc::new(RwLock::new(get_data()))
            )
        );
    }

    #[test]
    fn evaluate_attribute_union_step() {
        assert_eq!(
            json!(["value1", [1, 2, 3, 4, 5]]),
            BasicIdentifierExpression::new("action").evaluate(
                &[
                    Ast::AttributeUnionStep(Arc::from(vec![
                        Ast::String("key".to_string()),
                        Ast::String("array2".to_string())
                    ])),
                    get_expr_key_step()
                ],
                Arc::new(RwLock::new(get_data()))
            )
        );
    }

    #[test]
    fn evaluate_index_union_step() {
        assert_eq!(
            json!([3, 4]),
            BasicIdentifierExpression::new("action").evaluate(
                &[
                    Ast::KeyStep("array2".to_string()),
                    Ast::IndexUnionStep(Arc::from(vec![Ast::Integer(2), Ast::Integer(3)])),
                    get_expr_key_step()
                ],
                Arc::new(RwLock::new(get_data()))
            )
        );
    }

    #[test]
    fn evaluate_wildcard_step() {
        assert_eq!(
            json!(["value1", "value4", [{"key": "value2"}, {"key": "value3"}], [1, 2, 3, 4, 5]]),
            BasicIdentifierExpression::new("action").evaluate(
                &[Ast::WildcardStep, get_expr_key_step()],
                Arc::new(RwLock::new(get_data()))
            )
        );
    }
}
