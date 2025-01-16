use anyhow::{anyhow, Result};
use chrono::Datelike;
use chrono::{DateTime, Duration, Local, NaiveDate, Weekday};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref NEXT_WEEKDAY_REGEX: Regex = Regex::new(r"^n(monday|mon|tuesday|tue|wednesday|wed|thursday|thu|friday|fri|saturday|sat|sunday|sun)$").unwrap();
    static ref NUMBERED_WEEKDAY_REGEX: Regex = Regex::new(r"^(\d+)(monday|mon|tuesday|tue|wednesday|wed|thursday|thu|friday|fri|saturday|sat|sunday|sun)$").unwrap();
    static ref DATE_REGEX_YMD: Regex = Regex::new(r"^(\d{4})[-/](\d{1,2})[-/](\d{1,2})$").unwrap();
    static ref DATE_REGEX_DMY: Regex = Regex::new(r"^(\d{1,2})[-/](\d{1,2})[-/](\d{4})$").unwrap();
    static ref DAY_MONTH_REGEX: Regex = Regex::new(r"^(\d{1,2})[-/]([a-zA-Z]+)$").unwrap();
    static ref MONTH_DAY_REGEX: Regex = Regex::new(r"^([a-zA-Z]+)[-/](\d{1,2})$").unwrap();
    static ref FULL_DATE_ALPHA_DMY: Regex = Regex::new(r"^(\d{1,2})[-/]([a-zA-Z]+)[-/](\d{4})$").unwrap();
    static ref FULL_DATE_ALPHA_YMD: Regex = Regex::new(r"^(\d{4})[-/]([a-zA-Z]+)[-/](\d{1,2})$").unwrap();
    static ref SHORT_DATE_REGEX: Regex = Regex::new(r"^(\d{1,2})[-/](\d{1,2})$").unwrap();
    static ref ORDINAL_DATE_REGEX: Regex = Regex::new(r"^(\d{1,2})(st|nd|rd|th)$").unwrap();
    static ref RELATIVE_TIME_REGEX: Regex = Regex::new(
        r"^(-?\d+)(d|day|days|w|wk|wks|week|weeks|m|mth|mths|month|months|y|yr|yrs|year|years)$"
    ).unwrap();
    static ref MONTH_MAP: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("jan", 1);
        m.insert("january", 1);
        m.insert("feb", 2);
        m.insert("february", 2);
        m.insert("mar", 3);
        m.insert("march", 3);
        m.insert("apr", 4);
        m.insert("april", 4);
        m.insert("may", 5);
        m.insert("jun", 6);
        m.insert("june", 6);
        m.insert("jul", 7);
        m.insert("july", 7);
        m.insert("aug", 8);
        m.insert("august", 8);
        m.insert("sep", 9);
        m.insert("september", 9);
        m.insert("oct", 10);
        m.insert("october", 10);
        m.insert("nov", 11);
        m.insert("november", 11);
        m.insert("dec", 12);
        m.insert("december", 12);
        m
    };
}

