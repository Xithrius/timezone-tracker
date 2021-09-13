use anyhow::{anyhow, Context, Result};
use regex::Regex;

#[allow(dead_code)]
fn parse_timezone_offset(timezone_offset: &str) -> Result<i8> {
    let re = Regex::new("^UTC[-+]([0-9]{1,2})$").unwrap();

    let offset_string = re
        .captures(timezone_offset)
        .ok_or_else(|| anyhow!("Timezone string was formatted incorrectly."))?
        .get(1)
        .ok_or_else(|| anyhow!("Could not retrieve the integer offset from the timezone string."))?
        .as_str();

    offset_string.parse::<i8>().with_context(|| {
        format!(
            "Unable to convert {} to a valid integer offset.",
            timezone_offset
        )
    })
}
