// Define a simple macro for printing messages
#[macro_export]
macro_rules! print_message {
    ($msg:expr) => {
        println!("Message: {}", $msg);
    };
}

// Define a generic function to return the sum of two values
pub fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// Define a trait indicating that a message can be displayed
pub trait DisplayMessage {
    fn display(&self);
}

// Implement the DisplayMessage trait for a struct
pub struct Message {
    pub content: String,
}

impl DisplayMessage for Message {
    fn display(&self) {
        println!("Displaying message: {}", self.content);
    }
}

// Define a function that contains the closure
pub fn calculate_product(a: i32, b: i32) -> i32 {
    let multiply = |a: i32, b: i32| -> i32 { a * b };
    multiply(a, b)
}
