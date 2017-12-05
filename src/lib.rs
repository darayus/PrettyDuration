extern crate chrono;
extern crate time;

pub mod split;
pub mod pretty;

use chrono::prelude::*;
use time::Duration;

pub use pretty::{pretty_short, pretty_full};

pub trait NaturalTime {
    fn time_difference(&self) -> Duration;
    fn time_delta(&self) -> String {
        let delta = self.time_difference();
        let mut final_str = pretty_short(delta);

        if delta < Duration::seconds(0) {
            let mut new_str = String::from("in ");
            new_str.push_str(&final_str);
            final_str = new_str;
        } else {
            final_str.push_str(" ago");
        }
        return final_str
    }
}

impl NaturalTime for DateTime<Utc> {
    fn time_difference(&self) -> Duration {
        let now = Utc::now();
        return -self.signed_duration_since(now);
    }
}

impl NaturalTime for DateTime<Local> {
    fn time_difference(&self) -> Duration {
        let now = Local::now();
        return -self.signed_duration_since(now);
    }
}

#[test]
fn test_time_delta_correct() {
    let new_date = Utc::now() + Duration::days(10);
    assert_eq!(new_date.time_delta(), "in 1 week and 2 days");

    let new_date = Utc::now() - Duration::days(10);
    assert_eq!(new_date.time_delta(), "1 week and 3 days ago");
}

#[test]
fn test_time_delta_correct_local() {
    let new_date = Local::now() + Duration::days(10);
    assert_eq!(new_date.time_delta(), "in 1 week and 2 days");

    let new_date = Local::now() - Duration::days(10);
    assert_eq!(new_date.time_delta(), "1 week and 3 days ago");
}
