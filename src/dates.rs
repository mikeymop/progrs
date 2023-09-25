#[allow(dead_code)]
pub mod dates {
    use std::{time::Duration, thread};
    use chrono::{Local, Datelike};
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

    fn mk_progress(min: u32, max: u32, curr: u32) {
        let pb = ProgressBar::new(u64::from(max));

        for _ in min..curr {
            thread::sleep(Duration::from_millis(50));
            pb.inc(1);
        }

        pb.abandon_with_message("Progress for the year") // pb.finish_with_message("done");
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
        mk_progress(start_of_month.day0(), end_of_month.day(), local.day());
        get_percent(local.day(), end_of_month.day())
    }
}
