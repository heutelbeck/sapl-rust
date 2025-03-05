use crate::Rule;
use crate::PRATT_PARSER;
use std::rc::Rc;

#[derive(Debug)]
pub enum Expr {
    Expr {
        lhs: Rc<Expr>,
        op: Op,
        rhs: Rc<Expr>,
    },
    UnaryPlus(Rc<Expr>),
    UnaryMinus(Rc<Expr>),
    LogicalNot(Rc<Expr>),
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
                lhs: Rc::new(lhs),
                op,
                rhs: Rc::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_str() {
            "!" => Expr::LogicalNot(Rc::new(rhs)),
            "-" => Expr::UnaryMinus(Rc::new(rhs)),
            "+" => Expr::UnaryPlus(Rc::new(rhs)),
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
        match self {
            Expr::Boolean(b) => *b,
            Expr::Expr { lhs, op, rhs } => match (&**lhs, op, &**rhs) {
                (Expr::Boolean(lhs), Op::Equal, Expr::Boolean(rhs)) => lhs == rhs,
                (
                    Expr::Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    Op::Equal,
                    Expr::Boolean(rhs),
                ) => lhs.evaluate() == *rhs,
                (
                    Expr::Boolean(lhs),
                    Op::Equal,
                    Expr::Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => *lhs == rhs.evaluate(),
                (Expr::Boolean(lhs), Op::NotEqual, Expr::Boolean(rhs)) => lhs != rhs,
                (
                    Expr::Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    Op::NotEqual,
                    Expr::Boolean(rhs),
                ) => lhs.evaluate() != *rhs,
                (
                    Expr::Boolean(lhs),
                    Op::NotEqual,
                    Expr::Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => *lhs != rhs.evaluate(),
                (Expr::Boolean(lhs), Op::EagerAnd, Expr::Boolean(rhs)) => lhs & rhs,
                (
                    Expr::Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    Op::EagerAnd,
                    Expr::Boolean(rhs),
                ) => lhs.evaluate() & *rhs,
                (
                    Expr::Boolean(lhs),
                    Op::EagerAnd,
                    Expr::Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => *lhs & rhs.evaluate(),
                (Expr::Boolean(lhs), Op::EagerOr, Expr::Boolean(rhs)) => lhs | rhs,
                (
                    Expr::Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    Op::EagerOr,
                    Expr::Boolean(rhs),
                ) => lhs.evaluate() | *rhs,
                (
                    Expr::Boolean(lhs),
                    Op::EagerOr,
                    Expr::Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => *lhs | rhs.evaluate(),
                (Expr::Integer(lhs), Op::Equal, Expr::Integer(rhs)) => lhs == rhs,
                (Expr::Integer(lhs), Op::NotEqual, Expr::Integer(rhs)) => lhs != rhs,
                (Expr::Integer(lhs), Op::Less, Expr::Integer(rhs)) => lhs < rhs,
                (
                    Expr::Expr {
                        lhs: _,
                        op: _,
                        rhs: r,
                    },
                    Op::Less,
                    Expr::Integer(i),
                ) => {
                    lhs.evaluate()
                        & Expr::Expr {
                            lhs: r.clone(),
                            op: Op::Less,
                            rhs: Rc::new(Expr::Integer(*i)),
                        }
                        .evaluate()
                }
                (Expr::Integer(lhs), Op::Greater, Expr::Integer(rhs)) => lhs > rhs,
                (
                    Expr::Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    Op::EagerAnd,
                    Expr::Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => lhs.evaluate() & rhs.evaluate(),
                (
                    Expr::Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    Op::EagerOr,
                    Expr::Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => lhs.evaluate() | rhs.evaluate(),
                _ => panic!("expr::expr match not implemented for {:#?}", &self),
            },
            _ => panic!("expr::evaluation not possible for {:#?}", &self),
        }
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
    fn expr_lazy_or() {
        let mut pair = SaplParser::parse(Rule::expression, "a || b");
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::sapl_id
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::lazy_or
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::sapl_id
        );
    }

    #[test]
    fn expr_lazy_and() {
        let mut pair = SaplParser::parse(Rule::expression, "a && b");
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::sapl_id
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::lazy_and
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::sapl_id
        );
    }

    #[test]
    fn expr_eager_or() {
        let mut pair = SaplParser::parse(Rule::expression, "a | b");
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::sapl_id
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::eager_or
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::sapl_id
        );
    }

    #[test]
    fn expr_exclusive_or() {
        let mut pair = SaplParser::parse(Rule::expression, "a ^ b");
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::sapl_id
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::exclusive_or
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::sapl_id
        );
    }

    #[test]
    fn expr_eager_and() {
        let mut pair = SaplParser::parse(Rule::expression, "a & b");
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::sapl_id
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::eager_and
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::sapl_id
        );
    }

    #[test]
    fn expr_equal() {
        let mut pair = SaplParser::parse(Rule::expression, "a == b");
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::sapl_id
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::equal
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::sapl_id
        );
    }

    #[test]
    fn expr_boolean_literal() {
        let mut pair = SaplParser::parse(Rule::expression, "false == true");
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::boolean_literal
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::equal
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::boolean_literal
        );
    }

