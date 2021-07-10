use anyhow::{anyhow, Context, Result};
use diesel::SqliteConnection;
use regex::Regex;
use std::io;
use timezone_tracker::establish_connection;

fn user_input(buffer: &str) -> Result<String, io::Error> {
    let mut user_input_string = String::new();

    println!("{}", &buffer);

    io::stdin().read_line(&mut user_input_string)?;

    Ok(user_input_string.trim().to_string())
}

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

fn main() -> Result<()> {
    let conn: SqliteConnection = establish_connection();

    let local_timezone: String = user_input("Local timezone offset: ")?;
    let offset_timezone: String = user_input("another test: ")?;

    println!("{:?}", parse_timezone_offset(&local_timezone));
    println!("{:?}", parse_timezone_offset(&offset_timezone));

    Ok(())
}
