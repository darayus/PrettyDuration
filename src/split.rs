
use chrono::Duration;

pub struct SplitDuration {
    pub years: u64,
    pub months: u64,
    pub weeks: u64,
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
    pub milliseconds: u64,
}

impl SplitDuration {
    pub fn new(years: u64, months: u64, weeks: u64, days: u64,
        hours: u64, minutes: u64, seconds: u64, milliseconds: u64) -> SplitDuration {
        return SplitDuration {
            years: years,
            months: months,
            weeks: weeks,
            days: days,
            hours: hours,
            minutes: minutes,
            seconds: seconds,
            milliseconds: milliseconds,
        }
    }

    pub fn as_vec(&self) -> Vec<u64> {
        return vec![self.years, self.months, self.weeks, self.days,
        self.hours, self.minutes, self.seconds, self.milliseconds];
    }
}

pub fn split_duration(dur: Duration) -> SplitDuration {
    // TODO: Handle negative durations
    let mut split = Vec::with_capacity(8);

    // Calculate the years
    let num_days = dur.num_days();
    let years = num_days / 365;
    split.push(years);
    let dur = dur - Duration::days(years * 365);

    // Calculate the months
    let num_days = dur.num_days();
    let months = num_days / 30;
    split.push(months);
    let dur = dur - Duration::days(months * 30);

    // Calculate the weeks
    let weeks = dur.num_weeks();
    split.push(weeks);
    let dur = dur - Duration::weeks(weeks);

    // Calculate the days
    let days = dur.num_days();
    split.push(days);
    let dur = dur - Duration::days(days);

    // Calculate the hours
    let hours = dur.num_hours();
    split.push(hours);
    let dur = dur - Duration::hours(hours);

    // Calculate the minutes
    let minutes = dur.num_minutes();
    split.push(minutes);
    let dur = dur - Duration::minutes(minutes);

    // Calculate the seconds
    let seconds = dur.num_seconds();
    split.push(seconds);
    let dur = dur - Duration::seconds(seconds);

    // Calculate the milliseconds
    let milliseconds = dur.num_milliseconds();
    split.push(milliseconds);

    return SplitDuration::new(years as u64, months as u64, weeks as u64, days as u64,
        hours as u64, minutes as u64, seconds as u64, milliseconds as u64);
}

#[test]
fn test_split_duration_year_correct() {
    let dur = Duration::days(365);
    assert_eq!(split_duration(dur).as_vec(), vec![1, 0, 0, 0, 0, 0, 0, 0]);

    let dur = Duration::days(366);
    assert_eq!(split_duration(dur).as_vec(), vec![1, 0, 0, 1, 0, 0, 0, 0]);
}

#[test]
fn test_split_duration_hour_correct() {
    let dur = Duration::hours(2);
    assert_eq!(split_duration(dur).as_vec(), vec![0, 0, 0, 0, 2, 0, 0, 0]);

    let dur = Duration::hours(25);
    assert_eq!(split_duration(dur).as_vec(), vec![0, 0, 0, 1, 1, 0, 0, 0]);
}

#[test]
fn test_split_duration_none_correct() {
    let dur = Duration::hours(0);
    assert_eq!(split_duration(dur).as_vec(), vec![0, 0, 0, 0, 0, 0, 0, 0]);

    let dur = Duration::hours(25);
    assert_eq!(split_duration(dur).as_vec(), vec![0, 0, 0, 1, 1, 0, 0, 0]);
}
