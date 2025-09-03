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
use serde_json::Value;
use std::sync::{Arc, RwLock};

pub(crate) fn variable_assignment(
    va: &[Ast],
    variable_context: Arc<RwLock<Value>>,
) -> Result<Val, String> {
    if va.len() < 2 {
        return Ok(Val::Boolean(true));
    }

    if let Ok(Val::String(s)) = va[0].evaluate_inner(Arc::new(RwLock::new(Value::Null))) {
        let value: Value = va[1]
            .evaluate_inner(Arc::new(RwLock::new(Value::Null)))?
            .to_value();

        {
            let mut var = variable_context
                .write()
                .map_err(|e| format!("variable_assignment unable to get write lock {:?}", e))?;

            if let Some(obj) = var.as_object_mut() {
                obj.insert(s, value);
            }
        }
    }

    Ok(Val::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authorization_subscription::AuthorizationSubscription;
    use std::sync::Arc;

    #[test]
    fn add_variable_to_context() {
        let variable_context = Arc::new(RwLock::new(
            AuthorizationSubscription::new_example_subscription1(),
        ));

        let va = Arc::from([Ast::Id("test".to_string()), Ast::String("demo".to_string())]);
        let result = variable_assignment(&va, variable_context.clone());

        assert_eq!("{\"subject\":\"WILLI\",\"action\":\"read\",\"resource\":\"something\",\"test\":\"demo\"}".to_string(), variable_context.read().unwrap().to_string());
        assert_eq!(Ok(Val::Boolean(true)), result);
    }
}
