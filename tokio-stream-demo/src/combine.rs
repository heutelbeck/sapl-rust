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

use crate::Val;

use tokio_stream::Stream;

use core::pin::Pin;
use core::task::{Context, Poll};
use pin_project_lite::pin_project;

pin_project! {
    pub struct Combine<T, U> {
        #[pin]
        a: T,
        #[pin]
        b: U,
        // save the last stream results
        first_result: Val,
        second_result: Val,
    }
}

impl<T, U> Combine<T, U> {
    pub(super) fn new(a: T, b: U) -> Combine<T, U>
    where
        T: Stream<Item = Val>,
        U: Stream<Item = T::Item>,
    {
        Combine {
            a,
            b,
            first_result: Val::None,
            second_result: Val::None,
        }
    }
}

impl<T, U> Stream for Combine<T, U>
where
    T: Stream<Item = Val>,
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

                    return Ready(Some(Val::Boolean(val1.eq(&val2))));
                }
                (Ready(Some(_)), Ready(None)) => {
                    unreachable!("How to deal with the end of a stream?");
                }
                (Ready(Some(_)), Pending) => {
                    unreachable!("How to deal with the end of a stream?");
                }
                (Ready(None), Ready(Some(_))) => {
                    unreachable!("How to deal with the end of a stream?");
                }
                (Ready(None), Ready(None)) => return Ready(None),
                (Ready(None), Pending) => {
                    unreachable!("How to deal with the end of a stream?");
                }
                (Pending, Ready(Some(_))) => {
                    unreachable!("How to deal with the end of a stream?");
                }
                (Pending, Ready(None)) => {
                    unreachable!("How to deal with the end of a stream?");
                }
                (Pending, Pending) => return Pending,
            }
        }
    }
}
