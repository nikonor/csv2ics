use chrono::{Datelike, NaiveDate, Weekday::Mon};
use ics::components::Property;
use ics::properties::{Comment, Summary};
use ics::{Event, ICalendar};
use std::{io::Write, io::stderr};
use uuid::Uuid;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        writeln!(
            stderr(),
            "Usage: {} <first_monday_date> <file.csv> <file.ics>",
            args[0]
        )
        .unwrap();
        std::process::exit(1);
    }

    match run(args[1].as_str(), args[2].as_str(), args[3].as_str()) {
        Err(e) => {
            eprintln!("Error::{} ", e);
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}

fn run(first_monday: &str, csv_path: &str, ics_path: &str) -> Result<(), String> {
    let mut d = parse_date(first_monday).map_err(|e| format!("parse_date error={}", e))?;

    let mut calendar = ICalendar::new("2.0", "ics-rs");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(csv_path)
        .map_err(|e| format!("read csv error={}", e))?;

    for (idx, r) in rdr.records().enumerate() {
        let records = r.map_err(|e| format!("CSV error at line {}: {}", idx + 1, e))?;

        if records.is_empty() {
            return Err(format!("CSV line {} is empty", idx + 1));
        }

        let start = d.format("%Y%m%d").to_string();
        let end = d.succ_opt().unwrap().format("%Y%m%d").to_string();

        let mut event = Event::new(
            Uuid::new_v4().to_string(),
            d.format("%Y%m%dT000000").to_string(),
        );

        event.push(Summary::new(records[0].to_string()));
        if records.len() > 1 {
            event.push(Comment::new(records[1].to_string()));
        }
        event.push(Property::new("DTSTART;VALUE=DATE", start));
        event.push(Property::new("DTEND;VALUE=DATE", end));

        calendar.add_event(event);

        d = d.succ_opt().unwrap();
    }

    // запись ics-файл
    calendar
        .save_file(ics_path)
        .map_err(|e| format!("save ics file error={}", e))?;

    Ok(())
}

fn parse_date(s: &str) -> Result<NaiveDate, String> {
    match NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        Err(e) => Err(e.to_string()),
        Ok(d) => {
            if d.weekday() == Mon {
                Ok(d)
            } else {
                Err(format!("{} is not Monday", s))
            }
        }
    }
}

#[test]
fn test_parse_valid_date() {
    assert!(parse_date("2023-10-09").is_ok())
}

#[test]
fn test_parse_not_monday() {
    assert!(parse_date("2023-10-10").is_err())
}

#[test]
fn test_empty_date_string() {
    assert!(parse_date("").is_err());
}
