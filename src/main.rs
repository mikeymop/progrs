use clap::{Parser, Subcommand};

use crate::dates::dates::{gen_month};

mod dates;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    // name: Option<String>,

    /// Turn debugging information on
    /// counts the quantity of the `-d` flag eg: `-ddd`.
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
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
    // println!("Command: {:?}", &cli.command);

    // You can check the value provided by positional arguments, or option arguments
    // if let Some(name) = cli.name.as_deref() {
    //     println!("Value for name: {name}");
    // }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => {},
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }


    // Match out subcommands.
    match &cli.command {
        Some(Commands::Day { timezone }) => {
            let tz = &timezone.as_ref();
            println!("Progress for the day in {:?}", tz);
        }
        Some(Commands::Month { timezone }) => {
            let tz = &timezone.as_ref();
            let percent = gen_month();
            if tz.is_some() {
                println!("Using tz of {:?}", tz.unwrap());
            }
            println!("Progress for the month {:?}%", percent);
        }
        Some(Commands::Year { timezone }) => {
            let tz = &timezone.as_ref();
            println!("Progress for the year in {:?}", tz);
        }
        _ => {
            println!("Can't decide? Heres the progress for the day.")
        }
    }

    // Continued program logic goes here...
}