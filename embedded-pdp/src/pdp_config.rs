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

use sapl_core::CombiningAlgorithm;
use serde::Deserialize;
use std::{fs::File, path::Path};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdpConfig {
    pub algorithm: CombiningAlgorithm,
    _variables: Option<serde_json::Value>,
}

impl Default for PdpConfig {
    fn default() -> Self {
        PdpConfig {
            algorithm: CombiningAlgorithm::DENY_UNLESS_PERMIT,
            _variables: None,
        }
    }
}

impl PdpConfig {
    pub fn new(p: &Path) -> Self {
        match p.try_exists() {
            Ok(true) => {
                let file = File::open(p).unwrap();
                match serde_json::from_reader::<std::fs::File, PdpConfig>(file) {
                    Ok(config) => {
                        if config.algorithm == CombiningAlgorithm::FIRST_APPLICABLE {
                            panic!(
                                "Fist-applicable is not allowed on PDP level for document combination!"
                            );
                        }
                        config
                    }
                    Err(e) => {
                        println!("Error while reading pdp configuration: {e:#?}");
                        PdpConfig::default()
                    }
                }
            }
            _ => {
                println!("Pdp configuration policies/pdp.json not found");
                PdpConfig::default()
            }
        }
    }

    pub fn update_algorithm(&mut self, path: &Path) {
        self.algorithm = match PdpConfig::new(path).algorithm {
            CombiningAlgorithm::FIRST_APPLICABLE => {
                panic!("Fist-applicable is not allowed on PDP level for document combination!")
            }
            others => others,
        };
    }
}
