fn rw_locks() {
    // comparing with mutex which does not separate with reading and writing
    // to the data inside of mutex Read and Write lock, allows to separate that logic
    // like many readers and single writer, very close to RefCell.
    // but this idea is not new or unique to Rust, this idea existed before dawn
    // in Java and C++
    
    use std::sync::{Arc, RwLock};
    use std::thread;
    
    let data = Arc::new(RwLock::new("Hello World".to_string()));
    use std::time;
    let ten_millis = time::Duration::from_millis(10);
    let twenty_millis = time::Duration::from_millis(40);
    
    let reader_a = {
        let data = data.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let data_for_read = data.read().unwrap();
                println!("Data from reader_A: {} ",data_for_read);
                thread::sleep(ten_millis);
            }
        })
    };
    
    let reader_b = {
        let data = data.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let data_for_read = data.read().unwrap();
                println!("Data from reader_B: {} ",data_for_read);
                thread::sleep(ten_millis);
            }
        })
    };
    
    let writer = {
        let data = data.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let mut data_to_write = data.write().unwrap();
                data_to_write.push('!');
                println!("Updating data {} ",data_to_write);
                thread::sleep(twenty_millis);
                
            }
        })
        };
        
    reader_a.join().unwrap();
    reader_b.join().unwrap();
    writer.join().unwrap();
    
    println!("{:?}",data);
    
}

fn main() {
    println!("Starting Sim!\n");
    rw_locks();
}
