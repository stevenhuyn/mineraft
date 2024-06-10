use std::{
  io::{prelude::*, BufReader},
  net::{TcpListener, TcpStream},
};

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  let mut buffer = [0; 128];
  for stream in listener.incoming() {
    let mut stream = stream.unwrap();
    println!("{}", stream.read(&mut buffer).unwrap());
  }
}
