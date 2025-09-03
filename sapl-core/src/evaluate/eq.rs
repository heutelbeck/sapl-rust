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

pub(crate) fn eq(lhs: &Result<Val, String>, rhs: &Result<Val, String>) -> Result<Val, String> {
    use crate::Val::*;
    match (lhs, rhs) {
        (Ok(Boolean(l)), Ok(Boolean(r))) => Ok(Boolean(l == r)),
        (Ok(Integer(l)), Ok(Integer(r))) => Ok(Boolean(l == r)),
        (Ok(Float(l)), Ok(Float(r))) => Ok(Boolean(l == r)),
        (Ok(CompInteger(l, _)), Ok(Boolean(r))) => Ok(Boolean(l == r)),
        (Ok(Boolean(l)), Ok(CompInteger(r, _))) => Ok(Boolean(l == r)),
        (Ok(CompInteger(l, _)), Ok(CompInteger(r, _))) => Ok(Boolean(l == r)),
        (Ok(CompFloat(l, _)), Ok(Boolean(r))) => Ok(Boolean(l == r)),
        (Ok(Boolean(l)), Ok(CompFloat(r, _))) => Ok(Boolean(l == r)),
        (Ok(CompFloat(l, _)), Ok(CompFloat(r, _))) => Ok(Boolean(l == r)),
        (Ok(String(l)), Ok(String(r))) => Ok(Boolean(l.eq(r))),
        (Ok(Json(l)), Ok(Json(r))) => Ok(Boolean(l.eq(r))),
        (Ok(Json(l)), Ok(String(r))) => Ok(Boolean(l.to_string().eq(r))),
        (Err(e), _) => Err(e.clone()),
        (_, Err(e)) => Err(e.clone()),
        (lhs, rhs) => Err(format!(
            "Type mismatch. Equal operation expects comparable values, but got: {lhs:#?} and {rhs:#?}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::{Decimal, dec};

    #[test]
    fn eq_boolean() {
        fn init(lhs: bool, rhs: bool) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::Boolean(lhs)), Ok(Val::Boolean(rhs)))
        }

        let (lhs, rhs) = init(false, false);
        assert_eq!(Ok(Val::Boolean(true)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(true, false);
        assert_eq!(Ok(Val::Boolean(false)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(false, true);
        assert_eq!(Ok(Val::Boolean(false)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(true, true);
        assert_eq!(Ok(Val::Boolean(true)), eq(&lhs, &rhs));
    }

    #[test]
    fn eq_integer() {
        fn init(lhs: i64, rhs: i64) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::Integer(lhs)), Ok(Val::Integer(rhs)))
        }

        let (lhs, rhs) = init(42, 42);
        assert_eq!(Ok(Val::Boolean(true)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(23, 42);
        assert_eq!(Ok(Val::Boolean(false)), eq(&lhs, &rhs));
    }

    #[test]
    fn eq_float() {
        fn init(lhs: Decimal, rhs: Decimal) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::Float(lhs)), Ok(Val::Float(rhs)))
        }

        let (lhs, rhs) = init(dec!(42), dec!(42));
        assert_eq!(Ok(Val::Boolean(true)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(dec!(23), dec!(42));
        assert_eq!(Ok(Val::Boolean(false)), eq(&lhs, &rhs));
    }

    #[test]
    fn eq_boolean_with_comp_integer() {
        fn init(lhs: bool, rhs: bool) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::Boolean(lhs)), Ok(Val::CompInteger(rhs, 23)))
        }

        let (lhs, rhs) = init(false, false);
        assert_eq!(Ok(Val::Boolean(true)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(true, false);
        assert_eq!(Ok(Val::Boolean(false)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(false, true);
        assert_eq!(Ok(Val::Boolean(false)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(true, true);
        assert_eq!(Ok(Val::Boolean(true)), eq(&lhs, &rhs));
    }

    #[test]
    fn eq_comp_integer_with_boolean() {
        fn init(lhs: bool, rhs: bool) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::CompInteger(lhs, 42)), Ok(Val::Boolean(rhs)))
        }

        let (lhs, rhs) = init(false, false);
        assert_eq!(Ok(Val::Boolean(true)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(true, false);
        assert_eq!(Ok(Val::Boolean(false)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(false, true);
        assert_eq!(Ok(Val::Boolean(false)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(true, true);
        assert_eq!(Ok(Val::Boolean(true)), eq(&lhs, &rhs));
    }

    #[test]
    fn eq_comp_integer_with_comp_integer() {
        fn init(lhs: bool, rhs: bool) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::CompInteger(lhs, 42)), Ok(Val::CompInteger(rhs, 23)))
        }

        let (lhs, rhs) = init(false, false);
        assert_eq!(Ok(Val::Boolean(true)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(true, false);
        assert_eq!(Ok(Val::Boolean(false)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(false, true);
        assert_eq!(Ok(Val::Boolean(false)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(true, true);
        assert_eq!(Ok(Val::Boolean(true)), eq(&lhs, &rhs));
    }

    #[test]
    fn eq_boolean_with_comp_float() {
        fn init(lhs: bool, rhs: bool) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::Boolean(lhs)), Ok(Val::CompFloat(rhs, dec!(23))))
        }

        let (lhs, rhs) = init(false, false);
        assert_eq!(Ok(Val::Boolean(true)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(true, false);
        assert_eq!(Ok(Val::Boolean(false)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(false, true);
        assert_eq!(Ok(Val::Boolean(false)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(true, true);
        assert_eq!(Ok(Val::Boolean(true)), eq(&lhs, &rhs));
    }

    #[test]
    fn eq_comp_float_with_boolean() {
        fn init(lhs: bool, rhs: bool) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::CompFloat(lhs, dec!(42))), Ok(Val::Boolean(rhs)))
        }

        let (lhs, rhs) = init(false, false);
        assert_eq!(Ok(Val::Boolean(true)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(true, false);
        assert_eq!(Ok(Val::Boolean(false)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(false, true);
        assert_eq!(Ok(Val::Boolean(false)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(true, true);
        assert_eq!(Ok(Val::Boolean(true)), eq(&lhs, &rhs));
    }

    #[test]
    fn eq_comp_float_with_comp_float() {
        fn init(lhs: bool, rhs: bool) -> (Result<Val, String>, Result<Val, String>) {
            (
                Ok(Val::CompFloat(lhs, dec!(42))),
                Ok(Val::CompFloat(rhs, dec!(23))),
            )
        }

        let (lhs, rhs) = init(false, false);
        assert_eq!(Ok(Val::Boolean(true)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(true, false);
        assert_eq!(Ok(Val::Boolean(false)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(false, true);
        assert_eq!(Ok(Val::Boolean(false)), eq(&lhs, &rhs));

        let (lhs, rhs) = init(true, true);
        assert_eq!(Ok(Val::Boolean(true)), eq(&lhs, &rhs));
    }

    #[test]
    fn eq_string() {
        fn init(lhs: &str, rhs: &str) -> (Result<Val, String>, Result<Val, String>) {
            (
                Ok(Val::String(lhs.to_owned())),
                Ok(Val::String(rhs.to_owned())),
            )
        }

        let (lhs, rhs) = init("Hallo Welt!", "Hallo Welt!");
        assert_eq!(Ok(Val::Boolean(true)), eq(&lhs, &rhs));

        let (lhs, rhs) = init("Hallo", " Welt");
        assert_eq!(Ok(Val::Boolean(false)), eq(&lhs, &rhs));
    }

    #[test]
    fn eq_lhs_error() {
        let lhs = Err("Fault lhs".to_string());
        let rhs = Ok(Val::Boolean(false));

        assert_eq!(Err("Fault lhs".to_string()), eq(&lhs, &rhs));
    }

    #[test]
    fn eq_rhs_error() {
        let lhs = Ok(Val::Boolean(true));
        let rhs = Err("Fault rhs".to_string());

        assert_eq!(Err("Fault rhs".to_string()), eq(&lhs, &rhs));
    }

    #[test]
    fn eq_error() {
        let lhs = Ok(Val::String("something".to_string()));
        let rhs = Ok(Val::Boolean(true));

        let err = Err(
            "Type mismatch. Equal operation expects comparable values, but got: Ok(\n    String(\n        \"something\",\n    ),\n) and Ok(\n    Boolean(\n        true,\n    ),\n)".to_string(),
        );
        assert_eq!(err, eq(&lhs, &rhs));
    }
}
