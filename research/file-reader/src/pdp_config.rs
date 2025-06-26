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
use std::{
    fs::{self, File},
    path::Path,
};

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize)]
pub enum PolicyDocumentCombiningAlgorithm {
    DENY_UNLESS_PERMIT,
    PERMIT_UNLESS_DENY,
    ONLY_ONE_APPLICABLE,
    DENY_OVERRIDES,
    PERMIT_OVERRIDES,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdpConfig {
    pub algorithm: PolicyDocumentCombiningAlgorithm,
    _variables: Option<serde_json::Value>,
}

impl Default for PdpConfig {
    fn default() -> Self {
        PdpConfig {
            algorithm: PolicyDocumentCombiningAlgorithm::DENY_UNLESS_PERMIT,
            _variables: None,
        }
    }
}

impl PdpConfig {
    pub fn new() -> Self {
        let pdp_json_file_path = Path::new("policies/pdp.json");
        match fs::exists(pdp_json_file_path) {
            Ok(_) => {
                let file = File::open(pdp_json_file_path);
                match file {
                    Ok(f) => {
                        serde_json::from_reader(f).expect("error while reading or parsing pdp.json")
                    }
                    Err(_) => PdpConfig::default(),
                }
            }
            Err(_) => PdpConfig::default(),
        }
    }

    pub fn update_algorithm(&mut self) {
        self.algorithm = PdpConfig::new().algorithm;
    }
}
