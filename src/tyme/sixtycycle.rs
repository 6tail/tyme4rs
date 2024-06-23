use std::fmt::{Display, Formatter};

use crate::tyme::{Culture, LoopTyme, Tyme};
use crate::tyme::culture::{Direction, Element, Sound, Ten, Terrain, Zodiac};
use crate::tyme::culture::star::ten::TenStar;
use crate::tyme::enums::YinYang;

pub static HEAVEN_STEM_NAMES: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];

/// 天干
#[derive(Debug, Clone)]
pub struct HeavenStem {
  parent: LoopTyme,
}

impl Tyme for HeavenStem {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for HeavenStem {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl HeavenStem {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(HEAVEN_STEM_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(HEAVEN_STEM_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  pub fn get_element(&self) -> Element {
    Element::from_index((self.get_index() / 2) as isize)
  }

  pub fn get_yin_yang(&self) -> YinYang {
    match self.get_index() % 2 {
      0 => YinYang::YANG,
      _ => YinYang::YIN
    }
  }

  pub fn get_direction(&self) -> Direction {
    Direction::from_index([2, 8, 4, 6, 0][self.get_index() / 2])
  }

  pub fn get_joy_direction(&self) -> Direction {
    Direction::from_index([7, 5, 1, 8, 3][self.get_index() % 5])
  }

  pub fn get_yang_direction(&self) -> Direction {
    Direction::from_index([1, 1, 6, 5, 7, 0, 8, 7, 2, 3][self.get_index()])
  }

  pub fn get_yin_direction(&self) -> Direction {
    Direction::from_index([7, 0, 5, 6, 1, 1, 7, 8, 3, 2][self.get_index()])
  }

  pub fn get_wealth_direction(&self) -> Direction {
    Direction::from_index([7, 1, 0, 2, 8][self.get_index() / 2])
  }

  pub fn get_mascot_direction(&self) -> Direction {
    Direction::from_index([3, 3, 2, 2, 0, 8, 1, 1, 5, 6][self.get_index()])
  }

  pub fn get_terrain(&self, earth_branch: EarthBranch) -> Terrain {
    let mut earth_branch_index: isize = earth_branch.get_index() as isize;
    if self.get_yin_yang() == YinYang::YIN {
      earth_branch_index *= -1;
    }
    Terrain::from_index([1, 6, 10, 9, 10, 9, 7, 0, 4, 3][self.get_index()] + earth_branch_index)
  }

  pub fn get_ten_star(&self, target: HeavenStem) -> TenStar {
    let host: Element = self.get_element();
    let guest: Element = target.get_element();
    let mut index: isize = 0;
    let same_yin_yang: bool = self.get_yin_yang() == target.get_yin_yang();
    if host.get_reinforce() == guest {
      index = 1;
    } else if host.get_restrain() == guest {
      index = 2;
    } else if host.get_restrained() == guest {
      index = 3;
    } else if host.get_reinforced() == guest {
      index = 4;
    }
    TenStar::from_index(index * 2 + if same_yin_yang { 0 } else { 1 })
  }
}

impl Display for HeavenStem {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for HeavenStem {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for HeavenStem {}

impl Into<LoopTyme> for HeavenStem {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static EARTH_BRANCH_NAMES: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

/// 地支
#[derive(Debug, Clone)]
pub struct EarthBranch {
  parent: LoopTyme,
}

impl Tyme for EarthBranch {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for EarthBranch {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl EarthBranch {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(EARTH_BRANCH_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(EARTH_BRANCH_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  pub fn get_element(&self) -> Element {
    Element::from_index([4, 2, 0, 0, 2, 1, 1, 2, 3, 3, 2, 4][self.get_index()])
  }

  pub fn get_yin_yang(&self) -> YinYang {
    match self.get_index() % 2 {
      0 => YinYang::YANG,
      _ => YinYang::YIN
    }
  }

  pub fn get_zodiac(&self) -> Zodiac {
    Zodiac::from_index(self.get_index() as isize)
  }

  pub fn get_direction(&self) -> Direction {
    Direction::from_index([0, 4, 2, 2, 4, 8, 8, 4, 6, 6, 4, 0][self.get_index()])
  }

  pub fn get_opposite(&self) -> Self {
    self.next(6).unwrap()
  }

  pub fn get_ominous(&self) -> Direction {
    Direction::from_index([8, 2, 0, 6][self.get_index() % 4])
  }
}

impl Display for EarthBranch {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for EarthBranch {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for EarthBranch {}

impl Into<LoopTyme> for EarthBranch {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static SIXTY_CYCLE_NAMES: [&str; 60] = ["甲子", "乙丑", "丙寅", "丁卯", "戊辰", "己巳", "庚午", "辛未", "壬申", "癸酉", "甲戌", "乙亥", "丙子", "丁丑", "戊寅", "己卯", "庚辰", "辛巳", "壬午", "癸未", "甲申", "乙酉", "丙戌", "丁亥", "戊子", "己丑", "庚寅", "辛卯", "壬辰", "癸巳", "甲午", "乙未", "丙申", "丁酉", "戊戌", "己亥", "庚子", "辛丑", "壬寅", "癸卯", "甲辰", "乙巳", "丙午", "丁未", "戊申", "己酉", "庚戌", "辛亥", "壬子", "癸丑", "甲寅", "乙卯", "丙辰", "丁巳", "戊午", "己未", "庚申", "辛酉", "壬戌", "癸亥"];

/// 六十甲子(六十干支周)
#[derive(Debug, Clone)]
pub struct SixtyCycle {
  parent: LoopTyme,
}

impl Tyme for SixtyCycle {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for SixtyCycle {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl SixtyCycle {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(SIXTY_CYCLE_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(SIXTY_CYCLE_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  pub fn get_heaven_stem(&self) -> HeavenStem {
    HeavenStem::from_index((self.get_index() % HEAVEN_STEM_NAMES.len()) as isize)
  }

  pub fn get_earth_branch(&self) -> EarthBranch {
    EarthBranch::from_index((self.get_index() % EARTH_BRANCH_NAMES.len()) as isize)
  }

  pub fn get_sound(&self) -> Sound {
    Sound::from_index((self.get_index() / 2) as isize)
  }

  pub fn get_ten(&self) -> Ten {
    Ten::from_index((self.get_heaven_stem().get_index() as isize - self.get_earth_branch().get_index() as isize) / 2)
  }

  pub fn get_extra_earth_branches(&self) -> Vec<EarthBranch> {
    let mut l: Vec<EarthBranch> = Vec::new();
    let earth_branch: EarthBranch = EarthBranch::from_index(10 + (self.get_earth_branch().get_index() as isize) - (self.get_heaven_stem().get_index() as isize));
    let next_earth_branch: EarthBranch = earth_branch.next(1).unwrap();
    l.push(earth_branch);
    l.push(next_earth_branch);
    l
  }
}

impl Display for SixtyCycle {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for SixtyCycle {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for SixtyCycle {}

impl Into<LoopTyme> for SixtyCycle {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

#[cfg(test)]
mod tests {
  use crate::tyme::Culture;
  use crate::tyme::sixtycycle::{EarthBranch, HeavenStem, SixtyCycle};

  #[test]
  fn test1() {
    assert_eq!("子", EarthBranch::from_index(0).get_name());
  }

  #[test]
  fn test2() {
    assert_eq!(0, EarthBranch::from_index(0).get_index());
  }

  #[test]
  fn test3() {
    assert_eq!("甲", HeavenStem::from_index(0).get_name());
  }

  #[test]
  fn test4() {
    assert_eq!(0, HeavenStem::from_index(0).get_index());
  }

  #[test]
  fn test5() {
    assert_eq!(HeavenStem::from_name("丙").unwrap().get_element(), HeavenStem::from_name("甲").unwrap().get_element().get_reinforce());
  }

  #[test]
  fn test6() {
    assert_eq!("比肩", HeavenStem::from_name("甲").unwrap().get_ten_star(HeavenStem::from_name("甲").unwrap()).get_name());
    assert_eq!("劫财", HeavenStem::from_name("甲").unwrap().get_ten_star(HeavenStem::from_name("乙").unwrap()).get_name());
    assert_eq!("食神", HeavenStem::from_name("甲").unwrap().get_ten_star(HeavenStem::from_name("丙").unwrap()).get_name());
    assert_eq!("伤官", HeavenStem::from_name("甲").unwrap().get_ten_star(HeavenStem::from_name("丁").unwrap()).get_name());
    assert_eq!("偏财", HeavenStem::from_name("甲").unwrap().get_ten_star(HeavenStem::from_name("戊").unwrap()).get_name());
    assert_eq!("正财", HeavenStem::from_name("甲").unwrap().get_ten_star(HeavenStem::from_name("己").unwrap()).get_name());
    assert_eq!("七杀", HeavenStem::from_name("甲").unwrap().get_ten_star(HeavenStem::from_name("庚").unwrap()).get_name());
    assert_eq!("正官", HeavenStem::from_name("甲").unwrap().get_ten_star(HeavenStem::from_name("辛").unwrap()).get_name());
    assert_eq!("偏印", HeavenStem::from_name("甲").unwrap().get_ten_star(HeavenStem::from_name("壬").unwrap()).get_name());
    assert_eq!("正印", HeavenStem::from_name("甲").unwrap().get_ten_star(HeavenStem::from_name("癸").unwrap()).get_name());
  }

  #[test]
  fn test7() {
    assert_eq!("丁丑", SixtyCycle::from_index(13).get_name());
  }

  #[test]
  fn test8() {
    assert_eq!(13, SixtyCycle::from_name("丁丑").unwrap().get_index());
  }

  #[test]
  fn test9() {
    assert_eq!("石榴木", SixtyCycle::from_name("辛酉").unwrap().get_sound().get_name());
    assert_eq!("剑锋金", SixtyCycle::from_name("癸酉").unwrap().get_sound().get_name());
    assert_eq!("平地木", SixtyCycle::from_name("己亥").unwrap().get_sound().get_name());
  }

  #[test]
  fn test10() {
    assert_eq!("甲子", SixtyCycle::from_name("甲子").unwrap().get_ten().get_name());
    assert_eq!("甲寅", SixtyCycle::from_name("乙卯").unwrap().get_ten().get_name());
    assert_eq!("甲申", SixtyCycle::from_name("癸巳").unwrap().get_ten().get_name());
  }

  #[test]
  fn test11() {
    assert_eq!("长生", HeavenStem::from_name("丙").unwrap().get_terrain(EarthBranch::from_name("寅").unwrap()).get_name());
    assert_eq!("沐浴", HeavenStem::from_name("辛").unwrap().get_terrain(EarthBranch::from_name("亥").unwrap()).get_name());
  }
}