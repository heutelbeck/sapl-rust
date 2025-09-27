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
use sapl_core::authorization_subscription::AuthorizationSubscription;
use sapl_core::parse_sapl_file;

pub fn auth_sub() -> AuthorizationSubscription {
    let auth_sub: AuthorizationSubscription =
        serde_json::from_str(r#"{ "subject": "WILLI", "action": "read", "resource": "something"}"#)
            .unwrap();
    auth_sub
}

#[test]
fn simple_evaluate_to_deny() {
    let policy = parse_sapl_file("policy \"policy 1\" deny");
    assert!(policy.is_ok());
    let result = policy.unwrap().evaluate(&auth_sub());
    assert_eq!("Decision: DENY".to_string(), result.to_string());
}

#[test]
fn simple_evaluate_to_permit() {
    let policy = parse_sapl_file("policy \"policy 1\" permit");
    assert!(policy.is_ok());
    let result = policy.unwrap().evaluate(&auth_sub());
    assert_eq!("Decision: PERMIT".to_string(), result.to_string());
}

#[test]
fn simple_evaluate_to_not_applicable() {
    let policy = parse_sapl_file("policy \"policy 1\" permit action == \"test\"");
    assert!(policy.is_ok());
    let result = policy.unwrap().evaluate(&auth_sub());
    assert_eq!("Decision: NOT_APPLICABLE".to_string(), result.to_string());
}

#[test]
fn evaluate_with_variable_assignement() {
    let policy =
        parse_sapl_file("policy \"var\" permit where var test = \"read\"; action == test;");
    assert!(policy.is_ok());
    let result = policy.unwrap().evaluate(&auth_sub());
    assert_eq!("Decision: PERMIT".to_string(), result.to_string());
}

#[test]
fn evaluate_lazy_and_permit() {
    let policy =
        parse_sapl_file("policy \"var\" permit where subject == \"WILLI\" && action == \"read\";");
    assert!(policy.is_ok());
    let result = policy.unwrap().evaluate(&auth_sub());
    assert_eq!("Decision: PERMIT".to_string(), result.to_string());
}

#[test]
fn evaluate_lazy_and_not_applicable() {
    let policy =
        parse_sapl_file("policy \"var\" permit where subject == \"ILLIW\" && action == \"read\";");
    assert!(policy.is_ok());
    let result = policy.unwrap().evaluate(&auth_sub());
    assert_eq!("Decision: NOT_APPLICABLE".to_string(), result.to_string());
}

#[test]
fn evaluate_lazy_or_permit() {
    let policy =
        parse_sapl_file("policy \"var\" permit where subject == \"WILLI\" || action == \"demo\";");
    assert!(policy.is_ok());
    let result = policy.unwrap().evaluate(&auth_sub());
    assert_eq!("Decision: PERMIT".to_string(), result.to_string());
}

#[test]
fn evaluate_lazy_or_not_applicable() {
    let policy =
        parse_sapl_file("policy \"var\" permit where subject == \"WIL\" || action == \"demo\";");
    assert!(policy.is_ok());
    let result = policy.unwrap().evaluate(&auth_sub());
    assert_eq!("Decision: NOT_APPLICABLE".to_string(), result.to_string());
}
