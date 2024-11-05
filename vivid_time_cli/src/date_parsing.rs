use std::sync::LazyLock;

use anyhow::Result;
use dateparser::{parse, parse_with};

use chrono::{DateTime, NaiveTime, Utc};

static ZERO_CLOCK: LazyLock<NaiveTime> = LazyLock::new(|| NaiveTime::from_hms_opt(9, 30, 0).unwrap());

pub fn parse_date(date_str: &str) -> Result<DateTime<Utc>> {
    parse_with(date_str, &Utc, *ZERO_CLOCK)
}

pub fn parse_time(date_str: &str) -> Result<DateTime<Utc>> {
    parse(date_str)
}

#[cfg(test)]
mod test {
    use super::{parse_date, parse_time};
    use chrono::{Local, TimeZone, Utc};

    #[test]
    fn parse_date_success() {
        assert_eq!(
            Utc.with_ymd_and_hms(2024, 2, 1, 9, 30, 0).unwrap(),
            parse_date("2024-02-01").unwrap()
        )
    }

    #[test]
    fn parse_time_success() {
        assert_eq!(
            Local.with_ymd_and_hms(2024, 2, 1, 10, 12, 13).unwrap(),
            parse_time("2024-02-01 10:12:13").unwrap()
        )
    }
}
