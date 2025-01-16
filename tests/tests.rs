use chrono::{Datelike, Duration, Local, Weekday};
use temporis::parse_date;

#[test]
fn test_yyyy_mm_dd_format() {
    // Valid dates with different separators
    assert!(parse_date("2024-01-16").is_ok());
    assert!(parse_date("2024/01/16").is_ok());

    // Test padding variations
    assert!(parse_date("2024-1-16").is_ok());
    assert!(parse_date("2024-01-6").is_ok());
    assert!(parse_date("2024-1-6").is_ok());

    // Test different years
    assert!(parse_date("1900-01-01").is_ok());
    assert!(parse_date("2100-12-31").is_ok());

    // Test leap years
    assert!(parse_date("2024-02-29").is_ok()); // Leap year
    assert!(parse_date("2023-02-29").is_err()); // Not a leap year
    assert!(parse_date("2000-02-29").is_ok()); // Century leap year
    assert!(parse_date("2100-02-29").is_err()); // Not a century leap year
}

#[test]
fn test_dd_mm_yyyy_format() {
    // Valid dates with different separators
    assert!(parse_date("16-01-2024").is_ok());
    assert!(parse_date("16/01/2024").is_ok());

    // Test padding variations
    assert!(parse_date("6-01-2024").is_ok());
    assert!(parse_date("16-1-2024").is_ok());
    assert!(parse_date("6-1-2024").is_ok());

    // Test different years
    assert!(parse_date("01-01-1900").is_ok());
    assert!(parse_date("31-12-2100").is_ok());

    // Test leap years
    assert!(parse_date("29-02-2024").is_ok()); // Leap year
    assert!(parse_date("29-02-2023").is_err()); // Not a leap year
    assert!(parse_date("29-02-2000").is_ok()); // Century leap year
    assert!(parse_date("29-02-2100").is_err()); // Not a century leap year
}

#[test]
fn test_invalid_formats() {
    // Invalid months
    assert!(parse_date("2024-13-16").is_err());
    assert!(parse_date("2024-00-16").is_err());
    assert!(parse_date("16-13-2024").is_err());
    assert!(parse_date("16-00-2024").is_err());

    // Invalid days
    assert!(parse_date("2024-01-32").is_err());
    assert!(parse_date("2024-01-00").is_err());
    assert!(parse_date("32-01-2024").is_err());
    assert!(parse_date("00-01-2024").is_err());

    // Invalid days for specific months
    assert!(parse_date("2024-04-31").is_err()); // April has 30 days
    assert!(parse_date("2024-06-31").is_err()); // June has 30 days
    assert!(parse_date("2024-09-31").is_err()); // September has 30 days
    assert!(parse_date("2024-11-31").is_err()); // November has 30 days

    // Invalid separators
    assert!(parse_date("2024.01.16").is_err());
    assert!(parse_date("2024_01_16").is_err());
    assert!(parse_date("2024 01 16").is_err());

    // Malformed inputs
    assert!(parse_date("202-01-16").is_err()); // Wrong year format
    assert!(parse_date("2024-1x-16").is_err()); // Non-numeric month
    assert!(parse_date("2024-01-1x").is_err()); // Non-numeric day
}

#[test]
fn test_date_equality() {
    // Test that different formats for the same date return the same result
    let ymd = parse_date("2024-01-16").unwrap();
    let dmy = parse_date("16-01-2024").unwrap();
    assert_eq!(ymd, dmy);

    // Test with different separators
    let with_dash = parse_date("2024-01-16").unwrap();
    let with_slash = parse_date("2024/01/16").unwrap();
    assert_eq!(with_dash, with_slash);

    // Test with different padding
    let padded = parse_date("2024-01-06").unwrap();
    let unpadded = parse_date("2024-1-6").unwrap();
    assert_eq!(padded, unpadded);
}

#[test]
fn test_basic_natural_dates() {
    let today = Local::now().date_naive();

    // Test today variants with different casings
    assert_eq!(parse_date("today").unwrap(), today);
    assert_eq!(parse_date("TODAY").unwrap(), today);
    assert_eq!(parse_date("ToDay").unwrap(), today);
    assert_eq!(parse_date("tod").unwrap(), today);
    assert_eq!(parse_date("TOD").unwrap(), today);
    assert_eq!(parse_date("now").unwrap(), today);
    assert_eq!(parse_date("NOW").unwrap(), today);

    // Test yesterday variants with different casings
    assert_eq!(parse_date("yesterday").unwrap(), today - Duration::days(1));
    assert_eq!(parse_date("YESTERDAY").unwrap(), today - Duration::days(1));
    assert_eq!(parse_date("YesterDay").unwrap(), today - Duration::days(1));
    assert_eq!(parse_date("yes").unwrap(), today - Duration::days(1));
    assert_eq!(parse_date("YES").unwrap(), today - Duration::days(1));

    // Test tomorrow variants with different casings
    assert_eq!(parse_date("tomorrow").unwrap(), today + Duration::days(1));
    assert_eq!(parse_date("TOMORROW").unwrap(), today + Duration::days(1));
    assert_eq!(parse_date("ToMorrow").unwrap(), today + Duration::days(1));
    assert_eq!(parse_date("tom").unwrap(), today + Duration::days(1));
    assert_eq!(parse_date("TOM").unwrap(), today + Duration::days(1));
}

