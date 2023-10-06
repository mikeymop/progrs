use crate::progress::{Progress, OffsetProgress};
use chrono::{DateTime, Datelike, Local, Timelike};
use indicatif::ProgressBar;
use std::{cmp, thread, time::Duration};

/// Returns a NaiveDate for the start of the supplied month.
fn start_of_month(year: i32, month: u32) -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::from_ymd_opt(year, month, 1)
}

/// Returns a NaiveDate for the end of the supplied month.
fn last_of_month(year: i32, month: u32) -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::from_ymd_opt(year, month + 1, 1)
        .or_else(|| chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1))?
        .pred_opt()
}

/// Returns the current day of the year
fn day_of_year(dt: DateTime<Local>) -> u32 {
    let start = chrono::NaiveDate::from_ymd_opt(dt.year(), 1, 1).unwrap();
    let dn = dt.date_naive();
    let diff = dn - start;
    (diff.num_days() + 1) as u32
}

/// Generates a progress bar.
fn mk_progress(min: u32, max: u32, curr: u32, dur: Option<u64>) {
    let pb = ProgressBar::new(u64::from(max));
    let end = cmp::min(curr, max);

    for _ in min..end {
        // print!(" {} ", i);
        let wait = dur.unwrap_or(50);
        thread::sleep(Duration::from_millis(wait));
        pb.inc(1);
    }

    pb.abandon_with_message("Progress for the year");
}

fn get_percent(curr: u32, max: u32) -> u32 {
    let percent = f64::from(curr) / f64::from(max) * 100 as f64;
    println!(
        "curr: {}, max: {}, percent: {} ({})",
        curr,
        max,
        percent.floor(),
        percent
    );
    cmp::min(percent.floor() as u32, 100 as u32) as u32
}

pub struct Day {
    start: u32,
    end: u32,
    curr: u32,
}

impl Progress for Day {
    fn new() -> Self {
        let start = 0;
        let end = 24;
        let curr = Local::now().hour();
        Self { start, end, curr }
    }

    fn percent(&self) -> u32 {
        get_percent(self.curr, self.end)
    }

    fn print(&self) {
        mk_progress(self.start, self.end, self.curr, Some(10));
    }
}

pub struct Month {
    start: u32,
    end: u32,
    curr: u32,
}

impl Progress for Month {
    fn new() -> Self {
        let curr = Local::now();
        let start = start_of_month(curr.year(), curr.month()).unwrap().day0();
        let end = last_of_month(curr.year(), curr.month()).unwrap().day();
        Self {
            start,
            end,
            curr: curr.day(),
        }
    }

    fn percent(&self) -> u32 {
        get_percent(self.curr, self.end)
    }

    fn print(&self) {
        mk_progress(self.start, self.end, self.curr, Some(10));
    }
}

pub struct Year {
    start: u32,
    end: u32,
    curr: u32,
}

impl Progress for Year {
    fn new() -> Self {
        let now = Local::now();
        let today = day_of_year(now);
        // let t =
        Self {
            start: 0,
            end: 365,
            curr: today,
        }
    }

    fn percent(&self) -> u32 {
        get_percent(self.curr, self.end)
    }

    fn print(&self) {
        mk_progress(self.start, self.end, self.curr, Some(1));
    }
}

pub struct Work {
    start: u32,
    end: u32,
    curr: u32,
}

impl OffsetProgress for Work {
    fn new(end_of_work: u32) -> Self {
        // let max = 8;
        let now = Local::now();
        let end = chrono::NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
            .unwrap()
            .and_hms_opt(cmp::min(end_of_work, 23), 0, 0)
            .unwrap();

        Work {
            start: 0,
            end: end.hour(),
            curr: now.hour()
        }
    }

    fn percent(&self) -> u32 {
        get_percent(self.curr, self.end)
    }
    
    fn print(&self) {
        // mk_progress(0 as u32, max, cmp::min(now.hour(), max), Some(10));
        mk_progress(self.start, self.end, self.curr, Some(10));
    }
}
