use std::fmt::{Display, Formatter};

use crate::tyme::{AbstractCulture, AbstractCultureDay, AbstractTyme, Culture, LoopTyme, Tyme};

pub static PHENOLOGY_NAMES: [&str; 72] = ["蚯蚓结", "麋角解", "水泉动", "雁北乡", "鹊始巢", "雉始雊", "鸡始乳", "征鸟厉疾", "水泽腹坚", "东风解冻", "蛰虫始振", "鱼陟负冰", "獭祭鱼", "候雁北", "草木萌动", "桃始华", "仓庚鸣", "鹰化为鸠", "玄鸟至", "雷乃发声", "始电", "桐始华", "田鼠化为鴽", "虹始见", "萍始生", "鸣鸠拂其羽", "戴胜降于桑", "蝼蝈鸣", "蚯蚓出", "王瓜生", "苦菜秀", "靡草死", "麦秋至", "螳螂生", "鵙始鸣", "反舌无声", "鹿角解", "蜩始鸣", "半夏生", "温风至", "蟋蟀居壁", "鹰始挚", "腐草为萤", "土润溽暑", "大雨行时", "凉风至", "白露降", "寒蝉鸣", "鹰乃祭鸟", "天地始肃", "禾乃登", "鸿雁来", "玄鸟归", "群鸟养羞", "雷始收声", "蛰虫坯户", "水始涸", "鸿雁来宾", "雀入大水为蛤", "菊有黄花", "豺乃祭兽", "草木黄落", "蛰虫咸俯", "水始冰", "地始冻", "雉入大水为蜃", "虹藏不见", "天气上升地气下降", "闭塞而成冬", "鹖鴠不鸣", "虎始交", "荔挺出"];

/// 候
#[derive(Debug, Clone)]
pub struct Phenology {
  parent: LoopTyme,
}

impl Tyme for Phenology {
  fn next(&self, n: isize) -> Self {
    Self::from_index(self.parent.next_index(n) as isize)
  }
}

impl Culture for Phenology {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Phenology {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(PHENOLOGY_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Self {
    Self {
      parent: LoopTyme::from_name(PHENOLOGY_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name)
    }
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  pub fn get_three_phenology(&self) -> ThreePhenology {
    ThreePhenology::from_index((self.get_index() as isize) % 3)
  }
}

impl Display for Phenology {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Phenology {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Phenology {}

impl Into<LoopTyme> for Phenology {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static THREE_PHENOLOGY_NAMES: [&str; 3] = ["初候", "二候", "三候"];

/// 三候
#[derive(Debug, Clone)]
pub struct ThreePhenology {
  parent: LoopTyme,
}

impl Tyme for ThreePhenology {
  fn next(&self, n: isize) -> Self {
    Self::from_index(self.parent.next_index(n) as isize)
  }
}

impl Culture for ThreePhenology {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl ThreePhenology {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(THREE_PHENOLOGY_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Self {
    Self {
      parent: LoopTyme::from_name(THREE_PHENOLOGY_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name)
    }
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for ThreePhenology {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for ThreePhenology {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for ThreePhenology {}

impl Into<LoopTyme> for ThreePhenology {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

/// 七十二候
#[derive(Debug, Clone)]
pub struct PhenologyDay {
  parent: AbstractCultureDay,
  phenology: Phenology,
}

impl Culture for PhenologyDay {
  fn get_name(&self) -> String {
    self.phenology.get_name()
  }
}

impl PhenologyDay {
  pub fn new(phenology: Phenology, day_index: usize) -> Self {
    let loop_tyme: LoopTyme = phenology.clone().into();
    let abstract_tyme: AbstractTyme = loop_tyme.into();
    let culture: AbstractCulture = abstract_tyme.into();
    Self {
      parent: AbstractCultureDay::new(culture, day_index),
      phenology,
    }
  }

  pub fn get_phenology(&self) -> Phenology {
    self.phenology.clone()
  }

  pub fn get_day_index(&self) -> usize {
    self.parent.get_day_index()
  }
}

impl Display for PhenologyDay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}第{}天", self.get_name(), self.parent.get_day_index() + 1)
  }
}

impl PartialEq for PhenologyDay {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for PhenologyDay {}

impl Into<AbstractCultureDay> for PhenologyDay {
  fn into(self) -> AbstractCultureDay {
    self.parent
  }
}

#[cfg(test)]
mod tests {
  use crate::tyme::Culture;
  use crate::tyme::culture::phenology::{PhenologyDay, ThreePhenology};
  use crate::tyme::solar::SolarDay;

  #[test]
  fn test0() {
    let solar_day: SolarDay = SolarDay::from_ymd(2020, 4, 23);
    // 七十二候
    let phenology: PhenologyDay = solar_day.get_phenology_day();
    // 三候
    let three_phenology: ThreePhenology = phenology.get_phenology().get_three_phenology();
    assert_eq!("谷雨", solar_day.get_term().get_name());
    assert_eq!("初候", three_phenology.get_name());
    assert_eq!("萍始生", phenology.get_name());
    // 该候的第5天
    assert_eq!(4, phenology.get_day_index());
  }

  #[test]
  fn test1() {
    let solar_day: SolarDay = SolarDay::from_ymd(2021, 12, 26);
    // 七十二候
    let phenology: PhenologyDay = solar_day.get_phenology_day();
    // 三候
    let three_phenology: ThreePhenology = phenology.get_phenology().get_three_phenology();
    assert_eq!("冬至", solar_day.get_term().get_name());
    assert_eq!("二候", three_phenology.get_name());
    assert_eq!("麋角解", phenology.get_name());
    // 该候的第1天
    assert_eq!(0, phenology.get_day_index());
  }
}
