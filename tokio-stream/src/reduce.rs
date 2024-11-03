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

use tokio_stream::Stream;

use core::pin::Pin;
use core::task::{ready, Context, Poll};
use pin_project_lite::pin_project;

pin_project! {
    pub struct Reduce<St> {
        #[pin]
        stream: St,
        skip: bool,
    }
}

impl<St> Reduce<St> {
    pub(super) fn new(stream: St) -> Reduce<St>
    where
        St: Stream,
    {
        Reduce { stream, skip: true }
    }
}

impl<T> Stream for Reduce<T>
where
    T: Stream,
{
    type Item = T::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T::Item>> {
        loop {
            match ready!(self.as_mut().project().stream.poll_next(cx)) {
                Some(e) => {
                    if *self.as_mut().project().skip {
                        *self.as_mut().project().skip = !*self.as_mut().project().skip;
                        return Poll::Ready(Some(e));
                    }
                    *self.as_mut().project().skip = !*self.as_mut().project().skip;
                }
                None => return Poll::Ready(None),
            }
        }
    }
}