#[test]
fn test_invalid_natural_dates() {
    // Test similar but invalid words
    assert!(parse_date("todays").is_err());
    assert!(parse_date("tomorrows").is_err());
    assert!(parse_date("yesterdays").is_err());

    // Test with spaces
    assert!(parse_date("to day").is_err());
    assert!(parse_date("yester day").is_err());
    assert!(parse_date("to morrow").is_err());

    // Test with extra characters
    assert!(parse_date("today!").is_err());
    assert!(parse_date("yesterday.").is_err());
    assert!(parse_date("tomorrow?").is_err());

    // Test with trailing/leading spaces (these should actually work)
    assert!(parse_date(" today ").is_ok());
    assert!(parse_date(" yesterday ").is_ok());
    assert!(parse_date(" tomorrow ").is_ok());
}

#[test]
fn test_whitespace_handling() {
    let today = Local::now().date_naive();

    // Test with various whitespace combinations
    assert_eq!(parse_date("   today").unwrap(), today);
    assert_eq!(parse_date("today   ").unwrap(), today);
    assert_eq!(parse_date("   today   ").unwrap(), today);
    assert_eq!(parse_date("\ttoday").unwrap(), today);
    assert_eq!(parse_date("today\n").unwrap(), today);
}

#[test]
fn test_basic_weekday_formats() {
    let days = [
        ("monday", "mon"),
        ("tuesday", "tue"),
        ("wednesday", "wed"),
        ("thursday", "thu"),
        ("friday", "fri"),
        ("saturday", "sat"),
        ("sunday", "sun"),
    ];

    for (full, short) in days.iter() {
        // Test basic formats
        assert!(parse_date(full).is_ok());
        assert!(parse_date(short).is_ok());

        // Test that full and short forms return same date
        assert_eq!(parse_date(full).unwrap(), parse_date(short).unwrap());

        // Test uppercase variants
        assert_eq!(
            parse_date(&full.to_uppercase()).unwrap(),
            parse_date(full).unwrap()
        );
        assert_eq!(
            parse_date(&short.to_uppercase()).unwrap(),
            parse_date(short).unwrap()
        );
    }
}

#[test]
fn test_invalid_weekday_formats() {
    // Test invalid but similar words
    assert!(parse_date("mond").is_err());
    assert!(parse_date("tues").is_err());
    assert!(parse_date("wednes").is_err());
    assert!(parse_date("thurs").is_err());
    assert!(parse_date("frid").is_err());
    assert!(parse_date("satur").is_err());
    assert!(parse_date("sund").is_err());

    // Test with trailing characters
    assert!(parse_date("monday.").is_err());
    assert!(parse_date("mon,").is_err());
    assert!(parse_date("tuesday!").is_err());
    assert!(parse_date("tue?").is_err());

    // Test with spaces
    assert!(parse_date("mon day").is_err());
    assert!(parse_date("tues day").is_err());
    assert!(parse_date("wednes day").is_err());
}

#[test]
fn test_weekday_whitespace() {
    // Test various whitespace combinations
    assert!(parse_date(" monday").is_ok());
    assert!(parse_date("monday ").is_ok());
    assert!(parse_date(" monday ").is_ok());
    assert!(parse_date("\tmonday").is_ok());
    assert!(parse_date("monday\n").is_ok());

    // Verify whitespace doesn't affect the result
    let regular = parse_date("monday").unwrap();
    assert_eq!(parse_date(" monday").unwrap(), regular);
    assert_eq!(parse_date("monday ").unwrap(), regular);
    assert_eq!(parse_date(" monday ").unwrap(), regular);
    assert_eq!(parse_date("\tmonday").unwrap(), regular);
    assert_eq!(parse_date("monday\n").unwrap(), regular);
}

