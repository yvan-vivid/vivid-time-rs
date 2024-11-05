use anyhow::Result;
use dateparser::parse;
use vivid_time::{system_n::time::TimeWithFraction as VividTimeWithFraction, unix::units::Now};

mod cli;
mod outputs;

use cli::{parse_cli, Command};
use outputs::{print_with_options, OutputSpec};

fn main() -> Result<()> {
    let cli = parse_cli();
    match cli.command {
        Command::Json => print_with_options(&VividTimeWithFraction::now(), OutputSpec::Json),
        Command::Now {
            json,
            long,
            precision,
            full,
        } => {
            let output_spec = OutputSpec::from_flags(json, long, full, precision);
            print_with_options(&VividTimeWithFraction::now(), output_spec)
        }
        Command::Today { long } => {
            let output_spec = OutputSpec::from_date_flags(long);
            print_with_options(&VividTimeWithFraction::now(), output_spec)
        }
        Command::To {
            time,
            json,
            long,
            precision,
            full,
        } => {
            let output_spec = OutputSpec::from_flags(json, long, full, precision);
            let vt = VividTimeWithFraction::from(parse(&time)?);
            print_with_options(&vt, output_spec)
        }
    }
}
