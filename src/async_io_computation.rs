use std::time::Duration;
use tokio::time::sleep;

// Define five different asynchronous IO operations
async fn async_io_operation_1() {
    println!("Starting async IO operation 1...");
    sleep(Duration::from_secs(2)).await;
    println!("Async IO operation 1 completed.");
}

async fn async_io_operation_2() {
    println!("Starting async IO operation 2...");
    sleep(Duration::from_secs(3)).await;
    println!("Async IO operation 2 completed.");
}

async fn async_io_operation_3() {
    println!("Starting async IO operation 3...");
    sleep(Duration::from_secs(1)).await;
    println!("Async IO operation 3 completed.");
}

async fn async_io_operation_4() {
    println!("Starting async IO operation 4...");
    sleep(Duration::from_secs(4)).await;
    println!("Async IO operation 4 completed.");
}

async fn async_io_operation_5() {
    println!("Starting async IO operation 5...");
    sleep(Duration::from_secs(2)).await;
    println!("Async IO operation 5 completed.");
}

// Define a function for heavy computation with a name to distinguish tasks
async fn perform_heavy_computation(name: &str) {
    println!("Starting heavy computation {}...", name);
    for i in 0..5 {
        println!("{} Computing... Step {}", name, i);
        sleep(Duration::from_millis(500)).await;
    }
    println!("Heavy computation {} completed.", name);
}

// Main function to demonstrate asynchronous operations
pub async fn async_example() {
    // Spawn the async IO operations
    let handle1 = tokio::spawn(async_io_operation_1());
    let handle2 = tokio::spawn(async_io_operation_2());
    let handle3 = tokio::spawn(async_io_operation_3());
    let handle4 = tokio::spawn(async_io_operation_4());
    let handle5 = tokio::spawn(async_io_operation_5());

    // Spawn two heavy computation tasks
    let heavy_computation_handle_1 = tokio::spawn(perform_heavy_computation("Task 1"));
    let heavy_computation_handle_2 = tokio::spawn(perform_heavy_computation("Task 2"));

    // Wait for the heavy computation tasks to complete
    heavy_computation_handle_1.await.unwrap();
    heavy_computation_handle_2.await.unwrap();

    // Wait for the IO operations to complete
    handle1.await.unwrap();
    handle2.await.unwrap();
    handle3.await.unwrap();
    handle4.await.unwrap();
    handle5.await.unwrap();
}
