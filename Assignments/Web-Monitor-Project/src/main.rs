use std::fs::File;
use std::io::{Write, Result};
use std::result;
use std::io::{BufReader, BufRead};

use std::env;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::available_parallelism; // this will help dynamically retrieve core count
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use reqwest;
use std::time::Instant;


// Website status structure
#[derive(Clone, Debug)]
struct WebsiteStatus {
    url: String,
    action_status: result::Result<u16, String>,
    response_time: Duration,
    timestamp: SystemTime,
}

// vector for URLS, usize for number of workers, and unsigned 64 for timeout time
// note: usize is good for sizes and in or case a threadpool amount
fn parse_cli_args() -> (Vec<String>, usize, u64) {
    /*  From command line, we can get the following:
        1. File for URls
        2. Number of Workers
        3. Timeout time
        4. Extra URLs 

        We can use a match statement for the different input options
     */
    let args: Vec<String> = env::args().collect(); // makes a vector of all out cli arguments
    let mut urls = Vec::new(); // List of URLs
    let mut file_path: Option<String> = None; // this saves the path to our "sites.txt" file
    let mut workers = available_parallelism().unwrap().get(); // Get available parallelism
    let mut timeout_time = 5; // Default timeout time = 5s

    // i = 0 is the program so we start at index 1 of our CLI
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                        i += 1;
                        if i < args.len() {
                            file_path = Some(args[i].clone());
                        }
            }
            "--workers" => {
                        i += 1;
                        if i < args.len() {
                            workers = args[i].parse().unwrap_or(workers);
                        }
            }
            "--timeout" => {
                        i += 1;
                        if i < args.len() {
                            timeout_time = args[i].parse().unwrap_or(timeout_time);
                        }
            }
            // this pushes other non '--' inputs as URLS to our list. 
            // it can be invalid and still be processed later.
            other if !other.starts_with("--") => {
                        urls.push(other.to_string());
            }
            _ => {} // ignore unidentified '--' inputs as they do not exist in our program
        }
        i += 1;
    }

    // next we get the URLS from the txt file from the given file path
    if let Some(path) = file_path {
        if let Ok(file) = File::open(&path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(url) = line {
                    let trimmed = url.trim();
                    if !trimmed.is_empty() && !trimmed.starts_with('#') {
                        urls.push(trimmed.to_string());
                    }
                }
            }
        } else {
            eprintln!("Could not open file: {}", path);
        }
    }
    // exit code for when no urls are provided by a file or by direct address
    if urls.is_empty() {
        eprintln!("Usage: website_checker [--file sites.txt] [URL ...] [--workers N] [--timeout S]");
        std::process::exit(2);
    }
    // exit and return urls, number of workers for thread pool, and timeout time in seconds
    (urls, workers, timeout_time)
}

fn create_thread_pool(urls: Vec<String>, workers: usize, timeout: u64) -> Vec<WebsiteStatus> {
    let results = Arc::new(Mutex::new(Vec::<WebsiteStatus>::new()));
    // We create the thread pool channel and spawn thread next
    let (tx, rx) = mpsc::channel::<String>();
    let rx = Arc::new(Mutex::new(rx));

    // a second channel for live reporting & live reporter thread
    let (tx_report, rx_report) = mpsc::channel::<WebsiteStatus>();
    let reporter = thread::spawn(move || {
        for res in rx_report {
            println!(
                "{} - Status: {:?} - Time: {:.2?}",
                res.url, res.action_status, res.response_time
            );
        }
    });

    // Spawn worker threads
    let mut handles = vec![];

    for _ in 0..workers {
        let rx = Arc::clone(&rx);
        let results = Arc::clone(&results);
        let timeout = Duration::from_secs(timeout);

         let tx_report = tx_report.clone(); // each worker can report

        let handle = thread::spawn(move || {
            while let Ok(url) = rx.lock().unwrap().recv() {
                let start = Instant::now();
                let status = reqwest::blocking::Client::new() // used for passing status to struct
                    .get(&url)
                    .timeout(timeout)
                    .send()
                    .and_then(|res| Ok(res.status().as_u16()))
                    .map_err(|e| e.to_string());
                let elapsed = start.elapsed(); // used for passing response time to the struct

                // we can build the WebsiteStatus struct and push it to our list for later
                let result = WebsiteStatus {
                    url,
                    action_status: status,
                    response_time: elapsed,
                    timestamp: SystemTime::now(),
                };

                results.lock().unwrap().push(result.clone());

                // transmit result to reporter for live output^^^
                tx_report.send(result).unwrap();
            }
        });

        handles.push(handle);
    }
    // create url tasks for workers
    for url in urls {
        tx.send(url).unwrap();
    }
    drop(tx); // important: ends all threads when URLs are done
    for handle in handles {
        handle.join().unwrap();
    }

    drop(tx_report); // closes live reporter thread
    reporter.join().unwrap();

    
    let results = results.lock().unwrap();
    
    // Optional: you can print or return results here
    /*
    for res in results.iter() {
        println!(
            "{} - Status: {:?}",
            res.url, res.action_status
        );
    }
    */

    // return struct to main for further json processing
    results.clone()
}

// manual JSON file creation
fn create_json_file(results: &Vec<WebsiteStatus>) -> Result<()>{
    let mut file = File::create("status.json")?;
    writeln!(file, "[")?;

    for (i, res) in results.iter().enumerate() {
        let timestamp = res.timestamp.duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let status_str = match &res.action_status {
            Ok(code) => code.to_string(),
            Err(err) => format!("\"{}\"", err.replace('"', "\\\"")),
        };

        writeln!(
            file,
            "  {{ \"url\": \"{}\", \"status\": {}, \"response_time\": {:.3}, \"timestamp\": {} }}{}",
            res.url.replace('"', "\\\""),
            status_str,
            res.response_time.as_secs_f64(),
            timestamp,
            if i < results.len() - 1 { "," } else { "" }
        )?;
    }

    writeln!(file, "]")?;
    Ok(())
}

fn main() {
    // 1. Parse Commands Line Args & Retrieve URLs from txt file
    // URLs will save in a Vec<> dara structure
    // let mut url_list = Vec::new; same as in parse() function
    let (urls, workers, timeout) = parse_cli_args();

    println!("Loaded {} URLs", urls.len());
    println!("Using {} worker(s)", workers);
    println!("Timeout per request: {}s", timeout);

    // 2. Create Worker Thread Pool & Give URLs to Workers
    // 3. Workers Work and Build Struct for JSON File + Timeouts
    // Output should be live with the use of a "reporter" thread
    let result_struct = create_thread_pool(urls,workers,timeout);

    // 4. Build JSON File
    create_json_file(&result_struct).expect("Failed to create JSON file!");
}
