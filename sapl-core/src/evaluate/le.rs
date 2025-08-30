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

pub(crate) fn le(lhs: &Result<Val, String>, rhs: &Result<Val, String>) -> Result<Val, String> {
    use crate::Val::*;
    match (lhs, rhs) {
        (Ok(Integer(l)), Ok(Integer(r))) => Ok(CompInteger(l < r, *r)),
        (Ok(CompInteger(eval, l)), Ok(Integer(r))) => {
            if *eval {
                Ok(CompInteger(l < r, *r))
            } else {
                Ok(Boolean(false))
            }
        }
        (Ok(Float(l)), Ok(Float(r))) => Ok(CompFloat(l < r, *r)),
        (Ok(CompFloat(eval, l)), Ok(Float(r))) => {
            if *eval {
                Ok(CompFloat(l < r, *r))
            } else {
                Ok(Boolean(false))
            }
        }
        (Err(e), _) => Err(e.clone()),
        (_, Err(e)) => Err(e.clone()),
        (lhs, rhs) => Err(format!(
            "Type mismatch. Less operation expects decimal values, but got: {lhs:#?} and {rhs:#?}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::{Decimal, dec};

    #[test]
    fn le_integer() {
        fn init(lhs: i64, rhs: i64) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::Integer(lhs)), Ok(Val::Integer(rhs)))
        }

        let (lhs, rhs) = init(42, 23);
        assert_eq!(Ok(Val::CompInteger(false, 23)), le(&lhs, &rhs));

        let (lhs, rhs) = init(42, 42);
        assert_eq!(Ok(Val::CompInteger(false, 42)), le(&lhs, &rhs));

        let (lhs, rhs) = init(23, 42);
        assert_eq!(Ok(Val::CompInteger(true, 42)), le(&lhs, &rhs));
    }

    #[test]
    fn le_comp_integer_with_integer() {
        fn init(lhs_b: bool, lhs_i: i64, rhs: i64) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::CompInteger(lhs_b, lhs_i)), Ok(Val::Integer(rhs)))
        }

        let (lhs, rhs) = init(true, 42, 23);
        assert_eq!(Ok(Val::CompInteger(false, 23)), le(&lhs, &rhs));

        let (lhs, rhs) = init(false, 42, 42);
        assert_eq!(Ok(Val::Boolean(false)), le(&lhs, &rhs));

        let (lhs, rhs) = init(true, 42, 42);
        assert_eq!(Ok(Val::CompInteger(false, 42)), le(&lhs, &rhs));

        let (lhs, rhs) = init(true, 23, 42);
        assert_eq!(Ok(Val::CompInteger(true, 42)), le(&lhs, &rhs));
    }

    #[test]
    fn le_float() {
        fn init(lhs: Decimal, rhs: Decimal) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::Float(lhs)), Ok(Val::Float(rhs)))
        }

        let (lhs, rhs) = init(dec!(4.2), dec!(2.3));
        assert_eq!(Ok(Val::CompFloat(false, dec!(2.3))), le(&lhs, &rhs));

        let (lhs, rhs) = init(dec!(0.1), dec!(0.1));
        assert_eq!(Ok(Val::CompFloat(false, dec!(0.1))), le(&lhs, &rhs));

        let (lhs, rhs) = init(dec!(2.3), dec!(4.2));
        assert_eq!(Ok(Val::CompFloat(true, dec!(4.2))), le(&lhs, &rhs));
    }

    #[test]
    fn le_comp_float_with_float() {
        fn init(
            lhs_b: bool,
            lhs_i: Decimal,
            rhs: Decimal,
        ) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::CompFloat(lhs_b, lhs_i)), Ok(Val::Float(rhs)))
        }

        let (lhs, rhs) = init(true, dec!(4.2), dec!(2.3));
        assert_eq!(Ok(Val::CompFloat(false, dec!(2.3))), le(&lhs, &rhs));

        let (lhs, rhs) = init(false, dec!(4.2), dec!(4.2));
        assert_eq!(Ok(Val::Boolean(false)), le(&lhs, &rhs));

        let (lhs, rhs) = init(true, dec!(4.2), dec!(4.2));
        assert_eq!(Ok(Val::CompFloat(false, dec!(4.2))), le(&lhs, &rhs));

        let (lhs, rhs) = init(true, dec!(2.3), dec!(4.2));
        assert_eq!(Ok(Val::CompFloat(true, dec!(4.2))), le(&lhs, &rhs));
    }

    #[test]
    fn le_lhs_error() {
        let lhs = Err("Fault lhs".to_string());
        let rhs = Ok(Val::String("something".to_string()));

        assert_eq!(Err("Fault lhs".to_string()), le(&lhs, &rhs));
    }

    #[test]
    fn le_rhs_error() {
        let lhs = Ok(Val::String("something".to_string()));
        let rhs = Err("Fault rhs".to_string());

        assert_eq!(Err("Fault rhs".to_string()), le(&lhs, &rhs));
    }

    #[test]
    fn le_error() {
        let lhs = Ok(Val::String("something".to_string()));
        let rhs = Ok(Val::Integer(42));

        let err = Err(
            "Type mismatch. Less operation expects decimal values, but got: Ok(\n    String(\n        \"something\",\n    ),\n) and Ok(\n    Integer(\n        42,\n    ),\n)".to_string(),
        );
        assert_eq!(err, le(&lhs, &rhs));
    }
}
