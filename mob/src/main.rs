use core::{Call, RequestVote};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

mod core;

fn main() {
  // Get args
  let args: Vec<String> = std::env::args().collect();
  // let peers = &args[2..];

  // Start the TCP listener in a separate thread
  let address = args.get(1).unwrap_or(&String::from("127.0.0.1:8080")).clone();
  let listener_thread = thread::spawn(move || {
    let listener = TcpListener::bind(address.clone()).unwrap();
    println!("Listening on port {}...", address.clone());

    for stream in listener.incoming() {
      match stream {
        Ok(stream) => {
          println!("New connection: {}", stream.peer_addr().unwrap());
          thread::spawn(move || loop {
            let message: Result<Call, _> = bincode::deserialize_from(&stream);
            match message {
              Ok(message) => {
                println!("Received message: {:?}", message);
              }
              Err(e) => {
                println!("Error deserializing message: {:?}", e);
                break;
              }
            }
          });
        }
        Err(e) => {
          println!("Receiver Error: {}", e);
        }
      }
    }
  });

  // Start the TCP sender in the main thread
  let address = args.get(1).unwrap_or(&String::from("127.0.0.1:8080")).clone();
  let sender_thread = thread::spawn(move || {
    let mut stream = TcpStream::connect(address.clone()).unwrap();
    println!("Connected to server");

    let message = Call::RequestVote(RequestVote::new());
    match bincode::serialize_into(&mut stream, &message) {
      Ok(_) => {
        println!("Sent message: {:?}", message);
      }
      Err(e) => {
        println!("Sender Error: {}", e);
      }
    }

    // Sleep for 3 seconds
    std::thread::sleep(std::time::Duration::from_secs(3));

    println!("Sending 1 last message");
    match bincode::serialize_into(&mut stream, &message) {
      Ok(_) => {
        println!("Sent message: {:?}", message);
      }
      Err(e) => {
        println!("Sender Error: {}", e);
      }
    }
  });

  // Wait for both threads to finish
  listener_thread.join().unwrap();
  sender_thread.join().unwrap();
}
