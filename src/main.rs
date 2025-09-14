use std::io::stderr;
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        writeln!(stderr(), "Usage: {} <first_monday_date> <file.csv>",args[0]).unwrap();
        std::process::exit(1);
    }
}
