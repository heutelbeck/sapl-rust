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

use core::{
    pin::Pin,
    task::{Context, Poll},
};
use pin_project_lite::pin_project;
use tokio_stream::Stream;

use crate::Decision;
use crate::Entitlement;

pin_project! {
    pub struct CombineExpr<T, U> {
        #[pin]
        a: T,
        #[pin]
        b: U,
        // save the last stream results
        target: bool,
        condition: bool,
        e: Entitlement,
    }
}

impl<T, U> CombineExpr<T, U> {
    pub(super) fn new(a: T, b: U, e: Entitlement) -> CombineExpr<T, U>
    where
        T: Stream<Item = bool>,
        U: Stream<Item = T::Item>,
    {
        CombineExpr {
            a,
            b,
            target: false,
            condition: false,
            e,
        }
    }
}

impl<T, U> Stream for CombineExpr<T, U>
where
    T: Stream<Item = bool>,
    U: Stream<Item = T::Item>,
{
    type Item = Decision;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Decision>> {
        use Poll::*;

        match self.as_mut().project().a.poll_next(cx) {
            Ready(Some(val)) => {
                *self.as_mut().project().target = val;

                return Ready(Some(if val && *self.as_mut().project().condition {
                    Decision::entitlement(&self.e)
                } else {
                    Decision::NotApplicable
                }));
            }
            Ready(None) => return Ready(None),
            Pending => {}
        }

        if *self.as_mut().project().target {
            match self.as_mut().project().b.poll_next(cx) {
                Ready(Some(val)) => {
                    *self.as_mut().project().condition = val;
                    return Ready(Some(if val && *self.as_mut().project().target {
                        Decision::entitlement(&self.e)
                    } else {
                        Decision::NotApplicable
                    }));
                }
                Ready(None) => return Ready(None),
                Pending => {}
            }
        }

        Pending
    }
}
