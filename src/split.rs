
use chrono::Duration;

macro_rules! make_time_period {
    ($e:expr, $s:ident) => (
        #[derive(Debug, Eq, PartialEq)]
        pub struct $s(u64);

        impl $s {
            pub fn new(val: u64) -> $s {
                return $s(val);
            }
        }

        impl TimePeriod for $s {
            fn name(&self) -> &str {
                return $e;
            }
            fn value(&self) -> u64 {
                let $s(v) = *self;
                return v;
            }
        }
    );
}

make_time_period!("year", PeriodYear);
make_time_period!("month", PeriodMonth);
make_time_period!("week", PeriodWeek);
make_time_period!("day", PeriodDay);
make_time_period!("hour", PeriodHour);
make_time_period!("minute", PeriodMinute);
make_time_period!("second", PeriodSecond);
make_time_period!("millisecond", PeriodMillisecond);

pub trait TimePeriod {
    fn name(&self) -> &str;
    fn value(&self) -> u64;

    fn is_plural(&self) -> bool {
        return !(self.value() == 1);
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct SplitDuration {
    pub years: PeriodYear,
    pub months: PeriodMonth,
    pub weeks: PeriodWeek,
    pub days: PeriodDay,
    pub hours: PeriodHour,
    pub minutes: PeriodMinute,
    pub seconds: PeriodSecond,
    pub milliseconds: PeriodMillisecond,
}

impl SplitDuration {
    pub fn new(years: u64, months: u64, weeks: u64, days: u64,
        hours: u64, minutes: u64, seconds: u64, milliseconds: u64) -> SplitDuration {
        return SplitDuration {
            years: PeriodYear::new(years),
            months: PeriodMonth::new(months),
            weeks: PeriodWeek::new(weeks),
            days: PeriodDay::new(days),
            hours: PeriodHour::new(hours),
            minutes: PeriodMinute::new(minutes),
            seconds: PeriodSecond::new(seconds),
            milliseconds: PeriodMillisecond::new(milliseconds),
        }
    }

    pub fn as_vec(&self) -> Vec<u64> {
        return vec![self.years.value(), self.months.value(), self.weeks.value(), self.days.value(),
        self.hours.value(), self.minutes.value(), self.seconds.value(), self.milliseconds.value()];
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
