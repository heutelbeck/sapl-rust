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

use chrono::{DateTime, Datelike, Timelike, Weekday};

use crate::Val;

pub(crate) fn second_of(iso_date_time: Result<Val, String>) -> Result<Val, String> {
    match iso_date_time {
        Ok(Val::String(s)) => match DateTime::parse_from_rfc3339(&s) {
            Ok(dt) => Ok(Val::Integer(dt.second().into())),
            Err(e) => Err(e.to_string()),
        },
        other => Err(format!(
            "Type mismatch. Second_of operation expected ISO 8601/RFC 3339 string value, but got: {other:#?}"
        )),
    }
}

pub(crate) fn minute_of(iso_date_time: Result<Val, String>) -> Result<Val, String> {
    match iso_date_time {
        Ok(Val::String(s)) => match DateTime::parse_from_rfc3339(&s) {
            Ok(dt) => Ok(Val::Integer(dt.minute().into())),
            Err(e) => Err(e.to_string()),
        },
        other => Err(format!(
            "Type mismatch. Minute_of operation expected ISO 8601/RFC 3339 string value, but got: {other:#?}"
        )),
    }
}

pub(crate) fn hour_of(iso_date_time: Result<Val, String>) -> Result<Val, String> {
    match iso_date_time {
        Ok(Val::String(s)) => match DateTime::parse_from_rfc3339(&s) {
            Ok(dt) => Ok(Val::Integer(dt.hour().into())),
            Err(e) => Err(e.to_string()),
        },
        other => Err(format!(
            "Type mismatch. Hour_of operation expected ISO 8601/RFC 3339 string value, but got: {other:#?}"
        )),
    }
}

pub(crate) fn week_of_year(iso_date_time: Result<Val, String>) -> Result<Val, String> {
    match iso_date_time {
        Ok(Val::String(s)) => match DateTime::parse_from_rfc3339(&s) {
            Ok(dt) => Ok(Val::Integer(dt.iso_week().week().into())),
            Err(e) => Err(e.to_string()),
        },
        other => Err(format!(
            "Type mismatch. WeekOfYear operation expected ISO 8601/RFC 3339 string value, but got: {other:#?}"
        )),
    }
}

pub(crate) fn day_of_year(iso_date_time: Result<Val, String>) -> Result<Val, String> {
    match iso_date_time {
        Ok(Val::String(s)) => match DateTime::parse_from_rfc3339(&s) {
            Ok(dt) => Ok(Val::Integer(dt.ordinal().into())),
            Err(e) => Err(e.to_string()),
        },
        other => Err(format!(
            "Type mismatch. DayOfYear operation expected ISO 8601/RFC 3339 string value, but got: {other:#?}"
        )),
    }
}

pub(crate) fn day_of_week(iso_date_time: Result<Val, String>) -> Result<Val, String> {
    match iso_date_time {
        Ok(Val::String(s)) => match DateTime::parse_from_rfc3339(&s) {
            Ok(dt) => Ok(Val::String(dt.weekday().format_weekday())),
            Err(e) => Err(e.to_string()),
        },
        other => Err(format!(
            "Type mismatch. DayOfWeek operation expected ISO 8601/RFC 3339 string value, but got: {other:#?}"
        )),
    }
}

pub trait WeekdayFormat {
    fn format_weekday(&self) -> String;
}

impl WeekdayFormat for Weekday {
    fn format_weekday(&self) -> String {
        match self {
            Weekday::Mon => "MONDAY".to_string(),
            Weekday::Tue => "TUESDAY".to_string(),
            Weekday::Wed => "WEDNESDAY".to_string(),
            Weekday::Thu => "THURSDAY".to_string(),
            Weekday::Fri => "FRIDAY".to_string(),
            Weekday::Sat => "SATURDAY".to_string(),
            Weekday::Sun => "SUNDAY".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second_of_with_non_rfc_3339_input() {
        let result = second_of(Ok(Val::String("LoremIpsum".to_string())));
        assert!(result.is_err());
    }

    #[test]
    fn second_of_with_valid_input() {
        assert_eq!(
            Ok(Val::Integer(23)),
            second_of(Ok(Val::String("2025-07-26T20:00:23Z".to_string())))
        );
    }

    #[test]
    fn minute_of_with_non_rfc_3339_input() {
        let result = minute_of(Ok(Val::String("LoremIpsum".to_string())));
        assert!(result.is_err());
    }

    #[test]
    fn minute_of_with_valid_input() {
        assert_eq!(
            Ok(Val::Integer(42)),
            minute_of(Ok(Val::String("2025-07-26T20:42:23Z".to_string())))
        );
    }

    #[test]
    fn hour_of_with_non_rfc_3339_input() {
        let result = hour_of(Ok(Val::String("LoremIpsum".to_string())));
        assert!(result.is_err());
    }

    #[test]
    fn hour_of_with_valid_input() {
        assert_eq!(
            Ok(Val::Integer(20)),
            hour_of(Ok(Val::String("2025-07-26T20:00:23Z".to_string())))
        );
    }

    #[test]
    fn week_of_year_with_non_rfc_3339_input() {
        let result = week_of_year(Ok(Val::String("LoremIpsum".to_string())));
        assert!(result.is_err());
    }

    #[test]
    fn week_of_year_with_valid_input() {
        assert_eq!(
            Ok(Val::Integer(30)),
            week_of_year(Ok(Val::String("2025-07-26T20:00:23Z".to_string())))
        );
    }

    #[test]
    fn day_of_year_with_non_rfc_3339_input() {
        let result = day_of_year(Ok(Val::String("LoremIpsum".to_string())));
        assert!(result.is_err());
    }

    #[test]
    fn day_of_year_with_valid_input() {
        assert_eq!(
            Ok(Val::Integer(207)),
            day_of_year(Ok(Val::String("2025-07-26T20:00:23Z".to_string())))
        );
    }

    #[test]
    fn day_of_week_with_non_rfc_3339_input() {
        let result = day_of_week(Ok(Val::String("LoremIpsum".to_string())));
        assert!(result.is_err());
    }

    #[test]
    fn day_of_week_with_valid_input() {
        assert_eq!(
            Ok(Val::String("SATURDAY".to_string())),
            day_of_week(Ok(Val::String("2025-07-26T20:00:23Z".to_string())))
        );
    }
}
