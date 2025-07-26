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

use crate::Eval;
use crate::PRATT_PARSER;
use crate::Rule;
use crate::StreamSapl;
use crate::Val;
use crate::authorization_subscription::AuthorizationSubscription;
use crate::basic_identifier_expression::BasicIdentifierExpression;
use crate::once_val;
use crate::pip::Time;

use futures::Stream;
use std::collections::VecDeque;
use std::fmt::Display;
use std::pin::Pin;
use std::sync::Arc;

#[derive(PartialEq, Debug)]
pub enum Ast {
    Expr {
        lhs: Arc<Ast>,
        op: Op,
        rhs: Arc<Ast>,
    },
    SaplPairs(Arc<Vec<Ast>>),
    SaplPair {
        lhs: Arc<Ast>,
        rhs: Arc<Vec<Ast>>,
    },
    BasicEnvironmentAttribute(Arc<Vec<Ast>>),
    BasicEnvironmentHeadAttribute(Arc<Ast>),
    BasicIdentifier(Arc<Vec<Ast>>),
    BasicFunction(Arc<Vec<Ast>>),
    BasicValue(Arc<Vec<Ast>>),
    BasicGroup(Arc<Ast>),
    FilterComponent(Arc<Vec<Ast>>),
    FilterStatement(Arc<Vec<Ast>>),
    BasicIdentifierExpression(Arc<BasicIdentifierExpression>),
    FunctionIdentifier(Arc<Vec<Ast>>),
    VariableAssignment(Arc<Vec<Ast>>),
    Array(Arc<Vec<Ast>>),
    UnaryPlus(Arc<Ast>),
    UnaryMinus(Arc<Ast>),
    LogicalNot(Arc<Ast>),
    Arguments(Arc<Ast>),
    Subscript(Arc<Ast>),
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

impl Clone for Ast {
    fn clone(&self) -> Self {
        match self {
            Ast::Expr { lhs, op, rhs } => Ast::Expr {
                lhs: Arc::clone(lhs),
                op: op.clone(),
                rhs: Arc::clone(rhs),
            },
            Ast::SaplPairs(expr) => Ast::SaplPairs(Arc::clone(expr)),
            Ast::SaplPair { lhs, rhs } => Ast::SaplPair {
                lhs: Arc::clone(lhs),
                rhs: Arc::clone(rhs),
            },
            Ast::BasicEnvironmentAttribute(expr) => {
                Ast::BasicEnvironmentAttribute(Arc::clone(expr))
            }
            Ast::BasicEnvironmentHeadAttribute(expr) => {
                Ast::BasicEnvironmentHeadAttribute(Arc::clone(expr))
            }
            Ast::BasicIdentifier(expr) => Ast::BasicIdentifier(Arc::clone(expr)),
            Ast::BasicFunction(expr) => Ast::BasicFunction(Arc::clone(expr)),
            Ast::BasicGroup(expr) => Ast::BasicGroup(Arc::clone(expr)),
            Ast::BasicValue(expr) => Ast::BasicValue(Arc::clone(expr)),
            Ast::BasicIdentifierExpression(expr) => {
                Ast::BasicIdentifierExpression(Arc::clone(expr))
            }
            Ast::FunctionIdentifier(expr) => Ast::FunctionIdentifier(Arc::clone(expr)),
            Ast::VariableAssignment(expr) => Ast::VariableAssignment(Arc::clone(expr)),
            Ast::FilterComponent(expr) => Ast::FilterComponent(Arc::clone(expr)),
            Ast::FilterStatement(expr) => Ast::FilterStatement(Arc::clone(expr)),
            Ast::Array(expr) => Ast::Array(Arc::clone(expr)),
            Ast::UnaryPlus(expr) => Ast::UnaryPlus(Arc::clone(expr)),
            Ast::UnaryMinus(expr) => Ast::UnaryMinus(Arc::clone(expr)),
            Ast::LogicalNot(expr) => Ast::LogicalNot(Arc::clone(expr)),
            Ast::Arguments(expr) => Ast::Arguments(Arc::clone(expr)),
            Ast::Subscript(expr) => Ast::Subscript(Arc::clone(expr)),
            Ast::Boolean(val) => Ast::Boolean(*val),
            Ast::Integer(val) => Ast::Integer(*val),
            Ast::Float(val) => Ast::Float(*val),
            Ast::SignedNumber(val) => Ast::SignedNumber(val.clone()),
            Ast::String(val) => Ast::String(val.clone()),
            Ast::Id(val) => Ast::Id(val.clone()),
            Ast::KeyStep(val) => Ast::KeyStep(val.clone()),
            Ast::RecursiveKeyStep(val) => Ast::RecursiveKeyStep(val.clone()),
            Ast::RecursiveIndexStep(val) => Ast::RecursiveIndexStep(val.clone()),
            Ast::RecursiveWildcardStep => Ast::RecursiveWildcardStep,
            Ast::AttributeFinderStep(val) => Ast::AttributeFinderStep(val.clone()),
            Ast::HeadAttributeFinderStep(val) => Ast::HeadAttributeFinderStep(val.clone()),
            Ast::Concat => Ast::Concat,
            Ast::Div => Ast::Div,
            Ast::Null => Ast::Null,
            Ast::Undefined => Ast::Undefined,
        }
    }
}

impl Ast {
    pub fn parse(pairs: pest::iterators::Pairs<Rule>) -> Self {
        fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Ast {
            match pair.as_rule() {
                Rule::pairs => {
                    Ast::SaplPairs(Arc::new(pair.into_inner().map(parse_pair).collect()))
                }
                Rule::pair => {
                    let mut inner_rules = pair.into_inner();
                    let lhs = inner_rules.next().unwrap();
                    Ast::SaplPair {
                        lhs: Arc::new(Ast::new_string(lhs.as_str())),
                        rhs: Arc::new(inner_rules.map(parse_pair).collect()),
                    }
                }
                Rule::basic_group => Ast::parse(pair.into_inner()),
                Rule::basic_value => {
                    Ast::BasicValue(Arc::new(pair.into_inner().map(parse_pair).collect()))
                }
                Rule::basic_identifier => {
                    Ast::BasicIdentifier(Arc::new(pair.into_inner().map(parse_basics).collect()))
                }
                Rule::array => Ast::Array(Arc::new(pair.into_inner().map(parse_pair).collect())),
                Rule::string => Ast::new_string(pair.as_str()),
                Rule::boolean_literal => Ast::Boolean(pair.as_str().parse().unwrap()),
                Rule::integer => Ast::Integer(pair.as_str().trim().parse().unwrap()),
                Rule::float => Ast::Float(pair.as_str().trim().parse().unwrap()),
                Rule::div => Ast::Div,
                Rule::addition => Ast::Concat,
                rule => unreachable!(
                    "Ast::parse_pair expected pairs, pair, string, boolean_literal, integer or float, found {:?}",
                    rule
                ),
            }
        }

        fn parse_basics(pair: pest::iterators::Pair<Rule>) -> Ast {
            match pair.as_rule() {
                Rule::basic_identifier_expression => Ast::BasicIdentifierExpression(Arc::new(
                    BasicIdentifierExpression::new(pair.as_str()),
                )),
                Rule::function_identifier => {
                    Ast::FunctionIdentifier(Arc::new(pair.into_inner().map(parse_basics).collect()))
                }
                Rule::arguments => {
                    Ast::Arguments(Arc::new(parse_basics(pair.into_inner().next().unwrap())))
                }
                Rule::subscript => {
                    Ast::Subscript(Arc::new(parse_basics(pair.into_inner().next().unwrap())))
                }
                Rule::basic_environment_attribute => Ast::BasicEnvironmentAttribute(Arc::new(
                    pair.into_inner().map(parse_basics).collect(),
                )),
                Rule::basic_environment_head_attribute => Ast::BasicEnvironmentHeadAttribute(
                    Arc::new(parse_basics(pair.into_inner().next().unwrap())),
                ),
                Rule::basic_identifier => {
                    Ast::BasicIdentifier(Arc::new(pair.into_inner().map(parse_basics).collect()))
                }
                Rule::filter_statement => {
                    Ast::FilterStatement(Arc::new(pair.into_inner().map(parse_basics).collect()))
                }
                Rule::array => Ast::Array(Arc::new(pair.into_inner().map(parse_basics).collect())),
                Rule::signed_number => Ast::SignedNumber(pair.as_str().to_string()),
                Rule::key_step => Ast::KeyStep(pair.as_str().to_string()),
                Rule::recursive_key_step => Ast::RecursiveKeyStep(pair.as_str().to_string()),
                Rule::recursive_index_step => Ast::RecursiveIndexStep(pair.as_str().to_string()),
                Rule::recursive_wildcard_step => Ast::RecursiveWildcardStep,
                Rule::attribute_finder_step => Ast::AttributeFinderStep(pair.as_str().to_string()),
                Rule::head_attribute_finder_step => {
                    Ast::HeadAttributeFinderStep(pair.as_str().to_string())
                }
                Rule::pairs => {
                    Ast::SaplPairs(Arc::new(pair.into_inner().map(parse_pair).collect()))
                }
                Rule::string => Ast::new_string(pair.as_str()),
                Rule::integer => Ast::Integer(pair.as_str().trim().parse().unwrap()),
                Rule::id => Ast::Id(pair.as_str().to_string()),
                Rule::div => Ast::Div,
                Rule::null_literal => Ast::Null,
                Rule::undefined_literal => Ast::Undefined,
                rule => unreachable!(
                    "Ast::parse_basic_identifier expected basic_identifier_expression, key_step, recursive_key_step, or attribute_finder_step, found {:?}",
                    rule
                ),
            }
        }
        PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::pairs => Ast::SaplPairs(Arc::new(primary.into_inner().map(parse_pair).collect())),
            Rule::basic_identifier => Ast::BasicIdentifier(Arc::new(primary.into_inner().map(parse_basics).collect())),
            Rule::basic_function => Ast::BasicFunction(Arc::new(primary.into_inner().map(parse_basics).collect())),
            Rule::basic_group => Ast::BasicGroup(Arc::new(Ast::parse(primary.into_inner()))),
            Rule::basic_value => Ast::BasicValue(Arc::new(primary.into_inner().map(parse_basics).collect())),
            Rule::basic_environment_attribute => Ast::BasicEnvironmentAttribute(Arc::new(primary.into_inner().map(parse_basics).collect())),
            Rule::basic_environment_head_attribute => Ast::BasicEnvironmentHeadAttribute(Arc::new(parse_basics(primary.into_inner().next().unwrap()))),
            Rule::filter_component => Ast::FilterComponent(Arc::new(primary.into_inner().map(parse_basics).collect())),
            Rule::array => Ast::Array(Arc::new(primary.into_inner().map(parse_basics).collect())),
            Rule::string => Ast::new_string(primary.as_str()),
            Rule::boolean_literal => Ast::Boolean(primary.as_str().parse().unwrap()),
            Rule::integer => Ast::Integer(primary.as_str().parse().unwrap()),
            Rule::float => Ast::Float(primary.as_str().parse().unwrap()),
            rule => unreachable!("Ast::parse expected pairs, string, boolean_literal, integer or float, found {:?}", rule),
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
                rule => unreachable!("Ast::parse expected infix operation, found {:?}", rule),
            };
            Ast::Expr {
                lhs: Arc::new(lhs),
                op,
                rhs: Arc::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_str() {
            "!" => Ast::LogicalNot(Arc::new(rhs)),
            "-" => Ast::UnaryMinus(Arc::new(rhs)),
            "+" => Ast::UnaryPlus(Arc::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
    }

    pub fn parse_where_statement(pair: pest::iterators::Pair<Rule>) -> Ast {
        match pair.as_rule() {
            Rule::condition => Ast::parse(pair.into_inner()),
            Rule::variable_assignment => Ast::VariableAssignment(Arc::new(
                pair.into_inner().map(Ast::parse_where_statement).collect(),
            )),
            Rule::id => Ast::Id(pair.as_str().to_string()),
            rule => unreachable!(
                "parse_where_statement expected conditon, variable_assignment, id, string, integer, floar, boolean_literal, pairs or pair, found {:?}",
                rule
            ),
        }
    }

    fn new_string(src: &str) -> Ast {
        let mut s = src.to_string();
        s.retain(|c| c != '\"');
        Ast::String(s)
    }

    pub fn evaluate(&self, auth_subscription: &AuthorizationSubscription) -> Result<bool, String> {
        match self.evaluate_inner(auth_subscription) {
            Ok(Val::Boolean(b)) => Ok(b),
            Ok(Val::CompFloat(b, _)) => Ok(b),
            Ok(Val::CompInteger(b, _)) => Ok(b),
            other => Err(format!(
                "Ast::evaluate expected Boolean or Expr, found {other:#?}"
            )),
        }
    }

    pub fn evaluate_inner(
        &self,
        auth_subscription: &AuthorizationSubscription,
    ) -> Result<Val, String> {
        use self::Ast::*;
        use crate::evaluate::*;

        match self {
            Boolean(b) => Ok(Val::Boolean(*b)),
            Integer(i) => Ok(Val::Integer(*i)),
            Float(i) => Ok(Val::Float(*i)),
            String(s) => Ok(Val::String(s.clone())),
            BasicFunction(bf) => basic_function(bf, auth_subscription),
            BasicIdentifier(bi) => basic_identifier(bi, auth_subscription),
            SaplPairs(pairs) => sapl_pairs(pairs, auth_subscription),
            SaplPair { lhs, rhs } => sapl_pair(lhs, rhs, auth_subscription),
            Array(a) => array(a, auth_subscription),
            Expr { lhs, op, rhs } => match op {
                Op::Addition => add(
                    lhs.evaluate_inner(auth_subscription),
                    rhs.evaluate_inner(auth_subscription),
                ),
                Op::Comparison => panic!(),
                Op::Division => div(
                    lhs.evaluate_inner(auth_subscription),
                    rhs.evaluate_inner(auth_subscription),
                ),
                Op::EagerAnd => eager_and(
                    lhs.evaluate_inner(auth_subscription),
                    rhs.evaluate_inner(auth_subscription),
                ),
                Op::EagerOr => eager_or(
                    lhs.evaluate_inner(auth_subscription),
                    rhs.evaluate_inner(auth_subscription),
                ),
                Op::Equal => eq(
                    lhs.evaluate_inner(auth_subscription),
                    rhs.evaluate_inner(auth_subscription),
                ),
                Op::ExclusiveOr => xor(
                    lhs.evaluate_inner(auth_subscription),
                    rhs.evaluate_inner(auth_subscription),
                ),
                Op::Filter => panic!(),
                Op::Greater => ge(
                    lhs.evaluate_inner(auth_subscription),
                    rhs.evaluate_inner(auth_subscription),
                ),
                Op::GreaterEqual => ge_eq(
                    lhs.evaluate_inner(auth_subscription),
                    rhs.evaluate_inner(auth_subscription),
                ),
                Op::LazyAnd => panic!(),
                Op::LazyOr => panic!(),
                Op::Less => le(
                    lhs.evaluate_inner(auth_subscription),
                    rhs.evaluate_inner(auth_subscription),
                ),
                Op::LessEqual => le_eq(
                    lhs.evaluate_inner(auth_subscription),
                    rhs.evaluate_inner(auth_subscription),
                ),
                Op::Modulo => modulo(
                    lhs.evaluate_inner(auth_subscription),
                    rhs.evaluate_inner(auth_subscription),
                ),
                Op::Multiplication => mul(
                    lhs.evaluate_inner(auth_subscription),
                    rhs.evaluate_inner(auth_subscription),
                ),
                Op::NotEqual => non_eq(
                    lhs.evaluate_inner(auth_subscription),
                    rhs.evaluate_inner(auth_subscription),
                ),
                Op::Regex => panic!(),
                Op::Subtract => sub(
                    lhs.evaluate_inner(auth_subscription),
                    rhs.evaluate_inner(auth_subscription),
                ),
            },
            other => Err(format!(
                "Ast::evaluate_inner expected Boolean or Expr, found {other:#?}"
            )),
        }
    }

    pub fn validate_schema_expr(&self) -> Option<Vec<ValidationErr>> {
        let check = |e: &Arc<Ast>| -> Option<ValidationErr> {
            match &**e {
                Ast::AttributeFinderStep(_) => Some(ValidationErr::AttributeFinderStep),
                Ast::HeadAttributeFinderStep(_) => Some(ValidationErr::HeadAttributeFinderStep),
                Ast::BasicEnvironmentAttribute(_) => Some(ValidationErr::BasicEnvironmentAttribute),
                Ast::BasicEnvironmentHeadAttribute(_) => {
                    Some(ValidationErr::BasicEnvironmentHeadAttribute)
                }
                _ => None,
            }
        };

        self.validate(check)
    }

    pub fn validate_target_expr(&self) -> Option<Vec<ValidationErr>> {
        let check = |e: &Arc<Ast>| -> Option<ValidationErr> {
            match &**e {
                Ast::Expr { op, .. } => match op {
                    Op::LazyAnd => Some(ValidationErr::LazyAnd),
                    Op::LazyOr => Some(ValidationErr::LazyOr),
                    _ => None,
                },
                Ast::AttributeFinderStep(_) => Some(ValidationErr::AttributeFinderStep),
                Ast::HeadAttributeFinderStep(_) => Some(ValidationErr::HeadAttributeFinderStep),
                Ast::BasicEnvironmentAttribute(_) => Some(ValidationErr::BasicEnvironmentAttribute),
                Ast::BasicEnvironmentHeadAttribute(_) => {
                    Some(ValidationErr::BasicEnvironmentHeadAttribute)
                }
                _ => None,
            }
        };

        self.validate(check)
    }

    fn validate(
        &self,
        check: fn(&Arc<Ast>) -> Option<ValidationErr>,
    ) -> Option<Vec<ValidationErr>> {
        let result: Vec<_> = self
            .iter()
            .filter_map(|expr: Arc<Ast>| check(&expr))
            .collect();

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    pub fn simplify(&mut self) {
        self.simplify_expr();
    }

    pub fn simplify_expr(&mut self) -> &mut Self {
        use self::Ast::*;
        use crate::simplify::*;

        match self {
            Expr { lhs, op, rhs } => match op {
                Op::Addition => {
                    let mut lhs = Ast::simplify_inner_expr(lhs);
                    let mut rhs = Ast::simplify_inner_expr(rhs);

                    if let Some(result) = add(&mut lhs, &mut rhs) {
                        *self = result;
                    }
                    self
                }
                Op::Comparison => panic!(),
                Op::Division => panic!(),
                Op::EagerAnd => {
                    let mut lhs = Ast::simplify_inner_expr(lhs);
                    let mut rhs = Ast::simplify_inner_expr(rhs);

                    if let Some(result) = eager_and(&mut lhs, &mut rhs) {
                        *self = result;
                    }
                    self
                }
                Op::EagerOr => {
                    let mut lhs = Ast::simplify_inner_expr(lhs);
                    let mut rhs = Ast::simplify_inner_expr(rhs);

                    if let Some(result) = eager_or(&mut lhs, &mut rhs) {
                        *self = result;
                    }
                    self
                }
                Op::Equal => panic!(),
                Op::ExclusiveOr => panic!(),
                Op::Filter => panic!(),
                Op::Greater => panic!(),
                Op::GreaterEqual => panic!(),
                Op::LazyAnd => panic!(),
                Op::LazyOr => panic!(),
                Op::Less => panic!(),
                Op::LessEqual => panic!(),
                Op::Modulo => panic!(),
                Op::Multiplication => panic!(),
                Op::NotEqual => panic!(),
                Op::Regex => panic!(),
                Op::Subtract => panic!(),
            },
            _ => self,
        }
    }

    fn simplify_inner_expr(elem: &mut Arc<Ast>) -> Arc<Ast> {
        use self::Ast::*;

        match elem.as_ref() {
            Expr { .. } => {
                let simplified = Arc::make_mut(&mut elem.clone()).simplify_expr().clone();
                Arc::new(simplified)
            }
            _ => elem.clone(),
        }
    }

    pub(crate) fn iter(&self) -> ExprIter {
        let mut stack = VecDeque::new();
        stack.push_back(Arc::new(self.clone()));
        ExprIter { stack }
    }
}

impl<S> StreamSapl for S where S: Stream<Item = Result<Val, String>> + std::marker::Send {}
impl Eval for Ast {
    fn eval(
        &self,
        auth_subscription: &AuthorizationSubscription,
    ) -> Pin<Box<dyn Stream<Item = Result<Val, String>> + Send>> {
        match self {
            Ast::Expr { lhs, op, rhs } => match op {
                Op::EagerAnd => Box::pin(
                    lhs.eval(auth_subscription)
                        .eval_and(rhs.eval(auth_subscription)),
                ),
                Op::Equal => Box::pin(
                    lhs.eval(auth_subscription)
                        .eval_eq(rhs.eval(auth_subscription)),
                ),
                Op::Less => Box::pin(
                    lhs.eval(auth_subscription)
                        .eval_le(rhs.eval(auth_subscription)),
                ),
                Op::Greater => Box::pin(
                    lhs.eval(auth_subscription)
                        .eval_ge(rhs.eval(auth_subscription)),
                ),
                Op::Addition => Box::pin(
                    lhs.eval(auth_subscription)
                        .eval_add(rhs.eval(auth_subscription)),
                ),
                Op::Subtract => Box::pin(
                    lhs.eval(auth_subscription)
                        .eval_sub(rhs.eval(auth_subscription)),
                ),
                _ => unimplemented!(),
            },
            Ast::BasicIdentifier(bi) => Box::pin(once_val(
                crate::evaluate::basic_identifier(bi, auth_subscription).unwrap(),
            )),
            Ast::BasicFunction(_) => Box::pin(Time::new(1000).eval_seconds_of()),
            Ast::Boolean(b) => Box::pin(once_val(Val::Boolean(*b))),
            Ast::Integer(i) => Box::pin(once_val(Val::Integer(*i))),
            Ast::Float(f) => Box::pin(once_val(Val::Float(*f))),
            Ast::String(s) => Box::pin(once_val(Val::String(s.clone()))),
            others => unimplemented!("{}", format!("{:#?} is not implemented yet", others)),
        }
    }
}

pub struct ExprIter {
    stack: VecDeque<Arc<Ast>>,
}

impl Iterator for ExprIter {
    type Item = Arc<Ast>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop_front() {
            match &*node {
                Ast::Expr { lhs, op: _, rhs } => {
                    self.stack.push_back(Arc::clone(rhs));
                    self.stack.push_back(Arc::clone(lhs));
                }
                Ast::SaplPair { lhs, rhs } => {
                    self.stack.push_back(Arc::clone(lhs));
                    for elem in rhs.iter() {
                        self.stack.push_back(Arc::new(elem.clone()));
                    }
                }
                Ast::SaplPairs(expr)
                | Ast::BasicIdentifier(expr)
                | Ast::BasicEnvironmentAttribute(expr)
                | Ast::BasicFunction(expr)
                | Ast::BasicValue(expr)
                | Ast::FunctionIdentifier(expr)
                | Ast::VariableAssignment(expr)
                | Ast::FilterComponent(expr)
                | Ast::FilterStatement(expr)
                | Ast::Array(expr) => {
                    for elem in expr.iter() {
                        self.stack.push_back(Arc::new(elem.clone()));
                    }
                }
                Ast::UnaryPlus(expr)
                | Ast::UnaryMinus(expr)
                | Ast::LogicalNot(expr)
                | Ast::Arguments(expr)
                | Ast::Subscript(expr)
                | Ast::BasicGroup(expr)
                | Ast::BasicEnvironmentHeadAttribute(expr) => {
                    self.stack.push_back(Arc::clone(expr));
                }
                Ast::KeyStep(_)
                | Ast::RecursiveKeyStep(_)
                | Ast::RecursiveIndexStep(_)
                | Ast::RecursiveWildcardStep
                | Ast::AttributeFinderStep(_)
                | Ast::HeadAttributeFinderStep(_)
                | Ast::BasicIdentifierExpression(_)
                | Ast::Boolean(_)
                | Ast::Integer(_)
                | Ast::Float(_)
                | Ast::SignedNumber(_)
                | Ast::String(_)
                | Ast::Id(_)
                | Ast::Concat
                | Ast::Div
                | Ast::Null
                | Ast::Undefined => {}
            }

            Some(node)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Op {
    Addition,
    Comparison,
    Division,
    EagerAnd,
    EagerOr,
    Equal,
    ExclusiveOr,
    Filter,
    Greater,
    GreaterEqual,
    LazyAnd,
    LazyOr,
    Less,
    LessEqual,
    Modulo,
    Multiplication,
    NotEqual,
    Regex,
    Subtract,
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
    use pest::{Parser, iterators::Pairs};
    use tokio_stream::StreamExt;

    use super::*;

    fn parse_expression(input: &str) -> Pairs<Rule> {
        SaplParser::parse(Rule::expression, input)
            .unwrap_or_else(|e| panic!("Failed to parse '{input}': {e}"))
    }

    fn assert_next_rule(pairs: &mut Pairs<Rule>, expected: Rule) {
        let actual = pairs
            .next()
            .unwrap_or_else(|| panic!("No more pairs available"))
            .as_rule();
        assert_eq!(actual, expected, "Expected {expected:?}, got {actual:?}");
    }

    fn assert_rule_sequence(mut pairs: Pairs<Rule>, expected_rules: &[Rule]) {
        for &expected in expected_rules {
            assert_next_rule(&mut pairs, expected);
        }

        assert!(
            pairs.next().is_none(),
            "Expected no more rules, but found additional ones"
        );
    }

    fn parse_and_assert_rules(input: &str, expected_rules: &[Rule]) {
        let pairs = parse_expression(input);
        assert_rule_sequence(pairs, expected_rules);
    }

    macro_rules! test_parse_rules {
    ($test_name:ident, $input:expr, $($rule:expr),+ $(,)?) => {
        #[test]
        fn $test_name() {
            parse_and_assert_rules($input, &[$($rule),+]);
        }
    };
    }

    test_parse_rules!(
        expr_lazy_or,
        "a || b",
        Rule::basic_identifier,
        Rule::lazy_or,
        Rule::basic_identifier
    );

    test_parse_rules!(
        expr_lazy_and,
        "a && b",
        Rule::basic_identifier,
        Rule::lazy_and,
        Rule::basic_identifier
    );

    test_parse_rules!(
        expr_eager_or,
        "a | b",
        Rule::basic_identifier,
        Rule::eager_or,
        Rule::basic_identifier
    );

    test_parse_rules!(
        expr_exclusive_or,
        "a ^ b",
        Rule::basic_identifier,
        Rule::exclusive_or,
        Rule::basic_identifier
    );

    test_parse_rules!(
        expr_eager_and,
        "a & b",
        Rule::basic_identifier,
        Rule::eager_and,
        Rule::basic_identifier
    );

    test_parse_rules!(
        expr_equal,
        "a == b",
        Rule::basic_identifier,
        Rule::equal,
        Rule::basic_identifier
    );

    test_parse_rules!(
        expr_boolean_literal,
        "false == true",
        Rule::boolean_literal,
        Rule::equal,
        Rule::boolean_literal
    );

    fn parse(rule: Rule, input: &str) -> Result<Pairs<'_, Rule>, Box<pest::error::Error<Rule>>> {
        Ok(SaplParser::parse(rule, input)?)
    }

    fn parse_boolean_literal(
        input: &str,
    ) -> Result<Pairs<'_, Rule>, Box<pest::error::Error<Rule>>> {
        parse(Rule::boolean_literal, input)
    }

    fn parse_as_rule(rule: Rule, input: &str) -> Rule {
        parse(rule, input).unwrap().next().unwrap().as_rule()
    }

    #[test]
    fn boolean_literal_false() {
        assert_eq!(
            parse_as_rule(Rule::boolean_literal, "false"),
            Rule::boolean_literal,
        );

        assert!(parse_boolean_literal("FALSE").is_err());
        assert!(parse_boolean_literal("False").is_err());
        assert!(parse_boolean_literal("LoremIpsum").is_err());
    }

    #[test]
    fn boolean_literal_true() {
        assert_eq!(
            parse_as_rule(Rule::boolean_literal, "true"),
            Rule::boolean_literal,
        );

        assert!(parse_boolean_literal("TRUE").is_err());
        assert!(parse_boolean_literal("True").is_err());
        assert!(parse_boolean_literal("LoremIpsum").is_err());
    }

    #[test]
    fn evaluate_boolean_literal_false() {
        let pair = SaplParser::parse(Rule::target_expression, "false")
            .unwrap()
            .next()
            .unwrap();
        let expr = Ast::parse(pair.into_inner())
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
        let expr = Ast::parse(pair.into_inner())
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
        let expr = Ast::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "true & false")
            .unwrap()
            .next()
            .unwrap();
        let expr = Ast::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(!expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "true & true & true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Ast::parse(pair.into_inner())
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
        let expr = Ast::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "false | false")
            .unwrap()
            .next()
            .unwrap();
        let expr = Ast::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(!expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "false | false | true")
            .unwrap()
            .next()
            .unwrap();
        let expr = Ast::parse(pair.into_inner())
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
        let expr = Ast::parse(pair.into_inner())
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
        let expr = Ast::parse(pair.into_inner())
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
        let expr = Ast::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "5 < 10 < 15")
            .unwrap()
            .next()
            .unwrap();
        let expr = Ast::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "5 < 10 < 15 < 20")
            .unwrap()
            .next()
            .unwrap();
        let expr = Ast::parse(pair.into_inner())
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
        let expr = Ast::parse(pair.into_inner())
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
        let expr = Ast::parse(pair.into_inner())
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
        let expr = Ast::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "5 + 20 < 50")
            .unwrap()
            .next()
            .unwrap();
        let expr = Ast::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription1());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
        let pair = SaplParser::parse(Rule::target_expression, "5 + 20 < 50 + 10")
            .unwrap()
            .next()
            .unwrap();
        let expr = Ast::parse(pair.into_inner())
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
        let expr = Ast::parse(pair.into_inner())
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
        let expr = Ast::parse(pair.into_inner())
            .evaluate(&AuthorizationSubscription::new_example_subscription2());
        assert!(expr.is_ok());
        assert!(expr.unwrap());
    }

    #[test]
    fn validate_target_expr_without_error() {
        let pair = SaplParser::parse(Rule::target_expression, "false | 5 < 20 & 1 > 50 | true")
            .unwrap()
            .next()
            .unwrap();
        let validation_result = Ast::parse(pair.into_inner()).validate_target_expr();
        assert!(validation_result.is_none());
    }

    #[test]
    fn validate_target_expr_lazy_and() {
        let pair = SaplParser::parse(Rule::target_expression, "5 < 20 && 1 > 50")
            .unwrap()
            .next()
            .unwrap();
        let validation_result = Ast::parse(pair.into_inner()).validate_target_expr();
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
        let validation_result = Ast::parse(pair.into_inner()).validate_target_expr();
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
        let validation_result = Ast::parse(pair.into_inner()).validate_target_expr();
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
        let validation_result = Ast::parse(pair.into_inner()).validate_target_expr();
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
        let validation_result = Ast::parse(pair.into_inner()).validate_target_expr();
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
        let validation_result = Ast::parse(pair.into_inner()).validate_target_expr();
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
        let mut expr = Ast::parse(pair.into_inner());
        expr.simplify();
        assert_eq!(Ast::Boolean(true), expr);

        let pair = SaplParser::parse(Rule::target_expression, "true & true")
            .unwrap()
            .next()
            .unwrap();
        let mut expr = Ast::parse(pair.into_inner());
        expr.simplify();
        assert_eq!(Ast::Boolean(true), expr);

        let pair = SaplParser::parse(Rule::target_expression, "true & true & true")
            .unwrap()
            .next()
            .unwrap();
        let mut expr = Ast::parse(pair.into_inner());
        expr.simplify();
        assert_eq!(Ast::Boolean(true), expr);
    }

    #[test]
    fn simplify_boolean_expr_false() {
        let pair = SaplParser::parse(Rule::target_expression, "false")
            .unwrap()
            .next()
            .unwrap();
        let mut expr = Ast::parse(pair.into_inner());
        expr.simplify();
        assert_eq!(Ast::Boolean(false), expr);

        let pair = SaplParser::parse(Rule::target_expression, "true & false")
            .unwrap()
            .next()
            .unwrap();
        let mut expr = Ast::parse(pair.into_inner());
        expr.simplify();
        assert_eq!(Ast::Boolean(false), expr);

        let pair = SaplParser::parse(Rule::target_expression, "true & true & false")
            .unwrap()
            .next()
            .unwrap();
        let mut expr = Ast::parse(pair.into_inner());
        expr.simplify();
        assert_eq!(Ast::Boolean(false), expr);
    }

    #[test]
    fn simplify_integer_expr() {
        let pair = SaplParser::parse(Rule::target_expression, "40 + 2")
            .unwrap()
            .next()
            .unwrap();
        let mut expr = Ast::parse(pair.into_inner());
        expr.simplify();
        assert_eq!(Ast::Integer(42), expr);

        let pair = SaplParser::parse(Rule::target_expression, "40 + 1 + 1")
            .unwrap()
            .next()
            .unwrap();
        let mut expr = Ast::parse(pair.into_inner());
        expr.simplify();
        assert_eq!(Ast::Integer(42), expr);
    }

    #[tokio::test]
    async fn simple_bool_expr_stream_evaluation() {
        let pair = SaplParser::parse(Rule::target_expression, "true == true")
            .unwrap()
            .next()
            .unwrap();

        let mut stream = Ast::parse(pair.into_inner())
            .eval(&AuthorizationSubscription::new_example_subscription1());

        assert_eq!(stream.next().await, Some(Ok(Val::Boolean(true))));
    }

    #[tokio::test]
    async fn simple_integer_expr_stream_evaluation() {
        let pair = SaplParser::parse(Rule::target_expression, "40 < 42")
            .unwrap()
            .next()
            .unwrap();

        let mut stream = Ast::parse(pair.into_inner())
            .eval(&AuthorizationSubscription::new_example_subscription1());

        assert_eq!(stream.next().await, Some(Ok(Val::CompInteger(true, 42))));
    }

    #[tokio::test]
    async fn simple_integer_add_expr_stream_evaluation() {
        let pair = SaplParser::parse(Rule::target_expression, "40 + 42")
            .unwrap()
            .next()
            .unwrap();

        let mut stream = Ast::parse(pair.into_inner())
            .eval(&AuthorizationSubscription::new_example_subscription1());

        assert_eq!(stream.next().await, Some(Ok(Val::Integer(82))));
    }
}
