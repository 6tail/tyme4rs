use std::fmt::{Display, Formatter};

use crate::tyme::{Culture, LoopTyme, Tyme};

pub static TEN_STAR_NAMES: [&str; 10] = ["比肩", "劫财", "食神", "伤官", "偏财", "正财", "七杀", "正官", "偏印", "正印"];

/// 十神
#[derive(Debug, Clone)]
pub struct TenStar {
  parent: LoopTyme,
}

impl Tyme for TenStar {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for TenStar {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl TenStar {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(TEN_STAR_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(TEN_STAR_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for TenStar {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for TenStar {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for TenStar {}

impl Into<LoopTyme> for TenStar {
  fn into(self) -> LoopTyme {
    self.parent
  }
}
