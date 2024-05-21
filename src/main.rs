mod error_handling_functions;
mod multi_thread_processor;
mod network_handler;
mod shared_memory_concurrency;
use error_handling_functions::*;

fn ownership_example() {
    let s1 = String::from("Crypto"); // Create a new String s1 containing "Crypto"
    let s2 = s1; // Move ownership of s1 to s2, s1 is no longer valid
    println!("{}", s2); // Print the content of s2, which is "Crypto"
    // println!("{}", s1); // Attempting to use s1 here would result in a compile-time error
}

fn calculate_length(s: &String) -> usize {
    s.len() // Return the length of the String s
}

fn immutable_borrowing_example() {
    let s = String::from("Crypto"); // Create a new String s containing "Crypto"
    let len = calculate_length(&s); // Pass a reference to s to calculate its length
    println!("The length of '{}' is {}.", s, len); // Print the content of s and its length
    // s is still valid here because only an immutable reference to it is used
}

fn change(s: &mut String) {
    s.push_str(", Master    https://linktr.ee/cryptomastertime"); // Append ", Master" to the String s
}

fn mutable_borrowing_example() {
    let mut s = String::from("Crypto"); // Create a mutable String s containing "Crypto"
    change(&mut s); // Pass a mutable reference to s to append ", Master"
    println!("Changed string: {}", s); // Print the modified content of s
}

fn borrowing_rules_example() {
    let mut s = String::from("Crypto"); // Create a mutable String s containing "Crypto"
    {
        let r1 = &s; // Immutable borrow of s
        let r2 = &s; // Another immutable borrow of s
        println!("{} and {}", r1, r2); // Print the content of r1 and r2
    } // r1 and r2 go out of scope here

    let r3 = &mut s; // Mutable borrow of s
    r3.push_str(", Master"); // Modify the content of s
    println!("{}", r3); // Print the modified content of s
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x // Return x if its length is greater than y's length
    } else {
        y // Otherwise, return y
    }
}

fn lifetimes_example() {
    let string1 = String::from("long string is long"); // Create a new String string1
    let string2 = "xyz"; // Create a string slice string2 containing "xyz"
    let result = longest(string1.as_str(), string2); // Find the longest string between string1 and string2
    println!("The longest string is {}", result); // Print the longest string
}

struct ImportantExcerpt<'a> {
    part: &'a str, // Define a struct ImportantExcerpt with a lifetime parameter 'a
}

fn explicit_lifetimes_example() {
    let novel = String::from("Call me Jeff. Some years ago..."); // Create a new String novel
    let first_sentence = novel.split('.').next().expect("Could not find a '.'"); // Get the first sentence of the novel
    let i = ImportantExcerpt {
        part: first_sentence, // Create an instance of ImportantExcerpt with the first sentence
    };
    println!("Important excerpt: {}", i.part); // Print the important excerpt
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // Convert the string to bytes
    for (i, &item) in bytes.iter().enumerate() { // Iterate through the bytes
        if item == b' ' { // If a space is encountered
            return &s[0..i]; // Return the substring up to the space
        }
    }
    &s[..] // If no space is found, return the entire string
}

fn lifetime_elision_example() {
    let my_string = String::from("Crypto Master"); // Create a new String my_string
    let word = first_word(&my_string); // Get the first word of my_string
    println!("The first word is {}", word); // Print the first word
}

fn main() {
    ownership_example(); // Demonstrate ownership concepts
    immutable_borrowing_example(); // Demonstrate immutable borrowing
    mutable_borrowing_example(); // Demonstrate mutable borrowing
    borrowing_rules_example(); // Demonstrate borrowing rules
    lifetimes_example(); // Demonstrate lifetime annotations
    explicit_lifetimes_example(); // Demonstrate explicit lifetime annotations
    lifetime_elision_example(); // Demonstrate lifetime elision


    match file_handling() {
        Ok(_) => println!("File handling successful"),
        Err(err) => eprintln!("File handling error: {}", err),
    }

    match network_operation() {
        Ok(_) => println!("Network operation successful"),
        Err(err) => eprintln!("Network operation error: {}", err),
    }

    match parse_user_input("abc") {
        Ok(num) => println!("Parsed user input: {}", num),
        Err(err) => eprintln!("User input parsing error: {}", err),
    }

    match data_parsing("xyz") {
        Ok(num) => println!("Parsed data: {}", num),
        Err(err) => eprintln!("Data parsing error: {}", err),
    }

    match system_call() {
        Ok(_) => println!("System call successful"),
        Err(err) => eprintln!("System call error: {}", err),
    }

    match concurrent_operation() {
        Ok(_) => println!("Concurrent operation successful"),
        Err(err) => eprintln!("Concurrent operation error: {}", err),
    }

    match third_party_call() {
        Ok(_) => println!("Third party call successful"),
        Err(err) => eprintln!("Third party call error: {}", err),
    }

    match math_operation() {
        Ok(_) => println!("Math operation successful, if change the divided data to zero, it can not finish compilement process."),
        Err(err) => eprintln!("Math operation error: {}", err),
    }

    match memory_allocation() {
        Ok(_) => println!("Memory allocation successful"),
        Err(err) => eprintln!("Memory allocation error: {}", err),
    }

    match environment_configuration() {
        Ok(_) => println!("Environment configuration successful"),
        Err(err) => eprintln!("Environment configuration error: {}", err),
    }

    println!("\nStarting single-threaded computation...");
    let single_thread_result = multi_thread_processor::single_thread_computation();
    println!("Single-threaded result: {}", single_thread_result);

    println!("\nStarting multi-threaded computation...");
    let multi_thread_result = multi_thread_processor::multi_thread_computation();
    println!("Multi-threaded result: {}\n", multi_thread_result);

    // network_handler::start_network_handler();
    // loop{};

    shared_memory_concurrency::demonstrate_shared_memory_concurrency();

}
