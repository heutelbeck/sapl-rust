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
use sapl_core::{
    authorization_subscription::AuthorizationSubscription,
    parse_sapl_file,
    pip::time::time_policy_information_point::Time,
    stream_sapl::{StreamSapl, once_val},
};
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

    let auth_sub = AuthorizationSubscription::new_example_subscription1();

    let mut decision = pdp.decide(auth_sub);

    while let Some(d) = decision.next().await {
        println!("{d:?}");
    }
}

async fn stream_debug() {
    let stream1 = once_val(sapl_core::Val::Integer(20));
    let stream = Time::new(1000).eval_seconds_of();

    let mut _new_stream = stream.eval_le(stream1);

    let auth_sub = Arc::new(AuthorizationSubscription::new_example_subscription1());

    let sapl_document =
        parse_sapl_file("policy \"time change demo\" permit where time.secondOf(<time.now>) < 20;")
            .unwrap();

    let mut stream_sapl = sapl_document.evaluate_as_stream(&auth_sub);

    while let Some(v) = stream_sapl.next().await {
        println!("{v:?}");
    }
}
