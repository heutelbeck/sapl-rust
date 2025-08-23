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
    Build, Orbit, Rocket,
    fairing::{Fairing, Info, Kind},
    response::stream::{Event, EventStream},
    serde::json::{Json, Value, json},
    tokio,
};
use std::path::Path;

#[macro_use]
extern crate rocket;

#[allow(dead_code)]
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

#[post("/api/pdp/decide", format = "json", data = "<auth_sub>")]
fn decide(auth_sub: Json<AuthorizationSubscription>, pdp: &rocket::State<Pdp>) -> EventStream![] {
    let stream = pdp.decide(auth_sub.into_inner());

    EventStream! {
        for await decision in stream {
            yield Event::json(&decision);
        }
    }
}

#[post("/api/pdp/decide-once", format = "json", data = "<auth_sub>")]
fn decide_once(auth_sub: Json<AuthorizationSubscription>, pdp: &rocket::State<Pdp>) -> Value {
    pdp.decide_once(auth_sub.into_inner())
}

#[get("/api/pdp/health")]
fn health() -> Value {
    json!({"status": "up", "version": env!("CARGO_PKG_VERSION")})
}

#[launch]
fn rocket() -> Rocket<Build> {
    let pdp = Pdp::new(
        Some(Path::new("policies/pdp.json")),
        Some(Path::new("policies/")),
    );

    #[allow(unused_mut)]
    let mut rocket_builder = rocket::build().manage(pdp);

    // FileWatcher only in production used and not in tests
    #[cfg(not(test))]
    {
        rocket_builder = rocket_builder.attach(FileWatcher);
    }

    rocket_builder.mount("/", routes![decide, decide_once, health])
}

#[cfg(test)]
mod test {
    use super::*;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn health() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(health())).dispatch();
        assert_eq!(response.status(), Status::Ok);

        assert_eq!(
            response.into_string().unwrap(),
            "{\"status\":\"up\",\"version\":\"0.1.0\"}"
        );
    }

    #[test]
    fn decide_once_status_not_found() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(uri!(decide_once())).dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn decide_once_deny() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!(decide_once()))
            .header(ContentType::JSON)
            .body("{\"subject\":\"WILLI\",\"action\":\"write\", \"resource\":\"something\"}")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        assert_eq!(response.into_string().unwrap(), "{\"decision\":\"DENY\"}");
    }

    #[test]
    fn decide_once_permit_policy2() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!(decide_once()))
            .header(ContentType::JSON)
            .body("{\"subject\":\"WILLI\",\"action\": {\"java\": { \"name\": \"findById\"}}, \"resource\":\"something\"}")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        assert_eq!(response.into_string().unwrap(), "{\"decision\":\"PERMIT\"}");
    }

    #[test]
    fn decide_once_deny_simpsons_policy_set() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!(decide_once()))
            .header(ContentType::JSON)
            .body("{\"subject\":\"bs@simpsons.com\",\"action\": \"read\", \"resource\":\"something\"}")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        assert_eq!(response.into_string().unwrap(), "{\"decision\":\"DENY\"}");
    }

    #[test]
    fn decide_once_permit_transform_demo_set() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!(decide_once()))
            .header(ContentType::JSON)
            .body("{\"subject\":{\"name\": \"anonymous\"},\"action\": {\"http\": { \"contextPath\": \"/changedstring\", \"requestedURI\": \"http://localhost:8080/numbers\"}}, \"resource\":\"something\"}")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        assert_eq!(
            response.into_string().unwrap(),
            "{\"decision\":\"PERMIT\",\"resource\":\"***something***\",\"obligation\":{\"type\":\"logAccess\",\"message\":\"User anonymous has accessed: http://localhost:8080/numbers\"}}"
        );
    }

    #[test]
    fn decide_once_permit_obligation_advice_demo_set() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!(decide_once()))
            .header(ContentType::JSON)
            .body("{\"subject\":{\"name\": \"anonymous\"},\"action\": {\"http\": { \"contextPath\": \"/numbers\", \"requestedURI\": \"http://localhost:8080/numbers\"}}, \"resource\":\"something\"}")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        assert_eq!(
            response.into_string().unwrap(),
            "{\"decision\":\"PERMIT\",\"obligation\":{\"type\":\"sendEmail\",\"recipient\":\"Adam Admin <admin@example.com>\",\"subject\":\"Data was accessed.\",\"message\":\"Administrator anonymous has accessed: http://localhost:8080/numbers\"},\"advice\":{\"type\":\"logAccess\",\"message\":\"User anonymous has accessed: http://localhost:8080/numbers\"}}"
        );
    }
}
