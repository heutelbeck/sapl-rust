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

pub(crate) fn ge(lhs: Result<Val, String>, rhs: Result<Val, String>) -> Result<Val, String> {
    use crate::Val::*;
    match (lhs, rhs) {
        (Ok(Boolean(l)), Ok(Boolean(r))) => Ok(Boolean(l & !r)),
        (Ok(Integer(l)), Ok(Integer(r))) => Ok(CompInteger(l > r, r)),
        (Ok(CompInteger(eval, l)), Ok(Integer(r))) => {
            if eval {
                Ok(CompInteger(l > r, r))
            } else {
                Ok(Boolean(false))
            }
        }
        (Ok(Float(l)), Ok(Float(r))) => Ok(CompFloat(l > r, r)),
        (Ok(CompFloat(eval, l)), Ok(Float(r))) => {
            if eval {
                Ok(CompFloat(l > r, r))
            } else {
                Ok(Boolean(false))
            }
        }
        (Err(e), _) => Err(e),
        (_, Err(e)) => Err(e),
        (lhs, rhs) => Err(format!(
            "Type mismatch. Greater operation expects decimal values, but got: {lhs:#?} and {rhs:#?}"
        )),
    }
}
