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

use crate::{Ast, Val};
use rust_decimal::prelude::ToPrimitive;
use serde_json::{Number, Value};
use std::sync::{Arc, RwLock};

pub(crate) fn array(
    array: &Arc<[Ast]>,
    variable_context: Arc<RwLock<Value>>,
) -> Result<Val, String> {
    let mut json_array: Vec<Value> = Vec::new();
    for a in array.iter() {
        match a.evaluate_inner(variable_context.clone()) {
            Ok(Val::Json(obj)) => json_array.push(obj),
            Ok(Val::Integer(i)) => json_array.push(serde_json::Value::Number(
                Number::from_i128(i as i128).unwrap(),
            )),
            Ok(Val::Float(f)) => json_array.push(serde_json::Value::Number(
                Number::from_f64(f.to_f64().expect("Failed to convert Decimal to f64"))
                    .expect("Failed to convert f64 to json number"),
            )),
            Ok(Val::String(s)) => json_array.push(s.into()),
            _ => {}
        }
    }

    Ok(Val::Json(Value::Array(json_array)))
}
