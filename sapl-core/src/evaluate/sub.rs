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

use crate::Val;

pub(crate) fn sub(lhs: &Result<Val, String>, rhs: &Result<Val, String>) -> Result<Val, String> {
    use crate::Val::*;
    match (lhs, rhs) {
        (Ok(Integer(l)), Ok(Integer(r))) => Ok(Integer(l - r)),
        (Ok(Float(l)), Ok(Float(r))) => Ok(Float(l - r)),
        (Err(e), _) => Err(e.clone()),
        (_, Err(e)) => Err(e.clone()),
        (lhs, rhs) => Err(format!(
            "Type mismatch. Subtraction operation expects decimal values, but got: {lhs:#?} and {rhs:#?}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_integer() {
        let lhs = Ok(Val::Integer(44));
        let rhs = Ok(Val::Integer(2));

        assert_eq!(Ok(Val::Integer(42)), sub(&lhs, &rhs));
    }

    #[test]
    fn sub_float() {
        let lhs = Ok(Val::Float(0.25));
        let rhs = Ok(Val::Float(0.125));

        assert_eq!(Ok(Val::Float(0.125)), sub(&lhs, &rhs));
    }

    #[test]
    fn sub_lhs_error() {
        let lhs = Err("Fault lhs".to_string());
        let rhs = Ok(Val::String("something".to_string()));

        assert_eq!(Err("Fault lhs".to_string()), sub(&lhs, &rhs));
    }

    #[test]
    fn sub_rhs_error() {
        let lhs = Ok(Val::String("something".to_string()));
        let rhs = Err("Fault rhs".to_string());

        assert_eq!(Err("Fault rhs".to_string()), sub(&lhs, &rhs));
    }

    #[test]
    fn sub_error() {
        let lhs = Ok(Val::String("something".to_string()));
        let rhs = Ok(Val::Integer(42));

        let err = Err(
            "Type mismatch. Subtraction operation expects decimal values, but got: Ok(\n    String(\n        \"something\",\n    ),\n) and Ok(\n    Integer(\n        42,\n    ),\n)".to_string(),
        );
        assert_eq!(err, sub(&lhs, &rhs));
    }
}
