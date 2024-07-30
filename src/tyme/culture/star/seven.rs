use std::fmt::{Display, Formatter};

use crate::tyme::{Culture, LoopTyme, Tyme};

pub static SEVEN_STAR_NAMES: [&str; 7] = ["日", "月", "火", "水", "木", "金", "土"];

/// 七曜（七政、七纬、七耀）
#[derive(Debug, Clone)]
pub struct SevenStar {
  parent: LoopTyme,
}

impl Tyme for SevenStar {
  fn next(&self, n: isize) -> Self {
    Self::from_index(self.parent.next_index(n) as isize)
  }
}

impl Culture for SevenStar {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl SevenStar {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(SEVEN_STAR_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Self {
    Self {
      parent: LoopTyme::from_name(SEVEN_STAR_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name)
    }
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for SevenStar {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for SevenStar {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for SevenStar {}

impl Into<LoopTyme> for SevenStar {
  fn into(self) -> LoopTyme {
    self.parent
  }
}
