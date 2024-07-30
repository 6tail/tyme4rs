use std::fmt::{Display, Formatter};

use crate::tyme::{Culture, LoopTyme, Tyme};
use crate::tyme::culture::{Direction, Element};

pub static DIPPER_NAMES: [&str; 9] = ["天枢", "天璇", "天玑", "天权", "玉衡", "开阳", "摇光", "洞明", "隐元"];

/// 北斗九星
#[derive(Debug, Clone)]
pub struct Dipper {
  parent: LoopTyme,
}

impl Tyme for Dipper {
  fn next(&self, n: isize) -> Self {
    Self::from_index(self.parent.next_index(n) as isize)
  }
}

impl Culture for Dipper {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Dipper {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(DIPPER_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Self {
    Self {
      parent: LoopTyme::from_name(DIPPER_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name)
    }
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for Dipper {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Dipper {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Dipper {}

impl Into<LoopTyme> for Dipper {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static NINE_STAR_NAMES: [&str; 9] = ["一", "二", "三", "四", "五", "六", "七", "八", "九"];

/// 九星
#[derive(Debug, Clone)]
pub struct NineStar {
  parent: LoopTyme,
}

impl Tyme for NineStar {
  fn next(&self, n: isize) -> Self {
    Self::from_index(self.parent.next_index(n) as isize)
  }
}

impl Culture for NineStar {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl NineStar {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(NINE_STAR_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Self {
    Self {
      parent: LoopTyme::from_name(NINE_STAR_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name)
    }
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  pub fn get_color(&self) -> String {
    ["白", "黒", "碧", "绿", "黄", "白", "赤", "白", "紫"][self.get_index()].to_string()
  }

  pub fn get_element(&self) -> Element {
    Element::from_index([4, 2, 0, 0, 2, 3, 3, 2, 1][self.get_index()])
  }

  pub fn get_dipper(&self) -> Dipper {
    Dipper::from_index(self.get_index() as isize)
  }

  pub fn get_direction(&self) -> Direction {
    Direction::from_index(self.get_index() as isize)
  }
}

impl Display for NineStar {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}{}", self.get_name(), self.get_color(), self.get_element())
  }
}

impl PartialEq for NineStar {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for NineStar {}

impl Into<LoopTyme> for NineStar {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

#[cfg(test)]
mod tests {
  use crate::tyme::Culture;
  use crate::tyme::culture::star::nine::NineStar;
  use crate::tyme::lunar::{LunarDay, LunarHour, LunarMonth, LunarYear};
  use crate::tyme::solar::SolarDay;

  #[test]
  fn test0() {
    let nine_star: NineStar = LunarYear::from_year(1985).get_nine_star();
    assert_eq!("六", nine_star.get_name());
    assert_eq!("六白金", nine_star.to_string());
  }

  #[test]
  fn test1() {
    let nine_star: NineStar = LunarYear::from_year(2022).get_nine_star();
    assert_eq!("五黄土", nine_star.to_string());
    assert_eq!("玉衡", nine_star.get_dipper().to_string());
  }

  #[test]
  fn test2() {
    let nine_star: NineStar = LunarYear::from_year(2033).get_nine_star();
    assert_eq!("三碧木", nine_star.to_string());
    assert_eq!("天玑", nine_star.get_dipper().to_string());
  }

  #[test]
  fn test3() {
    let nine_star: NineStar = LunarMonth::from_ym(1985, 2).get_nine_star();
    assert_eq!("四绿木", nine_star.to_string());
    assert_eq!("天权", nine_star.get_dipper().to_string());
  }

  #[test]
  fn test4() {
    let nine_star: NineStar = LunarMonth::from_ym(1985, 2).get_nine_star();
    assert_eq!("四绿木", nine_star.to_string());
    assert_eq!("天权", nine_star.get_dipper().to_string());
  }

  #[test]
  fn test5() {
    let nine_star: NineStar = LunarMonth::from_ym(2022, 1).get_nine_star();
    assert_eq!("二黒土", nine_star.to_string());
    assert_eq!("天璇", nine_star.get_dipper().to_string());
  }

  #[test]
  fn test6() {
    let nine_star: NineStar = LunarMonth::from_ym(2033, 1).get_nine_star();
    assert_eq!("五黄土", nine_star.to_string());
    assert_eq!("玉衡", nine_star.get_dipper().to_string());
  }

  #[test]
  fn test7() {
    let nine_star: NineStar = SolarDay::from_ymd(1985, 2, 19).get_lunar_day().get_nine_star();
    assert_eq!("五黄土", nine_star.to_string());
    assert_eq!("玉衡", nine_star.get_dipper().to_string());
  }

  #[test]
  fn test8() {
    let nine_star: NineStar = LunarDay::from_ymd(2022, 1, 1).get_nine_star();
    assert_eq!("四绿木", nine_star.to_string());
    assert_eq!("天权", nine_star.get_dipper().to_string());
  }

  #[test]
  fn test9() {
    let nine_star: NineStar = LunarDay::from_ymd(2033, 1, 1).get_nine_star();
    assert_eq!("一白水", nine_star.to_string());
    assert_eq!("天枢", nine_star.get_dipper().to_string());
  }

  #[test]
  fn test10() {
    let nine_star: NineStar = LunarHour::from_ymd_hms(2033, 1, 1, 12, 0, 0).get_nine_star();
    assert_eq!("七赤金", nine_star.to_string());
    assert_eq!("摇光", nine_star.get_dipper().to_string());
  }

  #[test]
  fn test11() {
    let nine_star: NineStar = LunarHour::from_ymd_hms(2011, 5, 3, 23, 0, 0).get_nine_star();
    assert_eq!("七赤金", nine_star.to_string());
    assert_eq!("摇光", nine_star.get_dipper().to_string());
  }
}