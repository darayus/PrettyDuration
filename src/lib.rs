extern crate chrono;

mod split;

use chrono::Duration;

pub fn pretty_full(dur: Duration) -> String {
    let components = split::split_duration(dur);
    let mut final_str = String::new();

    for (i, component) in render_components(components.as_vec()).iter().enumerate() {
        if i != 0 {
            final_str.push_str(", ");
        }
        final_str.push_str(&component);
    }

    return final_str;
}

fn render_components(components: Vec<u64>) -> Vec<String> {
    let mut renders = Vec::with_capacity(components.len());
    renders.push(pluralise(components[0], "year"));
    renders.push(pluralise(components[1], "month"));
    renders.push(pluralise(components[2], "week"));
    renders.push(pluralise(components[3], "day"));
    renders.push(pluralise(components[4], "hour"));
    renders.push(pluralise(components[5], "minute"));
    renders.push(pluralise(components[6], "second"));
    renders.push(pluralise(components[7], "millisecond"));
    return renders;
}

fn pluralise(val: u64, str_name: &str) -> String {
    let plural_end = if val != 1 {
        "s"
    } else {
        ""
    };
    return format!("{} {}{}", val, str_name, plural_end);
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
fn test_pretty_full_simple_plural() {
}
