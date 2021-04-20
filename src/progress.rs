use chrono::prelude::*;
use chrono::Duration;

// Count the amount of days in specified month
pub fn count_days_of_month(year: i32, month: u32) -> i64 {
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
fn get(current: Date<Local>, start: Date<Local>, end: Date<Local>) -> f64 {
    let whole_diff = end - start;
    let whole_diff_in_seconds = whole_diff.num_days() * 86400 + whole_diff.num_seconds();

    let current_diff = current - start;
    let current_diff_in_seconds = current_diff.num_days() * 86400 + current_diff.num_seconds();
    // progress_ratio
    current_diff_in_seconds as f64 / whole_diff_in_seconds as f64
}

pub fn year() -> f64 {
    let current = Local::today();
    year_ratio(current)
}
pub fn year_ratio(current: Date<Local>) -> f64 {
    let start = Local.ymd(current.year(), 1, 1);
    let end = Local.ymd(current.year() + 1, 1, 1);
    get(current, start, end)
}

pub fn month() -> f64 {
    let current = Local::today();
    month_ratio(current)
}

pub fn month_ratio(current: Date<Local>) -> f64 {
    let start = Local.ymd(current.year(), current.month(), 1);
    let end = {
        let days_num = count_days_of_month(current.year(), current.month());
        if current.month() == 12 {
            Local.ymd(current.year(), current.month(), days_num as u32)
        } else {
            Local.ymd(current.year(), current.month(), days_num as u32)
        }
    };
    get(current, start, end)
}

pub fn week() -> f64 {
    let current = Local::today();
    week_ratio(current)
}

pub fn week_ratio(current: Date<Local>) -> f64 {
    let start = current - Duration::days(current.weekday().num_days_from_monday().into());
    let end = start + Duration::days(6);
    get(current, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_year_ratio() {
        // first day of the year
        let current = Local.ymd(2021, 1, 1);
        let ratio = year_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        // middle day of the year
        let current = Local.ymd(2021, 7, 3);
        let ratio = year_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5013698630136987);
        assert_eq!(ratio_int, 50);

        // last day of the year
        let current = Local.ymd(2021, 12, 31);
        let ratio = year_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.9972602739726028);
        assert_eq!(ratio_int, 99);
    }

    #[test]
    fn test_month_ratio() {
        // first day of the month
        let current = Local.ymd(2021, 1, 1);
        let ratio = month_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        // middle day of the month
        let current = Local.ymd(2021, 1, 15);
        let ratio = month_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.4666666666666667);
        assert_eq!(ratio_int, 46);

        // last day of the month
        let current = Local.ymd(2021, 1, 31);
        let ratio = month_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 1.0);
        assert_eq!(ratio_int, 100);
    }
    #[test]
    fn test_week_ratio() {
        // first day of the week
        let current = Local.ymd(2021, 1, 4);
        let ratio = week_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        // middle day of the week
        let current = Local.ymd(2021, 1, 7);
        let ratio = week_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5);
        assert_eq!(ratio_int, 50);

        // last day of the week
        let current = Local.ymd(2021, 1, 10);
        let ratio = week_ratio(current);
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 1.0);
        assert_eq!(ratio_int, 100);
    }
}