#[test]
fn test_next_week_behavior() {
    let today = Local::now().date_naive();
    let weekday = today.weekday();

    // Test that requesting today's weekday returns next week's occurrence
    let day_name = match weekday {
        Weekday::Mon => "monday",
        Weekday::Tue => "tuesday",
        Weekday::Wed => "wednesday",
        Weekday::Thu => "thursday",
        Weekday::Fri => "friday",
        Weekday::Sat => "saturday",
        Weekday::Sun => "sunday",
    };

    let next_occurrence = parse_date(day_name).unwrap();
    assert!(next_occurrence > today);
    assert_eq!((next_occurrence - today).num_days(), 7);
}

#[test]
fn test_basic_markers() {
    // Start markers with different cases
    for marker in ["sow", "SOW", "SoW"] {
        assert!(parse_date(marker).is_ok());
    }

    // End markers with different cases
    for marker in ["eow", "EOW", "EoW"] {
        assert!(parse_date(marker).is_ok());
    }

    // Next period markers with different cases
    for marker in ["eonw", "EONW", "EoNW"] {
        assert!(parse_date(marker).is_ok());
    }
}

#[test]
fn test_marker_relationships() {
    let now = Local::now();
    let sow = parse_date("sow").unwrap();
    let eow = parse_date("eow").unwrap();
    let eonw = parse_date("eonw").unwrap();

    // These relationships should always hold true regardless of current date
    assert!(sow > eow);
    assert!(eonw >= eow);

    // Start of week is always Monday, end of week is always Sunday
    assert_eq!(sow.weekday(), Weekday::Mon);
    assert_eq!(eow.weekday(), Weekday::Sun);

    // End of next week is always 7 days after end of this week
    assert_eq!((eonw - eow).num_days(), 7);

    // Month markers
    let som = parse_date("som").unwrap();
    let eom = parse_date("eom").unwrap();
    let eonm = parse_date("eonm").unwrap();

    // End of next month should be after end of current month
    assert!(som > eom);
    assert!(eonm >= eom);

    // The first day of next month should be the day after end of current month
    assert_eq!((som - eom).num_days(), 1);

    // Start of month should be day 1
    assert_eq!(som.day(), 1);

    // Quarter markers
    let soq = parse_date("soq").unwrap();
    let eoq = parse_date("eoq").unwrap();
    let eonq = parse_date("eonq").unwrap();

    assert!(soq > eoq);
    assert!(eonq >= eoq);

    // Start of quarter should be first day of a quarter month (1, 4, 7, or 10)
    assert!(matches!(soq.month(), 1 | 4 | 7 | 10));
    assert_eq!(soq.day(), 1);

    // The first day of next quarter should be the day after end of current quarter
    assert_eq!((soq - eoq).num_days(), 1);
}

#[test]
fn test_marker_current_date_dependence() {
    let now = Local::now();
    let today = now.date_naive();

    // Test week markers
    let sow = parse_date("sow").unwrap();
    let eow = parse_date("eow").unwrap();
    assert!(sow >= today);
    assert!(eow >= today);

    // Test month markers
    let som = parse_date("som").unwrap();
    let eom = parse_date("eom").unwrap();
    assert!(som >= today);

    // End of current month might be today
    assert!(eom >= today);

    // Test quarter markers
    let soq = parse_date("soq").unwrap();
    let eoq = parse_date("eoq").unwrap();
    assert!(soq >= today);

    // End of current quarter might be today
    assert!(eoq >= today);
}

#[test]
fn test_invalid_markers() {
    // Invalid but similar markers
    assert!(parse_date("sowy").is_err());
    assert!(parse_date("eowy").is_err());
    assert!(parse_date("sonw").is_err());
    assert!(parse_date("sonn").is_err());

    // With spaces
    assert!(parse_date("s o w").is_err());
    assert!(parse_date("e o w").is_err());
    assert!(parse_date("e o n w").is_err());

    // With punctuation
    assert!(parse_date("sow.").is_err());
    assert!(parse_date("eow!").is_err());
    assert!(parse_date("eonw?").is_err());
}

#[test]
fn test_workweek_markers() {
    let now = Local::now();
    let today_weekday = now.weekday();
    let soww = parse_date("soww").unwrap();
    let eoww = parse_date("eoww").unwrap();

    // Work week should always start on Monday and end on Saturday
    assert_eq!(soww.weekday(), Weekday::Mon);
    assert_eq!(eoww.weekday(), Weekday::Sat);

    // The difference between start and end depends on whether we're currently
    // in the work week or not
    match today_weekday {
        // If we're Sun, then soww is tomorrow (1) and eoww is in 6 days (6)
        Weekday::Sun => assert_eq!((eoww - soww).num_days(), 5),

        // If we're Sat, then soww is in 2 days (2) and eoww is in 7 days (7)
        Weekday::Sat => assert_eq!((eoww - soww).num_days(), 5),

        // If we're in the work week (Mon-Fri)
        // then soww is next week's Monday and eoww is next week's Saturday
        _ => assert_eq!((eoww - soww).num_days(), -2),
    }
}

