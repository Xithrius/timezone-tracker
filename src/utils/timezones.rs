use color_eyre::eyre::{anyhow, Context, Result};
use regex::{Captures, Regex};

/// Validates if the text inputted contains some username,
/// then a comma, then a timezone. The timezone can come with or without
/// "UTC" at the start.
pub fn validate_user_timezone_str(text: &str) -> Option<Captures> {
    let re = Regex::new("^(.*),(UTC)?([-+][0-9]{1,2})$").unwrap();

    if re.is_match(text) {
        re.captures(text)
    } else {
        None
    }
}

/// Parses the username and timezone offset, after validation.
pub fn parse_user_timezone(text: &str) -> Result<(String, i64)> {
    let captures = validate_user_timezone_str(text)
        .ok_or_else(|| anyhow!("User/offset not formatted properly."))?;

    let user = captures
        .get(1)
        .ok_or_else(|| anyhow!("Unable to retrieve username."))?
        .as_str();

    let offset_string = captures
        .get(captures.len() - 1)
        .ok_or_else(|| anyhow!("Unable to retrieve offset"))?
        .as_str();

    let offset = offset_string.parse::<i64>().with_context(|| {
        format!(
            "Unable to convert {} to a valid integer offset.",
            offset_string
        )
    });

    Ok((user.to_string(), offset.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_user_timezone_with_nothing() {
        if let Err(err) = parse_user_timezone("") {
            assert_eq!(err.to_string(), "User/offset not formatted properly.");
        }
    }

    #[test]
    fn test_parse_user_timezone_with_sep_no_username() {
        if let Err(err) = parse_user_timezone(",asdfUTC+8") {
            assert_eq!(err.to_string(), "User/offset not formatted properly.");
        }
    }

    #[test]
    fn test_parse_user_timezone_with_sep_no_timezone() {
        if let Err(err) = parse_user_timezone("ausername,") {
            assert_eq!(err.to_string(), "User/offset not formatted properly.");
        }
    }

    #[test]
    fn test_parse_user_timezone_with_valid_username_positive_timezone() {
        if let Ok((user, offset)) = parse_user_timezone("SomeName,UTC+4") {
            assert_eq!(user, "SomeName".to_string());
            assert_eq!(offset, 4);
        }
    }

    #[test]
    fn test_parse_user_timezone_with_valid_username_negative_timezone() {
        if let Ok((user, offset)) = parse_user_timezone("SomeName,UTC-4") {
            assert_eq!(user, "SomeName".to_string());
            assert_eq!(offset, -4);
        }
    }

    #[test]
    fn test_parse_user_timezone_with_and_without_utc_sep_positive_offset() {
        let (user0, offset0) = parse_user_timezone("SomeName,UTC+4").unwrap();
        let (user1, offset1) = parse_user_timezone("SomeName,+4").unwrap();

        assert_eq!(user0, user1);
        assert_eq!(offset0, offset1);
    }

    #[test]
    fn test_parse_user_timezone_with_and_without_utc_sep_negative_offset() {
        let (user0, offset0) = parse_user_timezone("SomeName,UTC-4").unwrap();
        let (user1, offset1) = parse_user_timezone("SomeName,-4").unwrap();

        assert_eq!(user0, user1);
        assert_eq!(offset0, offset1);
    }
}
