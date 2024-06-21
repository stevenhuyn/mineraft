use std::{
  net::{TcpListener, TcpStream},
  sync::Arc,
  thread::{self, JoinHandle},
  time::Duration,
};

use crate::core::{Call, MobState, RequestVote};

pub struct Mob {
  id: u64,
  address: &'static str,
  peers: Vec<String>,
  state: MobState,
}

impl Mob {
  pub fn new(id: u64, address: &'static str, peers: Vec<String>) -> Self {
    Self { id, address, peers, state: MobState::new() }
  }

  pub fn spawn(&self) -> Vec<JoinHandle<()>> {
    let receiver_address = self.address;
    let receiver = thread::spawn(move || {
      let listener = TcpListener::bind(receiver_address).unwrap();
      println!("Listening on port {}...", receiver_address);

      for stream in listener.incoming() {
        match stream {
          Ok(stream) => {
            println!(
              "Accept: {:-4} -> {:-4}",
              receiver_address.clone(),
              stream.peer_addr().unwrap()
            );
            let receiver_address = receiver_address.clone();
            thread::spawn(move || loop {
              let message: Result<Call, _> = bincode::deserialize_from(&stream);
              match message {
                Ok(_message) => {
                  println!(
                    "Receive: {:-4} -> {:-4}",
                    receiver_address.clone(),
                    stream.peer_addr().unwrap()
                  );
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

    let peers = self.peers.clone();
    let sender_address = self.address;
    let sender = thread::spawn(move || {
      for peer in peers.iter() {
        let stream = match TcpStream::connect(peer) {
          Ok(stream) => stream,
          Err(e) => {
            println!("Sender Error: {}", e);
            continue;
          }
        };

        Self::heartbeat(&stream, sender_address);
      }
    });

    vec![receiver, sender]
  }

  fn heartbeat(stream: &TcpStream, address: &str) {
    loop {
      // Sleep for random time between 2.5 and 3.5 seconds
      let sleep_time = rand::random::<f64>() + 2.5;
      thread::sleep(Duration::from_millis((sleep_time * 1000.0) as u64));

      let message = Call::RequestVote(RequestVote {
        term: 0,
        candidate_id: 0,
        last_log_index: 0,
        last_log_term: 0,
      });

      match bincode::serialize_into(stream, &message) {
        Ok(_) => {
          println!("Send: {:-4} to {:?}", address, stream.peer_addr());
        }
        Err(e) => {
          println!("Error serializing message: {:?}", e);
        }
      }
    }
  }
}
