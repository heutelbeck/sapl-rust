use crate::Rule;
use crate::PRATT_PARSER;

#[derive(Debug)]
pub enum Expr {
    Expr {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
    UnaryPlus(Box<Expr>),
    UnaryMinus(Box<Expr>),
    LogicalNot(Box<Expr>),
    Boolean(bool),
    Integer(i32),
    Float(f32),
    SaplId(String),
    String(String),
}

impl Expr {
    pub fn parse(pairs: pest::iterators::Pairs<Rule>) -> Self {
        PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::sapl_id => Expr::SaplId(primary.as_str().to_string()),
            Rule::string => Expr::new_string(primary.as_str()),
            Rule::boolean_literal => Expr::Boolean(primary.as_str().parse().unwrap()),
            Rule::integer => Expr::Integer(primary.as_str().trim().parse().unwrap()),
            Rule::float => Expr::Float(primary.as_str().trim().parse().unwrap()),
            rule => unreachable!("Expr::parse expected sapl_id, string, boolean_literal, integer or float, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::lazy_or => Op::LazyOr,
                Rule::lazy_and => Op::LazyAnd,
                Rule::eager_or => Op::EagerOr,
                Rule::exclusive_or => Op::ExclusiveOr,
                Rule::eager_and => Op::EagerAnd,
                Rule::equal => Op::Equal,
                Rule::not_equal => Op::NotEqual,
                Rule::unknown => Op::Unknown,
                Rule::comparison => Op::Comparison,
                Rule::less => Op::Less,
                Rule::greater => Op::Greater,
                Rule::less_equal => Op::LessEqual,
                Rule::greater_equal => Op::GreaterEqual,
                Rule::addition => Op::Addition,
                Rule::subtract => Op::Subtract,
                Rule::mul => Op::Multiplication,
                Rule::div => Op::Division,
                Rule::modulo => Op::Modulo,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expr::Expr {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_str() {
            "!" => Expr::LogicalNot(Box::new(rhs)),
            "-" => Expr::UnaryMinus(Box::new(rhs)),
            "+" => Expr::UnaryPlus(Box::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
    }

    fn new_string(src: &str) -> Expr {
        let mut s = src.to_string();
        s.retain(|c| c != '\"');
        Expr::String(s)
    }

    fn evaluate(&self) -> bool {
        if let Expr::Expr { lhs, op, rhs } = self {
            return match (&**lhs, op, &**rhs) {
                (Expr::Integer(lhs), Op::Less, Expr::Integer(rhs)) => lhs < rhs,
                _ => unreachable!(),
            };
        }
        panic!("expr::evaluation not possible");
    }
}

#[derive(Debug)]
pub enum Op {
    LazyOr,
    LazyAnd,
    EagerOr,
    ExclusiveOr,
    EagerAnd,
    Equal,
    NotEqual,
    Unknown,
    Comparison,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Addition,
    Subtract,
    Multiplication,
    Division,
    Modulo,
}

#[cfg(test)]
mod tests {
    use crate::SaplParser;
    use pest::Parser;

    use super::*;

    #[test]
    fn evaluate_integer_comp() {
        let pair = SaplParser::parse(Rule::target_expression, "5 < 10")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner());
        println!("{:#?}", expr);
        assert!(expr.evaluate());
    }

    // #[test]
    // fn evaluate_integer_comp_with_addition() {
    //     let pair = SaplParser::parse(Rule::target_expression, "5 < 10 + 5")
    //         .unwrap()
    //         .next()
    //         .unwrap();
    //     let expr = Expr::parse(pair.into_inner());
    //     println!("{:#?}", expr);
    //     assert!(expr.evaluate());
    // }
}
