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

extern crate sapl_core;
use sapl_core::parse_sapl_file;

use rstest::rstest;
use std::fs::read_to_string;
use std::path::PathBuf;

#[rstest]
fn test_grammer_and_parser_with_policy_file(#[files("tests/policy_files/*.sapl")] policy: PathBuf) {
    let s = read_to_string(policy);
    assert!(s.is_ok());
    assert!(parse_sapl_file(&s.unwrap()).is_ok());
}
