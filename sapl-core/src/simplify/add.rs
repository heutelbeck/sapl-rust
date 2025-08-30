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

use crate::Ast;
use std::sync::Arc;

pub(crate) fn add(lhs: &mut Arc<Ast>, rhs: &mut Arc<Ast>) -> Option<Ast> {
    use Ast::*;
    match (lhs.as_ref(), rhs.as_ref()) {
        (Integer(l), Integer(r)) => Some(Integer(l.saturating_add(*r))),
        (Float(l), Float(r)) => Some(Float(*l + *r)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::{Decimal, dec};
    use std::sync::Arc;

    fn init_int(lhs: i64, rhs: i64) -> (Arc<Ast>, Arc<Ast>) {
        (Arc::new(Ast::Integer(lhs)), Arc::new(Ast::Integer(rhs)))
    }

    fn init_float(lhs: Decimal, rhs: Decimal) -> (Arc<Ast>, Arc<Ast>) {
        (Arc::new(Ast::Float(lhs)), Arc::new(Ast::Float(rhs)))
    }

    #[test]
    fn test_add_positive_integers() {
        let (mut lhs, mut rhs) = init_int(5, 3);
        assert_eq!(Some(Ast::Integer(8)), add(&mut lhs, &mut rhs));
    }

    #[test]
    fn test_add_negative_integers() {
        let (mut lhs, mut rhs) = init_int(-10, -5);
        assert_eq!(Some(Ast::Integer(-15)), add(&mut lhs, &mut rhs));
    }

    #[test]
    fn test_add_mixed_sign_integers() {
        let (mut lhs, mut rhs) = init_int(10, -3);
        assert_eq!(Some(Ast::Integer(7)), add(&mut lhs, &mut rhs));
    }

    #[test]
    fn test_add_zero_integers() {
        let (mut lhs, mut rhs) = init_int(0, 5);
        assert_eq!(Some(Ast::Integer(5)), add(&mut lhs, &mut rhs));

        let (mut lhs, mut rhs) = init_int(42, 0);
        assert_eq!(Some(Ast::Integer(42)), add(&mut lhs, &mut rhs));
    }

    #[test]
    fn test_add_integer_overflow_saturating() {
        let (mut lhs, mut rhs) = init_int(i64::MAX, 1);
        assert_eq!(Some(Ast::Integer(i64::MAX)), add(&mut lhs, &mut rhs));
    }

    #[test]
    fn test_add_integer_underflow_saturating() {
        let (mut lhs, mut rhs) = init_int(i64::MIN, -1);
        assert_eq!(Some(Ast::Integer(i64::MIN)), add(&mut lhs, &mut rhs));
    }

    #[test]
    fn test_add_positive_floats() {
        let (mut lhs, mut rhs) = init_float(dec!(3.14), dec!(2.86));
        assert_eq!(Some(Ast::Float(dec!(6.0))), add(&mut lhs, &mut rhs));
    }

    #[test]
    fn test_add_negative_floats() {
        let (mut lhs, mut rhs) = init_float(dec!(-1.5), dec!(-2.5));
        assert_eq!(Some(Ast::Float(dec!(-4.0))), add(&mut lhs, &mut rhs));
    }

    #[test]
    fn test_add_mixed_sign_floats() {
        let (mut lhs, mut rhs) = init_float(dec!(10.7), dec!(-3.2));
        assert_eq!(Some(Ast::Float(dec!(7.5))), add(&mut lhs, &mut rhs));
    }

    #[test]
    fn test_add_float_with_zero() {
        let (mut lhs, mut rhs) = init_float(dec!(42.42), Decimal::ZERO);
        assert_eq!(Some(Ast::Float(dec!(42.42))), add(&mut lhs, &mut rhs));

        let (mut lhs, mut rhs) = init_float(Decimal::ZERO, dec!(42.42));
        assert_eq!(Some(Ast::Float(dec!(42.42))), add(&mut lhs, &mut rhs));
    }

    #[test]
    fn test_add_incompatible_types_integer_float() {
        let mut lhs = Arc::new(Ast::Integer(5));
        let mut rhs = Arc::new(Ast::Float(dec!(3.14)));
        assert_eq!(None, add(&mut lhs, &mut rhs));
        assert_eq!(None, add(&mut rhs, &mut lhs));
    }
}
