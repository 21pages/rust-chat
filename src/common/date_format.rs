// see https://serde.rs/custom-date-format.html

use chrono::{DateTime, Local, TimeZone};
use serde::{self, Deserialize, Deserializer, Serializer};
const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
lazy_static! {
    pub static ref INVALID_DATE: DateTime<Local> = Local.ymd(2000, 1, 1).and_hms(0, 0, 0);
}

pub mod my_date_format {
    use super::*;
    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Local
            .datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

pub mod option_date_format {

    use super::*;

    pub fn serialize<S>(option: &Option<DateTime<Local>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let date = option.unwrap_or(*INVALID_DATE);
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Local>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match Local.datetime_from_str(&s, FORMAT) {
            Ok(date) => Ok(Some(date)),
            Err(_) => Ok(None),
        }
    }
}
