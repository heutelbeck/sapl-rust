use crate::advice::Advice;
use crate::expr::Expr;
use crate::transformation::Transformation;
use crate::where_statement::WhereStatement;
use crate::Entitlement;
use crate::Rule;

#[derive(Debug, Default)]
pub struct Policy {
    pub name: String,
    entitlement: Entitlement,
    target_exp: Option<Box<Expr>>,
    where_statements: Option<Vec<WhereStatement>>,
    obligations: Option<Box<Expr>>,
    advice: Option<Vec<Advice>>,
    transformation: Option<Vec<Transformation>>,
}

impl Policy {
    pub fn new(pairs: pest::iterators::Pairs<Rule>) -> Self {
        let mut policy = Policy::default();

        for pair in pairs {
            match pair.as_rule() {
                Rule::policy_name => {
                    let mut name = pair.as_str().to_string();
                    name.retain(|c| c != '\"');
                    policy.name = name;
                }
                Rule::entitlement => policy.entitlement = Entitlement::new(pair.as_str()),
                Rule::target_expression => {
                    policy.target_exp = Some(Box::new(Expr::parse(pair.clone().into_inner())))
                }
                Rule::where_statement => {
                    policy.where_statements = Some(
                        pair.clone()
                            .into_inner()
                            .map(WhereStatement::parse)
                            .collect(),
                    );
                }
                Rule::obligation => {
                    policy.obligations = Some(Box::new(Expr::parse(pair.clone().into_inner())));
                }
                Rule::advice => {
                    policy.advice = Some(pair.clone().into_inner().map(Advice::parse).collect());
                }
                Rule::transformation => {
                    policy.transformation = Some(
                        pair.clone()
                            .into_inner()
                            .map(Transformation::parse)
                            .collect(),
                    );
                }
                rule => unreachable!(
                    "Sapl::parse expected policy_name or entitlement, found {:?}",
                    rule
                ),
            }
        }

        policy
    }

    pub fn validate(&self) -> Result<(), String> {
        match &self.target_exp {
            Some(exp) => match exp.validate_target_expr() {
                Some(err) => {
                    let mut joined = String::new();
                    joined.push_str(&format!("The validation of target expression in the policy {} was not successful for the following reasons:\n", &self.name));
                    for e in err {
                        joined.push_str(&format!("* {}\n", e));
                    }
                    Err(joined)
                }
                None => Ok(()),
            },
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_sapl_file;

    #[test]
    fn policy_parse_element_of() {
        let element_of = parse_sapl_file("policy \"doctor and nurse access to patient data\" permit action.java.name == \"findById\" where \"ROLE_DOCTOR\" in subject..authority || \"ROLE_NURSE\" in subject..authority;");
        assert!(element_of.is_ok());
    }
}
