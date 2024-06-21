use std::{
  net::TcpListener,
  thread::{self, JoinHandle},
};

use crate::core::Call;

pub fn spawn_mob(address: String, peers: Vec<String>) -> JoinHandle<()> {
  thread::spawn(move || {
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
  })
}
