use crate::tyme::{Culture, LoopTyme, Tyme};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

pub static SIX_STAR_NAMES: [&str; 6] = ["先胜", "友引", "先负", "佛灭", "大安", "赤口"];

/// 六曜（孔明六曜星）
#[derive(Debug, Clone)]
pub struct SixStar {
  parent: LoopTyme,
}

impl Deref for SixStar {
  type Target = LoopTyme;

  fn deref(&self) -> &Self::Target {
    &self.parent
  }
}

impl DerefMut for SixStar {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.parent
  }
}

impl Tyme for SixStar {
  fn next(&self, n: isize) -> Self {
    Self::from_index(self.parent.next_index(n) as isize)
  }
}

impl Culture for SixStar {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl SixStar {
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
}

impl Display for SixStar {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.get_name())
  }
}

impl PartialEq for SixStar {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for SixStar {}

impl From<SixStar> for LoopTyme {
  fn from(val: SixStar) -> Self {
    val.parent
  }
}

#[cfg(test)]
mod tests {
  use crate::tyme::culture::star::six::SixStar;
  use crate::tyme::solar::SolarDay;
  use crate::tyme::Culture;

  #[test]
  fn test1() {
    let star: SixStar = SolarDay::from_ymd(2020, 4, 23).get_lunar_day().get_six_star();
    assert_eq!("佛灭", star.get_name());
  }
}
