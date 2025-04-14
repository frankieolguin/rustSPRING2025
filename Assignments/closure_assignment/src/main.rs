use std::{thread, time::Duration};

fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        // Your implementation here
        tracker+=1;
        println!("update! tracker is {}", tracker);
    };

    update();
    update();
}
//with map
fn process_vector_with_map<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    // Your implementation here
    vec.into_iter().map(f).collect()
}
// for loop
fn process_vector_with_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    // Your implementation here
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x)); // Apply the closure
    }
    result
}
// task 5
struct ComputeCache<T>
where
    T: Fn() -> String,
{
    // Add fields here
    computation: T,
    result: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        // Your implementation here
        ComputeCache{
            computation,
            result: None,
        }
    }

    fn get_result(&mut self) -> String {
        // Your implementation here
        match &self.result {
            Some(r) => {
                println!("Retrieved from cache instantly!");
                r.clone()
            }
            None => {
                thread::sleep(Duration::from_secs(2));
                let r = (self.computation)();
                self.result = Some(r.clone());
                r
            }
        }
    }
}

fn main() {
    // task 1
    println!("Task 1\n");
    let operation = |a: i32, b: i32| {
        // Your implementation here
        // Write a closure named operation that multiplies 
        // two integers and returns the result. 
        // Test it with 10 * 5 and print the result.
        a*b
    };
    println!("Result: {}", operation(10, 5));

    // task 2
    println!("\nTask 2\n");
    // Write a closure named update inside a function track_changes. 
    // The closure should increment and print a counter each time it is called.
    track_changes();
    // task 3 & 4
    println!("\nTask 3 & 4\n");
    // Write a function process_vector that applies a closure to transform each element of a vector. Implement it in both ways:
    // Using map and collect
    // Using a for loop
    let numbers = vec![1, 2, 3];
    let original = numbers.clone();
    println!("Doubled with for loop.");
    let doubled = process_vector_with_loop(numbers.clone(), |x| {
        // Implement: multiply each number by 2
        x*2
    });
    println!("Replaced with for map and collect.\n");
    let replaced = process_vector_with_map(numbers, |x| {
        // Implement: if number > 2, replace with 0, else keep number
        if x>2 {0}
        else {x}
    });
    println!("Original {:?}", original);
    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);

    // task 5
    println!("\nTask 5\n");
    // Write a struct ComputeCache that accepts a closure during initialization. 
    // Cache the result after the first computation.
    //  Use thread::sleep to simulate an expensive computation.
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());

    println!("\nAssignment complete!");
}