#[test]
fn test_basic_ordinals() {
    // Test all valid ordinal patterns
    let ordinals = [
        "1st", "2nd", "3rd", "4th", "5th", "6th", "7th", "8th", "9th", "10th", "11th", "12th",
        "13th", "14th", "15th", "16th", "17th", "18th", "19th", "20th", "21st", "22nd", "23rd",
        "24th", "25th", "26th", "27th", "28th", "29th", "30th", "31st",
    ];

    for ordinal in ordinals.iter() {
        assert!(parse_date(ordinal).is_ok(), "Failed to parse {}", ordinal);
    }
}

#[test]
fn test_ordinal_case_sensitivity() {
    // Test different case variations
    let test_cases = [
        ("1ST", "1st"),
        ("2ND", "2nd"),
        ("3RD", "3rd"),
        ("4TH", "4th"),
        ("21ST", "21st"),
        ("22ND", "22nd"),
        ("23RD", "23rd"),
    ];

    for (upper, lower) in test_cases.iter() {
        assert!(parse_date(upper).is_ok());
        assert_eq!(parse_date(upper).unwrap(), parse_date(lower).unwrap());
    }
}

#[test]
fn test_invalid_ordinals() {
    let invalid_cases = [
        "0th",  // Zero
        "32nd", // Too high
        "33rd", // Too high
        "99th", // Too high
        "1rst", // Wrong suffix
        "1s",   // Incomplete suffix
        "2n",   // Incomplete suffix
        "3r",   // Incomplete suffix
        "4t",   // Incomplete suffix
        "st1",  // Wrong order
        "nd2",  // Wrong order
        "rd3",  // Wrong order
        "th4",  // Wrong order
    ];

    for invalid in invalid_cases.iter() {
        assert!(
            parse_date(invalid).is_err(),
            "Should fail to parse {}",
            invalid
        );
    }
}

#[test]
fn test_ordinal_next_occurrence() {
    let now = Local::now();
    let today = now.date_naive();

    // Get the ordinal for today
    let today_ordinal = format!("{}{}", today.day(), ordinal_suffix(today.day()));
    let result = parse_date(&today_ordinal).unwrap();

    // If we request today's date, we should get next month's occurrence
    assert!(result > today);

    // Test that we get next month's occurrence for a past date in current month
    if today.day() > 5 {
        let past_day = format!("{}th", today.day() - 5);
        let result = parse_date(&past_day).unwrap();
        assert!(result > today);
    }

    // Test that we get this month's occurrence for a future date in current month
    if today.day() < 25 {
        let future_day = format!("{}th", today.day() + 5);
        let result = parse_date(&future_day).unwrap();
        assert!(result > today);
        assert!(
            result.month() == today.month()
                || (result.month() == today.month() + 1 && today.month() < 12)
                || (result.month() == 1 && today.month() == 12)
        );
    }
}

#[test]
fn test_ordinal_whitespace() {
    let base_date = parse_date("15th").unwrap();

    // Test various whitespace combinations
    assert!(parse_date(" 15th").is_ok());
    assert!(parse_date("15th ").is_ok());
    assert!(parse_date(" 15th ").is_ok());
    assert!(parse_date("\t15th").is_ok());
    assert!(parse_date("15th\n").is_ok());

    // All whitespace variations should give same result
    assert_eq!(parse_date(" 15th").unwrap(), base_date);
    assert_eq!(parse_date("15th ").unwrap(), base_date);
    assert_eq!(parse_date(" 15th ").unwrap(), base_date);
    assert_eq!(parse_date("\t15th").unwrap(), base_date);
    assert_eq!(parse_date("15th\n").unwrap(), base_date);
}

// Helper function for test_ordinal_next_occurrence
fn ordinal_suffix(day: u32) -> &'static str {
    match day {
        1 | 21 | 31 => "st",
        2 | 22 => "nd",
        3 | 23 => "rd",
        _ => "th",
    }
}

