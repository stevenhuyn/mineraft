use mob::mob::{Identifier, Mob};

fn main() {
  let addresses = ["127.0.0.1:8080", "127.0.0.1:8081"];
  let identifiers: Vec<Identifier> =
    addresses.into_iter().enumerate().map(|(i, a)| Identifier::new(i as u64, a)).collect();
  let mut handles = vec![];
  for id in identifiers.iter() {
    let peers: Vec<Identifier> =``
      identifiers.iter().filter(|&p| p.address != id.address).cloned().collect();
    let mob = Mob::new(id.clone(), peers);
    let handle = mob.spawn();
    handles.extend(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }
}
