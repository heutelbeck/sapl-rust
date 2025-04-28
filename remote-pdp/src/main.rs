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

use embedded_pdp::{AuthorizationSubscription, Pdp, file_reader};
use rocket::{
    Orbit, Rocket,
    fairing::{Fairing, Info, Kind},
    serde::json::{Json, Value, json},
    tokio,
};
use std::path::Path;

#[macro_use]
extern crate rocket;

struct FileWatcher;

#[rocket::async_trait]
impl Fairing for FileWatcher {
    fn info(&self) -> Info {
        Info {
            name: "File Watcher",
            kind: Kind::Liftoff,
        }
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        if let Some(pdp) = rocket.state::<Pdp>() {
            let policy_folder = Path::new("policies/");

            let shared_config = pdp.config.clone();
            let shared_policies = pdp.policies.clone();

            tokio::spawn(async move {
                file_reader(policy_folder, shared_config, shared_policies).await;
            });
        }
    }
}

#[post("/api/pdp/decideOnce", format = "json", data = "<auth_sub>")]
fn decide_once(auth_sub: Json<AuthorizationSubscription>, pdp: &rocket::State<Pdp>) -> Value {
    pdp.decide_once(auth_sub.into_inner())
}

#[get("/api/pdp/health")]
fn health() -> Value {
    json!({"status": "up", "version": env!("CARGO_PKG_VERSION")})
}

#[launch]
fn entry_point() -> _ {
    let pdp = Pdp::new(
        Some(Path::new("policies/pdp.json")),
        Some(Path::new("policies/")),
    );

    rocket::build()
        .manage(pdp)
        .attach(FileWatcher)
        .mount("/", routes![decide_once, health])
}
