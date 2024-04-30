use std::fmt::{Display, Formatter};

use crate::tyme::{Culture, LoopTyme, Tyme};
use crate::tyme::culture::{Animal, Land, Luck, Zone};
use crate::tyme::culture::star::seven::SevenStar;

pub static TWENTY_EIGHT_STAR_NAMES: [&str; 28] = ["角", "亢", "氐", "房", "心", "尾", "箕", "斗", "牛", "女", "虚", "危", "室", "壁", "奎", "娄", "胃", "昴", "毕", "觜", "参", "井", "鬼", "柳", "星", "张", "翼", "轸"];

/// 二十八宿
#[derive(Debug, Clone)]
pub struct TwentyEightStar {
  parent: LoopTyme,
}

impl Tyme for TwentyEightStar {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for TwentyEightStar {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl TwentyEightStar {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(TWENTY_EIGHT_STAR_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(TWENTY_EIGHT_STAR_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  pub fn get_seven_star(&self) -> SevenStar {
    SevenStar::from_index((self.get_index() as isize) % 7 + 4)
  }

  pub fn get_land(&self) -> Land {
    Land::from_index([4, 4, 4, 2, 2, 2, 7, 7, 7, 0, 0, 0, 0, 5, 5, 5, 6, 6, 6, 1, 1, 1, 8, 8, 8, 3, 3, 3][self.get_index()])
  }

  pub fn get_zone(&self) -> Zone {
    Zone::from_index((self.get_index() as isize) / 7)
  }

  pub fn get_animal(&self) -> Animal {
    Animal::from_index(self.get_index() as isize)
  }

  pub fn get_luck(&self) -> Luck {
    Luck::from_index([0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0][self.get_index()])
  }
}

impl Display for TwentyEightStar {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for TwentyEightStar {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for TwentyEightStar {}

impl Into<LoopTyme> for TwentyEightStar {
  fn into(self) -> LoopTyme {
    self.parent
  }
}
