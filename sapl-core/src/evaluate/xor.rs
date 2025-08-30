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

pub(crate) fn xor(lhs: &Result<Val, String>, rhs: &Result<Val, String>) -> Result<Val, String> {
    use crate::Val::*;
    match (lhs, rhs) {
        (Ok(Boolean(l)), Ok(Boolean(r))) => Ok(Boolean(l ^ r)),
        (Ok(Integer(l)), Ok(Integer(r))) => Ok(Integer(l ^ r)),
        (Err(e), _) => Err(e.clone()),
        (_, Err(e)) => Err(e.clone()),
        (lhs, rhs) => Err(format!(
            "Type mismatch. Xor operation expects boolean or integer values, but got: {lhs:#?} and {rhs:#?}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_boolean() {
        fn init(lhs: bool, rhs: bool) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::Boolean(lhs)), Ok(Val::Boolean(rhs)))
        }

        let (lhs, rhs) = init(false, false);
        assert_eq!(Ok(Val::Boolean(false)), xor(&lhs, &rhs));

        let (lhs, rhs) = init(false, true);
        assert_eq!(Ok(Val::Boolean(true)), xor(&lhs, &rhs));

        let (lhs, rhs) = init(true, false);
        assert_eq!(Ok(Val::Boolean(true)), xor(&lhs, &rhs));

        let (lhs, rhs) = init(true, true);
        assert_eq!(Ok(Val::Boolean(false)), xor(&lhs, &rhs));
    }

    #[test]
    fn xor_integer() {
        fn init(lhs: i64, rhs: i64) -> (Result<Val, String>, Result<Val, String>) {
            (Ok(Val::Integer(lhs)), Ok(Val::Integer(rhs)))
        }

        let (lhs, rhs) = init(0b01i64, 0b10i64);
        assert_eq!(Ok(Val::Integer(0b11i64)), xor(&lhs, &rhs));

        let (lhs, rhs) = init(0b101i64, 0b110i64);
        assert_eq!(Ok(Val::Integer(0b011i64)), xor(&lhs, &rhs));

        let (lhs, rhs) = init(0b1010i64, 0b1100i64);
        assert_eq!(Ok(Val::Integer(0b0110i64)), xor(&lhs, &rhs));
    }

    #[test]
    fn xor_lhs_error() {
        let lhs = Err("Fault lhs".to_string());
        let rhs = Ok(Val::String("something".to_string()));

        assert_eq!(Err("Fault lhs".to_string()), xor(&lhs, &rhs));
    }

    #[test]
    fn xor_rhs_error() {
        let lhs = Ok(Val::String("something".to_string()));
        let rhs = Err("Fault rhs".to_string());

        assert_eq!(Err("Fault rhs".to_string()), xor(&lhs, &rhs));
    }

    #[test]
    fn xor_error() {
        let lhs = Ok(Val::String("something".to_string()));
        let rhs = Ok(Val::Integer(42));

        let err = Err(
            "Type mismatch. Xor operation expects boolean or integer values, but got: Ok(\n    String(\n        \"something\",\n    ),\n) and Ok(\n    Integer(\n        42,\n    ),\n)".to_string(),
        );
        assert_eq!(err, xor(&lhs, &rhs));
    }
}
