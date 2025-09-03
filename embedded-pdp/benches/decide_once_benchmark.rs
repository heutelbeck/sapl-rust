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

use criterion::{Criterion, criterion_group, criterion_main};
use embedded_pdp::{AuthorizationSubscription, Pdp};
use std::hint::black_box;
use std::path::Path;

fn criterion_decide_once_benchmark(c: &mut Criterion) {
    let pdp = Pdp::new(
        Some(Path::new("../sapl-server-rs/policies/pdp.json")),
        Some(Path::new("../sapl-server-rs/policies/")),
    );

    c.bench_function("decide_once", |b| {
        b.iter(|| {
            let auth_sub: AuthorizationSubscription = serde_json::from_str(
                r#"{ "subject": "WILLI", "action": "read", "resource": "something"}"#,
            )
            .unwrap();
            black_box(pdp.decide_once(black_box(auth_sub)))
        })
    });
}

criterion_group!(benches, criterion_decide_once_benchmark);
criterion_main!(benches);
