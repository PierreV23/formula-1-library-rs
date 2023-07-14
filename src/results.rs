use chrono::{DateTime, Utc};
use serde_json::Value;

use crate::weekend_common::{self, custom_date_format, weekend_session};
use crate::{utils::get_from_api, weekend_common::Sessions, weekend_common::Status};
use serde::{Deserialize, Deserializer, Serialize};
use std::error::Error;

pub fn get_results(session: &Sessions, meeting_key: &str) -> Result<SessionResult, Box<dyn Error>> {
    let (session_key, session_extra, json_key) = match session {
        Sessions::Race => ("race", None, "raceResultsRace"),
        Sessions::Sprint => ("sprint", None, "raceResultsSprint"),
        Sessions::Qualifying => ("qualifying", None, "raceResultsQualifying"),
        Sessions::FreePractice1 => ("practice", Some("1"), "raceResultsPractice1"),
        Sessions::FreePractice2 => ("practice", Some("2"), "raceResultsPractice2"),
        Sessions::FreePractice3 => ("practice", Some("3"), "raceResultsPractice3"),
        Sessions::SprintShootout => ("sprint-shootout", None, "raceResultsSprintShootout"),
        _ => panic!(),
    };
    let session_number = if let Some(n) = session_extra {
        format!("&session={n}")
    } else {
        String::new()
    };
    let url =
        format!("https://api.formula1.com/v1/fom-results/{session_key}?meeting={meeting_key}{session_number}");
    let json = get_from_api(&url)?;
    let mut v: Value = serde_json::from_str(&*json)?;
    let e: Value = v
        .get_mut(json_key)
        .ok_or(format!("Key `{json_key}` not in received json."))?
        .take();
    let session_result: SessionResult = serde_json::from_value(e)?;
    Ok(session_result)
}

/// Dit is technisch gezien een lijst, dus doorpasseren als lijst naar ...Result
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "session", content = "results")]
pub enum ResultSheet {
    #[serde(rename = "r")]
    Race(Vec<RaceResult>),
    #[serde(rename = "q")]
    Qualifying(Vec<DriverQualifyingResult>),
    #[serde(rename = "s")]
    Sprint(Vec<SprintResult>),
    #[serde(rename = "p1")]
    FreePractice1(Vec<PracticeResult>),
    #[serde(rename = "p2")]
    FreePractice2(Vec<PracticeResult>),
    #[serde(rename = "p3")]
    FreePractice3(Vec<PracticeResult>),
    // #[serde(rename = "ss")]
    // SprintShootout(),
}

