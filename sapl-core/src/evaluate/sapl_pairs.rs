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
use serde_json::{Map, Value};
use std::sync::{Arc, RwLock};

pub(crate) fn sapl_pairs(
    pairs: &Arc<[Ast]>,
    variable_context: Arc<RwLock<Value>>,
) -> Result<Val, String> {
    let mut map: Map<std::string::String, Value> = Map::new();

    for pair in pairs.iter() {
        if let Ok(Val::Json(Value::Object(obj))) = pair.evaluate_inner(variable_context.clone()) {
            for (k, v) in obj {
                map.insert(k, v);
            }
        }
    }

    Ok(Val::Json(Value::Object(map)))
}
