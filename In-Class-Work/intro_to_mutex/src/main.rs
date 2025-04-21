fn share_data_between_threads_and_mutate() {
    use std::sync::Arc; // atomic reference counter(smart pointer)
    use std::sync::Mutex; // mutex -> mutual exclusive
    // smart pointer which guarantess that only one thread with lock
    // acquired will be able to mutate the value inside
    
    println!("Intro to Concurrency");
    let steps =  Arc::new(Mutex::new(5));
    let thread = {
        let steps = steps.clone();
        std::thread::spawn(move ||{
            while *steps.lock().unwrap() > 0{
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("Thread step {}",steps.lock().unwrap());
                *steps.lock().unwrap() -=1 ;
                // unlock() gets called after we are done?
                // we need to keep calling lock in order maintain
                // control of the mutex.
            }
            "Goodbye!" // important thread could return values
        })
    };

    println!("Spawned a thread!");

    // Very important moment to understand closure captures
    // the environment
    
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("Main thread slept for 3 seconds");
    
    let result = thread.join().unwrap(); 
    println!("Thread returned: {:?}", result);
}

fn main() {
    println!("compiling!");
    share_data_between_threads_and_mutate();
}
// if mutex is locked and thread panics, then the thread does holding the "lock"
// and there is no unlock. This is deadlock.
// Imagine producer consumer problem where both try to access a mutex value and must wait for each other.
