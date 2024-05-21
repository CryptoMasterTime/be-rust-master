use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

const NUM_ELEMENTS: usize = 100_000_000; // 100 million elements
const NUM_THREADS: usize = 12; // Number of threads to use

// Single-threaded computation
pub fn single_thread_computation() -> u64 {
    let data = vec![1u64; NUM_ELEMENTS]; // Initialize an array with 100 million elements, each element is 1

    let start_time = Instant::now();
    let sum_of_squares: u64 = data.iter().map(|&x| x * x).sum();
    let duration = start_time.elapsed();

    // println!("Single-threaded computation took: {:?}", duration);
    println!("Single-threaded computation took: \x1b[31m{:?}\x1b[0m", duration);
    sum_of_squares
}

// Multi-threaded computation
pub fn multi_thread_computation() -> u64 {
    let data = Arc::new(vec![1u64; NUM_ELEMENTS]); // Wrap the array in an Arc to share it between threads

    let start_time = Instant::now();
    let mut handles = vec![];

    let chunk_size = NUM_ELEMENTS / NUM_THREADS;

    let result = Arc::new(Mutex::new(0u64));

    for i in 0..NUM_THREADS {
        let data = Arc::clone(&data);
        let result = Arc::clone(&result);

        let handle = thread::spawn(move || {
            let start = i * chunk_size;
            let end = if i == NUM_THREADS - 1 {
                NUM_ELEMENTS
            } else {
                start + chunk_size
            };

            let sum: u64 = data[start..end].iter().map(|&x| x * x).sum();
            let mut result = result.lock().unwrap();
            *result += sum;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start_time.elapsed();
    // println!("Multi-threaded computation took: {:?}", duration);
    println!("Multi-threaded computation took: \x1b[31m{:?}\x1b[0m", duration);


    let result = *result.lock().unwrap();
    result
}