impl ResultSheet {
    fn session(&self) -> Sessions {
        match self {
            ResultSheet::Race(_) => Sessions::Race,
            ResultSheet::Qualifying(_) => Sessions::Qualifying,
            ResultSheet::Sprint(_) => Sessions::Sprint,
            ResultSheet::FreePractice1(_) => Sessions::FreePractice1,
            ResultSheet::FreePractice2(_) => Sessions::FreePractice2,
            ResultSheet::FreePractice3(_) => Sessions::FreePractice3,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CompletionStatusCode {
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "DNF")]
    Dnf,
    // #[serde(rename = "N/A")]
    // Na,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Classifying {
    #[serde(rename = "classifiedTime")]
    classified_time: String,
    #[serde(rename = "completionStatusCode")]
    completion_status_code: CompletionStatusCode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RaceResult {
    #[serde(rename = "teamColourCode")]
    pub team_colour_code: String,
    #[serde(rename = "driverTLA")]
    pub driver_tla: String,
    #[serde(rename = "teamName")]
    pub team_name: String,
    #[serde(rename = "racingNumber")]
    pub racing_number: String,
    #[serde(rename = "positionNumber")]
    pub position_number: String,
    #[serde(rename = "driverFirstName")]
    pub driver_first_name: String,
    #[serde(rename = "driverLastName")]
    pub driver_last_name: String,
    #[serde(rename = "driverNameFormat")]
    pub driver_name_format: String,
    #[serde(rename = "gapToPrevious")]
    pub gap_to_previous: Option<String>,
    #[serde(rename = "gapToLeader")]
    pub gap_to_leader: Option<String>,
    #[serde(rename = "completionStatusCode")]
    pub completion_status_code: CompletionStatusCode,
    #[serde(rename = "raceTime")]
    pub race_time: String,
    #[serde(rename = "driverReference")]
    pub driver_reference: String,
    #[serde(rename = "lapsBehindLeader")]
    pub laps_behind_leader: Option<String>,
    #[serde(rename = "racePoints")]
    pub race_points: i32, // #TODO maybe an U type?
    #[serde(rename = "driverImage")]
    pub driver_image_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriverQualifyingResult {
    pub q1: Option<Classifying>,
    pub q2: Option<Classifying>,
    pub q3: Option<Classifying>,
    #[serde(rename = "teamColourCode")]
    pub team_colour_code: String,
    #[serde(rename = "driverTLA")]
    pub driver_tla: String,
    #[serde(rename = "teamName")]
    pub team_name: String,
    #[serde(rename = "racingNumber")]
    pub racing_number: String,
    #[serde(rename = "positionNumber")]
    pub position_number: String,
    #[serde(rename = "driverFirstName")]
    pub driver_first_name: String,
    #[serde(rename = "driverLastName")]
    pub driver_last_name: String,
    #[serde(rename = "driverNameFormat")]
    pub driver_name_format: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SprintResult {
    #[serde(rename = "teamColourCode")]
    pub team_colour_code: String,
    #[serde(rename = "driverTLA")]
    pub driver_tla: String,
    #[serde(rename = "teamName")]
    pub team_name: String,
    #[serde(rename = "racingNumber")]
    pub racing_number: String,
    #[serde(rename = "positionNumber")]
    pub position_number: String,
    #[serde(rename = "driverFirstName")]
    pub driver_first_name: String,
    #[serde(rename = "driverLastName")]
    pub driver_last_name: String,
    #[serde(rename = "driverNameFormat")]
    pub driver_name_format: String,
    #[serde(rename = "gapToPrevious")]
    pub gap_to_previous: Option<String>,
    #[serde(rename = "gapToLeader")]
    pub gap_to_leader: Option<String>,
    #[serde(rename = "completionStatusCode")]
    pub completion_status_code: CompletionStatusCode,
    #[serde(rename = "sprintQualifyingTime")]
    pub sprint_time: String,
    #[serde(rename = "driverReference")]
    pub driver_reference: String,
    #[serde(rename = "lapsBehindLeader")]
    pub laps_behind_leader: Option<String>,
    #[serde(rename = "sprintQualifyingPoints")]
    pub sprint_points: i32, // #TODO maybe an U type?
    #[serde(rename = "driverImage")]
    pub driver_image_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PracticeResult {
    #[serde(rename = "teamColourCode")]
    pub team_colour_code: String,
    #[serde(rename = "driverTLA")]
    pub driver_tla: String,
    #[serde(rename = "teamName")]
    pub team_name: String,
    #[serde(rename = "racingNumber")]
    pub racing_number: String,
    #[serde(rename = "positionNumber")]
    pub position_number: String,
    #[serde(rename = "driverFirstName")]
    pub driver_first_name: String,
    #[serde(rename = "driverLastName")]
    pub driver_last_name: String,
    #[serde(rename = "driverNameFormat")]
    pub driver_name_format: String,
    #[serde(rename = "gapToLeader")]
    pub gap_to_leader: String,
    #[serde(rename = "classifiedTime")]
    pub classified_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionInfo {
    #[serde(rename = "gmtOffset")]
    pub gmt_offset: String,
    pub description: String,
    #[serde(with = "custom_date_format")]
    #[serde(rename = "startTime")]
    pub start_date: DateTime<Utc>,
    #[serde(with = "custom_date_format")]
    #[serde(rename = "endTime")]
    pub end_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionResult {
    pub state: Status,
    // #[serde(with = "weekend_session")]
    // pub session: Sessions,
    #[serde(flatten)]
    pub session_info: Option<SessionInfo>,
    #[serde(flatten)]
    pub results: ResultSheet,
    // #[serde(with = "weekend_session")]
    // pub session: Sessions,
}

impl SessionResult {
    pub fn session(&self) -> Sessions {
        self.results.session()
    }
}

// #[macro_use]
macro_rules! from_get_value {
    ($value:expr, $field:literal, $key_not_present_err:expr) => {
        serde_json::from_value($value.get_mut($field).ok_or($key_not_present_err)?.take())
            .map_err(serde::de::Error::custom)?
    };
}

// impl<'de> Deserialize<'de> for SessionResult {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let mut value: serde_json::value::Value =
//             serde_json::value::Value::deserialize(deserializer)?;
//         let key_not_present = |t| serde::de::Error::custom(format!("Key `{t}` was not present."));
//         // let v = value.get_mut("state").ok_or(key_not_present("state"))?;

//         // temp note:
//         // |e| <D::Error as serde::de::Error>::custom(e.to_string())
//         // was used instead of
//         // serde::de::Error::custom
//         // #TODO Please if you read this, tell me if there is a better way than doing this every time...
//         // fn get_from_value(value: &mut Value, key: &str) -> Result<Value, _> {
//         //     value
//         //         .get_mut(key)
//         //         .ok_or(serde::de::Error::custom(format!("Key `{key}` was not present.")))?
//         //         .take()
//         // }
//         let va: String =
//             from_get_value!(value.clone(), "description", key_not_present("description"));
//         let state: Status = serde_json::from_value(
//             value
//                 .get_mut("state")
//                 .ok_or(key_not_present("state"))?
//                 .take(),
//         )
//         .map_err(serde::de::Error::custom)?;
//         // let session_raw = String::deserialize(value.get("session").ok_or(key_not_present("session"))?).map_err(serde::de::Error::custom)?;
//         let session: Sessions = weekend_session::deserialize(
//             value
//                 .get_mut("session")
//                 .ok_or(key_not_present("session"))?
//                 .take(),
//         )
//         .map_err(serde::de::Error::custom)?;
//         let gmt_offset = String::deserialize(
//             value
//                 .get_mut("gmtOffset")
//                 .ok_or(key_not_present("gmtOffset"))?
//                 .take(),
//         )
//         .map_err(serde::de::Error::custom)?;
//         let description = String::deserialize(
//             value
//                 .get_mut("description")
//                 .ok_or(key_not_present("description"))?
//                 .take(),
//         )
//         .map_err(serde::de::Error::custom)?;
//         let start_date = custom_date_format::deserialize(
//             value
//                 .get_mut("startTime")
//                 .ok_or(key_not_present("startDate"))?
//                 .take(),
//         )
//         .map_err(serde::de::Error::custom)?;
//         let end_date = custom_date_format::deserialize(
//             value
//                 .get_mut("endTime")
//                 .ok_or(key_not_present("endDate"))?
//                 .take(),
//         )
//         .map_err(serde::de::Error::custom)?;
//         #[derive(Serialize, Deserialize, Debug)]
//         struct Intermediary {
//             pub state: Status,
//             #[serde(with = "weekend_session")]
//             pub session: Sessions,
//             #[serde(rename = "gmtOffset")]
//             pub gmt_offset: String,
//             pub description: String,
//             #[serde(with = "custom_date_format")]
//             #[serde(rename = "startTime")]
//             pub start_date: DateTime<Utc>,
//             #[serde(with = "custom_date_format")]
//             #[serde(rename = "endTime")]
//             pub end_date: DateTime<Utc>,
//         }
//         let mut map = serde_json::Map::new();
//         map.insert(
//             match &session {
//                 Sessions::Race => "Race",
//                 Sessions::Sprint => "Sprint",
//                 Sessions::Qualifying => "q",
//                 Sessions::FreePractice1 => "FreePractice1",
//                 Sessions::FreePractice2 => "FreePractice2",
//                 Sessions::FreePractice3 => "FreePractice3",
//                 Sessions::SprintShootout => "SprintShootout",
//             }
//             .to_string(),
//             value
//                 .get_mut("results")
//                 .ok_or(key_not_present("results"))?
//                 .take(),
//         );
//         let results: ResultSheet =
//             serde_json::from_value(Value::Object(map)).map_err(serde::de::Error::custom)?;
//         let inter: Intermediary =
//             serde_json::from_value(value).map_err(serde::de::Error::custom)?;
//         let srr = SessionResult {
//             state: inter.state,
//             session: inter.session,
//             gmt_offset: inter.gmt_offset,
//             description: inter.description,
//             start_date: inter.start_date,
//             end_date: inter.end_date,
//             results,
//         };
//         // Ok(SessionResult {
//         //     state,
//         //     session,
//         //     gmt_offset,
//         //     description,
//         //     start_date,
//         //     end_date,
//         //     results,
//         // })
//         Ok(srr)
//     }
// }
