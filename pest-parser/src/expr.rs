/*
    Copyright 2025 Stefan Weng

    Licensed under the Apache License, Version 2.0 (the "License"); you may not
    use this file except in compliance with the License. You may obtain a copy
    of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
    WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
    License for the specific language governing permissions and limitations
    under the License.
*/

use crate::authorization_subscription::AuthorizationSubscription;
use crate::basic_identifier_expression::BasicIdentifierExpression;
use crate::Rule;
use crate::PRATT_PARSER;
use serde_json::Value;
use std::collections::VecDeque;
use std::fmt::Display;
use std::sync::Arc;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Expr {
        lhs: Arc<Expr>,
        op: Op,
        rhs: Arc<Expr>,
    },
    SaplPairs(Arc<Vec<Expr>>),
    SaplPair {
        lhs: Arc<Expr>,
        rhs: Arc<Vec<Expr>>,
    },
    BasicEnvironmentAttribute(Arc<Expr>),
    BasicEnvironmentHeadAttribute(Arc<Expr>),
    BasicIdentifier(Arc<Vec<Expr>>),
    BasicFunction(Arc<Vec<Expr>>),
    BasicValue(Arc<Vec<Expr>>),
    BasicGroup(Arc<Expr>),
    FilterComponent(Arc<Vec<Expr>>),
    FilterStatement(Arc<Vec<Expr>>),
    BasicIdentifierExpression(Arc<BasicIdentifierExpression>),
    FunctionIdentifier(Arc<Vec<Expr>>),
    Array(Arc<Vec<Expr>>),
    UnaryPlus(Arc<Expr>),
    UnaryMinus(Arc<Expr>),
    LogicalNot(Arc<Expr>),
    Arguments(Arc<Expr>),
    Subscript(Arc<Expr>),
    Boolean(bool),
    Integer(i32),
    Float(f32),
    SignedNumber(String),
    String(String),
    Id(String),
    KeyStep(String),
    RecursiveKeyStep(String),
    RecursiveIndexStep(String),
    RecursiveWildcardStep,
    AttributeFinderStep(String),
    HeadAttributeFinderStep(String),
    Concat,
    Div,
    Null,
    Undefined,
}

impl Clone for Expr {
    fn clone(&self) -> Self {
        match self {
            Expr::Expr { lhs, op, rhs } => Expr::Expr {
                lhs: Arc::clone(lhs),
                op: op.clone(),
                rhs: Arc::clone(rhs),
            },
            Expr::SaplPairs(expr) => Expr::SaplPairs(Arc::clone(expr)),
            Expr::SaplPair { lhs, rhs } => Expr::SaplPair {
                lhs: Arc::clone(lhs),
                rhs: Arc::clone(rhs),
            },
            Expr::BasicEnvironmentAttribute(expr) => {
                Expr::BasicEnvironmentAttribute(Arc::clone(expr))
            }
            Expr::BasicEnvironmentHeadAttribute(expr) => {
                Expr::BasicEnvironmentHeadAttribute(Arc::clone(expr))
            }
            Expr::BasicIdentifier(expr) => Expr::BasicIdentifier(Arc::clone(expr)),
            Expr::BasicFunction(expr) => Expr::BasicFunction(Arc::clone(expr)),
            Expr::BasicGroup(expr) => Expr::BasicGroup(Arc::clone(expr)),
            Expr::BasicValue(expr) => Expr::BasicValue(Arc::clone(expr)),
            Expr::BasicIdentifierExpression(expr) => {
                Expr::BasicIdentifierExpression(Arc::clone(expr))
            }
            Expr::FunctionIdentifier(expr) => Expr::FunctionIdentifier(Arc::clone(expr)),
            Expr::FilterComponent(expr) => Expr::FilterComponent(Arc::clone(expr)),
            Expr::FilterStatement(expr) => Expr::FilterStatement(Arc::clone(expr)),
            Expr::Array(expr) => Expr::Array(Arc::clone(expr)),
            Expr::UnaryPlus(expr) => Expr::UnaryPlus(Arc::clone(expr)),
            Expr::UnaryMinus(expr) => Expr::UnaryMinus(Arc::clone(expr)),
            Expr::LogicalNot(expr) => Expr::LogicalNot(Arc::clone(expr)),
            Expr::Arguments(expr) => Expr::Arguments(Arc::clone(expr)),
            Expr::Subscript(expr) => Expr::Subscript(Arc::clone(expr)),
            Expr::Boolean(val) => Expr::Boolean(*val),
            Expr::Integer(val) => Expr::Integer(*val),
            Expr::Float(val) => Expr::Float(*val),
            Expr::SignedNumber(val) => Expr::SignedNumber(val.clone()),
            Expr::String(val) => Expr::String(val.clone()),
            Expr::Id(val) => Expr::Id(val.clone()),
            Expr::KeyStep(val) => Expr::KeyStep(val.clone()),
            Expr::RecursiveKeyStep(val) => Expr::RecursiveKeyStep(val.clone()),
            Expr::RecursiveIndexStep(val) => Expr::RecursiveIndexStep(val.clone()),
            Expr::RecursiveWildcardStep => Expr::RecursiveWildcardStep,
            Expr::AttributeFinderStep(val) => Expr::AttributeFinderStep(val.clone()),
            Expr::HeadAttributeFinderStep(val) => Expr::HeadAttributeFinderStep(val.clone()),
            Expr::Concat => Expr::Concat,
            Expr::Div => Expr::Div,
            Expr::Null => Expr::Null,
            Expr::Undefined => Expr::Undefined,
        }
    }
}

