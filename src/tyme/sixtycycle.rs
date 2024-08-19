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
  fn next(&self, n: isize) -> Self {
    Self::from_index(self.parent.next_index(n) as isize)
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

  pub fn from_name(name: &str) -> Self {
    Self {
      parent: LoopTyme::from_name(HEAVEN_STEM_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name)
    }
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  /// 五行
  pub fn get_element(&self) -> Element {
    Element::from_index((self.get_index() / 2) as isize)
  }

  /// 阴阳
  pub fn get_yin_yang(&self) -> YinYang {
    match self.get_index() % 2 {
      0 => YinYang::YANG,
      _ => YinYang::YIN
    }
  }

  /// 方位
  pub fn get_direction(&self) -> Direction {
    self.get_element().get_direction()
  }

  /// 喜神方位（《喜神方位歌》甲己在艮乙庚乾，丙辛坤位喜神安。丁壬只在离宫坐，戊癸原在在巽间。）
  pub fn get_joy_direction(&self) -> Direction {
    Direction::from_index([7, 5, 1, 8, 3][self.get_index() % 5])
  }

  /// 阳贵神方位（《阳贵神歌》甲戊坤艮位，乙己是坤坎，庚辛居离艮，丙丁兑与乾，震巽属何日，壬癸贵神安。）
  pub fn get_yang_direction(&self) -> Direction {
    Direction::from_index([1, 1, 6, 5, 7, 0, 8, 7, 2, 3][self.get_index()])
  }

  /// 阴贵神方位（《阴贵神歌》甲戊见牛羊，乙己鼠猴乡，丙丁猪鸡位，壬癸蛇兔藏，庚辛逢虎马，此是贵神方。）
  pub fn get_yin_direction(&self) -> Direction {
    Direction::from_index([7, 0, 5, 6, 1, 1, 7, 8, 3, 2][self.get_index()])
  }

  /// 财神方位（《财神方位歌》甲乙东北是财神，丙丁向在西南寻，戊己正北坐方位，庚辛正东去安身，壬癸原来正南坐，便是财神方位真。）
  pub fn get_wealth_direction(&self) -> Direction {
    Direction::from_index([7, 1, 0, 2, 8][self.get_index() / 2])
  }

  /// 福神方位（《福神方位歌》甲乙东南是福神，丙丁正东是堪宜，戊北己南庚辛坤，壬在乾方癸在西。）
  pub fn get_mascot_direction(&self) -> Direction {
    Direction::from_index([3, 3, 2, 2, 0, 8, 1, 1, 5, 6][self.get_index()])
  }

  /// 地势(长生十二神)
  pub fn get_terrain(&self, earth_branch: EarthBranch) -> Terrain {
    let mut earth_branch_index: isize = earth_branch.get_index() as isize;
    if self.get_yin_yang() == YinYang::YIN {
      earth_branch_index *= -1;
    }
    Terrain::from_index([1, 6, 10, 9, 10, 9, 7, 0, 4, 3][self.get_index()] + earth_branch_index)
  }

  /// 十神（生我者，正印偏印。我生者，伤官食神。克我者，正官七杀。我克者，正财偏财。同我者，劫财比肩。）
  pub fn get_ten_star(&self, target: Self) -> TenStar {
    let target_index: isize = target.get_index() as isize;
    let index: isize = self.get_index() as isize;
    let mut offset: isize = target_index - index;
    if index % 2 != 0 && target_index % 2 == 0 {
      offset += 2;
    }
    return TenStar::from_index(offset);
  }

  /// 五合（甲己合，乙庚合，丙辛合，丁壬合，戊癸合）
  pub fn get_combine(&self) -> Self {
    self.next(5)
  }

  /// 合化（甲己合化土，乙庚合化金，丙辛合化水，丁壬合化木，戊癸合化火）
  ///
  /// 如果无法合化，返回None
  pub fn combine(&self, target: Self) -> Option<Element> {
    if self.get_combine() == target {
      Some(Element::from_index(self.get_index() as isize + 2))
    } else {
      None
    }
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
  fn next(&self, n: isize) -> Self {
    Self::from_index(self.parent.next_index(n) as isize)
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

  pub fn from_name(name: &str) -> Self {
    Self {
      parent: LoopTyme::from_name(EARTH_BRANCH_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name)
    }
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  /// 五行
  pub fn get_element(&self) -> Element {
    Element::from_index([4, 2, 0, 0, 2, 1, 1, 2, 3, 3, 2, 4][self.get_index()])
  }

  /// 阴阳
  pub fn get_yin_yang(&self) -> YinYang {
    match self.get_index() % 2 {
      0 => YinYang::YANG,
      _ => YinYang::YIN
    }
  }

  /// 生肖
  pub fn get_zodiac(&self) -> Zodiac {
    Zodiac::from_index(self.get_index() as isize)
  }

  /// 方位
  pub fn get_direction(&self) -> Direction {
    Direction::from_index([0, 4, 2, 2, 4, 8, 8, 4, 6, 6, 4, 0][self.get_index()])
  }

  /// 六冲（子午冲，丑未冲，寅申冲，辰戌冲，卯酉冲，巳亥冲）
  pub fn get_opposite(&self) -> Self {
    self.next(6)
  }

  /// 煞（逢巳日、酉日、丑日必煞东；亥日、卯日、未日必煞西；申日、子日、辰日必煞南；寅日、午日、戌日必煞北。）
  pub fn get_ominous(&self) -> Direction {
    Direction::from_index([8, 2, 0, 6][self.get_index() % 4])
  }

  /// 六合（子丑合，寅亥合，卯戌合，辰酉合，巳申合，午未合）
  pub fn get_combine(&self) -> Self {
    Self::from_index(1 - self.get_index() as isize)
  }

  /// 合化（子丑合化土，寅亥合化木，卯戌合化火，辰酉合化金，巳申合化水，午未合化土）
  ///
  /// 如果无法合化，返回None
  pub fn combine(&self, target: Self) -> Option<Element> {
    if self.get_combine() == target {
      Some(Element::from_index([2, 2, 0, 1, 3, 4, 2, 2, 4, 3, 1, 0][self.get_index()]))
    } else {
      None
    }
  }

  /// 六害（子未害、丑午害、寅巳害、卯辰害、申亥害、酉戌害）
  pub fn get_harm(&self) -> Self {
    Self::from_index(19 - self.get_index() as isize)
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
  fn next(&self, n: isize) -> Self {
    Self::from_index(self.parent.next_index(n) as isize)
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

  pub fn from_name(name: &str) -> Self {
    Self {
      parent: LoopTyme::from_name(SIXTY_CYCLE_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name)
    }
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
    let next_earth_branch: EarthBranch = earth_branch.next(1);
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
    assert_eq!(HeavenStem::from_name("丙").get_element(), HeavenStem::from_name("甲").get_element().get_reinforce());
  }

  #[test]
  fn test6() {
    assert_eq!("比肩", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("甲")).get_name());
    assert_eq!("劫财", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("乙")).get_name());
    assert_eq!("食神", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("丙")).get_name());
    assert_eq!("伤官", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("丁")).get_name());
    assert_eq!("偏财", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("戊")).get_name());
    assert_eq!("正财", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("己")).get_name());
    assert_eq!("七杀", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("庚")).get_name());
    assert_eq!("正官", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("辛")).get_name());
    assert_eq!("偏印", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("壬")).get_name());
    assert_eq!("正印", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("癸")).get_name());
  }

  #[test]
  fn test7() {
    assert_eq!("丁丑", SixtyCycle::from_index(13).get_name());
  }

  #[test]
  fn test8() {
    assert_eq!(13, SixtyCycle::from_name("丁丑").get_index());
  }

  #[test]
  fn test9() {
    assert_eq!("石榴木", SixtyCycle::from_name("辛酉").get_sound().get_name());
    assert_eq!("剑锋金", SixtyCycle::from_name("癸酉").get_sound().get_name());
    assert_eq!("平地木", SixtyCycle::from_name("己亥").get_sound().get_name());
  }

  #[test]
  fn test10() {
    assert_eq!("甲子", SixtyCycle::from_name("甲子").get_ten().get_name());
    assert_eq!("甲寅", SixtyCycle::from_name("乙卯").get_ten().get_name());
    assert_eq!("甲申", SixtyCycle::from_name("癸巳").get_ten().get_name());
  }

  #[test]
  fn test11() {
    assert_eq!("长生", HeavenStem::from_name("丙").get_terrain(EarthBranch::from_name("寅")).get_name());
    assert_eq!("沐浴", HeavenStem::from_name("辛").get_terrain(EarthBranch::from_name("亥")).get_name());
  }
}