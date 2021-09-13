use std::collections::VecDeque;

#[derive(Debug)]
pub struct User {
    pub(crate) name: String,
    pub(crate) offset: i8,
}

pub struct App {
    /// Users and their timezone offset.
    pub timezone_data: VecDeque<User>,
}

impl App {
    pub fn new() -> App {
        App {
            timezone_data: VecDeque::new(),
        }
    }
}
