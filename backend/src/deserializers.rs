pub mod gsheets_csv_date_format {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer};

    const FORMAT: &str = "%B %d, %Y";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub mod none_string_format {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let cleaned = s.trim();

        if cleaned.is_empty() || cleaned == "None" {
            return Ok(None);
        }

        Ok(Some(cleaned.to_string()))
    }
}

pub mod thecryptans_csv_datetime_format {
    use chrono::{DateTime, Duration, NaiveDateTime, Utc};
    use serde::{Deserialize, Deserializer};

    const FORMAT: &str = "%m/%d/%Y %I:%M:%S %p";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        let utc_dt = DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc);
        Ok(utc_dt - Duration::try_hours(6).expect("building with static parameters never fails"))
    }
}

pub mod thousands_seperated_integer {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let cleaned = s.trim().replace(',', "");
        cleaned.parse().map_err(serde::de::Error::custom)
    }
}
