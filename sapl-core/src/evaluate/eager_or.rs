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

pub(crate) fn eager_or(
    lhs: &Result<Val, String>,
    rhs: &Result<Val, String>,
) -> Result<Val, String> {
    use Val::*;
    match (lhs, rhs) {
        (Ok(Boolean(l)), Ok(Boolean(r))) => Ok(Boolean(l | r)),
        (Ok(CompInteger(l, _)), Ok(Boolean(r))) => Ok(Boolean(l | r)),
        (Ok(Boolean(l)), Ok(CompInteger(r, _))) => Ok(Boolean(l | r)),
        (Ok(CompInteger(l, _)), Ok(CompInteger(r, _))) => Ok(Boolean(l | r)),
        (Ok(CompFloat(l, _)), Ok(Boolean(r))) => Ok(Boolean(l | r)),
        (Ok(Boolean(l)), Ok(CompFloat(r, _))) => Ok(Boolean(l | r)),
        (Ok(CompFloat(l, _)), Ok(CompFloat(r, _))) => Ok(Boolean(l | r)),
        (Err(e), _) => Err(e.clone()),
        (_, Err(e)) => Err(e.clone()),
        (lhs, rhs) => Err(format!(
            "Type mismatch. Eager or operation expects boolean values, but got: {lhs:#?} and {rhs:#?}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::dec;

    #[test]
    fn eager_or_boolean() {
        fn init(lhs: bool, rhs: bool) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::Boolean(lhs)), Ok(Val::Boolean(rhs)))
        }

        let (lhs, rhs) = init(false, false);
        assert_eq!(Ok(Val::Boolean(false)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(true, false);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(false, true);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(true, true);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));
    }

    #[test]
    fn eager_or_comp_integer_with_boolean() {
        fn init(lhs: bool, rhs: bool) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::CompInteger(lhs, 42)), Ok(Val::Boolean(rhs)))
        }

        let (lhs, rhs) = init(false, false);
        assert_eq!(Ok(Val::Boolean(false)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(true, false);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(false, true);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(true, true);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));
    }

    #[test]
    fn eager_or_boolean_with_comp_integer() {
        fn init(lhs: bool, rhs: bool) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::Boolean(lhs)), Ok(Val::CompInteger(rhs, 42)))
        }

        let (lhs, rhs) = init(false, false);
        assert_eq!(Ok(Val::Boolean(false)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(true, false);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(false, true);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(true, true);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));
    }

    #[test]
    fn eager_or_comp_interger_with_comp_integer() {
        fn init(lhs: bool, rhs: bool) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::CompInteger(lhs, 23)), Ok(Val::CompInteger(rhs, 42)))
        }

        let (lhs, rhs) = init(false, false);
        assert_eq!(Ok(Val::Boolean(false)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(true, false);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(false, true);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(true, true);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));
    }

    #[test]
    fn eager_or_comp_float_with_boolean() {
        fn init(lhs: bool, rhs: bool) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::CompFloat(lhs, dec!(42))), Ok(Val::Boolean(rhs)))
        }

        let (lhs, rhs) = init(false, false);
        assert_eq!(Ok(Val::Boolean(false)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(true, false);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(false, true);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(true, true);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));
    }

    #[test]
    fn eager_or_boolean_with_comp_float() {
        fn init(lhs: bool, rhs: bool) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::Boolean(lhs)), Ok(Val::CompFloat(rhs, dec!(42))))
        }

        let (lhs, rhs) = init(false, false);
        assert_eq!(Ok(Val::Boolean(false)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(true, false);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(false, true);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(true, true);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));
    }

    #[test]
    fn eager_or_comp_float_with_comp_float() {
        fn init(lhs: bool, rhs: bool) -> (Result<Val, String>, Result<Val, String>) {
            (
                Ok(Val::CompFloat(lhs, dec!(23))),
                Ok(Val::CompFloat(rhs, dec!(42))),
            )
        }

        let (lhs, rhs) = init(false, false);
        assert_eq!(Ok(Val::Boolean(false)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(true, false);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(false, true);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));

        let (lhs, rhs) = init(true, true);
        assert_eq!(Ok(Val::Boolean(true)), eager_or(&lhs, &rhs));
    }

    #[test]
    fn eager_or_lhs_error() {
        let lhs = Err("Fault lhs".to_string());
        let rhs = Ok(Val::Boolean(false));

        assert_eq!(Err("Fault lhs".to_string()), eager_or(&lhs, &rhs));
    }

    #[test]
    fn eager_or_rhs_error() {
        let lhs = Ok(Val::Boolean(true));
        let rhs = Err("Fault rhs".to_string());

        assert_eq!(Err("Fault rhs".to_string()), eager_or(&lhs, &rhs));
    }

    #[test]
    fn eager_or_error() {
        let lhs = Ok(Val::String("something".to_string()));
        let rhs = Ok(Val::Boolean(true));

        let err = Err(
            "Type mismatch. Eager or operation expects boolean values, but got: Ok(\n    String(\n        \"something\",\n    ),\n) and Ok(\n    Boolean(\n        true,\n    ),\n)".to_string(),
        );
        assert_eq!(err, eager_or(&lhs, &rhs));
    }
}
