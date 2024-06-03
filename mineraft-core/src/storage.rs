use crate::call::Entry;

struct PersistentState {
  pub current_term: u64,
  pub voted_for: u64,
  pub log: Vec<Entry>,
}
