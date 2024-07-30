use std::fmt::{Display, Formatter};
use std::string::ToString;

use crate::tyme::{Culture, LoopTyme, Tyme};
use crate::tyme::sixtycycle::SixtyCycle;

/// 彭祖百忌
#[derive(Debug, Clone)]
pub struct PengZu {
  peng_zu_heaven_stem: PengZuHeavenStem,
  peng_zu_earth_branch: PengZuEarthBranch,
}

impl PengZu {
  pub fn from_sixty_cycle(sixty_cycle: SixtyCycle) -> Self {
    let peng_zu_heaven_stem: PengZuHeavenStem = PengZuHeavenStem::from_index(sixty_cycle.get_heaven_stem().get_index() as isize);
    let peng_zu_earth_branch: PengZuEarthBranch = PengZuEarthBranch::from_index(sixty_cycle.get_earth_branch().get_index() as isize);

    Self {
      peng_zu_heaven_stem,
      peng_zu_earth_branch,
    }
  }

  pub fn get_peng_zu_heaven_stem(&self) -> PengZuHeavenStem {
    self.peng_zu_heaven_stem.clone()
  }

  pub fn get_peng_zu_earth_branch(&self) -> PengZuEarthBranch {
    self.peng_zu_earth_branch.clone()
  }

  fn get_name(&self) -> String {
    format!("{} {}", self.peng_zu_heaven_stem, self.peng_zu_earth_branch)
  }
}

impl Display for PengZu {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for PengZu {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for PengZu {}

pub static PENG_ZU_HEAVEN_STEM_NAMES: [&str; 10] = ["甲不开仓财物耗散", "乙不栽植千株不长", "丙不修灶必见灾殃", "丁不剃头头必生疮", "戊不受田田主不祥", "己不破券二比并亡", "庚不经络织机虚张", "辛不合酱主人不尝", "壬不泱水更难提防", "癸不词讼理弱敌强"];

/// 天干彭祖百忌
#[derive(Debug, Clone)]
pub struct PengZuHeavenStem {
  parent: LoopTyme,
}

impl Tyme for PengZuHeavenStem {
  fn next(&self, n: isize) -> Self {
    Self::from_index(self.parent.next_index(n) as isize)
  }
}

impl Culture for PengZuHeavenStem {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl PengZuHeavenStem {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(PENG_ZU_HEAVEN_STEM_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Self {
    Self {
      parent: LoopTyme::from_name(PENG_ZU_HEAVEN_STEM_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name)
    }
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for PengZuHeavenStem {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for PengZuHeavenStem {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for PengZuHeavenStem {}

impl Into<LoopTyme> for PengZuHeavenStem {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static PENG_ZU_EARTH_BRANCH_NAMES: [&str; 12] = ["子不问卜自惹祸殃", "丑不冠带主不还乡", "寅不祭祀神鬼不尝", "卯不穿井水泉不香", "辰不哭泣必主重丧", "巳不远行财物伏藏", "午不苫盖屋主更张", "未不服药毒气入肠", "申不安床鬼祟入房", "酉不会客醉坐颠狂", "戌不吃犬作怪上床", "亥不嫁娶不利新郎"];

/// 地支彭祖百忌
#[derive(Debug, Clone)]
pub struct PengZuEarthBranch {
  parent: LoopTyme,
}

impl Tyme for PengZuEarthBranch {
  fn next(&self, n: isize) -> Self {
    Self::from_index(self.parent.next_index(n) as isize)
  }
}

impl Culture for PengZuEarthBranch {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl PengZuEarthBranch {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(PENG_ZU_EARTH_BRANCH_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Self {
    Self {
      parent: LoopTyme::from_name(PENG_ZU_EARTH_BRANCH_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name)
    }
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for PengZuEarthBranch {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for PengZuEarthBranch {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for PengZuEarthBranch {}

impl Into<LoopTyme> for PengZuEarthBranch {
  fn into(self) -> LoopTyme {
    self.parent
  }
}
