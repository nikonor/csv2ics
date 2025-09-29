use chrono::{Datelike, Days, NaiveDate, Weekday::Mon};
use csv::Reader;
use std::fs::File;
use std::ops::Add;
use std::{io::Write, io::stderr};

struct Day {
    d: NaiveDate,
    title: String,
    description: String,
}

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

    let d = parse_date(args[1].as_str());
    match d {
        Err(_e) => {
            writeln!(stderr(), "Error::{}::date parse error {} ", args[1], _e).unwrap();
            std::process::exit(1);
        }
        Ok(_) => {}
    }
    let mut d = d.unwrap();

    let mut rdr: Reader<File>;

    let tmp = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(args[2].as_str());
    match tmp {
        Err(e) => {
            writeln!(stderr(), "Error::{}::read from::{} ", e, args[2]).unwrap();
            std::process::exit(1);
        }
        Ok(r) => rdr = r,
    }
    for r in rdr.records() {
        // print!("{}-{} => ", d, d.weekday());

        let mut day = Day {
            d,
            title: "".to_string(),
            description: "".to_string(),
        };

        match r {
            Ok(records) => {
                day.title = records[0].to_string();
                day.description = records[1].to_string();
            }
            Err(ee) => {
                writeln!(
                    stderr(),
                    "Error::{}::read string from csv::{} ",
                    ee,
                    args[2]
                )
                .unwrap();
                std::process::exit(1);
            }
        };
        println!("{}=>{}=>{}", day.d, day.title, day.description);

        d = next_day(d);
    }

    // TODO: запись ics-файла
}

fn next_day(d: NaiveDate) -> NaiveDate {
    d.add(Days::new(1))
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
    let valid_date_str = "2023-10-09".to_string();
    let ret = parse_date(valid_date_str.as_str());
    assert!(!ret.is_err())
}

#[test]
fn test_empty_date_string() {
    let empty_date_str = "".to_string();
    assert!(parse_date(empty_date_str.as_str()).is_err());
}