pub fn parse_date(date_str: &str) -> Result<NaiveDate, anyhow::Error> {
    let input = date_str.trim().to_lowercase();
    let now = Local::now();

    // Try standard date formats first
    if let Some(caps) = DATE_REGEX_YMD.captures(&input) {
        if let Ok(date) =
            NaiveDate::from_ymd_opt(caps[1].parse()?, caps[2].parse()?, caps[3].parse()?)
                .ok_or_else(|| anyhow!("Invalid date"))
        {
            return Ok(date);
        }
    }

    if let Some(caps) = DATE_REGEX_DMY.captures(&input) {
        if let Ok(date) =
            NaiveDate::from_ymd_opt(caps[3].parse()?, caps[2].parse()?, caps[1].parse()?)
                .ok_or_else(|| anyhow!("Invalid date"))
        {
            return Ok(date);
        }
    }

    // Natural language dates
    match input.as_str() {
        "today" | "tod" | "now" => return Ok(now.date_naive()),
        "yesterday" | "yes" => return Ok((now - Duration::days(1)).date_naive()),
        "tomorrow" | "tom" => return Ok((now + Duration::days(1)).date_naive()),
        _ => {}
    }

    // Weekdays
    match input.as_str() {
        "monday" | "mon" => return Ok(find_next_weekday(Weekday::Mon)),
        "tuesday" | "tue" => return Ok(find_next_weekday(Weekday::Tue)),
        "wednesday" | "wed" => return Ok(find_next_weekday(Weekday::Wed)),
        "thursday" | "thu" => return Ok(find_next_weekday(Weekday::Thu)),
        "friday" | "fri" => return Ok(find_next_weekday(Weekday::Fri)),
        "saturday" | "sat" => return Ok(find_next_weekday(Weekday::Sat)),
        "sunday" | "sun" => return Ok(find_next_weekday(Weekday::Sun)),
        _ => {}
    }

    // Next week's weekday (nfriday)
    if let Some(caps) = NEXT_WEEKDAY_REGEX.captures(&input) {
        let weekday = match &caps[1] {
            "monday" | "mon" => Weekday::Mon,
            "tuesday" | "tue" => Weekday::Tue,
            "wednesday" | "wed" => Weekday::Wed,
            "thursday" | "thu" => Weekday::Thu,
            "friday" | "fri" => Weekday::Fri,
            "saturday" | "sat" => Weekday::Sat,
            "sunday" | "sun" => Weekday::Sun,
            _ => return Err(anyhow!("Invalid weekday")),
        };
        return Ok(find_weekday_offset(weekday, 1));
    }

    // Numbered weekday (1friday, 2friday, etc.)
    if let Some(caps) = NUMBERED_WEEKDAY_REGEX.captures(&input) {
        let weeks_ahead: i64 = caps[1].parse()?;
        let weekday = match &caps[2] {
            "monday" | "mon" => Weekday::Mon,
            "tuesday" | "tue" => Weekday::Tue,
            "wednesday" | "wed" => Weekday::Wed,
            "thursday" | "thu" => Weekday::Thu,
            "friday" | "fri" => Weekday::Fri,
            "saturday" | "sat" => Weekday::Sat,
            "sunday" | "sun" => Weekday::Sun,
            _ => return Err(anyhow!("Invalid weekday")),
        };
        return Ok(find_weekday_offset(weekday, weeks_ahead));
    }

    // Business period markers
    match input.as_str() {
        "sow" => return Ok(find_next_weekday(Weekday::Mon)),
        "soww" => return Ok(find_next_weekday(Weekday::Mon)),
        "som" => return Ok(start_of_next_month(now)),
        "soq" => return Ok(start_of_next_quarter(now)),
        "soy" => return Ok(start_of_next_year(now)),
        "eow" => return Ok(find_next_weekday(Weekday::Mon) - Duration::days(1)),
        "eoww" => return Ok(find_next_weekday(Weekday::Sat)),
        "eom" => return Ok(end_of_current_month(now)),
        "eoq" => return Ok(end_of_current_quarter(now)),
        "eoy" => return Ok(end_of_current_year(now)),
        "eonw" => return Ok(find_next_weekday(Weekday::Mon) + Duration::days(6)),
        "eonm" => return Ok(end_of_next_month(now)),
        "eonq" => return Ok(end_of_next_quarter(now)),
        "eony" => return Ok(end_of_next_year(now)),
        _ => {}
    }

    // Ordinal dates (1st, 2nd, etc.)
    if let Some(caps) = ORDINAL_DATE_REGEX.captures(&input) {
        let day: u32 = caps[1].parse()?;
        if day <= 31 {
            return find_next_occurrence_of_day(now, day);
        }
    }

    // Relative time expressions
    if let Some(caps) = RELATIVE_TIME_REGEX.captures(&input) {
        let amount: i64 = caps[1].parse()?;
        let unit = &caps[2];
        let duration = match unit {
            "d" | "day" | "days" => Duration::days(amount),
            "w" | "wk" | "wks" | "week" | "weeks" => Duration::weeks(amount),
            "m" | "mth" | "mths" | "month" | "months" => Duration::days(amount * 30),
            "y" | "yr" | "yrs" | "year" | "years" => Duration::days(amount * 365),
            _ => return Err(anyhow!("Invalid time unit")),
        };
        return Ok(now.date_naive() + duration);
    }

    // Day-month formats
    if let Some(caps) = DAY_MONTH_REGEX.captures(&input) {
        let day: u32 = caps[1].parse()?;
        let month = parse_month(&caps[2])?;
        return Ok(find_next_occurrence(now.date_naive(), month, day)?);
    }

    // Month-day formats
    if let Some(caps) = MONTH_DAY_REGEX.captures(&input) {
        let month = parse_month(&caps[1])?;
        let day: u32 = caps[2].parse()?;
        return Ok(find_next_occurrence(now.date_naive(), month, day)?);
    }

    // Full date with alpha month
    if let Some(caps) = FULL_DATE_ALPHA_DMY.captures(&input) {
        let day: u32 = caps[1].parse()?;
        let month = parse_month(&caps[2])?;
        let year: i32 = caps[3].parse()?;
        return Ok(
            NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| anyhow!("Invalid date"))?
        );
    }

    if let Some(caps) = FULL_DATE_ALPHA_YMD.captures(&input) {
        let year: i32 = caps[1].parse()?;
        let month = parse_month(&caps[2])?;
        let day: u32 = caps[3].parse()?;
        return Ok(
            NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| anyhow!("Invalid date"))?
        );
    }

    // Short date (day/month with current year)
    if let Some(caps) = SHORT_DATE_REGEX.captures(&input) {
        let day: u32 = caps[1].parse()?;
        let month: u32 = caps[2].parse()?;
        return Ok(find_next_occurrence(now.date_naive(), month, day)?);
    }

    Err(anyhow!("Unrecognized date format"))
}

