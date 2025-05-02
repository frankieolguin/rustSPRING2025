use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    // added these constant values to match format from question in producer and consumer creation
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;
    // TODO: Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();
    let shared_rx = Arc::new(Mutex::new(rx));
    // TODO: Create 2 producer threads
    let mut producer_handles = vec![];
    for i in 0..NUM_PRODUCERS {
        let tx_clone = tx.clone();
        let items_per_producer = ITEM_COUNT / NUM_PRODUCERS;
        let handle = thread::spawn(move || {
            producer(i, tx_clone, items_per_producer);
        });
        producer_handles.push(handle);
    }
    
    // TODO: Create 3 consumer threads
    let mut consumer_handles = vec![];
    for i in 0..NUM_CONSUMERS {
        let rx_clone = Arc::clone(&shared_rx);
        let handle = thread::spawn(move || {
            consumer(i, rx_clone);
        });
        consumer_handles.push(handle);
    }
    
    // TODO: Wait for all threads to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }
    
    println!("20 items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal
    let mut rng = rand::thread_rng();
    for _ in 0..item_count {
        let value = rng.gen_range(1..100);
        println!("Producer {}   Produced: {}", id, value);
        tx.send(value).unwrap();
        thread::sleep(Duration::from_millis(100)); // Simulate work
    }
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal
    loop {
        let received = {
            let lock = rx.lock().unwrap();
            lock.recv()
        };

        match received {
            Ok(value) => {
                if value == TERMINATION_SIGNAL {
                    println!("Consumer {} Received termination signal. Exiting...", id);
                    break;
                } else {
                    println!("Consumer {}   Consumed: {}", id, value);
                    thread::sleep(Duration::from_millis(150)); // Simulate processing
                }
            }
            Err(_) => {
                println!("Exiting..."); // exits thread
                break;
            }
        }
    }
}