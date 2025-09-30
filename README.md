# cvs2ics

A simple CLI utility for converting a CSV file into an ICS calendar file.

## Description

The tool reads a CSV file with **two columns**:

1. **Title** – the event name  
2. **Description** – optional event description  

Each record in the CSV is converted into a calendar event (`.ics`).  
The first record corresponds to the given **first Monday date**, the second record to the next day, and so on.

## Usage

```bash
cvs2ics <first_monday_date> <input.csv> <output.ics>
```

* `<first_monday_date>` — the date of the first Monday (format: `YYYY-MM-DD`)
* `<input.csv>` — path to the source CSV file
* `<output.ics>` — path where the resulting ICS file will be written

Example:

```bash
cvs2ics 2023-10-09 events.csv calendar.ics
```

This will:

* Place the first record of `events.csv` on **2023-10-09**
* The second record on **2023-10-10**
* The third record on **2023-10-11**, etc.

## Notes

* The first date **must be a Monday**, otherwise the program will exit with an error.
* If a CSV row has only one field, it will be used as the event title, and the description will be left empty.
* Empty CSV rows are not allowed.

## Build

```bash
cargo build --release
```
