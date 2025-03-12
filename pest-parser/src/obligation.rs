use crate::Rule;

#[derive(Debug)]
pub enum Obligation {
    SaplPairs(Vec<Obligation>),
    SaplPair(Vec<Obligation>),
    Boolean(bool),
    Integer(i32),
    Float(f32),
    SaplId(String),
    String(String),
    Id(String),
    Addition,
}

impl Obligation {
    pub fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        use Obligation::*;

        match pair.as_rule() {
            Rule::sapl_id => SaplId(pair.as_str().to_string()),
            Rule::integer => Integer(pair.as_str().trim().parse().unwrap()),
            Rule::boolean_literal => Boolean(pair.as_str().parse().unwrap()),
            Rule::float => Float(pair.as_str().trim().parse().unwrap()),
            Rule::string => Self::new_string(pair.as_str()),
            Rule::addition => Addition,
            Rule::pairs => SaplPairs(pair.into_inner().map(Obligation::parse).collect()),
            Rule::pair => SaplPair(pair.into_inner().map(Obligation::parse).collect()),
            rule => unreachable!(
                "parse_obligation expected sapl_id, pairs, pair, integer, addition, float, boolean_literal or string, found {:?}",
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
