use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Debug)]
pub enum Sessions {
    // #[serde(rename = "r")]
    Race,
    // #[serde(rename = "s")]
    Sprint,
    // #[serde(rename = "q")]
    Qualifying,
    // #[serde(rename = "p1")]
    FreePractice1,
    // #[serde(rename = "p2")]
    FreePractice2,
    // #[serde(rename = "p3")]
    FreePractice3,
    // #[serde(rename = "ss")]
    SprintShootout,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "upcoming")]
    Upcoming,
    #[serde(rename = "N/A")]
    Na,
}

// https://serde.rs/custom-date-format.html
pub mod custom_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S"; // 2023-07-07T12:30:00

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

pub mod weekend_session {
    use super::Sessions;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(session: &Sessions, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match session {
            Sessions::Race => "r",
            // Sessions::Sprint => "s", // #TODO no way to be sure atm, as hungary is coming up, which does not have a sprint race.
            Sessions::Sprint => panic!(),
            Sessions::SprintShootout => panic!(),
            Sessions::Qualifying => "q",
            Sessions::FreePractice1 => "p1",
            Sessions::FreePractice2 => "p2",
            Sessions::FreePractice3 => "p3",
        };
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Sessions, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = &*String::deserialize(deserializer)?;
        Ok(match s {
            "r" => Sessions::Race,
            // "s" => Sessions::Sprint, // #TODO no way to be sure atm, as hungary is coming up, which does not have a sprint race.
            "s" => panic!(),
            "q" => Sessions::Qualifying,
            "p1" => Sessions::FreePractice1,
            "p2" => Sessions::FreePractice2,
            "p3" => Sessions::FreePractice3,
            _ => panic!(), // #TODO should not happen, maybe make a error out of this later.
        })
    }
}