    #[test]
    fn boolean_literal_false() {
        let pair = SaplParser::parse(Rule::boolean_literal, "false");
        assert_eq!(
            pair.unwrap().next().unwrap().as_rule(),
            Rule::boolean_literal,
        );
        let pair = SaplParser::parse(Rule::boolean_literal, "FALSE");
        assert!(pair.is_err());
        let pair = SaplParser::parse(Rule::boolean_literal, "False");
        assert!(pair.is_err());
        let pair = SaplParser::parse(Rule::boolean_literal, "LoremIpsum");
        assert!(pair.is_err());
    }

    #[test]
    fn boolean_literal_true() {
        let pair = SaplParser::parse(Rule::boolean_literal, "true");
        assert_eq!(
            pair.unwrap().next().unwrap().as_rule(),
            Rule::boolean_literal,
        );
        let pair = SaplParser::parse(Rule::boolean_literal, "TRUE");
        assert!(pair.is_err());
        let pair = SaplParser::parse(Rule::boolean_literal, "True");
        assert!(pair.is_err());
        let pair = SaplParser::parse(Rule::boolean_literal, "LoremIpsum");
        assert!(pair.is_err());
    }

    #[test]
    fn evaluate_boolean_literal_false() {
        let pair = SaplParser::parse(Rule::target_expression, "false")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner());
        assert!(!expr.evaluate());
    }
    #[test]
    fn evaluate_boolean_literal_true() {
        let pair = SaplParser::parse(Rule::target_expression, "true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner());
        assert!(expr.evaluate());
    }

    #[test]
    fn evaluate_bool_eager_and() {
        let pair = SaplParser::parse(Rule::target_expression, "true & true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner());
        assert!(expr.evaluate());
        let pair = SaplParser::parse(Rule::target_expression, "true & false")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner());
        assert!(!expr.evaluate());
        let pair = SaplParser::parse(Rule::target_expression, "true & true & true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner());
        assert!(expr.evaluate());
    }

    #[test]
    fn evaluate_bool_eager_or() {
        let pair = SaplParser::parse(Rule::target_expression, "false | true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner());
        assert!(expr.evaluate());
        let pair = SaplParser::parse(Rule::target_expression, "false | false")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner());
        assert!(!expr.evaluate());
        let pair = SaplParser::parse(Rule::target_expression, "false | false | true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner());
        assert!(expr.evaluate());
    }

    #[test]
    fn evaluate_bool_equal() {
        let pair = SaplParser::parse(Rule::target_expression, "true == true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner());
        assert!(expr.evaluate());
    }

    #[test]
    fn evaluate_bool_not_equal() {
        let pair = SaplParser::parse(Rule::target_expression, "false != true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner());
        assert!(expr.evaluate());
    }

    #[test]
    fn evaluate_integer_comp() {
        let pair = SaplParser::parse(Rule::target_expression, "5 < 10")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner());
        assert!(expr.evaluate());
        let pair = SaplParser::parse(Rule::target_expression, "5 < 10 < 15")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner());
        assert!(expr.evaluate());
        let pair = SaplParser::parse(Rule::target_expression, "5 < 10 < 15 < 20")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner());
        assert!(expr.evaluate());
    }

    #[test]
    fn evaluate_integer_comp_eager_and() {
        let pair = SaplParser::parse(Rule::target_expression, "5 < 10 & 50 > 42")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner());
        assert!(expr.evaluate());
    }

    #[test]
    fn evaluate_integer_comp_eager_or() {
        let pair = SaplParser::parse(Rule::target_expression, "5 < 1 | 50 > 42")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner());
        assert!(expr.evaluate());
    }
}
