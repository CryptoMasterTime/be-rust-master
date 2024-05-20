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
    s.push_str(", Crypto"); // Append ", Master" to the String s
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
}
