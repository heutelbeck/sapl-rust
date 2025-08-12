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

pub(super) fn deny_unless_permit(
    decisions: &[Option<AuthorizationDecision>],
) -> AuthorizationDecision {
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

pub(super) fn permit_unless_deny(
    decisions: &[Option<AuthorizationDecision>],
) -> AuthorizationDecision {
    let mut combined = AuthorizationDecision::new(Decision::Permit);

    for decision in decisions.iter().flatten() {
        if decision.decision == Decision::Deny {
            println!("✅ Found Permit decision, returning immediately");
            return decision.clone();
        }
        combined.collect(decision.clone());
    }

    println!("🔒 No Permit found, returning combined Deny");
    combined
}
