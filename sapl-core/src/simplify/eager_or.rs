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

pub(crate) fn eager_or(lhs: &mut Arc<Ast>, rhs: &mut Arc<Ast>) -> Option<Ast> {
    use Ast::*;
    match (lhs.as_ref(), rhs.as_ref()) {
        (Boolean(l), Boolean(r)) => Some(Boolean(l | r)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::dec;
    use std::sync::Arc;

    #[test]
    fn test_eager_or_boolean() {
        fn init(lhs: bool, rhs: bool) -> (Arc<Ast>, Arc<Ast>) {
            (Arc::new(Ast::Boolean(lhs)), Arc::new(Ast::Boolean(rhs)))
        }
        let (mut lhs, mut rhs) = init(false, false);
        assert_eq!(Some(Ast::Boolean(false)), eager_or(&mut lhs, &mut rhs));

        let (mut lhs, mut rhs) = init(false, true);
        assert_eq!(Some(Ast::Boolean(true)), eager_or(&mut lhs, &mut rhs));

        let (mut lhs, mut rhs) = init(true, false);
        assert_eq!(Some(Ast::Boolean(true)), eager_or(&mut lhs, &mut rhs));

        let (mut lhs, mut rhs) = init(true, true);
        assert_eq!(Some(Ast::Boolean(true)), eager_or(&mut lhs, &mut rhs));
    }

    #[test]
    fn test_add_incompatible_types_integer_float() {
        let mut lhs = Arc::new(Ast::Integer(5));
        let mut rhs = Arc::new(Ast::Float(dec!(3.14)));
        assert_eq!(None, eager_or(&mut lhs, &mut rhs));
        assert_eq!(None, eager_or(&mut rhs, &mut lhs));
    }
}
