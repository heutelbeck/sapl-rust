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
use rust_decimal::prelude::Signed;
use serde_json::Value;
use std::sync::Arc;

pub(crate) fn evaluate(key: &Arc<[Ast]>, src: &Value) -> Value {
    let mut results = Vec::new();

    match src {
        Value::Array(arr) => {
            let start = match key.first() {
                Some(Ast::Integer(i)) => *i,
                _ => 0,
            };
            let end = match key.get(1) {
                Some(Ast::Integer(i)) => match i.is_negative() {
                    true => arr.len() as i64 + *i,
                    false => *i,
                },
                _ => 0,
            };
            let step: usize = match key.get(2) {
                Some(Ast::Integer(i)) => usize::try_from(*i).unwrap(),
                _ => 1,
            };

            assert_ne!(0, step, "array slicing step zero is not allowed");

            for i in (start..end).step_by(step) {
                if let Ok(index) = usize::try_from(i)
                    && let Some(data) = arr.get(index)
                {
                    results.push(data.clone());
                }

                if i > arr.len() as i64 {
                    break;
                }
            }
        }
        _ => {
            return Value::Null;
        }
    }

    Value::Array(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn get_array1() -> Value {
        json!([1, 2, 3, 4, 5])
    }

    #[test]
    fn evaluate_array1_positive_end_with_step() {
        assert_eq!(
            json!([1, 3, 5]),
            evaluate(
                &Arc::from(vec![Ast::Integer(0), Ast::Integer(100), Ast::Integer(2)]),
                &get_array1()
            )
        );
    }

    #[test]
    fn evaluate_array1_offset_startpositive_end_with_step() {
        assert_eq!(
            json!([4]),
            evaluate(
                &Arc::from(vec![Ast::Integer(3), Ast::Integer(100), Ast::Integer(2)]),
                &get_array1()
            )
        );
    }

    #[test]
    fn evaluate_array1_negtive_end_with_step() {
        assert_eq!(
            json!([1, 3]),
            evaluate(
                &Arc::from(vec![Ast::Integer(0), Ast::Integer(-2), Ast::Integer(2)]),
                &get_array1()
            )
        );
    }

    #[test]
    fn evaluate_array1_positive_end_without_step() {
        assert_eq!(
            json!([1, 2, 3, 4, 5]),
            evaluate(
                &Arc::from(vec![Ast::Integer(0), Ast::Integer(100)]),
                &get_array1()
            )
        );
    }

    #[test]
    fn evaluate_array1_negtive_end_without_step() {
        assert_eq!(
            json!([1, 2, 3]),
            evaluate(
                &Arc::from(vec![Ast::Integer(0), Ast::Integer(-2)]),
                &get_array1()
            )
        );
    }
}
