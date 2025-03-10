use crate::Rule;
use crate::PRATT_PARSER;
use std::collections::VecDeque;
use std::fmt::Display;
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

impl Clone for Expr {
    fn clone(&self) -> Self {
        match self {
            Expr::Expr { lhs, op, rhs } => Expr::Expr {
                lhs: Rc::clone(lhs),
                op: op.clone(),
                rhs: Rc::clone(rhs),
            },
            Expr::UnaryPlus(expr) => Expr::UnaryPlus(Rc::clone(expr)),
            Expr::UnaryMinus(expr) => Expr::UnaryMinus(Rc::clone(expr)),
            Expr::LogicalNot(expr) => Expr::LogicalNot(Rc::clone(expr)),
            Expr::Boolean(val) => Expr::Boolean(*val),
            Expr::Integer(val) => Expr::Integer(*val),
            Expr::Float(val) => Expr::Float(*val),
            Expr::SaplId(val) => Expr::SaplId(val.clone()),
            Expr::String(val) => Expr::String(val.clone()),
        }
    }
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

    fn eval(&self) -> Expr {
        use self::Expr::*;
        use Op::*;

        match self {
            Boolean(b) => Boolean(*b),
            Expr { lhs, op, rhs } => match (&**lhs, op, &**rhs) {
                (Boolean(lhs), Equal, Boolean(rhs)) => Boolean(lhs == rhs),
                (
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    Equal,
                    Boolean(_),
                ) => Expr {
                    lhs: lhs.eval().into(),
                    op: Equal,
                    rhs: rhs.clone(),
                }
                .eval(),
                (
                    Boolean(_),
                    Equal,
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => Expr {
                    lhs: lhs.clone(),
                    op: Equal,
                    rhs: rhs.eval().into(),
                }
                .eval(),
                (Boolean(lhs), NotEqual, Boolean(rhs)) => Boolean(lhs != rhs),
                (
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    NotEqual,
                    Boolean(_),
                ) => Expr {
                    lhs: lhs.eval().into(),
                    op: NotEqual,
                    rhs: rhs.clone(),
                }
                .eval(),
                (
                    Boolean(_),
                    NotEqual,
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => Expr {
                    lhs: lhs.clone(),
                    op: NotEqual,
                    rhs: rhs.eval().into(),
                }
                .eval(),
                (Boolean(lhs), EagerAnd, Boolean(rhs)) => Boolean(lhs & rhs),
                (
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    EagerAnd,
                    Boolean(_),
                ) => Expr {
                    lhs: lhs.eval().into(),
                    op: EagerAnd,
                    rhs: rhs.clone(),
                }
                .eval(),
                (
                    Boolean(_),
                    EagerAnd,
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => Expr {
                    lhs: lhs.clone(),
                    op: EagerAnd,
                    rhs: rhs.eval().into(),
                }
                .eval(),
                (Boolean(lhs), EagerOr, Boolean(rhs)) => Boolean(lhs | rhs),
                (
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    EagerOr,
                    Boolean(_),
                ) => Expr {
                    lhs: lhs.eval().into(),
                    op: EagerOr,
                    rhs: rhs.clone(),
                }
                .eval(),
                (
                    Boolean(_),
                    EagerOr,
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => Expr {
                    lhs: lhs.clone(),
                    op: EagerOr,
                    rhs: rhs.eval().into(),
                }
                .eval(),
                (Integer(lhs), Equal, Integer(rhs)) => Boolean(lhs == rhs),
                (Integer(lhs), NotEqual, Integer(rhs)) => Boolean(lhs != rhs),
                (Integer(lhs), Less, Integer(rhs)) => Boolean(lhs < rhs),
                (Expr { lhs: _, op, rhs: r }, Less, Integer(_)) => match op {
                    Less => Expr {
                        lhs: lhs.eval().into(),
                        op: EagerAnd,
                        rhs: Expr {
                            lhs: r.clone(),
                            op: Less,
                            rhs: rhs.clone(),
                        }
                        .eval()
                        .into(),
                    }
                    .eval(),
                    Addition => Expr {
                        lhs: lhs.eval().into(),
                        op: Less,
                        rhs: rhs.clone(),
                    }
                    .eval(),
                    _ => panic!(
                        "Expr::eval (Integer, Less, Integer) not catch: {:#?}",
                        &self
                    ),
                },
                (
                    Integer(_),
                    Less,
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => Expr {
                    lhs: lhs.clone(),
                    op: Less,
                    rhs: rhs.eval().into(),
                }
                .eval(),
                (Integer(lhs), Greater, Integer(rhs)) => Boolean(lhs > rhs),
                (
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: r,
                    },
                    Greater,
                    Integer(_),
                ) => Expr {
                    lhs: lhs.eval().into(),
                    op: EagerAnd,
                    rhs: Expr {
                        lhs: r.clone(),
                        op: Greater,
                        rhs: rhs.clone(),
                    }
                    .eval()
                    .into(),
                }
                .eval(),
                (Integer(lhs), Addition, Integer(rhs)) => Integer(lhs + rhs),
                (
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    Addition,
                    Integer(_),
                ) => Expr {
                    lhs: lhs.eval().into(),
                    op: Addition,
                    rhs: rhs.clone(),
                }
                .eval(),
                (
                    Integer(_),
                    Addition,
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => Expr {
                    lhs: lhs.clone(),
                    op: Addition,
                    rhs: rhs.eval().into(),
                }
                .eval(),
                (
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    EagerAnd,
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => Expr {
                    lhs: lhs.eval().into(),
                    op: EagerAnd,
                    rhs: rhs.eval().into(),
                }
                .eval(),
                (
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    EagerOr,
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => Expr {
                    lhs: lhs.eval().into(),
                    op: EagerOr,
                    rhs: rhs.eval().into(),
                }
                .eval(),
                (
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    Equal,
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => Expr {
                    lhs: lhs.eval().into(),
                    op: Equal,
                    rhs: rhs.eval().into(),
                }
                .eval(),
                (
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    NotEqual,
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => Expr {
                    lhs: lhs.eval().into(),
                    op: NotEqual,
                    rhs: rhs.eval().into(),
                }
                .eval(),
                (
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    Less,
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => Expr {
                    lhs: lhs.eval().into(),
                    op: Less,
                    rhs: rhs.eval().into(),
                }
                .eval(),
                (
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    Greater,
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                ) => Expr {
                    lhs: lhs.eval().into(),
                    op: Greater,
                    rhs: rhs.eval().into(),
                }
                .eval(),
                _ => panic!("expr::expr match not implemented for {:#?}", &self),
            },
            _ => panic!("expr::evaluation not possible for {:#?}", &self),
        }
    }

    pub fn evaluate(&self) -> Result<bool, String> {
        match self.eval() {
            Expr::Boolean(b) => Ok(b),
            other => Err(format!(
                "Expr::evaluation result expected Boolean, found {:#?}",
                other
            )),
        }
    }

    pub fn validate_schema_expr(&self) -> Option<Vec<ValidationErr>> {
        //TODO needs to be edit after
        //changing the grammer
        let check = |e: &Rc<Expr>| -> Option<ValidationErr> {
            if let Expr::Expr { op, .. } = &**e {
                match op {
                    Op::LazyAnd => Some(ValidationErr::LazyAnd),
                    Op::LazyOr => Some(ValidationErr::LazyOr),
                    _ => None,
                }
            } else {
                None
            }
        };

        self.validate(check)
    }

    pub fn validate_target_expr(&self) -> Option<Vec<ValidationErr>> {
        let check = |e: &Rc<Expr>| -> Option<ValidationErr> {
            if let Expr::Expr { op, .. } = &**e {
                match op {
                    Op::LazyAnd => Some(ValidationErr::LazyAnd),
                    Op::LazyOr => Some(ValidationErr::LazyOr),
                    _ => None,
                }
            } else {
                None
            }
        };

        self.validate(check)
    }

    fn validate(
        &self,
        check: fn(&Rc<Expr>) -> Option<ValidationErr>,
    ) -> Option<Vec<ValidationErr>> {
        let result: Vec<_> = self
            .iter()
            .filter_map(|expr: Rc<Expr>| check(&expr))
            .collect();

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    pub fn iter(&self) -> ExprIter {
        let mut stack = VecDeque::new();
        stack.push_back(Rc::new(self.clone()));
        ExprIter { stack }
    }
}

pub struct ExprIter {
    stack: VecDeque<Rc<Expr>>,
}

impl Iterator for ExprIter {
    type Item = Rc<Expr>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop_front() {
            match &*node {
                Expr::Expr { lhs, op: _, rhs } => {
                    self.stack.push_back(Rc::clone(rhs));
                    self.stack.push_back(Rc::clone(lhs));
                }
                Expr::UnaryPlus(expr) | Expr::UnaryMinus(expr) | Expr::LogicalNot(expr) => {
                    self.stack.push_back(Rc::clone(expr));
                }
                Expr::Boolean(_)
                | Expr::Integer(_)
                | Expr::Float(_)
                | Expr::SaplId(_)
                | Expr::String(_) => {}
            }

            Some(node)
        } else {
            None
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

impl Clone for Op {
    fn clone(&self) -> Self {
        use Op::*;
        match self {
            LazyOr => LazyOr,
            LazyAnd => LazyAnd,
            EagerOr => EagerOr,
            ExclusiveOr => ExclusiveOr,
            EagerAnd => EagerAnd,
            Equal => Equal,
            NotEqual => NotEqual,
            Unknown => Unknown,
            Comparison => Comparison,
            Less => Less,
            Greater => Greater,
            LessEqual => LessEqual,
            GreaterEqual => GreaterEqual,
            Addition => Addition,
            Subtract => Subtract,
            Multiplication => Multiplication,
            Division => Division,
            Modulo => Modulo,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ValidationErr {
    LazyAnd,
    LazyOr,
    AttributeFinderStep,
    HeadAttributeFingerStep,
    BasicEnvironmentAttribute,
    BasicEnvironmentHeadAttribute,
}

impl Display for ValidationErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ValidationErr::*;

        write!(
            f,
            "{}",
            match &self {
                LazyAnd => "Lazy and (&&) is not allowed, please use eager and (&) instead.",
                LazyOr => "Lazy or (||) is not allowed, please use eager or (|) instead.",
                AttributeFinderStep => "Attribute access () is forbidden",
                HeadAttributeFingerStep =>
                    "HeadAttributeFinderStep is not allowed in target expression.",
                BasicEnvironmentAttribute =>
                    "BasicEnvironmentAttribute is not allowed in target expression.",
                BasicEnvironmentHeadAttribute =>
                    "BasicEnvironmentHeadAttribute is not allowed in target expression.",
            }
        )
    }
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
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(!expr.unwrap());
    }

    #[test]
    fn evaluate_boolean_literal_true() {
        let pair = SaplParser::parse(Rule::target_expression, "true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_bool_eager_and() {
        let pair = SaplParser::parse(Rule::target_expression, "true & true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "true & false")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(!expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "true & true & true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_bool_eager_or() {
        let pair = SaplParser::parse(Rule::target_expression, "false | true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "false | false")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(!expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "false | false | true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_bool_equal() {
        let pair = SaplParser::parse(Rule::target_expression, "true == true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_bool_not_equal() {
        let pair = SaplParser::parse(Rule::target_expression, "false != true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_integer_comp() {
        let pair = SaplParser::parse(Rule::target_expression, "5 < 10")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "5 < 10 < 15")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "5 < 10 < 15 < 20")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_integer_comp_eager_and() {
        let pair = SaplParser::parse(Rule::target_expression, "5 < 10 & 50 > 42")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_integer_comp_eager_or() {
        let pair = SaplParser::parse(Rule::target_expression, "5 < 1 | 50 > 42")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_integer_add_comp_less() {
        let pair = SaplParser::parse(Rule::target_expression, "5 < 1 + 50")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "5 + 20 < 50")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "5 + 20 < 50 + 10")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner()).evaluate();
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn validate_target_expr_without_error() {
        let pair = SaplParser::parse(Rule::target_expression, "false | 5 < 20 & 1 > 50 | true")
            .unwrap()
            .next()
            .unwrap();
        let validation_result = Expr::parse(pair.into_inner()).validate_target_expr();
        assert!(validation_result.is_none());
    }

    #[test]
    fn validate_target_expr_lazy_and() {
        let pair = SaplParser::parse(Rule::target_expression, "5 < 20 && 1 > 50")
            .unwrap()
            .next()
            .unwrap();
        let validation_result = Expr::parse(pair.into_inner()).validate_target_expr();
        assert!(validation_result.is_some());
        let mut validation_result = validation_result.unwrap().into_iter();
        assert_eq!(Some(ValidationErr::LazyAnd), validation_result.next());
        assert_eq!(None, validation_result.next());
    }

    #[test]
    fn validate_target_expr_lazy_or() {
        let pair = SaplParser::parse(Rule::target_expression, "true || false")
            .unwrap()
            .next()
            .unwrap();
        let validation_result = Expr::parse(pair.into_inner()).validate_target_expr();
        assert!(validation_result.is_some());
        let mut validation_result = validation_result.unwrap().into_iter();
        assert_eq!(Some(ValidationErr::LazyOr), validation_result.next());
        assert_eq!(None, validation_result.next());
    }

    #[test]
    fn validate_target_expr_lazy_and_with_lazy_or() {
        let pair = SaplParser::parse(Rule::target_expression, "5 < 20 && 1 > 50 || false")
            .unwrap()
            .next()
            .unwrap();
        let validation_result = Expr::parse(pair.into_inner()).validate_target_expr();
        assert!(validation_result.is_some());
        let mut validation_result = validation_result.unwrap().into_iter();
        assert_eq!(Some(ValidationErr::LazyOr), validation_result.next());
        assert_eq!(Some(ValidationErr::LazyAnd), validation_result.next());
        assert_eq!(None, validation_result.next());
    }
}
