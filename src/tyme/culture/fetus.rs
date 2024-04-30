use std::fmt::{Display, Formatter};
use std::string::ToString;
use crate::tyme::culture::Direction;
use crate::tyme::enums::Side;
use crate::tyme::{Culture, LoopTyme, Tyme};
use crate::tyme::lunar::{LunarDay, LunarMonth};
use crate::tyme::sixtycycle::SixtyCycle;

/// 逐日胎神
#[derive(Debug, Clone)]
pub struct FetusDay {
  fetus_heaven_stem: FetusHeavenStem,
  fetus_earth_branch: FetusEarthBranch,
  side: Side,
  direction: Direction,
}

impl FetusDay {
  pub fn from_lunar_day(lunar_day: LunarDay) -> Self {
    let sixty_cycle: SixtyCycle = lunar_day.get_sixty_cycle();
    let fetus_heaven_stem: FetusHeavenStem = FetusHeavenStem::from_index((sixty_cycle.get_heaven_stem().get_index() as isize) % 5);
    let fetus_earth_branch: FetusEarthBranch = FetusEarthBranch::from_index((sixty_cycle.get_earth_branch().get_index() as isize) % 6);
    let index: isize = [3, 3, 8, 8, 8, 8, 8, 1, 1, 1, 1, 1, 1, 6, 6, 6, 6, 6, 5, 5, 5, 5, 5, 5, 0, 0, 0, 0, 0, -9, -9, -9, -9, -9, -5, -5, -1, -1, -1, -3, -7, -7, -7, -7, -5, 7, 7, 7, 7, 7, 7, 2, 2, 2, 2, 2, 3, 3, 3, 3][sixty_cycle.get_index()];
    let side: Side = Side::from_code(if index < 0 { 0 } else { 1 }).unwrap();
    let direction: Direction = Direction::from_index(index);

    Self {
      fetus_heaven_stem,
      fetus_earth_branch,
      side,
      direction,
    }
  }

  pub fn get_fetus_heaven_stem(&self) -> FetusHeavenStem {
    self.fetus_heaven_stem.clone()
  }

  pub fn get_fetus_earth_branch(&self) -> FetusEarthBranch {
    self.fetus_earth_branch.clone()
  }

  pub fn get_side(&self) -> Side {
    self.side.clone()
  }

  pub fn get_direction(&self) -> Direction {
    self.direction.clone()
  }

  fn get_name(&self) -> String {
    let mut s: String = format!("{}{}", self.fetus_heaven_stem.get_name(), self.fetus_earth_branch.get_name());
    if "门门" == s {
      s = "占大门".to_string();
    } else if "碓磨碓" == s {
      s = "占碓磨".to_string();
    } else if "房床床" == s {
      s = "占房床".to_string();
    } else if s.starts_with("门") {
      s = format!("{}{}", "占", s);
    }

    s = format!("{}{}", s, " ");

    if Side::IN == self.side {
      s = format!("{}{}", s, "房");
    }
    s = format!("{}{}", s, self.side.get_name());

    if Side::OUT == self.side && "北南西东".contains(self.direction.get_name().as_str()) {
      s = format!("{}{}", s, "正");
    }
    s = format!("{}{}", s, self.direction.get_name());
    return s;
  }
}

impl Display for FetusDay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for FetusDay {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for FetusDay {}

pub static FETUS_HEAVEN_STEM_NAMES: [&str; 5] = ["门", "碓磨", "厨灶", "仓库", "房床"];

/// 天干六甲胎神（《天干六甲胎神歌》甲己之日占在门，乙庚碓磨休移动。丙辛厨灶莫相干，丁壬仓库忌修弄。戊癸房床若移整，犯之孕妇堕孩童。）
#[derive(Debug, Clone)]
pub struct FetusHeavenStem {
  parent: LoopTyme,
}

impl Tyme for FetusHeavenStem {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for FetusHeavenStem {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl FetusHeavenStem {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(FETUS_HEAVEN_STEM_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for FetusHeavenStem {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for FetusHeavenStem {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for FetusHeavenStem {}

impl Into<LoopTyme> for FetusHeavenStem {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static FETUS_EARTH_BRANCH_NAMES: [&str; 6] = ["碓", "厕", "炉", "门", "栖", "床"];

/// 地支六甲胎神（《地支六甲胎神歌》子午二日碓须忌，丑未厕道莫修移。寅申火炉休要动，卯酉大门修当避。辰戌鸡栖巳亥床，犯着六甲身堕胎。）
#[derive(Debug, Clone)]
pub struct FetusEarthBranch {
  parent: LoopTyme,
}

impl Tyme for FetusEarthBranch {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for FetusEarthBranch {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl FetusEarthBranch {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(FETUS_EARTH_BRANCH_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for FetusEarthBranch {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for FetusEarthBranch {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for FetusEarthBranch {}

impl Into<LoopTyme> for FetusEarthBranch {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static FETUS_MONTH_NAMES: [&str; 12] = ["占房床", "占户窗", "占门堂", "占厨灶", "占房床", "占床仓", "占碓磨", "占厕户", "占门房", "占房床", "占灶炉", "占房床"];

/// 逐月胎神（正十二月在床房，二三九十门户中，四六十一灶勿犯，五甲七子八厕凶。）
#[derive(Debug, Clone)]
pub struct FetusMonth {
  parent: LoopTyme,
}

impl Tyme for FetusMonth {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for FetusMonth {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl FetusMonth {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(FETUS_MONTH_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_lunar_month(lunar_month: LunarMonth) -> Option<Self> {
    match lunar_month.is_leap() {
      true => None,
      _ => Some(Self::from_index((lunar_month.get_month() as isize) - 1))
    }
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for FetusMonth {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for FetusMonth {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for FetusMonth {}

impl Into<LoopTyme> for FetusMonth {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

#[cfg(test)]
mod tests {
  use crate::tyme::solar::SolarDay;

  #[test]
  fn test1() {
    assert_eq!("碓磨厕 外东南", SolarDay::from_ymd(2021, 11, 13).unwrap().get_lunar_day().get_fetus_day().get_name());
  }

  #[test]
  fn test2() {
    assert_eq!("占门碓 外东南", SolarDay::from_ymd(2021, 11, 12).unwrap().get_lunar_day().get_fetus_day().get_name());
  }

  #[test]
  fn test3() {
    assert_eq!("厨灶厕 外西南", SolarDay::from_ymd(2011, 11, 12).unwrap().get_lunar_day().get_fetus_day().get_name());
  }
}