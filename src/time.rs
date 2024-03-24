
use chrono::Utc;
use diff::Diff;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Diff)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Time {
    // millis
    pub timestamp: i64
}

impl From<chrono::DateTime<Utc>> for Time {
    fn from(value: chrono::DateTime<Utc>) -> Self {
        Self {
            timestamp: value.timestamp_millis()
        }
    }
}

impl Into<chrono::DateTime<Utc>> for &Time {
    fn into(self) -> chrono::DateTime<Utc> {
        match chrono::DateTime::from_timestamp_millis(self.timestamp) {
            Some(date_time) => date_time,
            None => panic!()
        }
    }
}

impl Time {

    pub fn new(timestamp: i64) -> Self {
        Self {
            timestamp
        }
    }

    pub fn now() -> Self {
        Utc::now().into()
    }

    pub fn since(&self, then: &Self) -> chrono::TimeDelta {
        let now: chrono::DateTime<Utc> = self.into();

        let then: chrono::DateTime<Utc> = then.into();

        now.signed_duration_since(then)
    }

    pub fn elapsed(&self) -> chrono::TimeDelta {

        let now = Self::now();

        now.since(self)

        
    }
}
