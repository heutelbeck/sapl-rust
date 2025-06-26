use crate::{expr::Ast, Eval, Expression};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio_stream::Stream;

#[derive(Debug, PartialEq)]
pub struct ConstantBoolExpr {
    pub value: Option<bool>,
}

impl Expression for ConstantBoolExpr {}
impl Eval for ConstantBoolExpr {
    fn eval(&self) -> Ast {
        Ast::Finished
    }
}
impl Stream for ConstantBoolExpr {
    type Item = Ast;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.value.take())
    }
}
