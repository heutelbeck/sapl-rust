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

pub(crate) fn mul(lhs: &Result<Val, String>, rhs: &Result<Val, String>) -> Result<Val, String> {
    use crate::Val::*;
    match (lhs, rhs) {
        (Ok(Integer(l)), Ok(Integer(r))) => Ok(Integer(l.saturating_mul(*r))),
        (Ok(Float(l)), Ok(Float(r))) => Ok(Float(l * r)),
        (Err(e), _) => Err(e.clone()),
        (_, Err(e)) => Err(e.clone()),
        (lhs, rhs) => Err(format!(
            "Type mismatch. Multiplication operation expects decimal value, but got: {lhs:#?} and {rhs:#?}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::dec;

    #[test]
    fn mul_integer() {
        let lhs = Ok(Val::Integer(21));
        let rhs = Ok(Val::Integer(2));

        assert_eq!(Ok(Val::Integer(42)), mul(&lhs, &rhs));
    }

    #[test]
    fn mul_float() {
        let lhs = Ok(Val::Float(dec!(0.3)));
        let rhs = Ok(Val::Float(dec!(2)));

        assert_eq!(Ok(Val::Float(dec!(0.6))), mul(&lhs, &rhs));
    }

    #[test]
    fn mul_lhs_error() {
        let lhs = Err("Fault lhs".to_string());
        let rhs = Ok(Val::String("something".to_string()));

        assert_eq!(Err("Fault lhs".to_string()), mul(&lhs, &rhs));
    }

    #[test]
    fn mul_rhs_error() {
        let lhs = Ok(Val::String("something".to_string()));
        let rhs = Err("Fault rhs".to_string());

        assert_eq!(Err("Fault rhs".to_string()), mul(&lhs, &rhs));
    }

    #[test]
    fn mul_error() {
        let lhs = Ok(Val::String("something".to_string()));
        let rhs = Ok(Val::Integer(42));

        let err = Err(
            "Type mismatch. Multiplication operation expects decimal value, but got: Ok(\n    String(\n        \"something\",\n    ),\n) and Ok(\n    Integer(\n        42,\n    ),\n)".to_string(),
        );
        assert_eq!(err, mul(&lhs, &rhs));
    }
}
