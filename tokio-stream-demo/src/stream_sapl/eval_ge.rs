use crate::val::Val;
use futures::Stream;
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project! {
    /// Stream returned by the [`eval_eq`] method.
    pub struct EvalGe<T, U> {
        #[pin]
        a: T,
        #[pin]
        b: U,
        // save the last stream results
        lhs: Val,
        rhs: Val,

    }
}

impl<T, U> EvalGe<T, U> {
    pub(super) fn new(a: T, b: U) -> EvalGe<T, U>
    where
        T: Stream<Item = Val>,
        U: Stream<Item = T::Item>,
    {
        EvalGe {
            a,
            b,
            lhs: Val::None,
            rhs: Val::None,
        }
    }
}

impl<T, U> Stream for EvalGe<T, U>
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

                Ready(Some(greater_val(val1, val2)))
            }
            (Ready(Some(val1)), Pending) => {
                *self.as_mut().project().lhs = val1;

                Ready(Some(greater_val(val1, *self.as_mut().project().rhs)))
            }
            (Pending, Ready(Some(val2))) => {
                *self.as_mut().project().rhs = val2;

                Ready(Some(greater_val(*self.as_mut().project().lhs, val2)))
            }
            (Pending, Pending) => Pending,
            (_, _) => Ready(None),
        }
    }
}

fn greater_val(lhs: Val, rhs: Val) -> Val {
    use Val::*;
    match (lhs, rhs) {
        (Boolean(l), Boolean(r)) => Boolean(l & !r),
        (Integer(l), Integer(r)) => Boolean(l > r),
        (Float(l), Float(r)) => Boolean(l > r),
        _ => Boolean(false),
    }
}
