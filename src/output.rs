use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Display;
use std::io::Write;
use std::str::FromStr;

use serde::Serialize;

#[derive(Debug)]
pub enum Format {
    JSON,
    Plain,
}

impl TryFrom<&str> for Format {
    type Error = String;

    fn try_from(value: &str) -> Result<Format, String> {
        match value.to_lowercase().as_str() {
            "json" => Ok(Format::JSON),
            "plain" => Ok(Format::Plain),
            _ => Err(format!("Not a recognised value for format: {}", &value)),
        }
    }
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Format, String> {
        TryFrom::try_from(s)
    }
}

pub fn progress_bar(p: u32, ps: u32) {
    let columns: u32 = match term_size::dimensions() {
        Some((w, _)) => w as u32,
        None => return,
    };

    let width: u32 = columns - 11;

    let progress = ((p as f64) / (ps as f64) * (width as f64)) as u32;
    let remaining = width - progress;

    let line = format!(
        " [ {}{} ] {:>3}% ",
        "=".repeat(progress as usize),
        " ".repeat(remaining as usize),
        ((p as f64) / (ps as f64) * 100f64) as u32,
    );

    print!("\r{}", &line);
    std::io::stdout().flush().expect("Failed to flush stdio.");
}

pub fn print_server_msgs(caveats: &[HashMap<String, String>]) {
    if !caveats.is_empty() {
        println!("Server messages:");
        for caveat in caveats {
            for (key, value) in caveat {
                println!("{}: {}", key, value);
            }
        }
    }
}

pub fn json_pretty_print<T: ?Sized>(input: &T) -> Result<(), String>
where
    T: Serialize,
{
    let pretty = match serde_json::to_string_pretty(input) {
        Ok(string) => string,
        Err(_) => return Err(String::from("Failed to format output as JSON.")),
    };

    println!("{}", &pretty);
    Ok(())
}

pub fn pretty_output<T>(input: &T, format: Format) -> Result<(), String>
where
    T: Serialize + Display,
{
    match format {
        Format::Plain => println!("{}", input),
        Format::JSON => json_pretty_print(input)?,
    }

    Ok(())
}

pub fn pretty_outputs<T>(input: &[T], format: Format) -> Result<(), String>
where
    T: Serialize + Display,
{
    match format {
        Format::Plain => {
            for item in input {
                println!("{}", item);
            }
        },
        Format::JSON => {
            json_pretty_print(input)?;
        },
    }

    Ok(())
}
