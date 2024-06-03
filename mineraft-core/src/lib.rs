use call::Entry;

mod call;
mod state;
mod storage;

struct VolatileState {
  pub commit_index: u64,
  pub last_applied: u64,
}

pub struct LeaderVolatileState {
  pub next_index: Vec<u64>,
  pub match_index: Vec<u64>,
}

pub enum NodeState {
  Follower,
  Candidate,
  Leader,
}

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
