use std::{
  net::{TcpListener, TcpStream},
  thread::{self, JoinHandle},
  time::Duration,
};

use crate::core::{Call, RequestVote};

pub fn spawn_mob(address: String, peers: Vec<String>) -> Vec<JoinHandle<()>> {
  let receiver = thread::spawn(move || {
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

  let sender = thread::spawn(move || {
    for peer in peers {
      let stream = match TcpStream::connect(peer) {
        Ok(stream) => stream,
        Err(e) => {
          println!("Sender Error: {}", e);
          continue;
        }
      };

      loop {
        // Sleep for 3 seconds
        thread::sleep(Duration::from_secs(3));

        let message = Call::RequestVote(RequestVote {
          term: 0,
          candidate_id: 0,
          last_log_index: 0,
          last_log_term: 0,
        });

        match bincode::serialize_into(&stream, &message) {
          Ok(_) => {
            println!("Sent message: {:?}", message);
          }
          Err(e) => {
            println!("Error serializing message: {:?}", e);
          }
        }
      }
    }
  });

  vec![receiver, sender]
}
