use crate::Rule;

#[derive(Debug)]
pub enum Import {
    Function(String),
    Library { name: String, alias: String },
    Wildcard(String),
}

impl Import {
    pub fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        fn parse_import_statement_sapl_ids(pair: pest::iterators::Pair<Rule>) -> String {
            match pair.as_rule() {
                Rule::sapl_id => pair.as_str().to_string(),
                rule => unreachable!(
                    "parse_import_statement_sapl_ids expected sapl_id, found {:?}",
                    rule
                ),
            }
        }

        match pair.as_rule() {
            Rule::function_import => {
                let sapl_ids: Vec<String> = pair
                    .into_inner()
                    .map(parse_import_statement_sapl_ids)
                    .collect();
                Import::Function(sapl_ids.join(".").to_string())
            }
            Rule::library_import => {
                let mut sapl_ids: Vec<String> = pair
                    .into_inner()
                    .map(parse_import_statement_sapl_ids)
                    .collect();
                let alias = sapl_ids.pop().unwrap();
                Import::Library {
                    name: sapl_ids.join("."),
                    alias,
                }
            }
            Rule::wildcard_import => {
                let sapl_ids: Vec<String> = pair
                    .into_inner()
                    .map(parse_import_statement_sapl_ids)
                    .collect();
                let mut import = sapl_ids.join(".").to_string();
                import.push_str(".*");
                Import::Wildcard(import)
            }
            rule => unreachable!(
            "Sapl::parse expected function_import, library_import or wildcard_import, found {:?}",
            rule
        ),
        }
    }
}
