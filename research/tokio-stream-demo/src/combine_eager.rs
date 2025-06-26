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
use core::task::{Context, Poll};
use pin_project_lite::pin_project;

pin_project! {
    pub struct CombineEager<T, U> {
        #[pin]
        a: T,
        #[pin]
        b: U,
        // save the last stream results
        first_result: bool,
        second_result: bool,
    }
}

impl<T, U> CombineEager<T, U> {
    pub(super) fn new(a: T, b: U) -> CombineEager<T, U>
    where
        T: Stream<Item = bool>,
        U: Stream<Item = T::Item>,
    {
        CombineEager {
            a,
            b,
            first_result: false,
            second_result: false,
        }
    }
}

impl<T, U> Stream for CombineEager<T, U>
where
    T: Stream<Item = bool>,
    U: Stream<Item = T::Item>,
{
    type Item = T::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T::Item>> {
        loop {
            use Poll::*;

            match (
                self.as_mut().project().a.poll_next(cx),
                self.as_mut().project().b.poll_next(cx),
            ) {
                (Ready(Some(val1)), Ready(Some(val2))) => {
                    *self.as_mut().project().first_result = val1;
                    *self.as_mut().project().second_result = val2;

                    if val1 && val2 {
                        return Ready(Some(true));
                    }
                }
                (Ready(Some(_)), Ready(None)) => {
                    unreachable!("How to deal with the end of a stream?");
                }
                (Ready(Some(val1)), Pending) => {
                    *self.as_mut().project().first_result = val1;

                    if *self.as_mut().project().second_result && val1 {
                        return Ready(Some(true));
                    }
                }
                (Ready(None), Ready(Some(_))) => {
                    unreachable!("How to deal with the end of a stream?");
                }
                (Ready(None), Ready(None)) => return Ready(None),
                (Ready(None), Pending) => {
                    unreachable!("How to deal with the end of a stream?");
                }
                (Pending, Ready(Some(val2))) => {
                    if *self.as_mut().project().first_result && val2 {
                        return Ready(Some(true));
                    }
                }
                (Pending, Ready(None)) => {
                    unreachable!("How to deal with the end of a stream?");
                }
                (Pending, Pending) => return Pending,
            }
        }
    }
}
