use crate::tyme::culture::{Element, Luck};
use crate::tyme::{Culture, LoopTyme, Tyme};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

pub static SIX_STAR_NAMES: [&str; 6] = ["大安", "留连", "速喜", "赤口", "小吉", "空亡"];

/// 小六壬
#[derive(Debug, Clone)]
pub struct MinorRen {
  parent: LoopTyme,
}

impl Deref for MinorRen {
  type Target = LoopTyme;

  fn deref(&self) -> &Self::Target {
    &self.parent
  }
}

impl DerefMut for MinorRen {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.parent
  }
}

impl Tyme for MinorRen {
  fn next(&self, n: isize) -> Self {
    Self::from_index(self.parent.next_index(n) as isize)
  }
}

impl Culture for MinorRen {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl MinorRen {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(SIX_STAR_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Self {
    Self {
      parent: LoopTyme::from_name(SIX_STAR_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name)
    }
  }

  /// 吉凶
  pub fn get_luck(&self) -> Luck {
    Luck::from_index(self.get_index() as isize % 2)
  }

  /// 五行
  pub fn get_element(&self) -> Element {
    Element::from_index([0, 4, 1, 3, 0, 2][self.get_index()])
  }
}

impl Display for MinorRen {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.get_name())
  }
}

impl PartialEq for MinorRen {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for MinorRen {}

impl From<MinorRen> for LoopTyme {
  fn from(val: MinorRen) -> Self {
    val.parent
  }
}

#[cfg(test)]
mod tests {
  use crate::tyme::culture::ren::minor::MinorRen;
  use crate::tyme::lunar::{LunarDay, LunarHour, LunarMonth};
  use crate::tyme::Culture;

  #[test]
  fn test1() {
    let minor_ren: MinorRen = LunarDay::from_ymd(2024, 3, 5).get_minor_ren();
    assert_eq!("大安", minor_ren.get_name());
  }

  #[test]
  fn test2() {
    let minor_ren: MinorRen = LunarHour::from_ymd_hms(2024, 9, 7, 10, 0, 0).get_minor_ren();
    assert_eq!("留连", minor_ren.get_name());
  }

  #[test]
  fn test3() {
    let minor_ren: MinorRen = LunarMonth::from_ym(1991, 3).get_minor_ren();
    assert_eq!("速喜", minor_ren.get_name());
  }
}
