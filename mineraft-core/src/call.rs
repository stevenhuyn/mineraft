pub enum Call {
  RequestVote(RequestVote),
  AppendEntries(),
}

pub struct RequestVote {
  pub term: u64,
  pub candidate_id: u64,
  pub last_log_index: u64,
  pub last_log_term: u64,
}

pub struct AppendEntries {
  pub term: u64,
  pub leader_id: u64,
  pub prev_log_index: u64,
  pub prev_log_term: u64,
  pub entries: Vec<Entry>,
  pub leader_commit: u64,
}

pub struct Entry;
