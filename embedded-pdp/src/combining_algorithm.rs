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

mod deny_unless_permit_stream;
pub(crate) use deny_unless_permit_stream::DenyUnlessPermit;

mod permit_unless_deny_stream;
pub(crate) use permit_unless_deny_stream::PermitUnlessDeny;

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Default)]
pub enum PolicyDocumentCombiningAlgorithm {
    #[default]
    DENY_UNLESS_PERMIT,
    PERMIT_UNLESS_DENY,
    ONLY_ONE_APPLICABLE,
    DENY_OVERRIDES,
    PERMIT_OVERRIDES,
}
