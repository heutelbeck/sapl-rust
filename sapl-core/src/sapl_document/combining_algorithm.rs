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

use serde::Deserialize;

mod deny_overrides;
pub(crate) use deny_overrides::DenyOverrides;

mod deny_unless_permit_stream;
pub(crate) use deny_unless_permit_stream::DenyUnlessPermit;

mod first_applicable;
pub(crate) use first_applicable::FirstApplicable;

mod only_one_applicable;
pub(crate) use only_one_applicable::OnlyOneApplicable;

mod permit_overrides;
pub(crate) use permit_overrides::PermitOverrides;

mod permit_unless_deny_stream;
pub(crate) use permit_unless_deny_stream::PermitUnlessDeny;

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
