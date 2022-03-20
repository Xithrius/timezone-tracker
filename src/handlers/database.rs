use std::collections::VecDeque;

use rusqlite::{params, Connection as SqliteConnection};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct User {
    pub name: String,
    pub offset: i64,
}

impl User {
    pub fn new(name: String, offset: i64) -> Self {
        Self { name, offset }
    }
}

#[derive(Debug)]
pub struct Database {
    conn: SqliteConnection,
    pub users: VecDeque<User>,
}

#[allow(dead_code)]
impl Database {
    pub fn new(conn: SqliteConnection) -> Self {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                timezone_offset INTEGER
            )",
            [],
        )
        .expect("Failed to create database table");

        let users = conn
            .prepare("SELECT name, timezone_offset FROM users")
            .unwrap()
            .query_map([], |row| {
                Ok(User::new(row.get(0).unwrap(), row.get(1).unwrap()))
            })
            .unwrap()
            .flatten()
            .collect::<VecDeque<User>>();

        Self { conn, users }
    }

    pub fn add(&mut self, name: String, offset: i64) {
        if self.contains(&name) {
            self.conn
                .execute(
                    "UPDATE users SET timezone_offset = ?2 WHERE name = ?1",
                    params![name, offset.to_string()],
                )
                .unwrap();

            let i = self.users.iter().position(|x| x.name == name).unwrap();

            self.users[i].offset = offset;
        } else {
            self.conn
                .execute(
                    "INSERT INTO users (name, timezone_offset) VALUES (?1, ?2)",
                    params![name, offset.to_string()],
                )
                .unwrap();

            self.users.push_back(User::new(name, offset));
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        for user in &self.users {
            if user.name == name {
                return true;
            }
        }

        false
    }
}
