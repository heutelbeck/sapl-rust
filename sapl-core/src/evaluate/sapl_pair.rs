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

use crate::{Ast, AuthorizationSubscription, Val};
use serde_json::{Value, json};
use std::sync::Arc;

pub(crate) fn sapl_pair(
    lhs: &Arc<Ast>,
    rhs: &Arc<[Ast]>,
    auth_subscription: &AuthorizationSubscription,
) -> Result<Val, String> {
    if let Ok(Val::String(key)) = lhs.evaluate_inner(auth_subscription) {
        return Ok(Val::Json(
            json!({key: extract_string_from_saplpair_rhs(rhs, auth_subscription)}),
        ));
    }

    Err(format!(
        "Evaluation SaplPair not possible, got: {lhs:#?} and {rhs:#?}"
    ))
}

fn extract_string_from_saplpair_rhs(
    ast: &Arc<[Ast]>,
    auth_subscription: &AuthorizationSubscription,
) -> Value {
    use crate::evaluate::basic_identifier;

    let mut result = String::new();
    for item in ast.iter() {
        match item {
            Ast::BasicIdentifier(bi) => {
                if let Ok(Val::String(s)) = basic_identifier(bi, auth_subscription) {
                    result.push_str(&s);
                }
            }
            Ast::Array(_) => {
                if let Ok(Val::Json(obj)) = item.evaluate_inner(auth_subscription) {
                    return obj;
                }
            }
            Ast::String(s) => result.push_str(s),
            Ast::Integer(i) => {
                return Value::Number((*i).into());
            }
            _ => {}
        }
    }
    Value::String(result)
}
