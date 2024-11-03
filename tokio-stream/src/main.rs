/*
    Copyright 2024 Stefan Weng

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

mod boolean_stream;
mod delay;
mod reduce;

use boolean_stream::BooleanInterval;
use std::time::Duration;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    tokio::join!(
        simple_stream(),
        hello(),
        merge_stream(),
        filter_merge_stream(),
        reduce(),
    );
}

async fn reduce() {
    let mut stream = BooleanInterval::new(Duration::from_secs(1)).reduce();

    while let Some(s) = stream.next().await {
        println!("reduce = {:?}", s);
    }
}

async fn simple_stream() {
    let mut stream = BooleanInterval::new(Duration::from_secs(1)).filter(|e| *e);

    while let Some(v) = stream.next().await {
        println!("simple_stream = {:?}", v);
    }
}

async fn filter_merge_stream() {
    let stream1 = BooleanInterval::new(Duration::from_millis(250));
    let stream2 = BooleanInterval::new(Duration::from_millis(500));
    let mut stream = stream1.filter(|x| *x).merge(stream2.filter(|x| *x));

    while let Some(v) = stream.next().await {
        println!("filter_merge_stream = {:?}", v);
    }
}

async fn merge_stream() {
    let mut rx = BooleanInterval::new(Duration::from_millis(250))
        .merge(BooleanInterval::new(Duration::from_millis(500)));

    while let Some(v) = rx.next().await {
        println!("merge_stream = {:?}", v);
    }
}

async fn hello() {
    println!("Hallo async");
}
