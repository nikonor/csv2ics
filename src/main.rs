use chrono::{Days, NaiveDate, ParseError};
use std::io::{Write, stderr};
use std::ops::Add; // Modified import to include all needed Chrono traits

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
            writeln!(stderr(), "wrong date {}::{}", args[1], e.to_string()).unwrap();
            std::process::exit(1);
        }
        Ok(_) => {}
    }
    let d = d.unwrap().add(Days::new(1));
    println!("d={}", d);
}
