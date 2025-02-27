use std::fs::File;
use std::io::prelude::*;

struct Config {
    username: String,
    api_key: String,
    port: u16,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let username = lines.next().unwrap().to_string();// name
        let api_key = lines.next().unwrap().to_string();// SID student ID
        let port = lines.next().unwrap().parse().unwrap();

        Config { username, api_key, port }
    }
}

fn reading_from_file() {
    let config = Config::from_file("config.txt");
    println!("Student Name: {}", config.username);
    println!("SID: {}", config.api_key);
    println!("port: {}", config.port);
}
fn main() {
   // reading_from_console();
    reading_from_file();
}