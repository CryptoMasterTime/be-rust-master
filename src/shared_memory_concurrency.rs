use std::sync::{Arc, Mutex};
use std::thread;

pub fn demonstrate_shared_memory_concurrency() {
    // Create a shared mutable integer, wrapped in a Mutex to ensure thread safety
    let shared_data = Arc::new(Mutex::new(0));

    // Start multiple threads to increment the shared integer value
    let handles: Vec<_> = (0..5).map(|_| {
        let shared_data = Arc::clone(&shared_data);
        thread::spawn(move || {
            for _ in 0..10 {
                // Use the lock method of Mutex to acquire the lock and access the shared data
                let mut data = shared_data.lock().unwrap();
                *data += 1;
                println!("Thread {:?} updated shared data: {}", thread::current().id(), *data);
            }
        })
    }).collect();

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value of the shared data
    let final_data = shared_data.lock().unwrap();
    println!("Final shared data value: {}", *final_data);
}
