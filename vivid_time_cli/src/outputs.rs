use anyhow::{Error, Result};
use serde::Serialize;
use vivid_time::{
    formatting::{
        system_n::{DateFormatter, DepthStyle, TimeFormatter, TimeWithFractionFormatter},
        types::Formatter,
    },
    system_n::time::{Date as VividDate, Time as VividTime, TimeWithFraction as VividTimeWithFraction},
};

pub struct DateOptions {
    pub style: DepthStyle,
}

pub enum TimeOptions {
    FullWithFraction,
    Full,
    Precision(usize),
}

pub enum OutputSpec {
    Json,
    Date(DateOptions),
    // Clock(TimeOptions),
    Time { date: DateOptions, time: TimeOptions },
}

impl DateOptions {
    pub fn from_flags(long: bool) -> Self {
        Self {
            style: if long { DepthStyle::Long } else { DepthStyle::Short },
        }
    }
}

impl TimeOptions {
    pub fn from_flags(full: bool, precision: Option<usize>) -> Self {
        if full {
            if precision.is_some() {
                println!("Warning: using '--full' output, '--precision' is ignored")
            }
            return Self::FullWithFraction;
        }
        precision.map(Self::Precision).unwrap_or(Self::Full)
    }
}

impl OutputSpec {
    pub fn from_flags(json: bool, long: bool, full: bool, precision: Option<usize>) -> Self {
        if json {
            if long || full || precision.is_some() {
                println!("Warning: using '--json' output, other flags ignored")
            }
            return Self::Json;
        }
        Self::Time {
            date: DateOptions::from_flags(long),
            time: TimeOptions::from_flags(full, precision),
        }
    }

    pub fn from_date_flags(long: bool) -> Self {
        Self::Date(DateOptions::from_flags(long))
    }
}

pub fn write_date(data: &VividDate, style: DepthStyle) -> Result<String> {
    DateFormatter::standard(" ∘ ", style).format(data).map_err(Error::new)
}

pub fn write_time(data: &VividTime, style: DepthStyle, precision: Option<usize>) -> Result<String> {
    TimeFormatter::standard(" ∘ ", style, precision)
        .format(data)
        .map_err(Error::new)
}

pub fn write_time_with_fraction(data: &VividTimeWithFraction, style: DepthStyle) -> Result<String> {
    TimeWithFractionFormatter::standard(" // ", " ∘ ", style)
        .format(data)
        .map_err(Error::new)
}

fn output_json<T: Serialize>(t: &T) -> Result<String> {
    serde_json::to_string(t).map_err(Error::from)
}

pub fn print_with_options(twf: &VividTimeWithFraction, options: OutputSpec) -> Result<()> {
    let output = match options {
        OutputSpec::Json => output_json(twf)?,
        OutputSpec::Date(DateOptions { style }) => write_date(&twf.time.date, style)?,
        OutputSpec::Time {
            date: DateOptions { style },
            time,
        } => match time {
            TimeOptions::FullWithFraction => write_time_with_fraction(twf, style)?,
            TimeOptions::Full => write_time(&twf.time, style, None)?,
            TimeOptions::Precision(precision) => write_time(&twf.time, style, Some(precision))?,
        },
    };
    println!("{}", output);
    Ok(())
}
