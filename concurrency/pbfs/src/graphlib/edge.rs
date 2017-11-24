// Author: Takeshi I.
use std::cmp::Ordering;
#[derive(Eq)]
#[allow(unused)]
pub struct Edge {
  pub v: u32,
  pub w: u32,
  pub distance: u32,
}

#[allow(unused)]
impl Edge {
  pub fn new() -> Edge {
    Edge {
      v: 0,
      w: 0,
      distance: 0,
    }
  }
}


impl PartialOrd for Edge {
  fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Edge {
  fn cmp(&self, other: &Edge) -> Ordering {
    self.distance.cmp(&other.distance)
  }
}

impl PartialEq for Edge {
  fn eq(&self, other: &Edge) -> bool {
    self.distance == other.distance
  }
}