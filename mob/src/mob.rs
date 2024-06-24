use std::{
  net::{TcpListener, TcpStream},
  thread::{self, JoinHandle},
  time::Duration,
};

use crate::core::{Call, MobState, RequestVote};

pub struct Mob {
  id: Identifier,
  peers: Vec<Identifier>,
  state: MobState,
}

#[derive(Debug, Clone)]
pub struct Identifier {
  pub id: u64,
  pub address: &'static str,
}

impl Identifier {
  pub fn new(id: u64, address: &'static str) -> Self {
    Self { id, address }
  }
}

impl Mob {
  pub fn new(id: Identifier, peers: Vec<Identifier>) -> Self {
    Self { id, peers, state: MobState::new() }
  }

  pub fn spawn(&self) -> Vec<JoinHandle<()>> {
    let receiver = thread::spawn({
      let receiver_address = self.id.address;
      let receiver_id = self.id.id;
      move || {
        let listener = TcpListener::bind(receiver_address).unwrap();
        println!("Listening on port {}...", receiver_address);

        for peer_stream in listener.incoming() {
          match peer_stream {
            Ok(peer_stream) => Self::setup_listener(peer_stream, receiver_id),
            Err(e) => println!("Receiver Error: {}", e),
          }
        }
      }
    });

    let sender = thread::spawn({
      let peers = self.peers.clone();
      let source_id = self.id.id;
      move || {
        for peer in peers.iter() {
          let stream = match TcpStream::connect(peer.address) {
            Ok(stream) => stream,
            Err(e) => {
              println!("Sender Error: {}", e);
              continue;
            }
          };

          Self::heartbeat(&stream, source_id, peer.id);
        }
      }
    });

    vec![receiver, sender]
  }

  /// Setups a listener for incoming messages from a specific peer
  fn setup_listener(peer_stream: TcpStream, receiver_id: u64) {
    thread::spawn({
      move || loop {
        let message: Result<Call, _> = bincode::deserialize_from(&peer_stream);
        match message {
          Ok(message) => {
            println!("{}: Receive from {}", receiver_id, message.id())
          }
          Err(e) => {
            println!("Error deserializing message: {:?}", e);
            break;
          }
        }
      }
    });
  }

  fn heartbeat(stream: &TcpStream, source_id: u64, peer_id: u64) {
    loop {
      // Sleep for random time between 2.5 and 3.5 seconds
      let sleep_time = rand::random::<f64>() + 2.5;
      thread::sleep(Duration::from_millis((sleep_time * 1000.0) as u64));

      let message = Call::RequestVote(RequestVote {
        term: 0,
        candidate_id: source_id,
        last_log_index: 0,
        last_log_term: 0,
      });

      match bincode::serialize_into(stream, &message) {
        Ok(_) => {
          println!("{}: send to {}", source_id, peer_id);
        }
        Err(e) => {
          println!("Error serializing message: {:?}", e);
        }
      }
    }
  }
}
