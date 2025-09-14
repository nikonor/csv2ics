use chrono::{Datelike, Days, NaiveDate, ParseError, Weekday::Mon};
use std::io::{Write, stderr};
use std::ops::Add;

fn parse_date(s: String) -> Result<NaiveDate, ParseError> {
    match NaiveDate::parse_from_str(s.as_str(), "%Y-%m-%d") {
        Err(e) => Err(e),
        Ok(d) => Ok(d),
    }
}

#[test]
fn test_parse_valid_date() {
    let valid_date_str = "2023-10-09".to_string();
    let ret = parse_date(valid_date_str);
    assert!(!ret.is_err())
}

#[test]
fn test_empty_date_string() {
    let empty_date_str = "".to_string();
    assert!(parse_date(empty_date_str).is_err());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        writeln!(
            stderr(),
            "Usage: {} <first_monday_date> <file.csv>",
            args[0]
        )
        .unwrap();
        std::process::exit(1);
    }

    let d = parse_date(args[1].to_string());
    match d {
        Err(e) => {
            writeln!(stderr(), "Error::wrong date {}::{}", args[1], e.to_string()).unwrap();
            std::process::exit(1);
        }
        Ok(d) => {
            if d.weekday() != Mon {
                writeln!(stderr(), "Error::{} is not monday", args[1]).unwrap();
                std::process::exit(1);
            }
        }
    }
    let d = d.unwrap().add(Days::new(1));
    println!("d={}", d);
    println!("d={}", d.weekday());
}
