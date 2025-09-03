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

//add files to source tree
mod file_reader;
mod pdp_config;

//Re-export types from sapl-core that users need
pub use crate::file_reader::*;
pub use sapl_core::authorization_subscription::AuthorizationSubscription;

use crate::pdp_config::PdpConfig;
use futures::Stream;
use sapl_core::{
    AuthorizationDecision, CombiningAlgorithm, DecisionCombinedStream, SaplDocument,
    deny_overrides, deny_unless_permit, only_one_applicable, parse_sapl_file, permit_overrides,
    permit_unless_deny, stream_sapl::StreamSaplDecision,
};
use serde_json::Value;
use std::{
    fs::{self, read_dir},
    path::{Path, PathBuf},
    pin::Pin,
    sync::{Arc, RwLock},
};

pub struct Pdp {
    pub config: Arc<RwLock<PdpConfig>>,
    pub policies: Arc<RwLock<Vec<SaplDocument>>>,
}

impl Pdp {
    pub fn new(config_file: Option<&Path>, policies_folder: Option<&Path>) -> Self {
        let cfg = match config_file {
            Some(p) => PdpConfig::new(p),
            None => PdpConfig::default(),
        };

        let policies = match policies_folder {
            Some(p) => read_all_policies(recurse(p)),
            None => vec![],
        };

        Pdp {
            config: Arc::new(RwLock::new(cfg)),
            policies: Arc::new(RwLock::new(policies)),
        }
    }

    pub fn decide_once(&self, auth_sub: AuthorizationSubscription) -> Value {
        use CombiningAlgorithm::*;

        // Acquire read locks to access the inner data
        let config_guard = self.config.read().expect("Failed to read config lock");
        let policies_guard = self.policies.read().expect("Failed to read policies lock");
        let auth_sub = Arc::new(auth_sub);

        let decisions = policies_guard
            .iter()
            .map(|p| Some(p.evaluate(&auth_sub)))
            .collect::<Box<[Option<AuthorizationDecision>]>>();

        match &config_guard.algorithm {
            DENY_OVERRIDES => deny_overrides(&decisions).into(),
            DENY_UNLESS_PERMIT => deny_unless_permit(&decisions).into(),
            FIRST_APPLICABLE => {
                panic!("First-applicable is not allowed on PDP level for document combination!")
            }
            ONLY_ONE_APPLICABLE => only_one_applicable(&decisions).into(),
            PERMIT_OVERRIDES => permit_overrides(&decisions).into(),
            PERMIT_UNLESS_DENY => permit_unless_deny(&decisions).into(),
        }
    }

    pub fn decide(
        &self,
        auth_sub: AuthorizationSubscription,
    ) -> Pin<Box<(dyn Stream<Item = Value> + Send)>> {
        use CombiningAlgorithm::*;

        // Acquire read locks to access the inner data
        let config_guard = self.config.read().expect("Failed to read config lock");
        let policies_guard = self.policies.read().expect("Failed to read policies lock");
        let auth_sub = Arc::new(auth_sub);

        let policy_streams = policies_guard
            .iter()
            .map(|p| p.evaluate_as_stream(&auth_sub))
            .collect();

        match &config_guard.algorithm {
            DENY_OVERRIDES => Box::pin(
                DecisionCombinedStream::new(policy_streams, deny_overrides).to_json_value(),
            ),
            DENY_UNLESS_PERMIT => Box::pin(
                DecisionCombinedStream::new(policy_streams, deny_unless_permit).to_json_value(),
            ),
            FIRST_APPLICABLE => {
                panic!("First-applicable is not allowed on PDP level for document combination!")
            }
            ONLY_ONE_APPLICABLE => Box::pin(
                DecisionCombinedStream::new(policy_streams, only_one_applicable).to_json_value(),
            ),
            PERMIT_OVERRIDES => Box::pin(
                DecisionCombinedStream::new(policy_streams, permit_overrides).to_json_value(),
            ),
            PERMIT_UNLESS_DENY => Box::pin(
                DecisionCombinedStream::new(policy_streams, permit_unless_deny).to_json_value(),
            ),
        }
    }
}

fn recurse(path: impl AsRef<Path>) -> Vec<PathBuf> {
    let Ok(entries) = read_dir(path) else {
        return vec![];
    };
    entries
        .flatten()
        .flat_map(|entry| {
            let Ok(meta) = entry.metadata() else {
                return vec![];
            };
            if meta.is_dir() {
                return recurse(entry.path());
            }
            if meta.is_file() {
                if let Some(extension) = entry.path().extension()
                    && extension.eq("sapl")
                {
                    return vec![entry.path()];
                }
                return vec![];
            }
            vec![]
        })
        .collect()
}

fn read_all_policies(sapl_files: Vec<PathBuf>) -> Vec<SaplDocument> {
    sapl_files
        .iter()
        .flat_map(|sapl| {
            let Ok(s) = fs::read_to_string(sapl) else {
                return vec![];
            };
            if let Ok(policy) = parse_sapl_file(&s) {
                match policy.validate() {
                    Ok(_) => {
                        return vec![policy];
                    }
                    Err(_) => return vec![],
                }
            }
            vec![]
        })
        .collect()
}
