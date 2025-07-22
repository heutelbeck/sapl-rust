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
use tokio_stream_demo::*;

use futures::pin_mut;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};
use tokio::{select, time::interval};
use tokio_stream::{Stream, StreamExt, wrappers::IntervalStream};

#[tokio::main]
async fn main() {
    tokio::join!(
        stream_expr(),
        simple_stream_1(),
        simple_stream_2(),
        simple_stream_3(),
        hello(),
        merge_stream(),
        filter_merge_stream(),
        combine_eager_solution1(),
        combine_eager_solution2(),
        combine_lazy_solution1(),
    );
}

async fn stream_expr() {
    //let mut e = Ast::Boolean(true).eval();
    //println!("Die Expr lautet wie folgt: {:#?}", e.eval());
    let _e = Ast::Boolean(true).eval();
    let _e = Ast::Integer(42).eval().fuse();
    //let mut e = Ast::BooleanStream(stream1).eval().fuse();
    let stream1 = BooleanIntervalNew::new(Duration::from_millis(1000));
    let stream2 = BooleanIntervalNew::new(Duration::from_millis(10000));
    let _e = stream1.eval_and(stream2);

    let stream1 = IntegerInterval::new(5, Duration::from_millis(1000));
    let stream2 = IntegerInterval::new(10, Duration::from_millis(2000));
    let _e = stream1.eval_ge(stream2);

    let e = TimeSecInterval::default();
    let mut e = e.eval_ge(once(Val::Integer(30)));

    while let Some(v) = e.next().await {
        println!("evaluation stream: {v:#?}");
    }
}

async fn combine_lazy_solution1() {
    let stream1 = BooleanInterval::new(Duration::from_millis(1000));
    let stream2 = BooleanInterval::new(Duration::from_millis(5000));
    let mut stream = stream1.combine_lazy(stream2);

    while let Some(v) = stream.next().await {
        println!("stream_combine_lazy #1 = {v:?}");
    }
}

async fn combine_eager_solution1() {
    let stream1 = BooleanInterval::new(Duration::from_millis(1000));
    let stream2 = BooleanInterval::new(Duration::from_millis(5000));
    let mut stream = stream1.combine_eager(stream2);

    while let Some(v) = stream.next().await {
        println!("stream_combine_eager #1 = {v:?}");
    }
}

async fn combine_eager_solution2() {
    let stream1 = BooleanInterval::new(Duration::from_millis(1000));
    let stream2 = BooleanInterval::new(Duration::from_millis(5000));
    let stream = both_true_stream(stream1, stream2).await;

    pin_mut!(stream);

    while (stream.next().await).is_some() {
        println!("stream_combine_eager #2 = Both are true");
    }
}

async fn simple_stream_1() {
    let b = AtomicBool::new(false);
    let mut stream = IntervalStream::new(interval(Duration::from_secs(1)))
        .map(move |_| b.fetch_not(Ordering::SeqCst));

    while let Some(v) = stream.next().await {
        println!("simple_stream = {v:?}");
    }
}

async fn simple_stream_2() {
    let mut stream = BooleanInterval::new(Duration::from_millis(2500));

    while let Some(v) = stream.next().await {
        println!("simple_stream 2500ms = {v:?}");
    }
}

async fn simple_stream_3() {
    let mut stream = BooleanInterval::new(Duration::from_millis(5000));

    while let Some(v) = stream.next().await {
        println!("simple_stream 5000ms = {v:?}");
    }
}

async fn filter_merge_stream() {
    let stream1 = BooleanInterval::new(Duration::from_millis(250));
    let stream2 = BooleanInterval::new(Duration::from_millis(500));
    let mut stream = stream1.filter(|x| *x).merge(stream2.filter(|x| *x));

    while let Some(v) = stream.next().await {
        println!("filter_merge_stream = {v:?}");
    }
}

async fn merge_stream() {
    let mut rx = BooleanInterval::new(Duration::from_millis(2500))
        .merge(BooleanInterval::new(Duration::from_millis(5000)));

    while let Some(v) = rx.next().await {
        println!("merge_stream = {v:?}");
    }
}

async fn hello() {
    println!("Hallo async");
}

async fn both_true_stream<
    A: Stream<Item = bool> + std::marker::Unpin,
    B: Stream<Item = bool> + std::marker::Unpin,
>(
    a: A,
    b: B,
) -> impl Stream<Item = ()> {
    async_stream::stream! {
            pin_mut!(a);
            pin_mut!(b);
            let mut aa = false;
            let mut bb = false;
            loop {
                select! {
                    v = a.next() => aa = v.unwrap(),
                    v = b.next() => bb = v.unwrap(),
                    else => break,
                }
                if aa && bb {
                    yield ();
            }
        }
    }
}
