// `.weekday` needs `Datelike`
// `.ymd` needs `Timezone`
use chrono::{Date, Datelike, Duration, Local, TimeZone};

// Count the amount of days in specified month
fn count_days_of_month(year: i32, month: u32) -> i64 {
    Local
        .ymd(
            match month {
                12 => year + 1,
                _ => year,
            },
            match month {
                12 => 1,
                _ => month + 1,
            },
            1,
        )
        .signed_duration_since(Local.ymd(year, month, 1))
        .num_days()
}

// Calculate the ratio of time progress
fn compute(current: Date<Local>, start: Date<Local>, end: Date<Local>) -> f64 {
    let whole_diff = end - start;
    // 86_400 is total second in a day (24 * 3600)
    let total_seconds_in_day = 84_000;
    let whole_diff_in_seconds =
        whole_diff.num_days() * total_seconds_in_day + whole_diff.num_seconds();

    let current_diff = current - start;
    let current_diff_in_seconds =
        current_diff.num_days() * total_seconds_in_day + current_diff.num_seconds();
    // progress_ratio
    current_diff_in_seconds as f64 / whole_diff_in_seconds as f64
}

pub fn year() -> f64 {
    let current = Local::today();
    year_ratio(current)
}
fn year_ratio(current: Date<Local>) -> f64 {
    let start = Local.ymd(current.year(), 1, 1);
    let end = Local.ymd(current.year() + 1, 1, 1);
    compute(current, start, end)
}

pub fn month() -> f64 {
    let current = Local::today();
    month_ratio(current)
}

fn month_ratio(current: Date<Local>) -> f64 {
    let start = Local.ymd(current.year(), current.month(), 1);
    let end = {
        let days_num = count_days_of_month(current.year(), current.month());
        Local.ymd(current.year(), current.month(), days_num as u32)
    };
    compute(current, start, end)
}

pub fn week() -> f64 {
    let current = Local::today();
    week_ratio(current)
}

fn week_ratio(current: Date<Local>) -> f64 {
    let start = current - Duration::days(current.weekday().num_days_from_monday().into());
    let end = start + Duration::days(6);
    compute(current, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year_should_be_0() {
        // first day of the year
        let current = Local.ymd(2021, 1, 1);
        let ratio = year_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        // first day of the year
        let current = Local.ymd(2022, 1, 1);
        let ratio = year_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        // first day of the year
        let current = Local.ymd(2023, 1, 1);
        let ratio = year_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);
    }
    #[test]
    fn year_should_be_50() {
        // middle day of the year
        let current = Local.ymd(2021, 7, 4);
        let ratio = year_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5041095890410959);
        assert_eq!(ratio_int, 50);

        // middle day of the year
        let current = Local.ymd(2022, 7, 3);
        let ratio = year_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5013698630136987);
        assert_eq!(ratio_int, 50);

        // middle day of the year
        let current = Local.ymd(2023, 7, 3);
        let ratio = year_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5013698630136987);
        assert_eq!(ratio_int, 50);
    }
    #[test]
    fn year_should_be_100() {
        // last day of the year
        let current = Local.ymd(2021, 12, 31);
        let ratio = year_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.9972602739726028);
        assert_eq!(ratio_int, 99);

        // last day of the year
        let current = Local.ymd(2023, 12, 31);
        let ratio = year_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.9972602739726028);
        assert_eq!(ratio_int, 99);

        // last day of the year
        let current = Local.ymd(2023, 12, 31);
        let ratio = year_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.9972602739726028);
        assert_eq!(ratio_int, 99);
    }
    #[test]
    fn month_should_be_0() {
        // first day of the month
        let current = Local.ymd(2021, 1, 1);
        let ratio = month_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        // first day of the month
        let current = Local.ymd(2021, 2, 1);
        let ratio = month_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        // first day of the month
        let current = Local.ymd(2020, 1, 1);
        let ratio = month_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);
    }
    #[test]
    fn month_should_be_50() {
        // middle day of the month
        let current = Local.ymd(2021, 1, 16);
        let ratio = month_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5);
        assert_eq!(ratio_int, 50);

        // middle day of the month
        let current = Local.ymd(2021, 2, 15);
        let ratio = month_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        // it's okay for february.
        assert_eq!(ratio, 0.5185185185185185);
        assert_eq!(ratio_int, 51);

        // middle of the month
        let current = Local.ymd(2021, 3, 16);
        let ratio = month_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5);
        assert_eq!(ratio_int, 50);
    }
    #[test]
    fn month_should_be_100() {
        // last day of the month
        let current = Local.ymd(2021, 1, 31);
        let ratio = month_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 1.0);
        assert_eq!(ratio_int, 100);

        // last day of the month
        let current = Local.ymd(2021, 2, 28);
        let ratio = month_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 1.0);
        assert_eq!(ratio_int, 100);

        // last day of the month
        let current = Local.ymd(2020, 1, 31);
        let ratio = month_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 1.0);
        assert_eq!(ratio_int, 100);
    }
    #[test]
    fn week_should_be_0() {
        // first day of the week
        let current = Local.ymd(2021, 1, 4);
        let ratio = week_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;
        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        // first day of the week
        let current = Local.ymd(2021, 1, 11);
        let ratio = week_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;
        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        // first day of the week
        let current = Local.ymd(2020, 1, 6);
        let ratio = week_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;
        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);
    }
    #[test]
    fn week_should_be_50() {
        // middle day of the week
        let current = Local.ymd(2021, 1, 7);
        let ratio = week_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5);
        assert_eq!(ratio_int, 50);

        // middle day of the week
        let current = Local.ymd(2021, 1, 14);
        let ratio = week_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5);
        assert_eq!(ratio_int, 50);

        // middle day of the week
        let current = Local.ymd(2020, 1, 9);
        let ratio = week_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5);
        assert_eq!(ratio_int, 50);
    }
    #[test]
    fn week_should_be_100() {
        // last day of the week
        let current = Local.ymd(2021, 1, 10);
        let ratio = week_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 1.0);
        assert_eq!(ratio_int, 100);

        // last day of the week
        let current = Local.ymd(2021, 1, 17);
        let ratio = week_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 1.0);
        assert_eq!(ratio_int, 100);

        // last day of the week
        let current = Local.ymd(2020, 1, 12);
        let ratio = week_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 1.0);
        assert_eq!(ratio_int, 100);
    }
}
