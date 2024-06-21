use mob::mob::Mob;

fn main() {
  let addresses = ["127.0.0.1:8080", "127.0.0.1:8081", "127.0.0.1:8082"];
  let mut handles = vec![];
  for (i, &address) in addresses.iter().enumerate() {
    let peers: Vec<String> =
      addresses.iter().filter(|&&p| p != address).map(|a| a.to_string()).collect();
    let mob = Mob::new(i as u64, address, peers);
    let handle = mob.spawn();
    handles.extend(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }
}
