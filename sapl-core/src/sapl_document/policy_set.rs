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

use crate::{
    Ast, AuthorizationSubscription, CombiningAlgorithm, Decision, Policy, Rule,
    sapl_document::combining_algorithm::{
        DenyOverrides, DenyUnlessPermit, FirstApplicable, OnlyOneApplicable, PermitOverrides,
        PermitUnlessDeny,
    },
};
use futures::Stream;
use std::{pin::Pin, sync::Arc};

#[derive(Debug, Default)]
pub struct PolicySet {
    pub name: String,
    combining_algorithm: CombiningAlgorithm,
    target_exp: Option<Arc<Ast>>,
    policies: Vec<Policy>,
}

impl PolicySet {
    pub fn new(pairs: pest::iterators::Pairs<Rule>) -> Self {
        let mut policy_set = PolicySet::default();

        for pair in pairs {
            match pair.as_rule() {
                Rule::policy_set_name => {
                    let mut name = pair.as_str().to_string();
                    name.retain(|c| c != '\"');
                    policy_set.name = name;
                }
                Rule::combining_algorithm => {
                    policy_set.combining_algorithm = CombiningAlgorithm::new(pair.as_str())
                }
                Rule::target_expression => {
                    policy_set.target_exp = Some(Arc::new(Ast::parse(pair.clone().into_inner())))
                }
                Rule::policy => policy_set.policies.push(Policy::new(pair.into_inner())),
                rule => unreachable!(
                    "Sapl::parse expected policy_set_name, combining_algorithm or policy, found {rule:?}"
                ),
            }
        }

        policy_set
    }

