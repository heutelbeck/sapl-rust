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

use embedded_pdp::{AuthorizationSubscription, Pdp};
use futures::StreamExt;
use std::{path::Path, sync::Arc};

#[tokio::main]
async fn main() {
    let pdp = Arc::new(Pdp::new(
        Some(Path::new("../../sapl-server-rs/policies/pdp.json")),
        Some(Path::new("../../sapl-server-rs/policies/")),
    ));

    decide_once(pdp.clone());
    tokio::join!(decide(pdp));
}

fn decide_once(pdp: Arc<Pdp>) {
    println!("Pdp decide-once result: {:#?}", pdp.decide_once(auth_sub()));
}

async fn decide(pdp: Arc<Pdp>) {
    let mut decision = pdp.decide(auth_sub());

    while let Some(d) = decision.next().await {
        println!("{d:?}");
    }
}

pub fn auth_sub() -> AuthorizationSubscription {
    let json: AuthorizationSubscription =
        serde_json::from_str(r#"{ "subject": "WILLI", "action": "read", "resource": "something"}"#)
            .unwrap();
    json
}
