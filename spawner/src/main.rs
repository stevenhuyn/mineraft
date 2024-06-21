use mob::mob::spawn_mob;

fn main() {
  let addresses = ["127.0.0.1:8080", "127.0.0.1:8081", "127.0.0.1:8082"];
  let mut handles = vec![];
  for &address in addresses.iter() {
    let peers: Vec<String> =
      addresses.iter().filter(|&&p| p != address).map(|a| a.to_string()).collect();
    handles.extend(spawn_mob(address.to_string(), peers));
  }

  for handle in handles {
    handle.join().unwrap();
  }
}
