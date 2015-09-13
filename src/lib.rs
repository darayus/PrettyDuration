extern crate chrono;

pub mod split;
pub mod pretty;

use chrono::*;

pub use pretty::{pretty_short, pretty_full};

pub trait NaturalTime {
    fn time_delta(&self) -> String;
}

impl NaturalTime for DateTime<UTC> {
    fn time_delta(&self) -> String {
        let now = UTC::now();
        let delta = now - *self;
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

#[test]
fn test_time_delta_correct() {
    let new_date = UTC::now() + Duration::days(10);
    assert_eq!(new_date.time_delta(), "in 1 week and 2 days");

    let new_date = UTC::now() - Duration::days(10);
    assert_eq!(new_date.time_delta(), "1 week and 3 days ago");
}
