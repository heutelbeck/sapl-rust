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

mod pdp_config;

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use pdp_config::PdpConfig;
use std::{
    path::Path,
    sync::{Arc, RwLock, mpsc::channel},
    thread,
    time::{Duration, Instant},
};

fn main() {
    let config = PdpConfig::new();
    let shared_config = Arc::new(RwLock::new(config));

    {
        let config_reader = shared_config.read().unwrap();
        println!("Der aktuelle Algorithmus lautet: {config_reader:#?}");
    }

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

    watcher
        .watch(Path::new("policies/"), RecursiveMode::Recursive)
        .unwrap();

    let mut last_event_time = Instant::now();
    let debounce_time = Duration::from_millis(250);

    for e in rx {
        match e {
            Ok(e) => policy_folder_update(
                shared_config.clone(),
                debounce_time,
                &mut last_event_time,
                e,
            ),
            Err(e) => eprintln!("{e:?}"),
        }
    }
}

fn policy_folder_update(
    shared_config: Arc<RwLock<PdpConfig>>,
    debounce_time: Duration,
    last_event_time: &mut Instant,
    e: Event,
) {
    for p in e.paths {
        match p {
            p if p.ends_with(".sapl") => println!("SAPL Policies update needed"),
            p if p.ends_with("pdp.json") => {
                update_pdp_config(shared_config.clone(), debounce_time, last_event_time)
            }
            _ => (),
        }
    }
}

fn update_pdp_config(
    shared_config: Arc<RwLock<PdpConfig>>,
    debounce_time: Duration,
    last_event_time: &mut Instant,
) {
    let now = Instant::now();
    if now.duration_since(*last_event_time) > debounce_time {
        println!("Update pdp.json");
        thread::sleep(Duration::from_secs(1));
        let mut config = shared_config.write().unwrap();
        config.update_algorithm();
        *last_event_time = Instant::now();

        println!("Der aktuelle Algorithmus nach dem Update lautet: {config:#?}");
    }
}
