use crate::Rule;

#[derive(Debug)]
pub enum Advice {
    SaplPair(Vec<Advice>),
    Boolean(bool),
    Integer(i32),
    Float(f32),
    SaplId(String),
    String(String),
    Id(String),
    Addition,
}

impl Advice {
    pub fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        use Advice::*;

        match pair.as_rule() {
            Rule::sapl_id => SaplId(pair.as_str().to_string()),
            Rule::integer => Integer(pair.as_str().trim().parse().unwrap()),
            Rule::boolean_literal => Boolean(pair.as_str().parse().unwrap()),
            Rule::float => Float(pair.as_str().trim().parse().unwrap()),
            Rule::string => Self::new_string(pair.as_str()),
            Rule::pair => SaplPair(pair.into_inner().map(Advice::parse).collect()),
            Rule::addition => Addition,
            rule => unreachable!(
                "parse_advice expected sapl_id, object, pair or string, found {:?}",
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
