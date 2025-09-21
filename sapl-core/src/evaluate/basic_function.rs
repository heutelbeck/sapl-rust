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

use serde_json::Value;

use crate::{
    Ast, Val,
    functions::temporal_function_library::{
        day_of_week, day_of_year, hour_of, minute_of, second_of, week_of_year,
    },
    pip::Time,
};
use std::sync::{Arc, RwLock};

pub(crate) fn basic_function(
    bf: &Arc<[Ast]>,
    _variable_context: Arc<RwLock<Value>>,
) -> Result<Val, String> {
    match bf.first() {
        Some(fi) => evaluate_function_identifier(
            fi,
            if bf.len() == 2 {
                evaluate_arguments(&bf[1])
            } else {
                Ok(Val::None)
            },
        ),
        None => Err("basic_function no function identifier found".to_string()),
    }
}

fn evaluate_function_identifier(fi: &Ast, arg: Result<Val, String>) -> Result<Val, String> {
    match fi {
        Ast::FunctionIdentifier(path) => match path.as_ref() {
            [Ast::Id(namespace), Ast::Id(function)]
                if namespace == "time" && function == "secondOf" =>
            {
                second_of(&arg)
            }
            [Ast::Id(namespace), Ast::Id(function)]
                if namespace == "time" && function == "minuteOf" =>
            {
                minute_of(&arg)
            }
            [Ast::Id(namespace), Ast::Id(function)]
                if namespace == "time" && function == "hourOf" =>
            {
                hour_of(&arg)
            }
            [Ast::Id(namespace), Ast::Id(function)]
                if namespace == "time" && function == "weekOfYear" =>
            {
                week_of_year(&arg)
            }
            [Ast::Id(namespace), Ast::Id(function)]
                if namespace == "time" && function == "dayOfYear" =>
            {
                day_of_year(&arg)
            }
            [Ast::Id(namespace), Ast::Id(function)]
                if namespace == "time" && function == "dayOfWeek" =>
            {
                day_of_week(&arg)
            }
            [Ast::Id(namespace), Ast::Id(function)] if namespace == "time" && function == "now" => {
                Time::now()
            }
            other => Err(format!(
                "evaluate_function_identifier function {other:#?} is unkown"
            )),
        },
        other => Err(format!(
            "evaluate_function_identifier expect Ast::FunctionIdentifier, but got: {other:#?}"
        )),
    }
}

fn evaluate_arguments(input: &Ast) -> Result<Val, String> {
    use Ast::*;
    match input {
        Arguments(args) => match &**args {
            Boolean(b) => Ok(Val::Boolean(*b)),
            Integer(i) => Ok(Val::Integer(*i)),
            String(s) => Ok(Val::String(s.clone())),
            BasicEnvironmentAttribute(bea) => evaluate_function_identifier(
                bea.first().unwrap(),
                if bea.len() == 2 {
                    evaluate_arguments(&bea[1])
                } else {
                    Ok(Val::None)
                },
            ),
            other => todo!("Unkown argument: {other:#?}"),
        },
        other => Err(format!(
            "evaluate_arguments expects Ast::Arguments, but got: {other:#?}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AuthorizationSubscription;

    fn get_basic_function_time_req(first: &str, second: &str) -> Arc<[Ast]> {
        let fi = Ast::FunctionIdentifier(Arc::new([
            Ast::Id(first.to_string()),
            Ast::Id(second.to_string()),
        ]));
        let arg = Ast::Arguments(Arc::new(Ast::String("2025-07-26T20:00:23Z".to_string())));

        Arc::new([fi, arg])
    }

    fn basic_function_call(input: Arc<[Ast]>) -> Result<Val, String> {
        basic_function(
            &input,
            Arc::new(RwLock::new(
                AuthorizationSubscription::new_example_subscription1(),
            )),
        )
    }

    fn basic_function_time_call(first: &str, second: &str) -> Result<Val, String> {
        basic_function_call(get_basic_function_time_req(first, second))
    }

    #[test]
    fn time_second_of() {
        assert_eq!(
            Ok(Val::Integer(23)),
            basic_function_time_call("time", "secondOf")
        );
    }

    #[test]
    fn time_minute_of() {
        assert_eq!(
            Ok(Val::Integer(0)),
            basic_function_time_call("time", "minuteOf")
        );
    }

    #[test]
    fn time_hour_of() {
        assert_eq!(
            Ok(Val::Integer(20)),
            basic_function_time_call("time", "hourOf")
        );
    }

    #[test]
    fn time_week_of_year() {
        assert_eq!(
            Ok(Val::Integer(30)),
            basic_function_time_call("time", "weekOfYear")
        );
    }

    #[test]
    fn time_day_of_year() {
        assert_eq!(
            Ok(Val::Integer(207)),
            basic_function_time_call("time", "dayOfYear")
        );
    }

    #[test]
    fn time_day_of_week() {
        assert_eq!(
            Ok(Val::String("SATURDAY".to_string())),
            basic_function_time_call("time", "dayOfWeek")
        );
    }

    #[test]
    fn basic_function_time_second_of_and_time_now() {
        let fi = Ast::FunctionIdentifier(Arc::new([
            Ast::Id("time".to_string()),
            Ast::Id("secondOf".to_string()),
        ]));
        let fi2 = Ast::FunctionIdentifier(Arc::new([
            Ast::Id("time".to_string()),
            Ast::Id("now".to_string()),
        ]));
        let bea = Arc::new(Ast::BasicEnvironmentAttribute(Arc::new([fi2])));
        let arg = Ast::Arguments(bea);

        let result = basic_function(
            &Arc::from([fi, arg]),
            Arc::new(RwLock::new(
                AuthorizationSubscription::new_example_subscription1(),
            )),
        );
        let second_now = second_of(&Time::now());
        assert_eq!(second_now, result);
    }
}
