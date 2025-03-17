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
