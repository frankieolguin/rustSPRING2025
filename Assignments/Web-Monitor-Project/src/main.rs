use std::env;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use reqwest;
use std::time::Instant;


// Website status structure
struct WebsiteStatus {
    url: String,
    action_status: Result<u16, String>,
    response_time: Duration,
    timestamp: SystemTime,
}







fn main() {
    println!("Hello, world!");
}
