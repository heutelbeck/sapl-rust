use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio_stream::Stream;

use crate::BooleanIntervalNew;
use crate::Eval;
use crate::Expression;
//use crate::{constant_bool_expr::ConstantBoolExpr, ConstantIntegerExpr};
use crate::stream_sapl::StreamSapl;
use crate::val::Val;

#[derive(Debug)]
pub enum Ast {
    //Expr(Arc<dyn Expression<Item = Val>>),
    Boolean(bool),
    BooleanStream(BooleanIntervalNew),
    Integer(i64),
    Finished,
}

impl<S> StreamSapl for S where S: Stream<Item = Val> {}
impl Expression for Ast {}
impl Eval for Ast {
    fn eval(&self) -> Pin<Box<(dyn Stream<Item = Val>)>> {
        match self {
            Ast::BooleanStream(stream) => {
                let owend_stream = (*stream).clone();
                Box::pin(owend_stream)
            }
            Ast::Boolean(b) => Box::pin(crate::once::once(Val::Boolean(*b))),
            Ast::Integer(i) => Box::pin(crate::once::once(Val::Integer(*i))),
            _ => unimplemented!(),
        }
    }
}

impl Stream for Ast {
    type Item = bool;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(None)
    }
}
