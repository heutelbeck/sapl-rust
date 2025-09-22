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

use crate::{
    Ast, Val,
    evaluate::{index_step, key_step},
};
use log::error;
use serde_json::{Value, json};
use std::sync::{Arc, RwLock};

pub(crate) fn evaluate(expr: &Ast, keys: &[Ast], src: &Value) -> Value {
    match expr.evaluate_inner(Arc::new(RwLock::new(json!("")))) {
        Ok(Val::Integer(i)) => match src.get(usize::try_from(i).unwrap()) {
            Some(data) => match keys.first() {
                Some(Ast::KeyStep(s)) | Some(Ast::EscapedKeyStep(s)) => {
                    key_step::evaluate(s, keys.get(1..).unwrap_or(&[]), data)
                }
                Some(Ast::IndexStep(i)) => {
                    index_step::evaluate(*i, keys.get(1..).unwrap_or(&[]), data)
                }
                None => data.clone(),
                _ => Value::Null,
            },
            None => Value::Null,
        },
        Ok(Val::String(s)) => match src.get(s) {
            Some(data) => match keys.first() {
                Some(Ast::KeyStep(s)) | Some(Ast::EscapedKeyStep(s)) => {
                    key_step::evaluate(s, keys.get(1..).unwrap_or(&[]), data)
                }
                Some(Ast::IndexStep(i)) => {
                    index_step::evaluate(*i, keys.get(1..).unwrap_or(&[]), data)
                }
                None => data.clone(),
                _ => Value::Null,
            },
            None => Value::Null,
        },
        other => {
            error!("Expression Step evaluation expected integer or string, but got {other:#?}");
            Value::Null
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn get_data() -> Value {
        json!([1, 2, 3, 4, 5])
    }

    fn get_data2() -> Value {
        json!([1, 2, 3, 4])
    }

    fn get_expr() -> Arc<Ast> {
        Arc::new(Ast::Expr {
            lhs: Ast::Integer(3).into(),
            op: crate::Op::Addition,
            rhs: Ast::Integer(1).into(),
        })
    }

    #[test]
    fn evaluate_to_null() {
        assert_eq!(Value::Null, evaluate(&get_expr(), &[], &get_data2()));
    }

    #[test]
    fn evaluate_to_value1() {
        assert_eq!(json!(5), evaluate(&get_expr(), &[], &get_data()));
    }
}
