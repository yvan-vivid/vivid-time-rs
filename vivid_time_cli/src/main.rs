use anyhow::Result;
use vivid_time::{
    system_n::time::{Date as VividDate, Time as VividTime, TimeWithFraction as VividTimeWithFraction},
    unix::units::Now,
};

mod cli;
mod outputs;

use cli::{parse_cli, Command};
use outputs::{print_date, print_json, print_time, print_time_with_fraction};

fn main() -> Result<()> {
    let cli = parse_cli();
    match cli.command {
        Command::Json => print_json(&VividTimeWithFraction::now()),
        Command::Now { long, precision, full } => {
            if full {
                print_time_with_fraction(&VividTimeWithFraction::now(), long)
            } else {
                print_time(&VividTime::now(), long, precision)
            }
        }
        Command::Today { long } => print_date(&VividDate::now(), long),
    }
}
