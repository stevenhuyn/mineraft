use std::{
  io::{Read, Write},
  net::TcpStream,
};

fn main() {
  let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

  stream.write_all(&[1]).unwrap();
  stream.read_exact(&mut [0; 128]).unwrap();
}
