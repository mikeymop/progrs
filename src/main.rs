use clap::{Parser, Subcommand};

use crate::{
    dates::{Work, Day, Month, Year},
    progress::{Progress, OffsetProgress}
};

mod dates;
mod progress;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    /// counts the quantity of the `-d` flag eg: `-ddd`.
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Prints the progress for a std 9-5 workday
    Work {
        #[arg(short, long)]
        /// Override the 5pm ending time for the workday.
        end: Option<String>,
        /// Override the default timezone.
        timezone: Option<String>, 
    },
    /// Prints the progress for the current day.
    Day {
        /// Override the default utc timezone.
        #[arg(short, long)]
        timezone: Option<String>,
    },
    /// Prints the progress for the current month.
    Month {
        /// Override the default utc timezone.
        #[arg(short, long)]
        timezone: Option<String>,
    },
    /// Prints the progress for the current year.
    Year {
        /// Override the default utc timezone.
        #[arg(short, long)]
        timezone: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    // Debug verbosity level
    // Note: only flags can have multiple occurrences
    match cli.debug {
        0 => {},
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }


    // Match out subcommands.
    match &cli.command {
        Some(Commands::Work { end, timezone }) => {
            let end = end.as_ref();
            let tz = timezone.as_ref();
            let end_of_work = end.unwrap_or(&String::from("17")).as_str()
                .parse::<u32>()
                .unwrap_or(17);
            
            if end.is_some() {
                println!("Working past 5? 🫠");
            }

            if tz.is_some() {
                println!("Using tz of {:?}", tz.unwrap());
            }
            
            let work = Work::new(end_of_work);
            println!("Progress for the workday {:?}%", work.percent());
            work.print();
        }
        Some(Commands::Day { timezone }) => {
            let tz = &timezone.as_ref();

            if tz.is_some() {
                println!("Using tz of {:?}", tz.unwrap());
            }

            let day = Day::new();
            println!("Progress for the day {:?}%", day.percent());
            day.print();
        }
        Some(Commands::Month { timezone }) => {
            let tz = &timezone.as_ref();

            if tz.is_some() {
                println!("Using tz of {:?}", tz.unwrap());
            }

            let month = Month::new();
            println!("Progress for the month {:?}%", month.percent());
            month.print();
        }
        Some(Commands::Year { timezone }) => {
            let tz = &timezone.as_ref();

            if tz.is_some() {
                println!("Using tz of {:?}", tz.unwrap());
            }
            
            let year = Year::new();
            println!("Progress for the year {:?}%", year.percent());
            year.print();
        }
        _ => {
            println!("Can't decide? Heres the progress for the day.");
            let day = Day::new();
            day.print();
        }
    }
}