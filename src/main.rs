use chrono::{Datelike, Days, NaiveDate, Weekday::Mon};
use std::io::{Write, stderr};
use std::ops::Add;

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
        Err(_e) => {
            writeln!(stderr(), "Error::{}::date parse error {} ", args[1], _e).unwrap();
            std::process::exit(1);
        }
        Ok(_) => {}
    }

    let mut d = d.unwrap();
    for _ in 0..10 {
        println!("{}-{}", d, d.weekday());
        d = next_day(d);
    }
}

fn next_day(d: NaiveDate) -> NaiveDate {
    d.add(Days::new(1))
}

fn parse_date(s: String) -> Result<NaiveDate, String> {
    match NaiveDate::parse_from_str(s.as_str(), "%Y-%m-%d") {
        Err(e) => Err(e.to_string()),
        Ok(d) => {
            if d.weekday() == Mon {
                return Ok(d);
            }
            Err(format!("{} is not Monday", s))
        }
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