#[test]
fn test_days() {
    let today = Local::now().date_naive();

    // Test different day formats
    let variations = ["d", "day", "days"];
    for suffix in variations.iter() {
        // Test positive values
        let five_days = format!("5{}", suffix);
        assert_eq!(parse_date(&five_days).unwrap(), today + Duration::days(5));

        // Test negative values
        let neg_five_days = format!("-5{}", suffix);
        assert_eq!(
            parse_date(&neg_five_days).unwrap(),
            today - Duration::days(5)
        );
    }

    // Test zero and edge cases
    assert!(parse_date("0d").is_ok());
    assert_eq!(parse_date("0d").unwrap(), today);
    assert!(parse_date("999999d").is_ok()); // Large values should work
}

#[test]
fn test_weeks() {
    let today = Local::now().date_naive();

    // Test different week formats
    let variations = ["w", "wk", "wks", "week", "weeks"];
    for suffix in variations.iter() {
        // Test positive values
        let three_weeks = format!("3{}", suffix);
        assert_eq!(
            parse_date(&three_weeks).unwrap(),
            today + Duration::weeks(3)
        );

        // Test negative values
        let neg_three_weeks = format!("-3{}", suffix);
        assert_eq!(
            parse_date(&neg_three_weeks).unwrap(),
            today - Duration::weeks(3)
        );
    }

    // Test zero and edge cases
    assert!(parse_date("0w").is_ok());
    assert_eq!(parse_date("0w").unwrap(), today);
    assert!(parse_date("52w").is_ok()); // Full year
}

#[test]
fn test_months() {
    let today = Local::now().date_naive();

    // Test different month formats
    let variations = ["m", "mth", "mths", "month", "months"];
    for suffix in variations.iter() {
        // Test positive values
        let two_months = format!("2{}", suffix);
        let result = parse_date(&two_months).unwrap();
        assert!(result > today);

        // Test negative values
        let neg_two_months = format!("-2{}", suffix);
        let result = parse_date(&neg_two_months).unwrap();
        assert!(result < today);
    }

    // Test zero
    assert!(parse_date("0m").is_ok());
    assert_eq!(parse_date("0m").unwrap(), today);

    // Test 12 months equals roughly a year
    let twelve_months = parse_date("12m").unwrap();
    let one_year = parse_date("1y").unwrap();
    assert!((twelve_months - one_year).num_days().abs() <= 5); // Allow small difference due to month length variations
}

#[test]
fn test_years() {
    let today = Local::now().date_naive();

    // Test different year formats
    let variations = ["y", "yr", "yrs", "year", "years"];
    for suffix in variations.iter() {
        // Test positive values
        let one_year = format!("1{}", suffix);
        let result = parse_date(&one_year).unwrap();
        assert_eq!(result.year(), today.year() + 1);

        // Test negative values
        let neg_one_year = format!("-1{}", suffix);
        let result = parse_date(&neg_one_year).unwrap();
        assert_eq!(result.year(), today.year() - 1);
    }

    // Test zero
    assert!(parse_date("0y").is_ok());
    assert_eq!(parse_date("0y").unwrap(), today);
}

#[test]
fn test_invalid_relative_formats() {
    let invalid_cases = [
        "5",        // No unit
        "d5",       // Unit before number
        "5 d",      // Space between
        "5dd",      // Double unit
        "5dw",      // Mixed units
        "5.5d",     // Decimal values
        "five d",   // Text number
        "+5d",      // Explicit positive
        "--5d",     // Double negative
        "d",        // Missing number
        "5dy",      // Invalid unit
        "5seconds", // Invalid time unit
    ];

    for invalid in invalid_cases.iter() {
        assert!(
            parse_date(invalid).is_err(),
            "Should fail to parse {}",
            invalid
        );
    }
}

#[test]
fn test_case_relative_sensitivity() {
    let today = Local::now().date_naive();
    let base = parse_date("5d").unwrap();

    // Test different casings
    let variations = ["5D", "5DAY", "5Day", "5dAY"];
    for variant in variations.iter() {
        assert!(parse_date(variant).is_ok());
        assert_eq!(parse_date(variant).unwrap(), base);
    }
}

#[test]
fn test_day_month_formats() {
    let today = Local::now().date_naive();

    // Test with different separators
    assert!(parse_date("16-jan").is_ok());
    assert!(parse_date("16/jan").is_ok());

    // Test all months with both separators
    let months = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
    ];
    for month in months.iter() {
        assert!(parse_date(&format!("16-{}", month)).is_ok());
        assert!(parse_date(&format!("16/{}", month)).is_ok());
    }

    // Test different day paddings
    assert!(parse_date("6-jan").is_ok());
    assert!(parse_date("06-jan").is_ok());

    // Test that requesting past date gives next occurrence
    let result = parse_date("16-jan").unwrap();
    if today.month() > 1 || (today.month() == 1 && today.day() > 16) {
        assert!(result.year() == today.year() + 1);
    } else {
        assert!(result.year() == today.year());
    }
}

