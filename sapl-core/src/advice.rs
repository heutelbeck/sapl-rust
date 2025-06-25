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

use crate::{Ast, Rule};
use std::sync::Arc;

#[derive(Debug)]
pub struct Advice {
    expr: Arc<Ast>,
}

impl Advice {
    pub fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        Advice {
            expr: Arc::new(Ast::parse(pair.into_inner())),
        }
    }
}
