use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::weekend_common::{custom_date_format, weekend_session};

use serde_json::Value;

use crate::{
    utils::get_from_api,
    weekend_common::{Sessions, Status},
};

pub fn get_current_weekend() -> Result<CurrentWeekend, Box<dyn Error>> {
    let url = format!("https://api.formula1.com/v1/event-tracker");
    let json = get_from_api(&url)?;
    let mut v: Value = serde_json::from_str(&*json)?;
    let e: Value = v
        .get_mut("seasonContext")
        .ok_or("Key `seasonContext` not in received json.")?
        .take();
    let current_weekend: CurrentWeekend = serde_json::from_value(e)?;
    Ok(current_weekend)
}

pub fn get_timetables(meeting_key: &str) -> Result<Vec<Timetable>, Box<dyn Error>> {
    let url = format!(
        "https://api.formula1.com/v1/fom-results/timetables?meeting={}",
        meeting_key
    );
    let json = get_from_api(&url)?;
    let mut v: Value = serde_json::from_str(&*json)?;
    let e: Value = v
        .get_mut("timetables")
        .ok_or("Key `timetables` not in received json.")?
        .take();
    Ok(serde_json::from_value(e)?)
}

pub fn get_calendar(year: &str) -> Result<Vec<Event>, Box<dyn Error>> {
    let url = format!("https://api.formula1.com/v1/editorial-eventlisting/events?season={year}");
    let json = get_from_api(&url)?;
    let mut v: Value = serde_json::from_str(&*json)?;
    let e: Value = v
        .get_mut("events")
        .ok_or("Key `events` not in received json.")?
        .take();
    let events: Vec<Event> = serde_json::from_value(e)?;
    Ok(events)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventType {
    #[serde(rename = "race")]
    Race,
    #[serde(rename = "fom-testing")]
    Testing,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "circuitMediumImage")]
    circuit_medium_image_url: String,
    #[serde(rename = "meetingCountryName")]
    meeting_country_name: String,
    #[serde(rename = "meetingLocation")]
    meeting_location: String,
    #[serde(rename = "meetingOfficialName")]
    meeting_official_name: String,
    #[serde(with = "custom_date_format")]
    #[serde(rename = "meetingStartDate")]
    meeting_start_date: DateTime<Utc>,
    #[serde(with = "custom_date_format")]
    #[serde(rename = "meetingEndDate")]
    meeting_end_date: DateTime<Utc>,
    #[serde(rename = "meetingKey")]
    meeting_key: String,
    #[serde(rename = "meetingName")]
    meeting_name: String,
    #[serde(rename = "gmtOffset")]
    gmt_offset: String,
    status: Status,
    #[serde(rename = "roundText")]
    rount_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timetable {
    state: Status,
    #[serde(with = "weekend_session")]
    session: Sessions,
    description: String,
    #[serde(with = "custom_date_format")]
    #[serde(rename = "startTime")]
    start_date: DateTime<Utc>,
    #[serde(with = "custom_date_format")]
    #[serde(rename = "endTime")]
    end_date: DateTime<Utc>,
    #[serde(rename = "gmtOffset")]
    gmt_offset: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentWeekend {
    #[serde(rename = "seasonYear")]
    season_year: String,
    #[serde(rename = "currentOrNextMeetingKey")]
    current_or_next_meeting_key: String,
    timetables: Vec<Timetable>,
}