fn parse_month(month_str: &str) -> Result<u32, anyhow::Error> {
    MONTH_MAP
        .get(month_str.to_lowercase().as_str())
        .copied()
        .ok_or_else(|| anyhow!("Invalid month name"))
}

fn find_next_weekday(weekday: Weekday) -> NaiveDate {
    let now = Local::now();
    let today_weekday = now.weekday();
    let mut days_until_target =
        weekday.num_days_from_monday() as i64 - today_weekday.num_days_from_monday() as i64;
    if days_until_target <= 0 {
        days_until_target += 7;
    }
    now.date_naive() + Duration::days(days_until_target)
}

fn find_next_occurrence_of_day(now: DateTime<Local>, day: u32) -> Result<NaiveDate, anyhow::Error> {
    let mut month = now.month();
    let mut year = now.year();
    let start_year = year;

    if now.day() >= day {
        month += 1;
        if month > 12 {
            month = 1;
            year += 1;
        }
    }

    loop {
        if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
            return Ok(date);
        }
        month += 1;
        if month > 12 {
            month = 1;
            year += 1;
        }
        if year > start_year + 2 {
            return Err(anyhow!(
                "Could not find valid date within reasonable timeframe"
            ));
        }
    }
}

fn find_next_occurrence(
    today: NaiveDate,
    month: u32,
    day: u32,
) -> Result<NaiveDate, anyhow::Error> {
    let this_year = today.year();
    let next_year = this_year + 1;

    if let Some(date) = NaiveDate::from_ymd_opt(this_year, month, day) {
        if date >= today {
            return Ok(date);
        }
    }

    NaiveDate::from_ymd_opt(next_year, month, day)
        .ok_or_else(|| anyhow!("Invalid date: the specified day does not exist for this month"))
}

fn start_of_next_month(now: DateTime<Local>) -> NaiveDate {
    let mut year = now.year();
    let mut month = now.month();
    month += 1;
    if month > 12 {
        year += 1;
        month = 1;
    }
    NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid date in start_of_next_month")
}

fn start_of_next_quarter(now: DateTime<Local>) -> NaiveDate {
    let mut month = ((now.month() - 1) / 3 + 1) * 3 + 1;
    let mut year = now.year();
    if month > 12 {
        month -= 12;
        year += 1;
    }
    NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid date in start_of_next_quarter")
}

fn start_of_next_year(now: DateTime<Local>) -> NaiveDate {
    NaiveDate::from_ymd_opt(now.year() + 1, 1, 1).expect("Invalid date in start_of_next_year")
}

fn end_of_current_month(now: DateTime<Local>) -> NaiveDate {
    let mut year = now.year();
    let mut month = now.month();
    month += 1;
    if month > 12 {
        year += 1;
        month = 1;
    }
    NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid date in end_of_current_month")
        - Duration::days(1)
}

fn end_of_current_quarter(now: DateTime<Local>) -> NaiveDate {
    let mut month = ((now.month() - 1) / 3 + 1) * 3 + 1;
    let mut year = now.year();
    if month > 12 {
        month -= 12;
        year += 1;
    }
    NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid date in end_of_current_quarter")
        - Duration::days(1)
}

fn end_of_current_year(now: DateTime<Local>) -> NaiveDate {
    NaiveDate::from_ymd_opt(now.year() + 1, 1, 1).expect("Invalid date in end_of_current_year")
        - Duration::days(1)
}

fn end_of_next_month(now: DateTime<Local>) -> NaiveDate {
    let mut year = now.year();
    let mut month = now.month() + 2; // Add 2 to get to end of next month
    if month > 12 {
        year += 1;
        month -= 12;
    }
    NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid date in end_of_next_month")
        - Duration::days(1)
}

fn end_of_next_quarter(now: DateTime<Local>) -> NaiveDate {
    let current_quarter = (now.month() - 1) / 3;
    let next_quarter = current_quarter + 2; // Add 2 to get to end of next quarter
    let year = now.year() + (next_quarter as i32 / 4);
    let month = ((next_quarter % 4) * 3) + 1;
    NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid date in end_of_next_quarter")
        - Duration::days(1)
}

fn end_of_next_year(now: DateTime<Local>) -> NaiveDate {
    NaiveDate::from_ymd_opt(now.year() + 2, 1, 1).expect("Invalid date in end_of_next_year")
        - Duration::days(1)
}

fn find_weekday_offset(weekday: Weekday, weeks_ahead: i64) -> NaiveDate {
    let now = Local::now();
    let today_weekday = now.weekday();
    let mut days_until_target =
        weekday.num_days_from_monday() as i64 - today_weekday.num_days_from_monday() as i64;

    if days_until_target <= 0 && weeks_ahead == 0 {
        days_until_target += 7;
    }

    days_until_target += weeks_ahead * 7;
    now.date_naive() + Duration::days(days_until_target)
}
