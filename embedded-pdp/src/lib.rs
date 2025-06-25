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
mod combining_algorithm;
mod file_reader;
mod pdp_config;

//Re-export types from pest-parser that users need
pub use crate::file_reader::*;
pub use sapl_core::authorization_subscription::AuthorizationSubscription;

use crate::{
    combining_algorithm::{DenyUnlessPermit, PolicyDocumentCombiningAlgorithm},
    pdp_config::PdpConfig,
};
use futures::Stream;
use sapl_core::{Decision, SaplDocument, parse_sapl_file};
use serde_json::{Value, json};
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
        use PolicyDocumentCombiningAlgorithm::*;

        // Acquire read locks to access the inner data
        // TODO check if files gets updated how to handle this?
        let config_guard = self.config.read().expect("Failed to read config lock");
        let policies_guard = self.policies.read().expect("Failed to read policies lock");

        match &config_guard.algorithm {
            DENY_UNLESS_PERMIT => policies_guard.iter().deny_unless_permit(&auth_sub),
            PERMIT_UNLESS_DENY => policies_guard.iter().permit_unless_deny(&auth_sub),
            others => unimplemented!("The pdp alogrithm {:#?} is not yet implemented.", others),
        }
    }

    pub fn decide(
        &self,
        auth_sub: AuthorizationSubscription,
    ) -> Pin<Box<(dyn Stream<Item = Value> + Send)>> {
        use PolicyDocumentCombiningAlgorithm::*;

        // Acquire read locks to access the inner data
        // TODO check if files gets updated how to handle this?
        let config_guard = self.config.read().expect("Failed to read config lock");
        let policies_guard = self.policies.read().expect("Failed to read policies lock");

        let policy_streams = policies_guard
            .iter()
            .map(|p| p.evaluate_as_stream(&auth_sub))
            .collect();

        match &config_guard.algorithm {
            DENY_UNLESS_PERMIT => Box::pin(DenyUnlessPermit::new(policy_streams)),
            _ => unimplemented!(),
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
                if let Some(extension) = entry.path().extension() {
                    if extension.eq("sapl") {
                        return vec![entry.path()];
                    }
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

trait PolicyDecisionAlgorithmExt<'a> {
    fn deny_unless_permit(self, auth_sub: &AuthorizationSubscription) -> Value;
    fn permit_unless_deny(self, auth_sub: &AuthorizationSubscription) -> Value;
}

impl<'a, I> PolicyDecisionAlgorithmExt<'a> for I
where
    I: Iterator<Item = &'a SaplDocument>,
{
    fn deny_unless_permit(self, auth_sub: &AuthorizationSubscription) -> Value {
        for policy in self {
            if Decision::Permit == policy.evaluate(auth_sub) {
                return json!({"decision": "PERMIT"});
            }
        }

        json!({"decision": "DENY"})
    }

    fn permit_unless_deny(self, auth_sub: &AuthorizationSubscription) -> Value {
        for policy in self {
            if Decision::Deny == policy.evaluate(auth_sub) {
                return json!({"decision": "DENY"});
            }
        }

        json!({"decision": "PERMIT"})
    }
}
