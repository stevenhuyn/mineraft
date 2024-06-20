use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

mod core;

fn main() {
  // Get args
  let args: Vec<String> = std::env::args().collect();
  // let peers = &args[2..];

  // Create a channel for communication between the listener and sender
  let (tx, rx) = mpsc::channel();

  // Start the TCP listener in a separate thread
  let listener_tx = tx.clone();
  let address = args[1].clone();
  let listener_thread = thread::spawn(move || {
    let listener = TcpListener::bind(address.clone()).unwrap();
    println!("Listening on port {}...", address.clone());

    for stream in listener.incoming() {
      match stream {
        Ok(stream) => {
          println!("New connection: {}", stream.peer_addr().unwrap());
          let listener_tx = listener_tx.clone();
          thread::spawn(move || {
            handle_client(stream, listener_tx);
          });
        }
        Err(e) => {
          println!("Error: {}", e);
        }
      }
    }
  });

  // Start the TCP sender in the main thread
  let sender_tx = tx.clone();
  let address = args[1].clone();
  let sender_thread = thread::spawn(move || {
    let mut stream = TcpStream::connect(address.clone()).unwrap();
    println!("Connected to server");
    sender_tx.send("Sup bro".to_string()).unwrap();

    loop {
      // Wait for messages from the listener
      let received_message = rx.recv().unwrap();
      println!("Received from listener: {}", received_message);

      // Send a response based on the received message
      let response = format!("Response to: {}", received_message);
      stream.write_all(response.as_bytes()).unwrap();
      println!("Sent: {}", response);
    }
  });

  // Wait for both threads to finish
  listener_thread.join().unwrap();
  sender_thread.join().unwrap();
}

fn handle_client(mut stream: TcpStream, tx: mpsc::Sender<String>) {
  let mut buffer = [0; 1024];
  let bytes_read = stream.read(&mut buffer).unwrap();
  let message = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
  println!("Received: {}", message);

  // Send the received message to the sender thread
  tx.send(message).unwrap();

  let mut response = String::new();
  stream.read_to_string(&mut response).unwrap();
  println!("Received response: {}", response);
}
