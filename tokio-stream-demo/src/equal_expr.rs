
use crate::expr::Ast;
use crate::Eval;
use crate::Expression;

use core::pin::Pin;
use core::task::{Context, Poll};
use futures::StreamExt;
use pin_project_lite::pin_project;
use std::sync::Arc;
use tokio_stream::Stream;

pin_project! {
    #[derive(Debug)]
    pub struct EqualExpr {
        #[pin]
        pub lhs: Arc<dyn Expression>,
        #[pin]
        pub rhs: Arc<dyn Expression>,
    }
}

impl Expression for EqualExpr {}
impl Eval for EqualExpr {
    fn eval(&self) -> Pin<Box<dyn Stream(Item = Val + 'static)>> {
        use crate::expr::Ast::*;

        let lhs_result = self.lhs.eval().next().await;
    }
}

// impl EqualExpr {
//     fn combine(self, other: U) -> Pin<Box<dyn Stream(Item = Val + 'static)>> {
//         Combine::new(self, other)
//     }
// }

// impl Stream for EqualExpr {
//     type Item = Ast;
//
//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//         use self::Ast::*;
//         let lhs_result = self.as_mut().project().lhs.poll_next_unpin(cx);
//         let rhs_result = self.as_mut().project().rhs.poll_next_unpin(cx);
//
//         match (lhs_result, rhs_result) {
//             //Boolean(b) => tokio_stream::once(b),
//             //Expr(_) => Poll::Ready(Some(self.get_mut().eval())),
//             _ => Poll::Ready(None),
//         }
//     }
// }
