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
use log::error;
use rust_decimal::prelude::Signed;
use serde_json::Value;
use std::sync::Arc;

pub(crate) fn evaluate(key: &Arc<[Ast]>, src: &Value) -> Value {
    let mut results = Vec::new();

    match src {
        Value::Array(arr) => {
            let mut start = (0, false);
            let mut end = (arr.len() as i64, false);
            let mut step = 1;

            for entry in key.iter() {
                match entry {
                    Ast::ArraySlicingStart(i) => {
                        start = (*i, true);
                    }
                    Ast::ArraySlicingEnd(i) => match i.is_negative() {
                        true => {
                            end = (arr.len() as i64 + *i, true);
                        }
                        false => {
                            end = (*i, true);
                        }
                    },
                    Ast::ArraySlicingStepSize(i) => {
                        step = *i;
                    }
                    other => {
                        error!(
                            "Type mismatch. Array slicing evaluation expects ArraySlicingStart, ArraySlicingEnd or ArraySlicingStepSize, but got: {other:#?}"
                        );
                    }
                };
            }

            let range: Vec<i64> = if step.is_negative() {
                if !start.1 {
                    start = (arr.len() as i64 - 1, true); //start defaults to arr.len() -1
                };
                if !end.1 {
                    end = (-1, true); //end defaults to minus one
                };
                (end.0..=start.0).rev().collect()
            } else {
                (start.0..end.0).collect()
            };

            assert_ne!(0, step, "array slicing step zero is not allowed");

            for i in range
                .into_iter()
                .step_by(usize::try_from(step.abs()).unwrap())
            {
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
                &Arc::from(vec![
                    Ast::ArraySlicingStart(0),
                    Ast::ArraySlicingEnd(100),
                    Ast::ArraySlicingStepSize(2)
                ]),
                &get_array1()
            )
        );
    }

    #[test]
    fn evaluate_array1_offset_startpositive_end_with_step() {
        assert_eq!(
            json!([4]),
            evaluate(
                &Arc::from(vec![
                    Ast::ArraySlicingStart(3),
                    Ast::ArraySlicingEnd(100),
                    Ast::ArraySlicingStepSize(2)
                ]),
                &get_array1()
            )
        );
    }

    #[test]
    fn evaluate_array1_negative_end_with_step() {
        assert_eq!(
            json!([1, 3]),
            evaluate(
                &Arc::from(vec![
                    Ast::ArraySlicingStart(0),
                    Ast::ArraySlicingEnd(-2),
                    Ast::ArraySlicingStepSize(2)
                ]),
                &get_array1()
            )
        );
    }

    #[test]
    fn evaluate_array1_positive_end_without_step() {
        assert_eq!(
            json!([1, 2, 3, 4, 5]),
            evaluate(
                &Arc::from(vec![Ast::ArraySlicingStart(0), Ast::ArraySlicingEnd(100)]),
                &get_array1()
            )
        );
    }

    #[test]
    fn evaluate_array1_negative_end_without_step() {
        assert_eq!(
            json!([1, 2, 3]),
            evaluate(
                &Arc::from(vec![Ast::ArraySlicingStart(0), Ast::ArraySlicingEnd(-2)]),
                &get_array1()
            )
        );
    }

    #[test]
    fn evaluate_array1_only_negative_step_size() {
        assert_eq!(
            json!([5, 3, 1]),
            evaluate(
                &Arc::from(vec![Ast::ArraySlicingStepSize(-2)]),
                &get_array1()
            )
        );
    }

    #[test]
    fn evaluate_array1_only_start() {
        assert_eq!(
            json!([3, 4, 5]),
            evaluate(&Arc::from(vec![Ast::ArraySlicingStart(2)]), &get_array1())
        );
    }

    #[test]
    fn evaluate_array1_only_end() {
        assert_eq!(
            json!([1, 2, 3]),
            evaluate(&Arc::from(vec![Ast::ArraySlicingEnd(3)]), &get_array1())
        );
    }

    #[test]
    fn evaluate_array1_without_any_parameter() {
        assert_eq!(
            json!([1, 2, 3, 4, 5]),
            evaluate(&Arc::from(vec![]), &get_array1())
        );
    }
}
