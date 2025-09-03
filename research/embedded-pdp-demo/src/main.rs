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

use embedded_pdp::Pdp;
use futures::StreamExt;
use sapl_core::{authorization_subscription::AuthorizationSubscription, parse_sapl_file};
use std::{path::Path, sync::Arc};

#[tokio::main]
async fn main() {
    tokio::join!(stream_debug_pdp(), stream_debug());
}

async fn stream_debug_pdp() {
    let pdp = Pdp::new(
        Some(Path::new(
            "/home/stefan/Dokumente/uni/Thesis/sapl-rust/sapl-server-rs/policies/pdp.json",
        )),
        Some(Path::new(
            "/home/stefan/Dokumente/uni/Thesis/sapl-rust/sapl-server-rs/policies/",
        )),
    );

    let mut decision = pdp.decide(auth_sub());

    while let Some(d) = decision.next().await {
        println!("{d:?}");
    }
}

async fn stream_debug() {
    let auth_sub = Arc::new(auth_sub());

    let sapl_document =
        parse_sapl_file("policy \"time change demo\" permit where time.secondOf(<time.now>) < 20;")
            .unwrap();

    let mut stream_sapl = sapl_document.evaluate_as_stream(&auth_sub);

    while let Some(v) = stream_sapl.next().await {
        println!("{v:?}");
    }
}

pub fn auth_sub() -> AuthorizationSubscription {
    let json: AuthorizationSubscription =
        serde_json::from_str(r#"{ "subject": "WILLI", "action": "read", "resource": "something"}"#)
            .unwrap();
    json
}
