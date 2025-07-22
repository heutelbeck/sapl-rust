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
use std::fmt::Debug;
use std::pin::Pin;
use tokio_stream::Stream;

mod stream_sapl;
//mod add_expr;
mod boolean_stream;
mod boolean_stream_new;
mod combine_eager;
mod combine_lazy;
mod integer_stream;
mod time_sec_interval;
//mod constant_bool_expr;
//mod constant_integer_expr;
mod delay;
//mod equal_expr;
//mod evaluate_expr;
mod expr;
mod once;
mod val;

//pub use add_expr::AddExpr;
pub use boolean_stream::BooleanInterval;
pub use boolean_stream_new::BooleanIntervalNew;
pub use integer_stream::IntegerInterval;
pub use time_sec_interval::TimeSecInterval;
//pub use constant_integer_expr::ConstantIntegerExpr;
//pub use equal_expr::EqualExpr;
pub use expr::Ast;
pub use once::once;
pub use stream_sapl::StreamSapl;
pub use val::Val;

pub trait Expression: Eval + Debug {}
pub trait Eval {
    fn eval(&self) -> Pin<Box<(dyn Stream<Item = Val> + 'static)>>;
}
