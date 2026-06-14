use chrono::{DateTime, Utc};
use phenotype_time::Timestamp;

#[test]
fn parse_valid_iso8601_utc() {
    let dt = DateTime::<Utc>::parse("2024-03-15T12:30:00Z").unwrap();
    assert_eq!(dt.to_iso(), "2024-03-15T12:30:00+00:00");
}

#[test]
fn parse_valid_iso8601_with_offset() {
    let dt = DateTime::<Utc>::parse("2024-03-15T12:30:00+05:30").unwrap();
    assert_eq!(dt.to_iso(), "2024-03-15T07:00:00+00:00");
}

#[test]
fn parse_unix_epoch() {
    let dt = DateTime::<Utc>::parse("1970-01-01T00:00:00Z").unwrap();
    assert_eq!(dt.timestamp(), 0);
}

#[test]
fn parse_invalid_string_fails() {
    let result = DateTime::<Utc>::parse("not-a-date");
    assert!(result.is_err());
}

#[test]
fn parse_empty_string_fails() {
    let result = DateTime::<Utc>::parse("");
    assert!(result.is_err());
}
