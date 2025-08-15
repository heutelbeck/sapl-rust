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

use crate::pdp_config::PdpConfig;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use sapl_core::{SaplDocument, parse_sapl_file};
use std::{
    fs::{self, read_dir},
    path::{Path, PathBuf},
    sync::{Arc, RwLock, mpsc::channel},
    thread,
    time::{Duration, Instant},
};

pub async fn file_reader(
    file_path: &Path,
    shared_config: Arc<RwLock<PdpConfig>>,
    shared_policies: Arc<RwLock<Vec<SaplDocument>>>,
) {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

    watcher
        .watch(Path::new(file_path), RecursiveMode::Recursive)
        .unwrap();

    let mut last_event_time = Instant::now();
    let debounce_time = Duration::from_millis(250);

    for e in rx {
        match e {
            Ok(e) => folder_update(
                file_path,
                shared_config.clone(),
                shared_policies.clone(),
                debounce_time,
                &mut last_event_time,
                e,
            ),
            Err(e) => eprintln!("{e:?}"),
        }
    }
}

fn folder_update(
    file_path: &Path,
    shared_config: Arc<RwLock<PdpConfig>>,
    shared_policies: Arc<RwLock<Vec<SaplDocument>>>,
    debounce_time: Duration,
    last_event_time: &mut Instant,
    e: Event,
) {
    for p in e.paths {
        // Update pdp configuration
        if p.ends_with("pdp.json") {
            update_pdp_config(&p, shared_config.clone(), debounce_time, last_event_time);
            return;
        }

        // Update sapl policies
        match p.extension() {
            Some(ext) if ext == "sapl" => update_policies(
                file_path,
                shared_policies.clone(),
                debounce_time,
                last_event_time,
            ),
            _ => {}
        }
    }
}

fn update_pdp_config(
    file: &Path,
    shared_config: Arc<RwLock<PdpConfig>>,
    debounce_time: Duration,
    last_event_time: &mut Instant,
) {
    let now = Instant::now();
    if now.duration_since(*last_event_time) > debounce_time {
        println!("Update pdp.json");
        thread::sleep(Duration::from_secs(1));
        let mut config = shared_config.write().unwrap();
        config.update_algorithm(file);
        *last_event_time = Instant::now();
    }
}

fn update_policies(
    path: &Path,
    shared_policies: Arc<RwLock<Vec<SaplDocument>>>,
    debounce_time: Duration,
    last_event_time: &mut Instant,
) {
    let now = Instant::now();
    if now.duration_since(*last_event_time) > debounce_time {
        println!("Update policies: {path:?}");
        thread::sleep(Duration::from_secs(1));
        let mut policies = shared_policies.write().unwrap();
        *policies = read_all_policies(recurse(path));
        *last_event_time = Instant::now();
    }
}

fn read_all_policies(sapl_files: Vec<PathBuf>) -> Vec<SaplDocument> {
    sapl_files
        .iter()
        .flat_map(|sapl| {
            let Ok(s) = fs::read_to_string(sapl) else {
                return vec![];
            };
            if let Ok(policy) = parse_sapl_file(&s) {
                return vec![policy];
            }
            vec![]
        })
        .collect()
}

fn recurse(path: impl AsRef<Path>) -> Vec<PathBuf> {
    let Ok(entries) = read_dir(path) else {
        return vec![];
    };
    entries
        .flatten()
        .flat_map(|entry| {
            let Ok(meta) = entry.metadata() else {
                return vec![];
            };
            if meta.is_dir() {
                return recurse(entry.path());
            }
            if meta.is_file() {
                if let Some(extension) = entry.path().extension()
                    && extension.eq("sapl")
                {
                    return vec![entry.path()];
                }

                return vec![];
            }
            vec![]
        })
        .collect()
}
