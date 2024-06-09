#[derive(Debug, Clone, Default)]
struct PersistentState {
  pub current_term: u64,
  pub voted_for: u64,
  pub log: Vec<Entry>,
}

impl PersistentState {
  pub fn new() -> Self {
    Self { current_term: 0, voted_for: 0, log: Vec::new() }
  }
}

#[derive(Debug, Clone, Default)]
struct VolatileState {
  pub commit_index: u64,
  pub last_applied: u64,
}

impl VolatileState {
  pub fn new() -> Self {
    Self { commit_index: 0, last_applied: 0 }
  }
}

#[derive(Debug, Clone, Default)]
pub struct LeaderVolatileState {
  pub next_index: Vec<u64>,
  pub match_index: Vec<u64>,
}

impl LeaderVolatileState {
  pub fn new() -> Self {
    Self { next_index: Vec::new(), match_index: Vec::new() }
  }
}

#[derive(Debug, Clone)]
pub enum Role {
  Follower,
  Candidate,
  Leader(LeaderVolatileState),
}

#[derive(Debug, Clone)]
pub enum Call {
  RequestVote(RequestVote),
  AppendEntries(AppendEntries),
}

#[derive(Debug, Clone, Default)]
pub struct RequestVote {
  pub term: u64,
  pub candidate_id: u64,
  pub last_log_index: u64,
  pub last_log_term: u64,
}

impl RequestVote {
  pub fn new() -> Self {
    Self { term: 0, candidate_id: 0, last_log_index: 0, last_log_term: 0 }
  }
}

#[derive(Debug, Clone, Default)]
pub struct AppendEntries {
  pub term: u64,
  pub leader_id: u64,
  pub prev_log_index: u64,
  pub prev_log_term: u64,
  pub entries: Vec<Entry>,
  pub leader_commit: u64,
}

impl AppendEntries {
  pub fn new() -> Self {
    Self {
      term: 0,
      leader_id: 0,
      prev_log_index: 0,
      prev_log_term: 0,
      entries: Vec::new(),
      leader_commit: 0,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Entry;

#[derive(Debug, Clone)]
pub struct Node {
  persistent_state: PersistentState,
  volatile_state: VolatileState,
  role: Role,
}

impl Default for Node {
  fn default() -> Self {
    Self::new()
  }
}

impl Node {
  pub fn new() -> Self {
    Self {
      persistent_state: PersistentState::new(),
      volatile_state: VolatileState::new(),
      role: Role::Follower,
    }
  }
}
