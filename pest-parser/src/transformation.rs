use crate::basic_identifier_expression::BasicIdentifierExpression;
use crate::Rule;

#[derive(Debug)]
pub enum Transformation {
    SaplPairs(Vec<Transformation>),
    SaplPair(Vec<Transformation>),
    BasicIdent(Vec<Transformation>),
    BasicIdentExpr(BasicIdentifierExpression),
    String(String),
    KeyStep(String),
    Filter(String),
    FilterComponent(String),
}

impl Transformation {
    pub fn parse(pair: pest::iterators::Pair<Rule>) -> Transformation {
        use Transformation::*;

        match pair.as_rule() {
            Rule::string => Self::new_string(pair.as_str()),
            Rule::key_step => KeyStep(pair.as_str().to_string()),
            Rule::pairs => SaplPairs(pair.into_inner().map(Transformation::parse).collect()),
            Rule::pair => SaplPair(pair.into_inner().map(Transformation::parse).collect()),
            Rule::basic_identifier => BasicIdent(pair.into_inner().map(Transformation::parse).collect()),
            Rule::basic_identifier_expression => BasicIdentExpr(BasicIdentifierExpression::new(pair.as_str())),
            Rule::FILTER => Filter(pair.as_str().to_string()),
            Rule::filter_component => Transformation::FilterComponent(pair.as_str().to_string()),
            rule => unreachable!(
                "parse_transformation expected pairs, pair, string, key_step, id, basic_identifier, basic_identifier_expression, FILTER or filter_component, found {:?}",
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
