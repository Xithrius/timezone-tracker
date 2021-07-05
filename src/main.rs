use std::{io, num::ParseIntError};

fn user_input(buffer: &str) -> Result<String, io::Error> {
    let mut user_input_string = String::new();

    println!("{}", &buffer);

    io::stdin().read_line(&mut user_input_string)?;

    Ok(user_input_string.trim().to_string())
}

fn parse_timezone_offset(timezone_offset: &str) -> Result<i8, ParseIntError> {
    Ok(timezone_offset[3..].parse::<i8>().unwrap())
}

fn main() -> io::Result<()> {
    let local_timezone: String = user_input("Local timezone offset: ")?;
    let offset_timezone: String = user_input("another test: ")?;

    println!("\n{}", &local_timezone);
    println!("{}", &offset_timezone);

    println!("{:?}", parse_timezone_offset(&offset_timezone));

    Ok(())
}
