
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // TODO: Create a shared counter using Arc and Mutex
    let count = Arc::new(Mutex::new(0));
    
    // TODO: Create a vector to store thread handles
    let mut handles = vec![];
    
    // TODO: Spawn 5 threads
    for _ in 1..=5 {
        // TODO: Clone the Arc for the thread
        let count_clone = Arc::clone(&count);
        
        // TODO: Spawn a thread that increments the counter 10 times
        let handle = thread::spawn(move || {
            // TODO: Increment counter 10 times
            for _ in 0..10 {
                let mut num = count_clone.lock().unwrap();
                *num += 1; // increment  num 10 times total
                // out of scope here at end!
            }
        });
        
        handles.push(handle);
    }
    
    // TODO: Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // TODO: Print the final value of the counter
    let final_count = *count.lock().unwrap();
    println!("Final counter value: {}", final_count);
}