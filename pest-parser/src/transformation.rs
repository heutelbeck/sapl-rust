use crate::Rule;

#[derive(Debug)]
pub enum Transformation {
    SaplPair(Vec<Transformation>),
    String(String),
    SaplId(String),
    Filter(String),
}

impl Transformation {
    pub fn parse(pair: pest::iterators::Pair<Rule>) -> Transformation {
        use Transformation::*;

        match pair.as_rule() {
            Rule::sapl_id => SaplId(pair.as_str().to_string()),
            Rule::string => Self::new_string(pair.as_str()),
            Rule::pair => SaplPair(pair.into_inner().map(Transformation::parse).collect()),
            Rule::FILTER => Filter(pair.as_str().to_string()),
            Rule::filter_component => {
                let sapl_ids: Vec<Transformation> =
                    pair.into_inner().map(Transformation::parse).collect();
                let content: Vec<_> = sapl_ids
                    .iter()
                    .filter_map(|e| match e {
                        SaplId(s) => Some(s.to_owned()),
                        _ => None,
                    })
                    .collect();
                Transformation::SaplId(content.join(".").to_string())
            }
            rule => unreachable!(
                "parse_transformation expected sapl_id, FILTER or filter_component, found {:?}",
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
