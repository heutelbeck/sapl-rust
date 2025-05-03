use crate::val::Val;
use futures::Stream;
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project! {
    /// Stream returned by the [`eval_add`] method.
    pub struct EvalAdd<T, U> {
        #[pin]
        a: T,
        #[pin]
        b: U,
        // save the last stream results
        lhs: Val,
        rhs: Val,

    }
}

impl<T, U> EvalAdd<T, U> {
    pub(super) fn new(a: T, b: U) -> EvalAdd<T, U>
    where
        T: Stream<Item = Val>,
        U: Stream<Item = T::Item>,
    {
        EvalAdd {
            a,
            b,
            lhs: Val::None,
            rhs: Val::None,
        }
    }
}

impl<T, U> Stream for EvalAdd<T, U>
where
    T: Stream<Item = Val>,
    U: Stream<Item = T::Item>,
{
    type Item = T::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        use Poll::*;

        match (
            self.as_mut().project().a.poll_next(cx),
            self.as_mut().project().b.poll_next(cx),
        ) {
            (Ready(Some(val1)), Ready(Some(val2))) => {
                *self.as_mut().project().lhs = val1;
                *self.as_mut().project().rhs = val2;

                Ready(Some(equal_add(val1, val2)))
            }
            (Ready(Some(val1)), Pending) => {
                *self.as_mut().project().lhs = val1;

                Ready(Some(equal_add(val1, *self.as_mut().project().rhs)))
            }
            (Pending, Ready(Some(val2))) => {
                *self.as_mut().project().rhs = val2;

                Ready(Some(equal_add(*self.as_mut().project().lhs, val2)))
            }
            (Pending, Pending) => Pending,
            (_, _) => Ready(None),
        }
    }
}

fn equal_add(lhs: Val, rhs: Val) -> Val {
    use Val::*;
    match (lhs, rhs) {
        (Integer(l), Integer(r)) => Integer(l + r),
        (Float(l), Float(r)) => Float(l + r),
        _ => None,
    }
}
