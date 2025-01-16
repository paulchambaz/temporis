# Natural Date Parser

A flexible Rust library for parsing human-friendly date expressions into `NaiveDate` objects. This library understands a wide variety of date formats and natural language expressions.

## Features

### Standard Date Formats

- YYYY-MM-DD: `2024-01-16`
- DD-MM-YYYY: `16-01-2024`
- With alpha months: `16-Jan-2024`, `2024-Jan-16`
- Short dates: `16/01` (assumes current year)

### Natural Language

- Relative: `today`, `tomorrow`, `yesterday`
- Weekdays: `monday`, `tue`, `wed`
- Next week: `nfriday`, `nmon`
- Numbered weeks: `2monday` (2 Mondays from now)
- Ordinal dates: `1st`, `2nd`, `3rd`, `15th`

### Business Period Markers

- Week markers: `sow` (start of week), `eow` (end of week)
- Month markers: `som` (start of month), `eom` (end of month)
- Quarter markers: `soq` (start of quarter), `eoq` (end of quarter)
- Year markers: `soy` (start of year), `eoy` (end of year)
- Next period markers: `eonw`, `eonm`, `eonq`, `eony` (end of next week/month/quarter/year)

### Relative Time Expressions

- Days: `5d`, `5days`
- Weeks: `2w`, `2weeks`
- Months: `3m`, `3months`
- Years: `1y`, `1year`

## Usage

```rust
use date_parser::parse_date;

fn main() -> Result<(), anyhow::Error> {
    let date = parse_date("tomorrow")?;
    println!("Tomorrow is: {}", date);
    Ok(())
}
```

The library returns a `Result<NaiveDate, anyhow::Error>`, making it easy to handle parsing errors in your application.