#[test]
fn test_month_day_formats() {
    let today = Local::now().date_naive();

    // Test with different separators
    assert!(parse_date("jan-16").is_ok());
    assert!(parse_date("jan/16").is_ok());

    // Test all months with both separators
    let months = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
    ];
    for month in months.iter() {
        assert!(parse_date(&format!("{}-16", month)).is_ok());
        assert!(parse_date(&format!("{}/16", month)).is_ok());
    }

    // Test different day paddings
    assert!(parse_date("jan-6").is_ok());
    assert!(parse_date("jan-06").is_ok());

    // Verify both formats give same result
    assert_eq!(parse_date("jan-16").unwrap(), parse_date("16-jan").unwrap());
}

#[test]
fn test_full_date_formats() {
    // Test all variants of full date format
    assert!(parse_date("01-jan-2024").is_ok());
    assert!(parse_date("01/jan/2024").is_ok());
    assert!(parse_date("2024-jan-01").is_ok());
    assert!(parse_date("2024/jan/01").is_ok());

    // Test with different day/month combinations
    let test_date = "15-mar-2024";
    let result = parse_date(test_date).unwrap();
    assert_eq!(result.day(), 15);
    assert_eq!(result.month(), 3);
    assert_eq!(result.year(), 2024);

    // Test that all formats give same result
    let base = parse_date("01-jan-2024").unwrap();
    assert_eq!(parse_date("01/jan/2024").unwrap(), base);
    assert_eq!(parse_date("2024-jan-01").unwrap(), base);
    assert_eq!(parse_date("2024/jan/01").unwrap(), base);
}

#[test]
fn test_short_numeric_formats() {
    let today = Local::now().date_naive();

    // Test with different separators
    assert!(parse_date("16-1").is_ok());
    assert!(parse_date("16/1").is_ok());

    // Test different paddings
    assert!(parse_date("16-01").is_ok());
    assert!(parse_date("06-01").is_ok());
    assert!(parse_date("6-1").is_ok());

    // Test all valid month numbers
    for month in 1..=12 {
        assert!(parse_date(&format!("16-{}", month)).is_ok());
        assert!(parse_date(&format!("16/{}", month)).is_ok());
    }

    // Test that requesting past date gives next occurrence
    let result = parse_date("16-1").unwrap();
    if today.month() > 1 || (today.month() == 1 && today.day() > 16) {
        assert!(result.year() == today.year() + 1);
    } else {
        assert!(result.year() == today.year());
    }
}

#[test]
fn test_invalid_date_formats() {
    // Invalid days
    assert!(parse_date("32-jan").is_err());
    assert!(parse_date("00-jan").is_err());
    assert!(parse_date("jan-32").is_err());
    assert!(parse_date("jan-00").is_err());

    // Invalid months
    assert!(parse_date("16-jxn").is_err());
    assert!(parse_date("16-13").is_err());
    assert!(parse_date("16-0").is_err());

    // Invalid separators
    assert!(parse_date("16.jan").is_err());
    assert!(parse_date("16_jan").is_err());
    assert!(parse_date("16 jan").is_err());

    // Month-specific day validation
    assert!(parse_date("31-apr").is_err()); // April has 30 days
    assert!(parse_date("31-jun").is_err()); // June has 30 days
    assert!(parse_date("31-sep").is_err()); // September has 30 days
    assert!(parse_date("31-nov").is_err()); // November has 30 days
    assert!(parse_date("30-feb").is_err()); // February never has 30 days

    // Invalid full date formats
    assert!(parse_date("00-jan-2024").is_err());
    assert!(parse_date("32-jan-2024").is_err());
    assert!(parse_date("15-jxn-2024").is_err());
}

#[test]
fn test_case_sensitivity() {
    let base = parse_date("16-jan").unwrap();

    // Test different casings
    let variations = ["16-JAN", "16-Jan", "16-jAn", "JAN-16", "Jan-16", "jAn-16"];
    for variant in variations.iter() {
        assert!(parse_date(variant).is_ok());
        assert_eq!(parse_date(variant).unwrap(), base);
    }
}

const MONTHS: [(&str, &str); 12] = [
    ("jan", "january"),
    ("feb", "february"),
    ("mar", "march"),
    ("apr", "april"),
    ("may", "may"),
    ("jun", "june"),
    ("jul", "july"),
    ("aug", "august"),
    ("sep", "september"),
    ("oct", "october"),
    ("nov", "november"),
    ("dec", "december"),
];

