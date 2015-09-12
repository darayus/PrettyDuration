extern crate chrono;

mod split;

use chrono::Duration;

pub fn pretty_short(dur: Duration) -> String {
    let components = split::split_duration(dur).as_vec();

    let mut components_iter = components.iter().skip_while(|a| a.val() == 0);
    let first_component = components_iter.next();
    let second_component = components_iter.next();

    let final_str = match first_component {
        Some(x) => {
            match second_component {
                Some(y) => {format!("{} and {}", x.to_string(), y.to_string())},
                None => {x.to_string()}
            }
        },
        // The duration is 0
        None => {split::TimePeriod::Millisecond(0).to_string()},
    };

    return final_str;
}

pub fn pretty_full(dur: Duration) -> String {
    let components = split::split_duration(dur);
    let mut final_str = String::new();

    for (i, component) in components.as_vec().iter().enumerate() {
        if i != 0 {
            final_str.push_str(", ");
        }
        final_str.push_str(&component.to_string());
    }

    return final_str;
}

#[test]
fn test_pretty_full_simple() {
    let test_data = vec![
        (Duration::days(365), "1 year, 0 months, 0 weeks, 0 days, 0 hours, 0 minutes, 0 seconds, 0 milliseconds"),
        (Duration::days(30), "0 years, 1 month, 0 weeks, 0 days, 0 hours, 0 minutes, 0 seconds, 0 milliseconds"),
        (Duration::weeks(1), "0 years, 0 months, 1 week, 0 days, 0 hours, 0 minutes, 0 seconds, 0 milliseconds"),
        (Duration::days(1), "0 years, 0 months, 0 weeks, 1 day, 0 hours, 0 minutes, 0 seconds, 0 milliseconds"),
        (Duration::hours(1), "0 years, 0 months, 0 weeks, 0 days, 1 hour, 0 minutes, 0 seconds, 0 milliseconds"),
        (Duration::minutes(1), "0 years, 0 months, 0 weeks, 0 days, 0 hours, 1 minute, 0 seconds, 0 milliseconds"),
        (Duration::seconds(1), "0 years, 0 months, 0 weeks, 0 days, 0 hours, 0 minutes, 1 second, 0 milliseconds"),
        (Duration::milliseconds(1), "0 years, 0 months, 0 weeks, 0 days, 0 hours, 0 minutes, 0 seconds, 1 millisecond"),
    ];

    for (dur, final_str) in test_data {
        assert_eq!(pretty_full(dur), final_str);
    }
}

#[test]
fn test_pretty_short() {
    let test_data = vec![
        (Duration::milliseconds(0), "0 milliseconds"),
        (Duration::milliseconds(1), "1 millisecond"),
        (-Duration::milliseconds(1), "1 millisecond"),
        (Duration::milliseconds(200), "200 milliseconds"),
        (Duration::seconds(1) + Duration::milliseconds(200), "1 second and 200 milliseconds"),
        (Duration::days(1) + Duration::hours(2), "1 day and 2 hours"),
        (Duration::days(1) + Duration::seconds(2), "1 day and 0 hours"),
    ];

    for (dur, final_str) in test_data {
        assert_eq!(pretty_short(dur), final_str);
    }
}
