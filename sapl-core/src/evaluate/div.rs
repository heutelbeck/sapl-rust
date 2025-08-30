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

pub(crate) fn div(lhs: &Result<Val, String>, rhs: &Result<Val, String>) -> Result<Val, String> {
    use crate::Val::*;

    match (lhs, rhs) {
        (Ok(Integer(l)), Ok(Integer(r))) => {
            if *r == 0 {
                return Err("Divide by zero".to_string());
            };
            if *l == i64::MIN && *r == -1 {
                return Err("Integer overflow".to_string());
            };
            Ok(Val::Integer(l / r))
        }
        (Ok(Float(l)), Ok(Float(r))) => {
            if r.is_zero() {
                return Err("Divide by zero".to_string());
            };
            Ok(Val::Float(l / r))
        }
        (Err(e), _) => Err(e.clone()),
        (_, Err(e)) => Err(e.clone()),
        (lhs, rhs) => Err(format!(
            "Type mismatch. Division operation expects decimal values, but got: {lhs:#?} and {rhs:#?}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::{Decimal, dec};

    #[test]
    fn div_integer() {
        let lhs = Ok(Val::Integer(84));
        let rhs = Ok(Val::Integer(2));

        assert_eq!(Ok(Val::Integer(42)), div(&lhs, &rhs));
    }

    #[test]
    fn div_integer_by_zero() {
        let lhs = Ok(Val::Integer(42));
        let rhs = Ok(Val::Integer(0));

        assert_eq!(Err("Divide by zero".to_string()), div(&lhs, &rhs));
    }

    #[test]
    fn div_integer_overflow() {
        let lhs = Ok(Val::Integer(i64::MIN));
        let rhs = Ok(Val::Integer(-1));

        assert_eq!(Err("Integer overflow".to_string()), div(&lhs, &rhs));
    }

    #[test]
    fn div_float() {
        let lhs = Ok(Val::Float(dec!(0.2)));
        let rhs = Ok(Val::Float(dec!(2)));

        assert_eq!(Ok(Val::Float(dec!(0.1))), div(&lhs, &rhs));
    }

    #[test]
    fn div_float_by_zero() {
        let lhs = Ok(Val::Float(dec!(3.14)));
        let rhs = Ok(Val::Float(Decimal::ZERO));

        assert_eq!(Err("Divide by zero".to_string()), div(&lhs, &rhs));
    }

    #[test]
    fn div_lhs_error() {
        let lhs = Err("Fault lhs".to_string());
        let rhs = Ok(Val::Integer(42));

        assert_eq!(Err("Fault lhs".to_string()), div(&lhs, &rhs));
    }

    #[test]
    fn div_rhs_error() {
        let lhs = Ok(Val::Integer(42));
        let rhs = Err("Fault rhs".to_string());

        assert_eq!(Err("Fault rhs".to_string()), div(&lhs, &rhs));
    }

    #[test]
    fn div_error() {
        let lhs = Ok(Val::String("something".to_string()));
        let rhs = Ok(Val::Integer(42));

        let err = Err(
            "Type mismatch. Division operation expects decimal values, but got: Ok(\n    String(\n        \"something\",\n    ),\n) and Ok(\n    Integer(\n        42,\n    ),\n)".to_string(),
        );
        assert_eq!(err, div(&lhs, &rhs));
    }
}
