use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

pub fn start_network_handler() {
    // Create an mpsc channel for sending data from the network thread to the processing thread
    let (sender, receiver) = mpsc::channel();

    // Start the processing thread
    thread::spawn(move || {
        // The processing thread continuously receives data sent from the network thread and processes it
        loop {
            if let Ok(data) = receiver.recv() {
                // Here you can process the received data, such as printing or performing other operations
                println!("Received data: {}", data);
            }
        }
    });

    // Start the network thread
    thread::spawn(move || {
        // Create a TCP listener, listening on a local address and port
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        // Accept connections in a loop, handling each connection in a new thread
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            // Clone the sender for sending data within the closure
            let sender = sender.clone();

            // Start a new thread to handle the data on the connection
            thread::spawn(move || {
                handle_connection(stream, sender);
            });
        }
    });
}

// Handle data on a connection
fn handle_connection(stream: TcpStream, sender: mpsc::Sender<String>) {
    let reader = BufReader::new(&stream);

    // Read data from the connection and send it to the processing thread
    for line in reader.lines() {
        if let Ok(data) = line {
            // Send the data to the processing thread
            sender.send(data).unwrap();
        }
    }
}
