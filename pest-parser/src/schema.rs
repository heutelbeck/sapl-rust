use crate::Rule;

#[derive(Debug)]
pub enum Schema {
    Entry(Vec<Schema>),
    SaplPair(Vec<Schema>),
    Boolean(bool),
    Integer(i32),
    Float(f32),
    SaplId(String),
    String(String),
    Id(String),
}

impl Schema {
    pub fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        use Schema::*;

        match pair.as_rule() {
            Rule::schema_entry => Entry(pair.into_inner().map(Schema::parse).collect()),
            Rule::sapl_id => SaplId(pair.as_str().to_string()),
            Rule::integer => Integer(pair.as_str().trim().parse().unwrap()),
            Rule::boolean_literal => Boolean(pair.as_str().parse().unwrap()),
            Rule::float => Float(pair.as_str().trim().parse().unwrap()),
            Rule::string => Self::new_string(pair.as_str()),
            Rule::pair => SaplPair(pair.into_inner().map(Schema::parse).collect()),
            rule => unreachable!(
                "parse_schema expected sapl_id, object, pair or string, found {:?}",
                rule
            ),
        }
    }

    fn new_string(src: &str) -> Self {
        let mut s = src.to_string();
        s.retain(|c| c != '\"');
        Self::String(s)
    }
}