impl Expr {
    pub fn parse(pairs: pest::iterators::Pairs<Rule>) -> Self {
        fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Expr {
            match pair.as_rule() {
                Rule::pairs => {
                    Expr::SaplPairs(Arc::new(pair.into_inner().map(parse_pair).collect()))
                }
                Rule::pair => {
                    let mut inner_rules = pair.into_inner();
                    let lhs = inner_rules.next().unwrap();
                    Expr::SaplPair {
                        lhs: Arc::new(Expr::new_string(lhs.as_str())),
                        rhs: Arc::new(inner_rules.map(parse_pair).collect()),
                    }
                }
                Rule::basic_group => Expr::parse(pair.into_inner()),
                Rule::basic_value => Expr::BasicValue(Arc::new(pair.into_inner().map(parse_pair).collect())),
                Rule::basic_identifier => Expr::BasicIdentifier(Arc::new(pair.into_inner().map(parse_basics).collect())),
                Rule::array => Expr::Array(Arc::new(pair.into_inner().map(parse_pair).collect())),
                Rule::string => Expr::new_string(pair.as_str()),
                Rule::boolean_literal => Expr::Boolean(pair.as_str().parse().unwrap()),
                Rule::integer => Expr::Integer(pair.as_str().trim().parse().unwrap()),
                Rule::float => Expr::Float(pair.as_str().trim().parse().unwrap()),
                Rule::div => Expr::Div,
                Rule::addition => Expr::Concat,
                rule => unreachable!("Expr::parse_pair expected pairs, pair, string, boolean_literal, integer or float, found {:?}", rule),
            }
        }
        fn parse_basics(pair: pest::iterators::Pair<Rule>) -> Expr {
            match pair.as_rule() {
                Rule::basic_identifier_expression => Expr::BasicIdentifierExpression(Arc::new(BasicIdentifierExpression::new(pair.as_str()))),
                Rule::function_identifier => Expr::FunctionIdentifier(Arc::new(pair.into_inner().map(parse_basics).collect())),
                Rule::arguments => Expr::Arguments(Arc::new(parse_basics(pair.into_inner().next().unwrap()))),
                Rule::subscript => Expr::Subscript(Arc::new(parse_basics(pair.into_inner().next().unwrap()))),
                Rule::basic_environment_attribute => Expr::BasicEnvironmentAttribute(Arc::new(parse_basics(pair.into_inner().next().unwrap()))),
                Rule::basic_environment_head_attribute => Expr::BasicEnvironmentHeadAttribute(Arc::new(parse_basics(pair.into_inner().next().unwrap()))),
                Rule::basic_identifier => Expr::BasicIdentifier(Arc::new(pair.into_inner().map(parse_basics).collect())),
                Rule::filter_statement => Expr::FilterStatement(Arc::new(pair.into_inner().map(parse_basics).collect())),
                Rule::array => Expr::Array(Arc::new(pair.into_inner().map(parse_basics).collect())),
                Rule::signed_number => Expr::SignedNumber(pair.as_str().to_string()),
                Rule::key_step => Expr::KeyStep(pair.as_str().to_string()),
                Rule::recursive_key_step => Expr::RecursiveKeyStep(pair.as_str().to_string()),
                Rule::recursive_index_step => Expr::RecursiveIndexStep(pair.as_str().to_string()),
                Rule::recursive_wildcard_step => Expr::RecursiveWildcardStep,
                Rule::attribute_finder_step => Expr::AttributeFinderStep(pair.as_str().to_string()),
                Rule::head_attribute_finder_step => Expr::HeadAttributeFinderStep(pair.as_str().to_string()),
                Rule::pairs => Expr::SaplPairs(Arc::new(pair.into_inner().map(parse_pair).collect())),
                Rule::string => Expr::new_string(pair.as_str()),
                Rule::integer => Expr::Integer(pair.as_str().trim().parse().unwrap()),
                Rule::id => Expr::Id(pair.as_str().to_string()),
                Rule::div => Expr::Div,
                Rule::null_literal => Expr::Null,
                Rule::undefined_literal => Expr::Undefined,
                rule => unreachable!("Expr::parse_basic_identifier expected basic_identifier_expression, key_step, recursive_key_step, or attribute_finder_step, found {:?}", rule),
            }
        }
        PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::pairs => Expr::SaplPairs(Arc::new(primary.into_inner().map(parse_pair).collect())),
            Rule::basic_identifier => Expr::BasicIdentifier(Arc::new(primary.into_inner().map(parse_basics).collect())),
            Rule::basic_function => Expr::BasicFunction(Arc::new(primary.into_inner().map(parse_basics).collect())),
            Rule::basic_group => Expr::BasicGroup(Arc::new(Expr::parse(primary.into_inner()))),
            Rule::basic_value => Expr::BasicValue(Arc::new(primary.into_inner().map(parse_basics).collect())),
            Rule::basic_environment_attribute => Expr::BasicEnvironmentAttribute(Arc::new(parse_basics(primary.into_inner().next().unwrap()))),
            Rule::basic_environment_head_attribute => Expr::BasicEnvironmentHeadAttribute(Arc::new(parse_basics(primary.into_inner().next().unwrap()))),
            Rule::filter_component => Expr::FilterComponent(Arc::new(primary.into_inner().map(parse_basics).collect())),
            Rule::array => Expr::Array(Arc::new(primary.into_inner().map(parse_basics).collect())),
            Rule::string => Expr::new_string(primary.as_str()),
            Rule::boolean_literal => Expr::Boolean(primary.as_str().parse().unwrap()),
            Rule::integer => Expr::Integer(primary.as_str().parse().unwrap()),
            Rule::float => Expr::Float(primary.as_str().parse().unwrap()),
            rule => unreachable!("Expr::parse expected pairs, string, boolean_literal, integer or float, found {:?}", rule),
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
                Rule::regex => Op::Regex,
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
                Rule::FILTER => Op::Filter,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expr::Expr {
                lhs: Arc::new(lhs),
                op,
                rhs: Arc::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_str() {
            "!" => Expr::LogicalNot(Arc::new(rhs)),
            "-" => Expr::UnaryMinus(Arc::new(rhs)),
            "+" => Expr::UnaryPlus(Arc::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
    }

    fn new_string(src: &str) -> Expr {
        let mut s = src.to_string();
        s.retain(|c| c != '\"');
        Expr::String(s)
    }

    pub fn evaluate(&self, auth_subscription: &AuthorizationSubscription) -> Result<bool, String> {
        match self.eval_root(auth_subscription) {
            Expr::Boolean(b) => Ok(b),
            other => Err(format!(
                "Expr::evaluation result expected Boolean, found {:#?}",
                other
            )),
        }
    }

    pub fn validate_schema_expr(&self) -> Option<Vec<ValidationErr>> {
        let check = |e: &Arc<Expr>| -> Option<ValidationErr> {
            match &**e {
                Expr::AttributeFinderStep(_) => Some(ValidationErr::AttributeFinderStep),
                Expr::HeadAttributeFinderStep(_) => Some(ValidationErr::HeadAttributeFinderStep),
                Expr::BasicEnvironmentAttribute(_) => {
                    Some(ValidationErr::BasicEnvironmentAttribute)
                }
                Expr::BasicEnvironmentHeadAttribute(_) => {
                    Some(ValidationErr::BasicEnvironmentHeadAttribute)
                }
                _ => None,
            }
        };

        self.validate(check)
    }

    pub fn validate_target_expr(&self) -> Option<Vec<ValidationErr>> {
        let check = |e: &Arc<Expr>| -> Option<ValidationErr> {
            match &**e {
                Expr::Expr { op, .. } => match op {
                    Op::LazyAnd => Some(ValidationErr::LazyAnd),
                    Op::LazyOr => Some(ValidationErr::LazyOr),
                    _ => None,
                },
                Expr::AttributeFinderStep(_) => Some(ValidationErr::AttributeFinderStep),
                Expr::HeadAttributeFinderStep(_) => Some(ValidationErr::HeadAttributeFinderStep),
                Expr::BasicEnvironmentAttribute(_) => {
                    Some(ValidationErr::BasicEnvironmentAttribute)
                }
                Expr::BasicEnvironmentHeadAttribute(_) => {
                    Some(ValidationErr::BasicEnvironmentHeadAttribute)
                }
                _ => None,
            }
        };

        self.validate(check)
    }

    fn validate(
        &self,
        check: fn(&Arc<Expr>) -> Option<ValidationErr>,
    ) -> Option<Vec<ValidationErr>> {
        let result: Vec<_> = self
            .iter()
            .filter_map(|expr: Arc<Expr>| check(&expr))
            .collect();

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    pub fn simplify(&mut self) {
        use self::Expr::*;
        use Op::*;

        if let Expr { lhs, op, rhs } = self {
            match (&**lhs, op, &**rhs) {
                (Boolean(l), EagerAnd, Boolean(r)) => *self = Boolean(l & r),
                (
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    EagerAnd,
                    Boolean(_),
                ) => {
                    let mut result = Expr {
                        lhs: lhs
                            .eval_expr(&AuthorizationSubscription::new_example_subscription1())
                            .into(),
                        op: EagerAnd,
                        rhs: rhs.clone(),
                    };
                    result.simplify();
                    *self = result;
                }
                (Integer(l), Addition, Integer(r)) => *self = Integer(l + r),
                (
                    Expr {
                        lhs: _,
                        op: _,
                        rhs: _,
                    },
                    Addition,
                    Integer(_),
                ) => {
                    let mut result = Expr {
                        lhs: lhs
                            .eval_expr(&AuthorizationSubscription::new_example_subscription1())
                            .into(),
                        op: Addition,
                        rhs: rhs.clone(),
                    };
                    result.simplify();
                    *self = result;
                }
                others => unimplemented!("Expr::simplify needs to implement {:#?}", others),
            }
        }
    }

    fn eval_root(&self, auth_subscription: &AuthorizationSubscription) -> Expr {
        use self::Expr::*;

        match self {
            Boolean(b) => Boolean(*b),
            Expr { .. } => self.eval_expr(auth_subscription),
            others => unimplemented!("Expr::eval_root {:#?} is not implemented", others),
        }
    }

    fn eval_expr(&self, auth_subscription: &AuthorizationSubscription) -> Expr {
        use self::Expr::*;

        if let Expr { lhs, op, rhs } = self {
            match (&**lhs, op, &**rhs) {
                (Boolean(_), _, Boolean(_)) => self.eval_boolean_expr(),
                (Integer(_), _, Integer(_)) => self.eval_integer_expr(),
                (BasicIdentifier(_), _, _) => self.eval_basic_identifier(auth_subscription),
                (Expr { .. }, _, Integer(_)) => self.eval_expr_comp_integer(auth_subscription),
                (Expr { .. }, _, _) => Expr {
                    lhs: lhs.eval_expr(auth_subscription).into(),
                    op: op.clone(),
                    rhs: rhs.clone(),
                }
                .eval_expr(auth_subscription),
                (_, _, Expr { .. }) => Expr {
                    lhs: lhs.clone(),
                    op: op.clone(),
                    rhs: rhs.eval_expr(auth_subscription).into(),
                }
                .eval_expr(auth_subscription),
                others => unimplemented!("Expr::eval_expr {:#?} is not implemented", others),
            }
        } else {
            unimplemented!("Expr::eval_expr {:#?} is not implemented", self);
        }
    }

    fn eval_boolean_expr(&self) -> Expr {
        use self::Expr::*;
        use Op::*;

        if let Expr { lhs, op, rhs } = self {
            match (&**lhs, op, &**rhs) {
                (Boolean(lhs), LazyOr, Boolean(rhs)) => Boolean(*lhs || *rhs),
                (Boolean(lhs), LazyAnd, Boolean(rhs)) => Boolean(*lhs && *rhs),
                (Boolean(lhs), EagerOr, Boolean(rhs)) => Boolean(lhs | rhs),
                (Boolean(lhs), EagerAnd, Boolean(rhs)) => Boolean(lhs & rhs),
                (Boolean(lhs), Equal, Boolean(rhs)) => Boolean(lhs == rhs),
                (Boolean(lhs), NotEqual, Boolean(rhs)) => Boolean(lhs != rhs),
                (Boolean(lhs), Less, Boolean(rhs)) => Boolean(lhs > rhs),
                (Boolean(lhs), Greater, Boolean(rhs)) => Boolean(lhs < rhs),
                (Boolean(lhs), LessEqual, Boolean(rhs)) => Boolean(lhs <= rhs),
                (Boolean(lhs), GreaterEqual, Boolean(rhs)) => Boolean(lhs >= rhs),
                others => {
                    unimplemented!("Expr::eval_boolean_expr {:#?} is not implemented", others)
                }
            }
        } else {
            unimplemented!("Expr::eval_boolean_expr {:#?} is not implemented", self);
        }
    }

    fn eval_expr_comp_integer(&self, auth_subscription: &AuthorizationSubscription) -> Expr {
        use self::Expr::*;
        use Op::*;

        if let Expr {
            lhs,
            op: outer_op,
            rhs,
        } = self
        {
            match (&**lhs, outer_op, &**rhs) {
                (
                    Expr { op, rhs: r, .. },
                    Greater | Less | Equal | LessEqual | GreaterEqual,
                    Integer(_),
                ) => match op {
                    Greater | Less | Equal | LessEqual | GreaterEqual => Expr {
                        lhs: lhs.eval_expr(auth_subscription).into(),
                        op: EagerAnd,
                        rhs: Expr {
                            lhs: r.clone(),
                            op: op.clone(),
                            rhs: rhs.clone(),
                        }
                        .eval_expr(auth_subscription)
                        .into(),
                    }
                    .eval_expr(auth_subscription),
                    _ => Expr {
                        lhs: lhs.eval_expr(auth_subscription).into(),
                        op: outer_op.clone(),
                        rhs: rhs.clone(),
                    }
                    .eval_expr(auth_subscription),
                },
                _ => Expr {
                    lhs: lhs.eval_expr(auth_subscription).into(),
                    op: outer_op.clone(),
                    rhs: rhs.clone(),
                }
                .eval_expr(auth_subscription),
            }
        } else {
            unimplemented!(
                "Expr::eval_expr_comp_integer {:#?} is not implemented",
                self
            );
        }
    }

    fn eval_integer_expr(&self) -> Expr {
        use self::Expr::*;
        use Op::*;

        if let Expr { lhs, op, rhs } = self {
            match (&**lhs, op, &**rhs) {
                (Integer(lhs), Equal, Integer(rhs)) => Boolean(lhs == rhs),
                (Integer(lhs), NotEqual, Integer(rhs)) => Boolean(lhs != rhs),
                (Integer(lhs), Less, Integer(rhs)) => Boolean(lhs < rhs),
                (Integer(lhs), Greater, Integer(rhs)) => Boolean(lhs > rhs),
                (Integer(lhs), LessEqual, Integer(rhs)) => Boolean(lhs <= rhs),
                (Integer(lhs), GreaterEqual, Integer(rhs)) => Boolean(lhs >= rhs),
                (Integer(lhs), Addition, Integer(rhs)) => Integer(lhs.saturating_add(*rhs)),
                (Integer(lhs), Subtract, Integer(rhs)) => Integer(lhs.saturating_sub(*rhs)),
                (Integer(lhs), Multiplication, Integer(rhs)) => Integer(lhs.saturating_mul(*rhs)),
                (Integer(lhs), Division, Integer(rhs)) => Integer(lhs.saturating_div(*rhs)),
                (Integer(lhs), Modulo, Integer(rhs)) => Integer(lhs % rhs),
                others => {
                    unimplemented!("Expr::eval_integer_expr {:#?} is not implemented", others)
                }
            }
        } else {
            unimplemented!("Expr::eval_integer_expr {:#?} is not implemented", self);
        }
    }

    fn eval_basic_identifier(&self, auth_subscription: &AuthorizationSubscription) -> Expr {
        use self::Expr::*;
        use Op::*;

        if let Expr { lhs, op, rhs } = self {
            if let BasicIdentifier(bi) = &**lhs {
                let sapl_id = bi.first();

                let mut keys: VecDeque<_> = bi
                    .iter()
                    .skip(1)
                    .filter_map(|e| match e {
                        KeyStep(s) => Some(s.to_owned()),
                        _ => None,
                    })
                    .collect();

                let lhs_result: Value = match sapl_id {
                    Some(BasicIdentifierExpression(bie)) => {
                        bie.evaluate(&mut keys, auth_subscription)
                    }
                    _ => Value::Null,
                };

                return match (lhs_result, op, &**rhs) {
                    (Value::String(l), Equal, String(s)) => Boolean(l.eq(s)),
                    (Value::Number(l), _, Integer(r)) => {
                        let num = l.as_i64();
                        match num {
                            Some(n) => match op {
                                Equal => Boolean(n == *r as i64),
                                NotEqual => Boolean(n != *r as i64),
                                Less => Boolean(n < *r as i64),
                                Greater => Boolean(n > *r as i64),
                                LessEqual => Boolean(n <= *r as i64),
                                GreaterEqual => Boolean(n >= *r as i64),
                                _ => Boolean(false),
                            },
                            None => Boolean(false),
                        }
                    }
                    (Value::Bool(l), Equal, Boolean(r)) => Boolean(l == *r),
                    (Value::Bool(l), NotEqual, Boolean(r)) => Boolean(l != *r),
                    (Value::Null, _, _) => Boolean(false),
                    (Value::Object(_), _, _) => Boolean(false),
                    others => panic!(
                        "Expr::eval_basic_identifier {:#?} is not implemented",
                        others
                    ),
                };
            }
        }
        panic!("Expr::eval_basic_identifier expected, found {:#?}", self);
    }

    pub fn iter(&self) -> ExprIter {
        let mut stack = VecDeque::new();
        stack.push_back(Arc::new(self.clone()));
        ExprIter { stack }
    }
}

pub struct ExprIter {
    stack: VecDeque<Arc<Expr>>,
}

impl Iterator for ExprIter {
    type Item = Arc<Expr>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop_front() {
            match &*node {
                Expr::Expr { lhs, op: _, rhs } => {
                    self.stack.push_back(Arc::clone(rhs));
                    self.stack.push_back(Arc::clone(lhs));
                }
                Expr::SaplPair { lhs, rhs } => {
                    self.stack.push_back(Arc::clone(lhs));
                    for elem in rhs.iter() {
                        self.stack.push_back(Arc::new(elem.clone()));
                    }
                }
                Expr::SaplPairs(expr)
                | Expr::BasicIdentifier(expr)
                | Expr::BasicFunction(expr)
                | Expr::BasicValue(expr)
                | Expr::FunctionIdentifier(expr)
                | Expr::FilterComponent(expr)
                | Expr::FilterStatement(expr)
                | Expr::Array(expr) => {
                    for elem in expr.iter() {
                        self.stack.push_back(Arc::new(elem.clone()));
                    }
                }
                Expr::UnaryPlus(expr)
                | Expr::UnaryMinus(expr)
                | Expr::LogicalNot(expr)
                | Expr::Arguments(expr)
                | Expr::Subscript(expr)
                | Expr::BasicGroup(expr)
                | Expr::BasicEnvironmentAttribute(expr)
                | Expr::BasicEnvironmentHeadAttribute(expr) => {
                    self.stack.push_back(Arc::clone(expr));
                }
                Expr::KeyStep(_)
                | Expr::RecursiveKeyStep(_)
                | Expr::RecursiveIndexStep(_)
                | Expr::RecursiveWildcardStep
                | Expr::AttributeFinderStep(_)
                | Expr::HeadAttributeFinderStep(_)
                | Expr::BasicIdentifierExpression(_)
                | Expr::Boolean(_)
                | Expr::Integer(_)
                | Expr::Float(_)
                | Expr::SignedNumber(_)
                | Expr::String(_)
                | Expr::Id(_)
                | Expr::Concat
                | Expr::Div
                | Expr::Null
                | Expr::Undefined => {}
            }

            Some(node)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Op {
    LazyOr,
    LazyAnd,
    EagerOr,
    ExclusiveOr,
    EagerAnd,
    Equal,
    NotEqual,
    Regex,
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
    Filter,
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
            Regex => Regex,
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
            Filter => Filter,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ValidationErr {
    LazyAnd,
    LazyOr,
    AttributeFinderStep,
    HeadAttributeFinderStep,
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
                AttributeFinderStep => "Attribute access (< >) is forbidden",
                HeadAttributeFinderStep => "Attribute access (|< >) is forbidden.",
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
            Rule::basic_identifier,
            "wrong grammer rule"
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::lazy_or
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::basic_identifier
        );
    }

    #[test]
    fn expr_lazy_and() {
        let mut pair = SaplParser::parse(Rule::expression, "a && b");
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::basic_identifier
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::lazy_and
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::basic_identifier
        );
    }

    #[test]
    fn expr_eager_or() {
        let mut pair = SaplParser::parse(Rule::expression, "a | b");
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::basic_identifier
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::eager_or
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::basic_identifier
        );
    }

    #[test]
    fn expr_exclusive_or() {
        let mut pair = SaplParser::parse(Rule::expression, "a ^ b");
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::basic_identifier
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::exclusive_or
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::basic_identifier
        );
    }

    #[test]
    fn expr_eager_and() {
        let mut pair = SaplParser::parse(Rule::expression, "a & b");
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::basic_identifier
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::eager_and
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::basic_identifier
        );
    }

    #[test]
    fn expr_equal() {
        let mut pair = SaplParser::parse(Rule::expression, "a == b");
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::basic_identifier
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::equal
        );
        assert_eq!(
            pair.as_mut().unwrap().next().unwrap().as_rule(),
            Rule::basic_identifier
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
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(!expr.unwrap());
    }

    #[test]
    fn evaluate_boolean_literal_true() {
        let pair = SaplParser::parse(Rule::target_expression, "true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_bool_eager_and() {
        let pair = SaplParser::parse(Rule::target_expression, "true & true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "true & false")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(!expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "true & true & true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_bool_eager_or() {
        let pair = SaplParser::parse(Rule::target_expression, "false | true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "false | false")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(!expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "false | false | true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_bool_equal() {
        let pair = SaplParser::parse(Rule::target_expression, "true == true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_bool_not_equal() {
        let pair = SaplParser::parse(Rule::target_expression, "false != true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_integer_comp() {
        let pair = SaplParser::parse(Rule::target_expression, "5 < 10")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "5 < 10 < 15")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "5 < 10 < 15 < 20")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_integer_comp_eager_and() {
        let pair = SaplParser::parse(Rule::target_expression, "5 < 10 & 50 > 42")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_integer_comp_eager_or() {
        let pair = SaplParser::parse(Rule::target_expression, "5 < 1 | 50 > 42")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_integer_add_comp_less() {
        let pair = SaplParser::parse(Rule::target_expression, "5 < 1 + 50")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        println!("Das Ergebnis lautet: {:#?}", expr);
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "5 + 20 < 50")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "5 + 20 < 50 + 10")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_target_expr_basic_identifier_expression_action() {
        let pair = SaplParser::parse(Rule::target_expression, "action == \"read\"")
            .unwrap()
            .next()
            .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn evaluate_target_expr_basic_identifier_expression_action2() {
        let pair = SaplParser::parse(
            Rule::target_expression,
            "action.nutrients.calories == \"low\"",
        )
        .unwrap()
        .next()
        .unwrap();
        let expr = Expr::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription2());
        assert!(expr.is_ok());
        println!("action2 {:#?}", expr);
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

    #[test]
    fn validate_target_expr_attribute_finder_step() {
        let pair = SaplParser::parse(
            Rule::target_expression,
            "subject.name == resource.id.<patient.patientRecord>.attendingDoctor;",
        )
        .unwrap()
        .next()
        .unwrap();
        let validation_result = Expr::parse(pair.into_inner()).validate_target_expr();
        assert!(validation_result.is_some());
        let mut validation_result = validation_result.unwrap().into_iter();
        assert_eq!(
            Some(ValidationErr::AttributeFinderStep),
            validation_result.next()
        );
        assert_eq!(None, validation_result.next());
    }

    #[test]
    fn validate_target_expr_basic_environment_attribute() {
        let pair = SaplParser::parse(Rule::target_expression, "<subject.name>")
            .unwrap()
            .next()
            .unwrap();
        let validation_result = Expr::parse(pair.into_inner()).validate_target_expr();
        assert!(validation_result.is_some());
        let mut validation_result = validation_result.unwrap().into_iter();
        assert_eq!(
            Some(ValidationErr::BasicEnvironmentAttribute),
            validation_result.next()
        );
        assert_eq!(None, validation_result.next());
    }

    #[test]
    fn validate_target_expr_basic_environment_head_attribute() {
        let pair = SaplParser::parse(Rule::target_expression, "|<subject.name>")
            .unwrap()
            .next()
            .unwrap();
        let validation_result = Expr::parse(pair.into_inner()).validate_target_expr();
        assert!(validation_result.is_some());
        let mut validation_result = validation_result.unwrap().into_iter();
        assert_eq!(
            Some(ValidationErr::BasicEnvironmentHeadAttribute),
            validation_result.next()
        );
        assert_eq!(None, validation_result.next());
    }

    #[test]
    fn simplify_boolean_expr_true() {
        let pair = SaplParser::parse(Rule::target_expression, "true")
            .unwrap()
            .next()
            .unwrap();
        let mut expr = Expr::parse(pair.into_inner());
        expr.simplify();
        assert_eq!(Expr::Boolean(true), expr);

        let pair = SaplParser::parse(Rule::target_expression, "true & true")
            .unwrap()
            .next()
            .unwrap();
        let mut expr = Expr::parse(pair.into_inner());
        expr.simplify();
        assert_eq!(Expr::Boolean(true), expr);

        let pair = SaplParser::parse(Rule::target_expression, "true & true & true")
            .unwrap()
            .next()
            .unwrap();
        let mut expr = Expr::parse(pair.into_inner());
        expr.simplify();
        assert_eq!(Expr::Boolean(true), expr);
    }

    #[test]
    fn simplify_boolean_expr_false() {
        let pair = SaplParser::parse(Rule::target_expression, "false")
            .unwrap()
            .next()
            .unwrap();
        let mut expr = Expr::parse(pair.into_inner());
        expr.simplify();
        assert_eq!(Expr::Boolean(false), expr);

        let pair = SaplParser::parse(Rule::target_expression, "true & false")
            .unwrap()
            .next()
            .unwrap();
        let mut expr = Expr::parse(pair.into_inner());
        expr.simplify();
        assert_eq!(Expr::Boolean(false), expr);

        let pair = SaplParser::parse(Rule::target_expression, "true & true & false")
            .unwrap()
            .next()
            .unwrap();
        let mut expr = Expr::parse(pair.into_inner());
        expr.simplify();
        assert_eq!(Expr::Boolean(false), expr);
    }

    #[test]
    fn simplify_integer_expr() {
        let pair = SaplParser::parse(Rule::target_expression, "40 + 2")
            .unwrap()
            .next()
            .unwrap();
        let mut expr = Expr::parse(pair.into_inner());
        expr.simplify();
        assert_eq!(Expr::Integer(42), expr);

        let pair = SaplParser::parse(Rule::target_expression, "40 + 1 + 1")
            .unwrap()
            .next()
            .unwrap();
        let mut expr = Expr::parse(pair.into_inner());
        println!("{:#?}", expr);
        expr.simplify();
        assert_eq!(Expr::Integer(42), expr);
    }
}
