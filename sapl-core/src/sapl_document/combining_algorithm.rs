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

use crate::{AuthorizationDecision, Decision};
use serde::Deserialize;

//https://sapl.io/docs/3.0.0-SNAPSHOT/6_5_CombiningAlgorithm/
#[allow(non_camel_case_types)]
#[derive(Debug, Default, Deserialize, PartialEq)]
pub enum CombiningAlgorithm {
    #[default]
    DENY_OVERRIDES,
    DENY_UNLESS_PERMIT,
    FIRST_APPLICABLE, //(not allowed on PDP level for document combination)
    ONLY_ONE_APPLICABLE,
    PERMIT_OVERRIDES,
    PERMIT_UNLESS_DENY,
}

impl CombiningAlgorithm {
    pub(crate) fn new(s: &str) -> Self {
        if s.eq("deny-overrides") {
            CombiningAlgorithm::DENY_OVERRIDES
        } else if s.eq("permit-overrides") {
            CombiningAlgorithm::PERMIT_OVERRIDES
        } else if s.eq("first-applicable") {
            CombiningAlgorithm::FIRST_APPLICABLE
        } else if s.eq("only-one-applicable") {
            CombiningAlgorithm::ONLY_ONE_APPLICABLE
        } else if s.eq("deny-unless-permit") {
            CombiningAlgorithm::DENY_UNLESS_PERMIT
        } else if s.eq("permit-unless-deny") {
            CombiningAlgorithm::PERMIT_UNLESS_DENY
        } else {
            panic!("Input {s} could not be parsed as combining algorithm")
        }
    }
}

pub fn deny_overrides(decisions: &[Option<AuthorizationDecision>]) -> AuthorizationDecision {
    let mut indeterminate = false;
    let mut permit = false;
    let mut combined = AuthorizationDecision::new(Decision::Permit);

    for decision in decisions.iter().flatten() {
        match decision.decision {
            Decision::Deny => {
                println!("🔒 Found Deny decision, returning immediately");
                return decision.clone();
            }
            Decision::Indeterminate => {
                indeterminate = true;
            }
            Decision::Permit => {
                permit = true;
                combined.collect(decision.clone());
            }
            _ => {}
        }
    }

    if indeterminate {
        return AuthorizationDecision::new(Decision::Indeterminate);
    }

    if permit {
        println!("✅ Found Permit decision, returning combined Permit");
        return combined;
    }

    AuthorizationDecision::new(Decision::NotApplicable)
}

pub fn deny_unless_permit(decisions: &[Option<AuthorizationDecision>]) -> AuthorizationDecision {
    let mut combined = AuthorizationDecision::new(Decision::Deny);

    for decision in decisions.iter().flatten() {
        if decision.decision == Decision::Permit {
            println!("✅ Found Permit decision, returning immediately");
            return decision.clone();
        }
        combined.collect(decision.clone());
    }

    println!("🔒 No Permit found, returning combined Deny");
    combined
}

pub fn first_applicable(decisions: &[Option<AuthorizationDecision>]) -> AuthorizationDecision {
    for decision in decisions.iter().flatten() {
        if decision.decision != Decision::NotApplicable {
            println!("✅ Found decision, returning immediately");
            return decision.clone();
        }
    }

    println!("🔒 No Decision found, returning NotApplicable");
    AuthorizationDecision::new(Decision::NotApplicable)
}

pub fn only_one_applicable(decisions: &[Option<AuthorizationDecision>]) -> AuthorizationDecision {
    let mut cnt = 0;
    let mut combined = AuthorizationDecision::new(Decision::NotApplicable);

    for decision in decisions.iter().flatten() {
        match decision.decision {
            Decision::Indeterminate => {
                println!("🔒 Found Indeterminate, returning immediately");
                return AuthorizationDecision::new(Decision::Indeterminate);
            }
            Decision::Permit => {
                cnt += 1;
                combined = decision.clone();
            }
            Decision::Deny => {
                cnt += 1;
                combined = decision.clone();
            }
            _ => {}
        }
    }

    match cnt {
        0..=1 => combined,
        _ => AuthorizationDecision::new(Decision::Indeterminate),
    }
}

pub fn permit_overrides(decisions: &[Option<AuthorizationDecision>]) -> AuthorizationDecision {
    let mut indeterminate = false;
    let mut deny = false;
    let mut combined = AuthorizationDecision::new(Decision::Deny);

    for decision in decisions.iter().flatten() {
        match decision.decision {
            Decision::Permit => {
                println!("✅ Found Permit decision, returning immediately");
                return decision.clone();
            }
            Decision::Indeterminate => {
                indeterminate = true;
            }
            Decision::Deny => {
                deny = true;
                combined.collect(decision.clone());
            }
            _ => {}
        }
    }

    if indeterminate {
        return AuthorizationDecision::new(Decision::Indeterminate);
    }

    if deny {
        println!("✅ Found Deny decision, returning combined Deny");
        return combined;
    }

    AuthorizationDecision::new(Decision::NotApplicable)
}

pub fn permit_unless_deny(decisions: &[Option<AuthorizationDecision>]) -> AuthorizationDecision {
    let mut combined = AuthorizationDecision::new(Decision::Permit);

    for decision in decisions.iter().flatten() {
        if decision.decision == Decision::Deny {
            println!("✅ Found Deny decision, returning immediately");
            return decision.clone();
        }
        combined.collect(decision.clone());
    }

    println!("🔒 No Deny found, returning combined Permit");
    combined
}
