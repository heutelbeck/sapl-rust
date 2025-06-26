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

use core::{
    pin::Pin,
    task::{Context, Poll},
};
use pin_project_lite::pin_project;
use tokio_stream::Stream;

pin_project! {
    pub struct CombineLazy<T, U> {
        #[pin]
        a: T,
        #[pin]
        b: U,
        // save the last stream results
        first_result: bool,
        second_result: bool,
    }
}

impl<T, U> CombineLazy<T, U> {
    pub(super) fn new(a: T, b: U) -> CombineLazy<T, U>
    where
        T: Stream<Item = bool>,
        U: Stream<Item = T::Item>,
    {
        CombineLazy {
            a,
            b,
            first_result: false,
            second_result: false,
        }
    }
}

impl<T, U> Stream for CombineLazy<T, U>
where
    T: Stream<Item = bool>,
    U: Stream<Item = T::Item>,
{
    type Item = T::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T::Item>> {
        loop {
            use Poll::*;

            let mut pending = false;

            match self.as_mut().project().a.poll_next(cx) {
                Ready(Some(val)) => {
                    *self.as_mut().project().first_result = val;

                    if val && *self.as_mut().project().second_result {
                        return Ready(Some(true));
                    }
                }
                Ready(None) => return Ready(None),
                Pending => pending = true,
            }

            if *self.as_mut().project().first_result {
                match self.as_mut().project().b.poll_next(cx) {
                    Ready(Some(val)) => {
                        *self.as_mut().project().second_result = val;
                        if val && *self.as_mut().project().first_result {
                            return Ready(Some(true));
                        }
                    }
                    Ready(None) => return Ready(None),
                    Pending => pending = true,
                }
            }

            if pending {
                return Pending;
            }
        }
    }
}
