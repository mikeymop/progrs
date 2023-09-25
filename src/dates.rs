#[allow(dead_code)]
pub mod dates {
    use std::{time::Duration, thread};
    use chrono::{Local, Datelike, DateTime};
    use indicatif::ProgressBar;

    /// Returns a NaiveDate for the start of the supplied month.
    fn start_of_month(year: i32, month: u32) -> Option<chrono::NaiveDate> {
        chrono::NaiveDate::from_ymd_opt(year, month, 1)
    }

    #[allow(dead_code)]
    /// Returns a NaiveDate for the end of the supplied month.
    fn last_of_month(year: i32, month: u32) -> Option<chrono::NaiveDate> {
        chrono::NaiveDate::from_ymd_opt(year, month + 1, 1)
            .or_else(|| chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1))?
            .pred_opt()
    }

    fn day_of_year(dt: DateTime<Local>) -> u32 {
        let start = chrono::NaiveDate::from_ymd_opt(dt.year(), 1, 1).unwrap();
        let dn = dt.date_naive();
        let diff = dn - start;
        (diff.num_days() + 1) as u32

    }

    fn mk_progress(min: u32, max: u32, curr: u32, dur: Option<u64>) {
        let pb = ProgressBar::new(u64::from(max));

        for _ in min..curr {
            let wait = dur.unwrap_or(50);
            thread::sleep(Duration::from_millis(wait));
            pb.inc(1);
        }

        pb.abandon_with_message("Progress for the year")
    }

    fn get_percent(curr: u32, max: u32) -> u32 {
        let percent = f64::from(curr) / f64::from(max) * 100 as f64;
        // println!("curr: {}, max: {}, percent: {} ({})", curr, max, percent.floor(), percent);
        percent.floor() as u32
    }

    pub fn gen_month() -> u32 {
        let local = Local::now();
        let start_of_month = start_of_month(local.year(), local.month()).unwrap();
        let end_of_month = last_of_month(local.year(), local.month()).unwrap();
        // println!("now: {:?}\nstart of month: day {:?}\nend of month: day {:?}", local, start_of_month.day(), end_of_month.day());
        mk_progress(start_of_month.day0(), end_of_month.day(), local.day(), Some(10));
        get_percent(local.day(), end_of_month.day())
    }

    pub fn gen_year() -> u32 {
        let local = Local::now();
        let day = day_of_year(local);
        mk_progress(0, 365, day, Some(1));
        get_percent(day, 365)
    }
}
