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

pub(crate) fn non_eq(lhs: Result<Val, String>, rhs: Result<Val, String>) -> Result<Val, String> {
    use crate::Val::*;
    match (lhs, rhs) {
        (Ok(Boolean(l)), Ok(Boolean(r))) => Ok(Boolean(l != r)),
        (Ok(Integer(l)), Ok(Integer(r))) => Ok(Boolean(l != r)),
        (Ok(Float(l)), Ok(Float(r))) => Ok(Boolean(l != r)),
        (Ok(CompInteger(l, _)), Ok(Boolean(r))) => Ok(Boolean(l != r)),
        (Ok(Boolean(l)), Ok(CompInteger(r, _))) => Ok(Boolean(l != r)),
        (Ok(CompInteger(l, _)), Ok(CompInteger(r, _))) => Ok(Boolean(l != r)),
        (Ok(CompFloat(l, _)), Ok(Boolean(r))) => Ok(Boolean(l != r)),
        (Ok(Boolean(l)), Ok(CompFloat(r, _))) => Ok(Boolean(l != r)),
        (Ok(CompFloat(l, _)), Ok(CompFloat(r, _))) => Ok(Boolean(l != r)),
        (Ok(String(l)), Ok(String(r))) => Ok(Boolean(!l.eq(&r))),
        (Err(e), _) => Err(e),
        (_, Err(e)) => Err(e),
        (lhs, rhs) => Err(format!(
            "Type mismatch. Non equals operation expects comparable values, but got: {lhs:#?} and {rhs:#?}"
        )),
    }
}
