use std::fmt::{Display, Formatter};

use crate::tyme::{Culture, LoopTyme, Tyme};
use crate::tyme::culture::Luck;

pub static ECLIPTIC_NAMES: [&str; 2] = ["黄道", "黑道"];

/// 黄道黑道
#[derive(Debug, Clone)]
pub struct Ecliptic {
  parent: LoopTyme,
}

impl Tyme for Ecliptic {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Ecliptic {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Ecliptic {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(ECLIPTIC_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(ECLIPTIC_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  pub fn get_luck(&self) -> Luck {
    Luck::from_index(self.get_index() as isize)
  }
}

impl Display for Ecliptic {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Ecliptic {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Ecliptic {}

impl Into<LoopTyme> for Ecliptic {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static TWELVE_STAR_NAMES: [&str; 12] = ["青龙", "明堂", "天刑", "朱雀", "金匮", "天德", "白虎", "玉堂", "天牢", "玄武", "司命", "勾陈"];

/// 黄道黑道十二神
#[derive(Debug, Clone)]
pub struct TwelveStar {
  parent: LoopTyme,
}

impl Tyme for TwelveStar {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for TwelveStar {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl TwelveStar {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(TWELVE_STAR_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(TWELVE_STAR_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  pub fn get_ecliptic(&self) -> Ecliptic {
    Ecliptic::from_index([0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1][self.get_index()])
  }
}

impl Display for TwelveStar {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for TwelveStar {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for TwelveStar {}

impl Into<LoopTyme> for TwelveStar {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

#[cfg(test)]
mod tests {
  use crate::tyme::Culture;
  use crate::tyme::culture::star::twelve::TwelveStar;
  use crate::tyme::solar::SolarDay;

  #[test]
  fn test1() {
    let star: TwelveStar = SolarDay::from_ymd(2023, 10, 30).unwrap().get_lunar_day().get_twelve_star();
    assert_eq!("天德", star.get_name());
    assert_eq!("黄道", star.get_ecliptic().get_name());
    assert_eq!("吉", star.get_ecliptic().get_luck().get_name());
  }

  #[test]
  fn test2() {
    let star: TwelveStar = SolarDay::from_ymd(2023, 10, 19).unwrap().get_lunar_day().get_twelve_star();
    assert_eq!("白虎", star.get_name());
    assert_eq!("黑道", star.get_ecliptic().get_name());
    assert_eq!("凶", star.get_ecliptic().get_luck().get_name());
  }

  #[test]
  fn test3() {
    let star: TwelveStar = SolarDay::from_ymd(2023, 10, 7).unwrap().get_lunar_day().get_twelve_star();
    assert_eq!("天牢", star.get_name());
    assert_eq!("黑道", star.get_ecliptic().get_name());
    assert_eq!("凶", star.get_ecliptic().get_luck().get_name());
  }

  #[test]
  fn test4() {
    let star: TwelveStar = SolarDay::from_ymd(2023, 10, 8).unwrap().get_lunar_day().get_twelve_star();
    assert_eq!("玉堂", star.get_name());
    assert_eq!("黄道", star.get_ecliptic().get_name());
    assert_eq!("吉", star.get_ecliptic().get_luck().get_name());
  }
}
