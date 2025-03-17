use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum BasicIdentifierExpression {
    Subject,
    Action,
    Resource,
    Environment,
}

impl BasicIdentifierExpression {
    pub fn new(s: &str) -> Self {
        if s.eq("subject") {
            BasicIdentifierExpression::Subject
        } else if s.eq("action") {
            BasicIdentifierExpression::Action
        } else if s.eq("resource") {
            BasicIdentifierExpression::Resource
        } else if s.eq("environment") {
            BasicIdentifierExpression::Environment
        } else {
            panic!(
                "Input {} could not be parsed as basic identifier expression",
                s
            )
        }
    }
}

impl Display for BasicIdentifierExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BasicIdentifierExpression::*;

        write!(
            f,
            "{}",
            match &self {
                Subject => "subject",
                Action => "action",
                Resource => "resource",
                Environment => "environment",
            }
        )
    }
}