#[test]
fn test_basic_month_formats() {
    for (short, full) in MONTHS.iter() {
        // Test in day-month format with different separators
        let variations = [
            (format!("15-{}", short), format!("15-{}", full)),
            (format!("15/{}", short), format!("15/{}", full)),
        ];

        for (short_form, full_form) in variations {
            assert!(parse_date(&short_form).is_ok());
            assert!(parse_date(&full_form).is_ok());
            assert_eq!(
                parse_date(&short_form).unwrap(),
                parse_date(&full_form).unwrap()
            );
        }
    }
}

#[test]
fn test_month_day_variations() {
    for (short, full) in MONTHS.iter() {
        // Test different day values (start, middle, end of month)
        let days = ["01", "15", "28"];
        for day in days.iter() {
            // Test both separators
            let formats = [
                // Month-day format
                (format!("{}-{}", short, day), format!("{}-{}", full, day)),
                (format!("{}/{}", short, day), format!("{}/{}", full, day)),
                // Day-month format
                (format!("{}-{}", day, short), format!("{}-{}", day, full)),
                (format!("{}/{}", day, short), format!("{}/{}", day, full)),
            ];

            for (short_form, full_form) in formats {
                assert!(parse_date(&short_form).is_ok());
                assert!(parse_date(&full_form).is_ok());
                assert_eq!(
                    parse_date(&short_form).unwrap(),
                    parse_date(&full_form).unwrap()
                );
            }
        }
    }
}

#[test]
fn test_full_date_variations() {
    let years = ["2023", "2024", "2025"];
    let days = ["01", "15", "28"];

    for (short, full) in MONTHS.iter() {
        for year in years.iter() {
            for day in days.iter() {
                // Test all possible full date formats
                let formats = [
                    // DMY formats
                    (
                        format!("{}-{}-{}", day, short, year),
                        format!("{}-{}-{}", day, full, year),
                    ),
                    (
                        format!("{}/{}/{}", day, short, year),
                        format!("{}/{}/{}", day, full, year),
                    ),
                    // YMD formats
                    (
                        format!("{}-{}-{}", year, short, day),
                        format!("{}-{}-{}", year, full, day),
                    ),
                    (
                        format!("{}/{}/{}", year, short, day),
                        format!("{}/{}/{}", year, full, day),
                    ),
                ];

                for (short_form, full_form) in formats {
                    assert!(parse_date(&short_form).is_ok());
                    assert!(parse_date(&full_form).is_ok());
                    assert_eq!(
                        parse_date(&short_form).unwrap(),
                        parse_date(&full_form).unwrap()
                    );
                }
            }
        }
    }
}

#[test]
fn test_case_variations() {
    // Test one date with all possible case variations
    let base_date = "15-jan-2024";
    let case_variations = [
        "15-JAN-2024",
        "15-Jan-2024",
        "15-jAn-2024",
        "15-jaN-2024",
        "15-JANUARY-2024",
        "15-January-2024",
        "15-jAnUaRy-2024",
    ];

    let base_result = parse_date(base_date).unwrap();
    for variant in case_variations.iter() {
        assert!(parse_date(variant).is_ok());
        assert_eq!(parse_date(variant).unwrap(), base_result);
    }
}

#[test]
fn test_invalid_variations() {
    // Test similar but invalid month names
    let invalid_months = [
        "janu", "januarys", "feb.", "febs", "marc", "marshy", "ap", "aprl", "may.", "mayo", "jun.",
        "june.", "jul.", "julys", "aug.", "augst", "sept", "sep.", "oct.", "octs", "nov.", "novem",
        "dec.", "dece",
    ];

    for month in invalid_months.iter() {
        assert!(parse_date(&format!("15-{}", month)).is_err());
        assert!(parse_date(&format!("{}-15", month)).is_err());
        assert!(parse_date(&format!("15-{}-2024", month)).is_err());
        assert!(parse_date(&format!("2024-{}-15", month)).is_err());
    }
}

#[test]
fn test_month_specific_days() {
    // Test month-specific day limits
    let month_limits = [
        ("jan", 31),
        ("feb", 28),
        ("mar", 31),
        ("apr", 30),
        ("may", 31),
        ("jun", 30),
        ("jul", 31),
        ("aug", 31),
        ("sep", 30),
        ("oct", 31),
        ("nov", 30),
        ("dec", 31),
    ];

    for (month, max_days) in month_limits.iter() {
        // Valid max day should work
        assert!(parse_date(&format!("{}-{}", max_days, month)).is_ok());

        // Day after max should fail
        assert!(parse_date(&format!("{}-{}", max_days + 1, month)).is_err());
    }

    // Special test for February in leap year
    assert!(parse_date("29-feb-2024").is_ok()); // 2024 is leap year
    assert!(parse_date("29-feb-2023").is_err()); // 2023 is not leap year
}

