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
use crate::Val;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use serde_json::Value;
use std::sync::{Arc, RwLock};

pub(crate) fn basic_identifier(
    bi: &Arc<[Ast]>,
    variable_context: Arc<RwLock<Value>>,
) -> Result<Val, String> {
    use self::Ast::*;

    let result: Value = match bi.first() {
        Some(BasicIdentifierExpression(bie)) => {
            bie.evaluate(bi.get(1..).unwrap_or(&[]), variable_context)
        }
        Some(Id(id)) => {
            crate::evaluate::id::evaluate(id, bi.get(1..).unwrap_or(&[]), variable_context)
        }
        _ => Value::Null,
    };

    match result {
        Value::String(s) => Ok(Val::String(s.clone())),
        Value::Number(n) => {
            if n.is_i64() {
                Ok(Val::Integer(n.as_i64().unwrap()))
            } else {
                Ok(Val::Float(Decimal::from_str(&n.to_string()).unwrap()))
            }
        }
        Value::Bool(b) => Ok(Val::Boolean(b)),
        Value::Array(_) => Ok(Val::Json(result)),
        Value::Null => Ok(Val::Json(Value::Null)),
        Value::Object(_) => Ok(Val::Json(result)),
    }
}
