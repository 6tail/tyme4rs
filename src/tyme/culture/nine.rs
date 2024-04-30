use std::fmt::{Display, Formatter};
use std::string::ToString;
use crate::tyme::{AbstractCulture, AbstractCultureDay, AbstractTyme, Culture, LoopTyme, Tyme};

pub static NINE_NAMES: [&str; 9] = ["一九", "二九", "三九", "四九", "五九", "六九", "七九", "八九", "九九"];

/// 数九
#[derive(Debug, Clone)]
pub struct Nine {
  parent: LoopTyme,
}

impl Tyme for Nine {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Nine {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Nine {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(NINE_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(NINE_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for Nine {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Nine {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Nine {}

impl Into<LoopTyme> for Nine {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

/// 三伏天
#[derive(Debug, Clone)]
pub struct NineDay {
  parent: AbstractCultureDay,
  nine: Nine
}

impl Culture for NineDay {
  fn get_name(&self) -> String {
    self.nine.get_name()
  }
}

impl NineDay {
  pub fn new(nine: Nine, day_index: usize) -> Self {
    let loop_tyme: LoopTyme = nine.clone().into();
    let abstract_tyme: AbstractTyme = loop_tyme.into();
    let culture: AbstractCulture = abstract_tyme.into();
    Self {
      parent: AbstractCultureDay::new(culture, day_index),
      nine
    }
  }

  pub fn get_nine(&self) -> Nine {
    self.nine.clone()
  }

  pub fn get_day_index(&self) -> usize {
    self.parent.get_day_index()
  }
}

impl Display for NineDay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}第{}天", self.get_name(), self.parent.get_day_index() + 1)
  }
}

impl PartialEq for NineDay {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for NineDay {}

impl Into<AbstractCultureDay> for NineDay {
  fn into(self) -> AbstractCultureDay {
    self.parent
  }
}

#[cfg(test)]
mod tests {
  use crate::tyme::Culture;
  use crate::tyme::culture::nine::NineDay;
  use crate::tyme::solar::SolarDay;

  #[test]
  fn test0() {
    let d: NineDay = SolarDay::from_ymd(2020, 12, 21).unwrap().get_nine_day().unwrap();
    assert_eq!("一九", d.get_name());
    assert_eq!("一九", d.get_nine().to_string());
    assert_eq!("一九第1天", d.to_string());
  }

  #[test]
  fn test1() {
    let d: NineDay = SolarDay::from_ymd(2020, 12, 22).unwrap().get_nine_day().unwrap();
    assert_eq!("一九", d.get_name());
    assert_eq!("一九", d.get_nine().to_string());
    assert_eq!("一九第2天", d.to_string());
  }

  #[test]
  fn test2() {
    let d: NineDay = SolarDay::from_ymd(2020, 1, 7).unwrap().get_nine_day().unwrap();
    assert_eq!("二九", d.get_name());
    assert_eq!("二九", d.get_nine().to_string());
    assert_eq!("二九第8天", d.to_string());
  }

  #[test]
  fn test3() {
    let d: NineDay = SolarDay::from_ymd(2021, 1, 6).unwrap().get_nine_day().unwrap();
    assert_eq!("二九", d.get_name());
    assert_eq!("二九", d.get_nine().to_string());
    assert_eq!("二九第8天", d.to_string());
  }

  #[test]
  fn test4() {
    let d: NineDay = SolarDay::from_ymd(2021, 1, 8).unwrap().get_nine_day().unwrap();
    assert_eq!("三九", d.get_name());
    assert_eq!("三九", d.get_nine().to_string());
    assert_eq!("三九第1天", d.to_string());
  }

  #[test]
  fn test5() {
    let d: NineDay = SolarDay::from_ymd(2021, 3, 5).unwrap().get_nine_day().unwrap();
    assert_eq!("九九", d.get_name());
    assert_eq!("九九", d.get_nine().to_string());
    assert_eq!("九九第3天", d.to_string());
  }

  #[test]
  fn test6() {
    let d: Option<NineDay> = SolarDay::from_ymd(2021, 7, 5).unwrap().get_nine_day();
    assert_eq!(true, d.is_none());
  }
}