#[test]
fn test_empty_and_malformed() {
    let inputs = [
        "",         // Empty string
        " ",        // Just whitespace
        "\t",       // Tab
        "\n",       // Newline
        "invalid",  // Random text
        "garbage",  // More random text
        "date",     // Date-related but invalid
        "calendar", // Calendar-related but invalid
        "2024",     // Just year
        "january",  // Just month
        "15",       // Just day
    ];

    for input in inputs.iter() {
        assert!(
            parse_date(input).is_err(),
            "Expected error for input '{}' but got {:?}",
            input,
            parse_date(input).unwrap()
        );
    }
}

#[test]
fn test_invalid_standard_dates() {
    let inputs = [
        // Invalid months
        "2024-13-01", // Month > 12
        "2024-00-01", // Month = 0
        "2024-1a-01", // Non-numeric month
        // Invalid days
        "2024-01-32", // Day > 31
        "2024-01-00", // Day = 0
        "2024-01-1a", // Non-numeric day
        // Invalid years
        "10000-01-01", // Year > 9999
        "abc-01-01",   // Non-numeric year
        // Invalid separators
        "2024.01.01", // Dots
        "2024_01_01", // Underscores
        "2024 01 01", // Spaces
        "20240101",   // No separator
        // Month-specific invalid days
        "2024-04-31", // April has 30 days
        "2024-06-31", // June has 30 days
        "2024-09-31", // September has 30 days
        "2024-11-31", // November has 30 days
        "2023-02-29", // Non-leap year February
        "2100-02-29", // Non-leap century
    ];

    for input in inputs.iter() {
        assert!(
            parse_date(input).is_err(),
            "Expected error for input '{}' but got {:?}",
            input,
            parse_date(input).unwrap()
        );
    }
}

#[test]
fn test_invalid_month_formats() {
    let inputs = [
        // Invalid month names
        "15-jen",   // Misspelled
        "15-jann",  // Extra letter
        "15-feb.",  // With punctuation
        "15-marc",  // Incomplete
        "15-sept",  // Common mistake
        "15-sept.", // With punctuation
        // Invalid formats
        "jan15",   // No separator
        "15jan",   // No separator
        "jan-15-", // Trailing separator
        "-jan-15", // Leading separator
        "15--jan", // Double separator
        "15-jan-", // Incomplete
        // Invalid combinations
        "32-jan", // Invalid day
        "00-jan", // Day = 0
        "15-13",  // Invalid month number
        "15-0",   // Month = 0
    ];

    for input in inputs.iter() {
        assert!(
            parse_date(input).is_err(),
            "Expected error for input '{}' but got {:?}",
            input,
            parse_date(input).unwrap()
        );
    }
}

#[test]
fn test_invalid_invalid_relative_formats() {
    let inputs = [
        // Invalid numbers
        "1.5d", // Decimal not allowed
        "d",    // Missing number
        // Invalid units
        "5seconds", // Invalid unit
        "5mins",    // Invalid unit
        "5hours",   // Invalid unit
        "5ms",      // Invalid unit
        // Invalid formats
        "5 d",  // Space not allowed
        "5-d",  // Invalid separator
        "d5",   // Wrong order
        "++5d", // Multiple plus signs
        "--5d", // Multiple negative signs
        "5dd",  // Repeated unit
    ];

    for input in inputs.iter() {
        assert!(
            parse_date(input).is_err(),
            "Expected error for input '{}' but got {:?}",
            input,
            parse_date(input).unwrap()
        );
    }
}

#[test]
fn test_invalid_invalid_weekday_formats() {
    let inputs = [
        // Invalid weekday names
        "mondey", // Misspelled
        "tues",   // Incomplete
        "wedns",  // Misspelled
        "thur",   // Incomplete
        "fridy",  // Misspelled
        "sat.",   // With punctuation
        "sun.",   // With punctuation
        // Invalid formats
        "mon-garbage", // Invalid suffix
        "monday-15",   // Invalid format
        "next-monday", // Invalid prefix
        "monday2",     // Invalid suffix
        // Mixed formats
        "monday-jan",  // Mixed types
        "monday-2024", // Mixed types
        "mon-15-2024", // Invalid combination
    ];

    for input in inputs.iter() {
        assert!(
            parse_date(input).is_err(),
            "Expected error for input '{}' but got {:?}",
            input,
            parse_date(input).unwrap()
        );
    }
}
