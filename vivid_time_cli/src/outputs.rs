use anyhow::{Error, Result};
use serde::Serialize;
use vivid_time::{
    formatting::{
        system_n::{DateFormatter, DepthStyle, TimeFormatter, TimeWithFractionFormatter},
        types::Formatter,
    },
    system_n::time::{Date as VividDate, Time as VividTime, TimeWithFraction as VividTimeWithFraction},
};

fn style_from_flag(long: bool) -> DepthStyle {
    if long {
        DepthStyle::Long
    } else {
        DepthStyle::Short
    }
}

pub fn write_date(data: &VividDate, long: bool) -> Result<String> {
    DateFormatter::standard(" ∘ ", style_from_flag(long))
        .format(data)
        .map_err(Error::new)
}

pub fn write_time(data: &VividTime, long: bool, precision: Option<usize>) -> Result<String> {
    TimeFormatter::standard(" ∘ ", style_from_flag(long), precision)
        .format(data)
        .map_err(Error::new)
}

pub fn write_time_with_fraction(data: &VividTimeWithFraction, long: bool) -> Result<String> {
    TimeWithFractionFormatter::standard(" // ", " ∘ ", style_from_flag(long))
        .format(data)
        .map_err(Error::new)
}

fn output_json<T: Serialize>(t: &T) -> Result<String> {
    serde_json::to_string(t).map_err(Error::from)
}

fn print_output(serialized: String) -> Result<()> {
    println!("{}", serialized);
    Ok(())
}

pub fn print_date(date: &VividDate, long: bool) -> Result<()> {
    print_output(write_date(date, long)?)
}

pub fn print_time(time: &VividTime, long: bool, precision: Option<usize>) -> Result<()> {
    print_output(write_time(time, long, precision)?)
}

pub fn print_time_with_fraction(time: &VividTimeWithFraction, long: bool) -> Result<()> {
    print_output(write_time_with_fraction(time, long)?)
}

pub fn print_json(time: &VividTimeWithFraction) -> Result<()> {
    print_output(output_json(time)?)
}
