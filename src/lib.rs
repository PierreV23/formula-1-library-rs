use std::io::Error;

pub mod results;
pub mod schedule;
pub mod telemetry;
pub mod utils;
pub mod weekend_common;

pub type Result<T> = std::result::Result<T, Error>;

pub const API_KEY: &str = "t3DrvCuXvjDX8nIvPpcSNTbB9kae1DPs";

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    use chrono::DateTime;

    use crate::{
        results::{get_results, DriverQualifyingResult, ResultSheet, SessionResult},
        schedule::{get_calendar, get_current_weekend, get_timetables},
        weekend_common::{Sessions, Status},
    };

    #[test]
    fn schedule_year_calendar() {
        // println!("{}", schedule::get_timetables("1214").unwrap())
        let events = get_calendar("2023").unwrap();
        // println!("{:?}", events);
        let pretty = serde_json::to_string_pretty(&events).unwrap();
        // println!("{}", pretty);
    }

    #[test]
    fn curr_weekend() {
        let wknd = get_current_weekend().unwrap();
        let pretty = serde_json::to_string_pretty(&wknd).unwrap();
        // println!("{}", pretty);
    }

    #[test]
    fn time_tables_test() {
        let timetables = get_timetables("1214").unwrap(); // 1214 == silverstone 2023
        let pretty = serde_json::to_string_pretty(&timetables).unwrap();
        // println!("{}", pretty);
    }

    #[test]
    fn result_quali_test() {
        let quali_result =
            get_results(&crate::weekend_common::Sessions::Qualifying, "1213").unwrap(); // 1214 == silverstone 2023
        let pretty = serde_json::to_string_pretty(&quali_result).unwrap();
        // println!("{}", pretty);
    }

    #[test]
    fn result_test_all() {
        let sessions = vec![
            Sessions::Race,
            Sessions::Sprint,
            Sessions::Qualifying,
            Sessions::FreePractice1,
            Sessions::FreePractice2,
            Sessions::FreePractice3,
            // Sessions::SprintShootout,
        ];
        for sess in sessions {
            let result = get_results(&sess, "1213");
            let ok = result.unwrap();
            // let ok = match result {
            //     Ok(k) => k,
            //     Err(mut e) => {
            //         let l = e.as_mut();
            //         println!("{:?}", l);
            //         // let k = l.source().unwrap();
            //         // k.
            //         panic!();
            //     }
            // };
            let pretty = serde_json::to_string_pretty(&ok).unwrap();
            println!("{}", pretty);
        }
    }
}
