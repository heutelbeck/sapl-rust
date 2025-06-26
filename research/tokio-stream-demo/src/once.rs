use crate::val::Val;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio_stream::Stream;

// Function that creates a one-shot stream from Val
pub fn once(value: Val) -> impl Stream<Item = Val> {
    struct OnceStream {
        value: Option<Val>,
    }

    impl Stream for OnceStream {
        type Item = Val;

        fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Poll::Ready(self.value.take())
        }
    }

    OnceStream { value: Some(value) }
}