    pub fn validate(&self) -> Result<(), String> {
        let mut result = String::new();

        for p in &self.policies {
            if let Err(e) = p.validate() {
                result.push_str(&e);
            }
        }

        if result.is_empty() {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn evaluate(&self, auth_sub: &AuthorizationSubscription) -> Decision {
        use CombiningAlgorithm::*;

        match &self.combining_algorithm {
            DENY_OVERRIDES => self.policies.iter().deny_overrides(auth_sub),
            DENY_UNLESS_PERMIT => self.policies.iter().deny_unless_permit(auth_sub),
            FIRST_APPLICABLE => self.policies.iter().first_applicable(auth_sub),
            ONLY_ONE_APPLICABLE => self.policies.iter().only_one_applicable(auth_sub),
            PERMIT_OVERRIDES => self.policies.iter().permit_overrides(auth_sub),
            PERMIT_UNLESS_DENY => self.policies.iter().permit_unless_deny(auth_sub),
        }
    }

    pub fn evaluate_as_stream(
        &self,
        auth_sub: &AuthorizationSubscription,
    ) -> Pin<Box<dyn Stream<Item = Decision> + std::marker::Send>> {
        use CombiningAlgorithm::*;

        let policy_streams = self
            .policies
            .iter()
            .map(|p| p.evaluate_as_stream(auth_sub))
            .collect();

        match &self.combining_algorithm {
            DENY_OVERRIDES => Box::pin(DenyOverrides::new(policy_streams)),
            DENY_UNLESS_PERMIT => Box::pin(DenyUnlessPermit::new(policy_streams)),
            FIRST_APPLICABLE => Box::pin(FirstApplicable::new(policy_streams)),
            ONLY_ONE_APPLICABLE => Box::pin(OnlyOneApplicable::new(policy_streams)),
            PERMIT_OVERRIDES => Box::pin(PermitOverrides::new(policy_streams)),
            PERMIT_UNLESS_DENY => Box::pin(PermitUnlessDeny::new(policy_streams)),
        }
    }
}

trait PolicySetCombiningAlgorithmExt<'a> {
    fn deny_overrides(self, auth_sub: &AuthorizationSubscription) -> Decision;
    fn deny_unless_permit(self, auth_sub: &AuthorizationSubscription) -> Decision;
    fn first_applicable(self, auth_sub: &AuthorizationSubscription) -> Decision;
    fn only_one_applicable(self, auth_sub: &AuthorizationSubscription) -> Decision;
    fn permit_overrides(self, auth_sub: &AuthorizationSubscription) -> Decision;
    fn permit_unless_deny(self, auth_sub: &AuthorizationSubscription) -> Decision;
}

impl<'a, I> PolicySetCombiningAlgorithmExt<'a> for I
where
    I: Iterator<Item = &'a Policy>,
{
    fn deny_overrides(self, auth_sub: &AuthorizationSubscription) -> Decision {
        let mut indeterminate = false;
        let mut permit = false;
        for policy in self {
            let decision = policy.evaluate(auth_sub);
            if Decision::Deny == decision {
                return decision;
            }
            if Decision::Indeterminate == decision {
                indeterminate = true;
            }
            if Decision::Permit == decision {
                permit = true;
            }
        }

        if indeterminate {
            return Decision::Indeterminate;
        }

        if permit {
            return Decision::Permit;
        }

        Decision::NotApplicable
    }

    fn deny_unless_permit(self, auth_sub: &AuthorizationSubscription) -> Decision {
        for policy in self {
            let decision = policy.evaluate(auth_sub);
            if Decision::Permit == decision {
                return decision;
            }
        }

        Decision::Deny
    }

    fn first_applicable(self, auth_sub: &AuthorizationSubscription) -> Decision {
        for policy in self {
            let decision = policy.evaluate(auth_sub);
            if Decision::NotApplicable != decision {
                return decision;
            }
        }

        Decision::NotApplicable
    }

    fn only_one_applicable(self, auth_sub: &AuthorizationSubscription) -> Decision {
        let mut cnt = 0;
        let mut decision = Decision::NotApplicable;
        for policy in self {
            match policy.evaluate(auth_sub) {
                Decision::Indeterminate => return Decision::Indeterminate,
                Decision::Permit => {
                    cnt += 1;
                    decision = Decision::Permit;
                }
                Decision::Deny => {
                    cnt += 1;
                    decision = Decision::Deny;
                }
                Decision::NotApplicable => {}
            }
        }

        match cnt {
            0..=1 => decision,
            _ => Decision::Indeterminate,
        }
    }

    fn permit_overrides(self, auth_sub: &AuthorizationSubscription) -> Decision {
        let mut indeterminate = false;
        let mut deny = false;
        for policy in self {
            let decision = policy.evaluate(auth_sub);
            if Decision::Permit == decision {
                return decision;
            }
            if Decision::Indeterminate == decision {
                indeterminate = true;
            }
            if Decision::Deny == decision {
                deny = true;
            }
        }

        if indeterminate {
            return Decision::Indeterminate;
        }

        if deny {
            return Decision::Deny;
        }

        Decision::NotApplicable
    }

    fn permit_unless_deny(self, auth_sub: &AuthorizationSubscription) -> Decision {
        for policy in self {
            let decision = policy.evaluate(auth_sub);
            if Decision::Deny == decision {
                return decision;
            }
        }

        Decision::Permit
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_sapl_file;

    #[test]
    fn policy_set_validate_ok() {
        let ps = parse_sapl_file(
            "set \"demo\" first-applicable for action.java.name == \"getDocuments\" policy \"first\" permit where time.secondOf(<time.now>) < 20; policy \"second\" permit where time.secondOf(<time.now>) < 40; policy \"third\" permit",
        );
        assert!(ps.expect("parsing failed").validate().is_ok());
    }

    #[test]
    fn policy_set_valiate_target_expr_lazy_and() {
        let ps = parse_sapl_file(
            "set \"demo\" first-applicable for action.java.name == \"getDocuments\" policy \"first\" permit 5 < 6 && 4 > 3 policy \"second\" permit where time.secondOf(<time.now>) < 40; policy \"third\" permit",
        );

        assert!(ps.expect("parsing failed").validate().is_err());
    }

    #[test]
    fn policy_set_valiate_target_expr_lazy_or() {
        let ps = parse_sapl_file(
            "set \"demo\" first-applicable for action.java.name == \"getDocuments\" policy \"first\" permit permit where time.secondOf(<time.now>) < 20; policy \"second\" permit 5 < 6 || 4 > 3 policy \"third\" permit",
        );

        assert!(ps.expect("parsing failed").validate().is_err());
    }
}
