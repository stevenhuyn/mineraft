pub struct Follower;

pub struct Candidate;

pub struct Leader;

impl Follower {
  pub fn new() -> Self {
    Follower
  }

  pub fn on_request_vote(&self) {
    println!("Follower: RequestVote");
  }
}

impl Candidate {
  pub fn new() -> Self {
    Candidate
  }

  pub fn on_request_vote(&self) {
    println!("Candidate: RequestVote");
  }
}

impl Leader {
  pub fn new() -> Self {
    Leader
  }

  pub fn on_request_vote(&self) {
    println!("Leader: RequestVote");
  }
}
