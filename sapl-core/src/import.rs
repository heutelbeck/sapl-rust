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

use crate::Rule;

#[derive(Debug)]
pub enum Import {
    Function(String),
    Library { name: String, alias: String },
    Wildcard(String),
}

impl Import {
    pub fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        use Import::*;

        fn parse_import_statement_ids(pair: pest::iterators::Pair<Rule>) -> String {
            match pair.as_rule() {
                Rule::id => pair.as_str().to_string(),
                rule => unreachable!("parse_import_statement_ids expected id, found {:?}", rule),
            }
        }

        match pair.as_rule() {
            Rule::function_import => Import::Function(pair.as_str().to_string()),
            Rule::library_import => {
                println!("Hallo Welt 123");
                let mut ids: Vec<String> =
                    pair.into_inner().map(parse_import_statement_ids).collect();
                let alias = ids.pop().unwrap();
                Import::Library {
                    name: ids.join("."),
                    alias,
                }
            }
            Rule::wildcard_import => Wildcard(pair.as_str().to_string()),
            rule => unreachable!(
                "Sapl::parse expected function_import, library_import or wildcard_import, found {:?}",
                rule
            ),
        }
    }
}
