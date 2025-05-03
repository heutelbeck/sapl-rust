use futures::Stream;

use crate::val::Val;

mod eval_eq;
pub use eval_eq::EvalEq;

mod eval_add;
pub use eval_add::EvalAdd;

mod eval_and;
pub use eval_and::EvalAnd;

mod eval_ge;
pub use eval_ge::EvalGe;

pub trait StreamSapl: Stream<Item = Val> {
    fn eval_eq<U>(self, other: U) -> EvalEq<Self, U>
    where
        U: Stream<Item = Val>,
        Self: Sized,
    {
        EvalEq::new(self, other)
    }

    fn eval_add<U>(self, other: U) -> EvalAdd<Self, U>
    where
        U: Stream<Item = Val>,
        Self: Sized,
    {
        EvalAdd::new(self, other)
    }

    fn eval_and<U>(self, other: U) -> EvalAnd<Self, U>
    where
        U: Stream<Item = Val>,
        Self: Sized,
    {
        EvalAnd::new(self, other)
    }

    fn eval_ge<U>(self, other: U) -> EvalGe<Self, U>
    where
        U: Stream<Item = Val>,
        Self: Sized,
    {
        EvalGe::new(self, other)
    }
}
