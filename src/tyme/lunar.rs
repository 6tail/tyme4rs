use lazy_static::lazy_static;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

use crate::tyme::culture::fetus::{FetusDay, FetusMonth};
use crate::tyme::culture::ren::minor::MinorRen;
use crate::tyme::culture::star::nine::NineStar;
use crate::tyme::culture::star::six::SixStar;
use crate::tyme::culture::star::twelve::TwelveStar;
use crate::tyme::culture::star::twenty_eight::TwentyEightStar;
use crate::tyme::culture::{Direction, Duty, Element, God, KitchenGodSteed, Phase, PhaseDay, Taboo, Twenty, Week};
use crate::tyme::eightchar::provider::{DefaultEightCharProvider, EightCharProvider};
use crate::tyme::eightchar::EightChar;
use crate::tyme::festival::LunarFestival;
use crate::tyme::jd::{JulianDay, J2000};
use crate::tyme::sixtycycle::{EarthBranch, HeavenStem, SixtyCycle, SixtyCycleDay, SixtyCycleHour, ThreePillars};
use crate::tyme::solar::{SolarDay, SolarTerm, SolarTime};
use crate::tyme::unit::{DayUnit, MonthUnit, SecondUnit, WeekUnit, YearUnit};
use crate::tyme::util::ShouXingUtil;
use crate::tyme::{AbstractCulture, Culture, LoopTyme, Tyme};

lazy_static! {
  static ref LEAP_MONTH_YEAR: Vec<Vec<isize>> = {
    let mut list: Vec<Vec<isize>> = vec![];
    let chars: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_@";
    let months :[&str; 12] = [
      "080b0r0j0j0j0C0j0j0C0j0j0j0C0j0C0j0C0F0j0V0V0V0u0j0j0C0j0j0j0j0V0C0j1v0u0C0V1v0C0b080u110u0C0j0C1v9K1v2z0j1vmZbl1veN3s1v0V0C2S1v0V0C2S2o0C0j1Z1c2S1v0j1c0j2z1v0j1c0j392H0b2_2S0C0V0j1c0j2z0C0C0j0j1c0j0N250j0C0j0b081n080b0C0C0C1c0j0N",
      "0r1v1c1v0V0V0F0V0j0C0j0C0j0V0j0u1O0j0C0V0j0j0j0V0b080u0r0u080b0j0j0C0V0C0V0j0b080V0u080b0j0j0u0j1v0u080b1c0j080b0j0V0j0j0V0C0N1v0j1c0j0j1v2g1v420j1c0j2z1v0j1v5Q9z1v4l0j1vfn1v420j9z4l1v1v2S1c0j1v2S3s1v0V0C2S1v1v2S1c0j1v2S2_0b0j2_2z0j1c0j",
      "0z0j0j0j0C0j0j0C0j0j0j0C0j0C0j0j0j0j0m0j0C0j0j0C0j0j0j0j0b0V0j0j0C0j0j0j0j0V0j0j0j0V0b0V0V0C0V0C0j0j0b080u110u0V0C0j0N0j0b080b080b0j0r0b0r0b0j0j0j0j0C0j0b0r0C0j0b0j0C0C0j0j0j0j0j0j0j0j0j0b110j0b0j0j0j0C0j0C0j0j0j0j0b080b080b0V080b080b0j0j0j0j0j0j0V0j0j0u1v0j0j0j0C0j0j0j0V0C0N1c0j0C0C0j0j0j1n080b0j0V0C0j0C0C2g0j1c0j0j1v2g1v0j0j1v7N0j1c0j3L0j0j1v5Q1Z5Q1v4lfn1v420j1v5Q1Z5Q1v4l1v2z1v",
      "0H140r0N0r140r0u0r0V171c11140C0j0u110j0u0j1v0j0C0j0j0j0b080V0u080b0C1v0j0j0j0C0j0b080V0j0j0b080b0j0j0j0j0b080b0C080j0b080b0j0j0j0j0j0j0b080j0b080C0b080b080b080b0j0j0j0j080b0j0C0j0j0j0b0j0j080C0b0j0j0j0j0j0j0b08080b0j0C0j0j0j0b0j0j0K0b0j0C0j0j0j0b080b080j0C0b0j080b080b0j0j0j0j080b0j0b0r0j0j0j0b0j0C0r0b0j0j0j0j0j0j0j0b080j0b0r0C0j0b0j0j0j0r0b0j0C0j0j0j0u0r0b0C0j080b0j0j0j0j0j0j0j1c0j0b0j0j0j0C0j0j0j0j0j0j0j0b080j1c0u0j0j0j0C0j1c0j0u0j1c0j0j0j0j0j0j0j0j1c0j0u1v0j0j0V0j0j2g0j0j0j0C1v0C1G0j0j0V0C1Z1O0j0V0j0j2g1v0j0j0V0C2g5x1v4l1v421O7N0V0C4l1v2S1c0j1v2S2_",
      "050b080C0j0j0j0C0j0j0C0j0j0j0C0j0C0j0C030j0j0j0j0j0j0j0j0j0C0j0b080u0V080b0j0j0V0j0j0j0j0j0j0j0j0j0V0N0j0C0C0j0j0j0j0j0j0j0j1c0j0u0j1v0j0j0j0j0j0b080b080j0j0j0b080b080b080b080b0j0j0j080b0j0b080j0j0j0j0b080b0j0j0r0b080b0b080j0j0j0j0b080b080j0b080j0b080b080b080b080b0j0j0r0b0j0b080j0j0j0j0b080b0j0j0C080b0b080j0j0j0j0j0j0j0b080u080j0j0b0j0j0j0C0j0b080j0j0j0j0b080b080b080b0C080b080b080b0j0j0j0j0j0j0b0C080j0j0b0j0j0j0C0j0b080j0j0C0b080b080j0b0j0j0C080b0j0j0j0j0j0j0b0j0j080C0b0j080b0j0j0j0j0j0j0j0C0j0j0j0b0j0j0C080b0j0j0j0j0j0j0b080b080b0K0b080b080b0j0j0j0j0j0j0j0C0j0j0u0j0j0V0j080b0j0C0j0j0j0b0j0r0C0b0j0j0j0j0j0j0j0j0j0C0j0b080b080b0j0C0C0j0C0j0j0j0u110u0j0j0j0j0j0j0j0j0C0j0j0u0j1c0j0j0j0j0j0j0j0j0V0C0u0j0C0C0V0C1Z0j0j0j0C0j0j0j1v0u0j1c0j0j0j0C0j0j2g0j1c1v0C1Z0V0j4l0j0V0j0j2g0j1v0j1v2S1c7N1v",
      "0w0j1c0j0V0j0j0V0V0V0j0m0V0j0C1c140j0j0j0C0V0C0j1v0j0N0j0C0j0j0j0V0j0j1v0N0j0j0V0j0j0j0j0j0j080b0j0j0j0j0j0j0j080b0j0C0j0j0j0b0j0j080u080b0j0j0j0j0j0j0b080b080b080C0b0j080b080b0j0j0j0j080b0j0C0j0j0j0b0j0j080u080b0j0j0j0j0j0j0b080b080b080b0r0b0j080b080b0j0j0j0j080b0j0b0r0j0j0b080b0j0j080b0j080b0j080b080b0j0j0j0j0j0b080b0r0C0b080b0j0j0j0j080b0b080b080j0j0j0b080b080b080b0j0j0j0j080b0j0b080j0j0j0j0b080b0j0j0r0b080b0j0j0j0j0j0b080b080j0b0r0b080j0b080b0j0j0j0j080b0j0b080j0j0j0j0b080b0j080b0r0b0j080b080b0j0j0j0j0j0b080b0r0C0b080b0j0j0j0j0j0j0b080j0j0j0b080b080b080b0j0j0j0r0b0j0b080j0j0j0j0b080b0r0b0r0b0j080b080b0j0j0j0j0j0j0b0r0j0j0j0b0j0j0j0j080b0j0b080j0j0j0j0b080b080b0j0r0b0j080b0j0j0j0j0j0j0j0b0r0C0b0j0j0j0j0j0j0j080b0j0C0j0j0j0b0j0C0r0b0j0j0j0j0j0j0b080b080u0r0b0j080b0j0j0j0j0j0j0j0b0r0C0u0j0j0j0C0j080b0j0C0j0j0j0u110b0j0j0j0j0j0j0j0j0j0C0j0b080b0j0j0C0C0j0C0j0j0j0b0j1c0j080b0j0j0j0j0j0j0V0j0j0u0j1c0j0j0j0C0j0j2g0j0j0j0C0j0j0V0j0b080b1c0C0V0j0j2g0j0j0V0j0j1c0j1Z0j0j0C0C0j1v",
      "160j0j0V0j1c0j0C0j0C0j1f0j0V0C0j0j0C0j0j0j1G080b080u0V080b0j0j0V0j1v0j0u0j1c0j0j0j0C0j0j0j0C0C0j1D0b0j080b0j0j0j0j0C0j0b0r0C0j0b0j0C0C0j0j0j0j0j0j0j0j0j0b0r0b0r0j0b0j0j0j0C0j0b0r0j0j0j0b080b080j0b0C0j080b080b0j0j0j0j0j0j0b0C080j0j0b0j0j0j0C0j0b080j0j0j0j0b080b080j0b0C0r0j0b0j0j0j0j0j0j0b0C080j0j0b0j0j0j0C0j0j0j0j0C0j0j0b080b0j0j0C080b0j0j0j0j0j0j0b080b080b080C0b080b080b080b0j0j0j0j0j0b080C0j0j0b080b0j0j0C080b0j0j0j0j0j0j0b080j0b0C080j0j0b0j0j0j0j0j0j0b080j0b080C0b080b080b080b0j0j0j0j080b0j0C0j0j0b080b0j0j0C080b0j0j0j0j0j0j0b080j0b080u080j0j0b0j0j0j0j0j0j0b080C0j0j0b080b0j0j0C0j0j080b0j0j0j0j0j0b080b0C0r0b080b0j0j0j0j0j0j0b080j0b080u080b080b080b0j0j0j0C0j0b080j0j0j0j0b0j0j0j0C0j0j080b0j0j0j0j0j0b080b0C0r0b080b0j0j0j0j0j0j0b080j0b0r0b080b080b080b0j0j0j0r0b0j0b0r0j0j0j0b0j0j0j0r0b0j080b0j0j0j0j0j0j0j0b0r0C0b0j0j0j0j0j0j0j0b080j0C0u080b080b0j0j0j0r0b0j0C0C0j0b0j110b0j080b0j0j0j0j0j0j0u0r0C0b0j0j0j0j0j0j0j0j0j0C0j0j0j0b0j1c0j0C0j0j0j0b0j0814080b080b0j0j0j0j0j0j1c0j0u0j0j0V0j0j0j0j0j0j0j0u110u0j0j0j",
      "020b0r0C0j0j0j0C0j0j0V0j0j0j0j0j0C0j1f0j0C0j0V1G0j0j0j0j0V0C0j0C1v0u0j0j0j0V0j0j0C0j0j0j1v0N0C0V0j0j0j0K0C250b0C0V0j0j0V0j0j2g0C0V0j0j0C0j0j0b081v0N0j0j0V0V0j0j0u0j1c0j080b0j0j0j0j0j0j0V0j0j0u0j0j0V0j0j0j0C0j0b080b080V0b0j080b0j0j0j0j0j0j0j0b0r0C0j0b0j0j0j0C0j080b0j0j0j0j0j0j0u0r0C0u0j0j0j0j0j0j0b080j0C0j0b080b080b0j0C0j080b0j0j0j0j0j0j0b080b110b0j0j0j0j0j0j0j0j0j0b0r0j0j0j0b0j0j0j0r0b0j0b080j0j0j0j0b080b080b080b0r0b0j080b080b0j0j0j0j0j0j0b0r0C0b080b0j0j0j0j080b0j0b080j0j0j0j0b080b080b0j0j0j0r0b0j0j0j0j0j0j0b080b0j080C0b0j080b080b0j0j0j0j080b0j0b0r0C0b080b0j0j0j0j080b0j0j0j0j0j0b080b080b080b0j0j080b0r0b0j0j0j0j0j0j0b0j0j080C0b0j080b080b0j0j0j0j0j0b080C0j0j0b080b0j0j0C0j0b080j0j0j0j0b080b080b080b0C0C080b0j0j0j0j0j0j0b0C0C080b080b080b0j0j0j0j0j0j0b0C080j0j0b0j0j0j0C0j0b080j0b080j0j0b080b080b080b0C0r0b0j0j0j0j0j0j0b080b0r0b0r0b0j080b080b0j0j0j0j0j0j0b0r0C0j0b0j0j0j0j0j0j0b080j0C0j0b080j0b0j0j0K0b0j0C0j0j0j0b080b0j0K0b0j080b0j0j0j0j0j0j0V0j0j0b0j0j0j0C0j0j0j0j",
      "0l0C0K0N0r0N0j0r1G0V0m0j0V1c0C0j0j0j0j1O0N110u0j0j0j0C0j0j0V0C0j0u110u0j0j0j0C0j0j0j0C0C0j250j1c2S1v1v0j5x2g0j1c0j0j1c2z0j1c0j0j1c0j0N1v0V0C1v0C0b0C0V0j0j0C0j0C1v0u0j0C0C0j0j0j0C0j0j0j0u110u0j0j0j0C0j0C0C0C0b080b0j0C0j080b0j0C0j0j0j0u110u0j0j0j0C0j0j0j0C0j0j0j0u0C0r0u0j0j0j0j0j0j0b0r0b0V080b080b0j0C0j0j0j0V0j0j0b0j0j0j0C0j0j0j0j0j0j0j0b080j0b0C0r0j0b0j0j0j0C0j0b0r0b0r0j0b080b080b0j0C0j0j0j0j0j0j0j0j0b0j0C0r0b0j0j0j0j0j0j0b080b080j0b0r0b0r0j0b0j0j0j0j080b0j0b0r0j0j0j0b080b080b0j0j0j0j080b0j0j0j0j0j0j0b0j0j0j0r0b0j0j0j0j0j0j0b080b080b080b0r0C0b080b0j0j0j0j0j0b080b0r0C0b080b080b080b0j0j0j0j080b0j0C0j0j0j0b0j0j0C080b0j0j0j0j0j0j0b080j0b0C080j0j0b0j0j0j0j0j0j0b0r0b080j0j0b080b080b0j0j0j0j0j0j0b080j0j0j0j0b0j0j0j0r0b0j0b080j0j0j0j0j0b080b080b0C0r0b0j0j0j0j0j0j0b080b080j0C0b0j080b080b0j0j0j0j0j0j",
      "0a0j0j0j0j0C0j0j0C0j0C0C0j0j0j0j0j0j0j0m0C0j0j0j0j0u080j0j0j1n0j0j0j0j0C0j0j0j0V0j0j0j1c0u0j0C0V0j0j0V0j0j1v0N0C0V2o1v1O2S2o141v0j1v4l0j1c0j1v2S2o0C0u1v0j0C0C2S1v0j1c0j0j1v0N251c0j1v0b1c1v1n1v0j0j0V0j0j1v0N1v0C0V0j0j1v0b0C0j0j0V1c0j0u0j1c0j0j0j0j0j0j0j0j1c0j0u0j0j0V0j0j0j0j0j0j0b080u110u0j0j0j0j0j0j1c0j0b0j080b0j0C0j0j0j0V0j0j0u0C0V0j0j0j0C0j0b080j1c0j0b0j0j0j0C0j0C0j0j0j0b080b080b0j0C0j080b0j0j0j0j0j0j0j0b0C0r0u0j0j0j0j0j0j0b080j0b0r0C0j0b0j0j0j0r0b0j0b0r0j0j0j0b080b080b0j0r0b0j080b0j0j0j0j0j0j0b0j0r0C0b0j0j0j0j0j0j0b080j0j0C0j0j0b080b0j0j0j0j0j0j0j0j0j0j0b080b080b080b0C0j0j080b0j0j0j0j0j0j0b0j0j0C080b0j0j0j0j0j0j0j0j0b0C080j0j0b0j0j0j0j0j",
      "0n0Q0j1c14010q0V1c171k0u0r140V0j0j1c0C0N1O0j0V0j0j0j1c0j0u110u0C0j0C0V0C0j0j0b671v0j1v5Q1O2S2o2S1v4l1v0j1v2S2o0C1Z0j0C0C1O141v0j1c0j2z1O0j0V0j0j1v0b2H390j1c0j0V0C2z0j1c0j1v2g0C0V0j1O0b0j0j0V0C1c0j0u0j1c0j0j0j0j0j0j0j0j1c0N0j0j0V0j0j0C0j0j0b081v0u0j0j0j0C0j1c0N0j0j0C0j0j0j0C0j0j0j0u0C0r0u0j0j0j0C0j0b080j1c0j0b0j0C0C0j0C0C0j0b080b080u0C0j080b0j0C0j0j0j0u110u0j0j0j0j0j0j0j0j0C0C0j0b0j0j0j0C0j0C0C0j0b080b080b0j0C0j080b0j0C0j0j0j0b0j110b0j0j0j0j0j",
      "0B0j0V0j0j0C0j0j0j0C0j0C0j0j0C0j0m0j0j0j0j0C0j0C0j0j0u0j1c0j0j0C0C0j0j0j0j0j0j0j0j0u110N0j0j0V0C0V0j0b081n080b0CrU1O5e2SbX2_1Z0V2o141v0j0C0C0j2z1v0j1c0j7N1O420j1c0j1v2S1c0j1v2S2_0b0j0V0j0j1v0N1v0j0j1c0j1v140j0V0j0j0C0C0b080u1v0C0V0u110u0j0j0j0C0j0j0j0C0C0N0C0V0j0j0C0j0j0b080u110u0C0j0C0u0r0C0u080b0j0j0C0j0j0j"
    ];
    for m in months {
      let mut n: isize = 0;
      let size: usize = m.len() / 2;
      let mut l: Vec<isize> = vec![];
      for y in 0..size {
        let z: usize = y * 2;
        let s: &str = &m[z..z + 2];
        let mut t: isize = 0;
        let mut c: isize = 1;
        let mut x: isize = 1;
        while x > -1 {
          t += c * chars.find(s.chars().nth(x as usize).unwrap()).unwrap() as isize;
          c *= 64;
          x -= 1;
        }
        n += t;
        l.push(n);
      }
      list.push(l);
    }
    list
  };
}

/// 农历年
#[derive(Debug, Copy, Clone)]
pub struct LunarYear {
  /// 年
  parent: YearUnit,
}

impl Tyme for LunarYear {
  fn next(&self, n: isize) -> Self {
    Self::from_year(self.get_year() + n)
  }
}

impl Culture for LunarYear {
  fn get_name(&self) -> String {
    format!("农历{}年", self.get_sixty_cycle())
  }
}

impl Deref for LunarYear {
  type Target = YearUnit;

  fn deref(&self) -> &Self::Target {
    &self.parent
  }
}

impl DerefMut for LunarYear {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.parent
  }
}

impl LunarYear {
  pub fn new(year: isize) -> Result<Self, String> {
    Self::validate(year)?;
    Ok(Self {
      parent: YearUnit::new(year),
    })
  }

  pub fn validate(year: isize) -> Result<(), String> {
    if year < -1 || year > 9999 {
      Err(format!("illegal lunar year: {}", year))
    } else {
      Ok(())
    }
  }

  pub fn from_year(year: isize) -> Self {
    Self::new(year).unwrap()
  }

  pub fn get_day_count(&self) -> usize {
    let mut n: usize = 0;
    for m in self.get_months() {
      n += m.get_day_count();
    }
    n
  }

  /// 月数
  ///
  /// # 示例
  ///
  /// ```
  ///
  /// // 12
  /// use tyme4rs::tyme::lunar::LunarYear;
  ///
  /// let month_count: usize = LunarYear::from_year(2008).get_month_count();
  /// ```
  pub fn get_month_count(&self) -> usize {
    let mut n: usize = 12;
    if self.get_leap_month() > 0 {
      n = 13
    }
    n
  }

  pub fn get_months(&self) -> Vec<LunarMonth> {
    let mut l: Vec<LunarMonth> = Vec::new();
    let year: isize = self.get_year();
    let mut m: LunarMonth = LunarMonth::from_ym(year, 1);
    while m.get_year() == year {
      l.push(m);
      m = m.next(1);
    }
    l
  }

  pub fn get_leap_month(&self) -> usize {
    let year: isize = self.get_year();
    if year == -1 {
      return 11;
    }
    for i in 0.. LEAP_MONTH_YEAR.len() {
      if LEAP_MONTH_YEAR[i].contains(&year) {
        return i + 1;
      }
    }
    0
  }

  pub fn get_sixty_cycle(&self) -> SixtyCycle {
    SixtyCycle::from_index(self.get_year() - 4)
  }

  pub fn get_twenty(&self) -> Twenty {
    Twenty::from_index(((self.get_year() as f64 - 1864.0) / 20.0).floor() as isize)
  }

  pub fn get_jupiter_direction(&self) -> Direction {
    Direction::from_index([0, 7, 7, 2, 3, 3, 8, 1, 1, 6, 0, 0][self.get_sixty_cycle().get_earth_branch().get_index()])
  }

  pub fn get_nine_star(&self) -> NineStar {
    NineStar::from_index(63 + self.get_twenty().get_sixty().get_index() as isize * 3 - self.get_sixty_cycle().get_index() as isize)
  }

  pub fn get_kitchen_god_steed(&self) -> KitchenGodSteed {
    KitchenGodSteed::from_lunar_year(self.get_year())
  }
}

impl Display for LunarYear {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for LunarYear {
  fn eq(&self, other: &Self) -> bool {
    self.get_year() == other.get_year()
  }
}

impl Eq for LunarYear {}

pub static LUNAR_MONTH_NAMES: [&str; 12] = ["正月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "十一月", "十二月"];

/// 农历月
#[derive(Debug, Copy, Clone)]
pub struct LunarMonth {
  parent: MonthUnit,
  leap: bool,
}

impl Tyme for LunarMonth {
  fn next(&self, n: isize) -> Self {
    if n == 0 {
      return self.clone();
    }
    let mut m: isize = self.get_index_in_year() as isize + 1 + n;
    let mut y: LunarYear = self.get_lunar_year();
    let mut month_size: isize = y.get_month_count() as isize;
    let forward: bool = n > 0;
    let add: isize = if forward { 1 } else { -1 };
    while if forward { m > month_size } else { m <= 0 } {
      if forward {
        m -= month_size;
      }
      y = y.next(add);
      month_size = y.get_month_count() as isize;
      if !forward {
        m += month_size;
      }
    }
    let mut leap: bool = false;
    let leap_month: usize = y.get_leap_month();
    if leap_month > 0 {
      if m == leap_month as isize + 1 {
        leap = true;
      }
      if m > leap_month as isize {
        m -= 1;
      }
    }
    let month: isize = if leap { -m } else { m };
    Self::from_ym(y.get_year(), month)
  }
}

impl Deref for LunarMonth {
  type Target = MonthUnit;

  fn deref(&self) -> &Self::Target {
    &self.parent
  }
}

impl DerefMut for LunarMonth {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.parent
  }
}

impl Culture for LunarMonth {
  fn get_name(&self) -> String {
    let leap: &str = if self.leap { "闰" } else { "" };
    format!("{}{}", leap, LUNAR_MONTH_NAMES[self.get_month() - 1])
  }
}

impl LunarMonth {
  pub fn new(year: isize, month: isize) -> Result<Self, String> {
    Self::validate(year, month)?;
    Ok(Self {
      parent: MonthUnit::new(year, month.abs()),
      leap: month < 0,
    })
  }

  pub fn validate(year: isize, month: isize) -> Result<(), String> {
    if month == 0 || month > 12 || month < -12 {
      Err(format!("illegal lunar month: {}", month))
    } else {
      let m: usize = month.abs() as usize;
      if month < 0 && m != LunarYear::from_year(year).get_leap_month() {
        return Err(format!("illegal leap month {} in lunar year {}", m, year));
      }
      Ok(())
    }
  }

  pub fn from_ym(year: isize, month: isize) -> Self {
    Self::new(year, month).unwrap()
  }

  pub fn get_lunar_year(&self) -> LunarYear {
    LunarYear::from_year(self.get_year())
  }

  pub fn get_month(&self) -> usize {
    self.parent.get_month() as usize
  }

  pub fn get_month_with_leap(&self) -> isize {
    match self.leap {
      false => self.get_month() as isize,
      _ => -(self.get_month() as isize)
    }
  }

  fn get_new_moon(&self) -> f64 {
    let year: isize = self.get_year();
    // 冬至
    let dong_zhi_jd: f64 = SolarTerm::from_index(year, 0).get_cursory_julian_day();

    // 冬至前的初一，今年首朔的日月黄经差
    let mut w: f64 = ShouXingUtil::calc_shuo(dong_zhi_jd);
    if w > dong_zhi_jd {
      w -= 29.53;
    }

    // 计算正月初一的偏移
    let prev_year: LunarYear = LunarYear::from_year(year - 1);
    let prev_leap_month: usize = prev_year.get_leap_month();

    // 正常情况正月初一为第3个朔日，但有些特殊的
    let mut offset: f64 = 2.0;
    if year > 8 && year < 24 {
      offset = 1.0;
    } else if prev_leap_month > 10 && year != 239 && year != 240 {
      offset = 3.0;
    }

    // 本月初一
    w + 29.5306 * (offset + self.get_index_in_year() as f64)
  }

  pub fn get_day_count(&self) -> usize {
    let w: f64 = self.get_new_moon();
    // 本月天数 = 下月初一 - 本月初一
    (ShouXingUtil::calc_shuo(w + 29.5306) - ShouXingUtil::calc_shuo(w)) as usize
  }

  pub fn get_index_in_year(&self) -> usize {
    let month: usize = self.get_month();
    let mut index: usize = month - 1;
    if self.is_leap() {
      index += 1;
    } else {
      let leap_month: usize = self.get_lunar_year().get_leap_month();
      if leap_month > 0 && month > leap_month {
        index += 1;
      }
    }
    index
  }

  pub fn get_season(&self) -> LunarSeason {
    LunarSeason::from_index(self.get_month() as isize - 1)
  }

  pub fn get_first_julian_day(&self) -> JulianDay {
    JulianDay::from_julian_day(J2000 + ShouXingUtil::calc_shuo(self.get_new_moon()))
  }

  pub fn is_leap(&self) -> bool {
    self.leap
  }

  pub fn get_week_count(&self, start: usize) -> usize {
    ((AbstractCulture::new().index_of((self.get_first_julian_day().get_week().get_index() as isize) - (start as isize), 7) + self.get_day_count()) as f64 / 7.0).ceil() as usize
  }

  pub fn get_days(&self) -> Vec<LunarDay> {
    let mut l: Vec<LunarDay> = Vec::new();
    let size: usize = self.get_day_count();
    let y: isize = self.get_year();
    let m: isize = self.get_month_with_leap();
    for i in 0..size {
      l.push(LunarDay::from_ymd(y, m, i + 1));
    }
    l
  }

  pub fn get_first_day(&self) -> LunarDay {
    LunarDay::from_ymd(self.get_year(), self.get_month_with_leap(), 1)
  }

  pub fn get_weeks(&self, start: usize) -> Vec<LunarWeek> {
    let mut l: Vec<LunarWeek> = Vec::new();
    let size: usize = self.get_week_count(start);
    let y: isize = self.get_year();
    let m: isize = self.get_month_with_leap();
    for i in 0..size {
      l.push(LunarWeek::from_ym(y, m, i, start));
    }
    l
  }

  pub fn get_sixty_cycle(&self) -> SixtyCycle {
    let month: isize = self.get_month() as isize;
    SixtyCycle::from_name(format!("{}{}", HeavenStem::from_index(self.get_lunar_year().get_sixty_cycle().get_heaven_stem().get_index() as isize * 2 + month + 1).get_name(), EarthBranch::from_index(month + 1).get_name()).as_str())
  }

  pub fn get_jupiter_direction(&self) -> Direction {
    let sixty_cycle: SixtyCycle = self.get_sixty_cycle();
    let n: isize = [7, -1, 1, 3][sixty_cycle.get_earth_branch().next(-2).get_index() % 4];
    match n {
      -1 => sixty_cycle.get_heaven_stem().get_direction(),
      _ => Direction::from_index(n)
    }
  }

  /// 九星
  pub fn get_nine_star(&self) -> NineStar {
    let mut index: isize = self.get_sixty_cycle().get_earth_branch().get_index() as isize;
    if index < 2 {
      index += 3;
    }
    NineStar::from_index(27 - self.get_lunar_year().get_sixty_cycle().get_earth_branch().get_index() as isize % 3 * 3 - index)
  }

  /// 逐月胎神
  pub fn get_fetus(&self) -> Option<FetusMonth> {
    FetusMonth::from_lunar_month(*self)
  }

  /// 小六壬
  pub fn get_minor_ren(&self) -> MinorRen {
    MinorRen::from_index((self.get_month() as isize - 1) % 6)
  }
}

impl Display for LunarMonth {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.get_lunar_year(), self.get_name())
  }
}

impl PartialEq for LunarMonth {
  fn eq(&self, other: &Self) -> bool {
    self.get_year() == other.get_year() && self.get_month_with_leap() == other.get_month_with_leap()
  }
}

impl Eq for LunarMonth {}

pub static LUNAR_SEASON_NAMES: [&str; 12] = ["孟春", "仲春", "季春", "孟夏", "仲夏", "季夏", "孟秋", "仲秋", "季秋", "孟冬", "仲冬", "季冬"];

/// 农历季节
#[derive(Debug, Clone)]
pub struct LunarSeason {
  parent: LoopTyme,
}

impl Deref for LunarSeason {
  type Target = LoopTyme;

  fn deref(&self) -> &Self::Target {
    &self.parent
  }
}

impl DerefMut for LunarSeason {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.parent
  }
}

impl Tyme for LunarSeason {
  fn next(&self, n: isize) -> Self {
    Self::from_index(self.parent.next_index(n) as isize)
  }
}

impl Culture for LunarSeason {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl LunarSeason {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(LUNAR_SEASON_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Self {
    Self {
      parent: LoopTyme::from_name(LUNAR_SEASON_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name)
    }
  }
}

impl Display for LunarSeason {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for LunarSeason {
  fn eq(&self, other: &Self) -> bool {
    self.get_index() == other.get_index()
  }
}

impl Eq for LunarSeason {}

impl Into<LoopTyme> for LunarSeason {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

/// 农历周名称
pub static LUNAR_WEEK_NAMES: [&str; 6] = ["第一周", "第二周", "第三周", "第四周", "第五周", "第六周"];

/// 农历周
#[derive(Debug, Clone)]
pub struct LunarWeek {
  parent: WeekUnit,
}

impl Deref for LunarWeek {
  type Target = WeekUnit;

  fn deref(&self) -> &Self::Target {
    &self.parent
  }
}

impl DerefMut for LunarWeek {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.parent
  }
}

impl Tyme for LunarWeek {
  fn next(&self, n: isize) -> Self {
    if n == 0 {
      self.clone()
    } else {
      let mut d: isize = (self.get_index() as isize) + n;
      let mut m: LunarMonth = self.get_lunar_month();
      let start_index: usize = self.get_start();
      if n > 0 {
        let mut week_count: isize = m.get_week_count(start_index) as isize;
        while d >= week_count {
          d -= week_count;
          m = m.next(1);
          if m.get_first_day().get_week().get_index() != start_index {
            d += 1;
          }
          week_count = m.get_week_count(start_index) as isize;
        }
      } else {
        while d < 0 {
          if m.get_first_day().get_week().get_index() != start_index {
            d -= 1;
          }
          m = m.next(-1);
          d += m.get_week_count(start_index) as isize;
        }
      }
      Self::from_ym(m.get_year(), m.get_month_with_leap(), d as usize, start_index)
    }
  }
}

impl Culture for LunarWeek {
  fn get_name(&self) -> String {
    LUNAR_WEEK_NAMES[self.get_index()].to_string()
  }
}

impl LunarWeek {
  pub fn new(year: isize, month: isize, index: usize, start: usize) -> Result<Self, String> {
    Self::validate(year, month, index, start)?;
    Ok(Self {
      parent: WeekUnit::new(year, month, index, start),
    })
  }

  pub fn validate(year: isize, month: isize, index: usize, start: usize) -> Result<(), String> {
    WeekUnit::validate(index, start)?;
    let m: LunarMonth = LunarMonth::from_ym(year, month);
    if index >= m.get_week_count(start) {
      Err(format!("illegal lunar week index: {} in month: {}", index, m))
    } else {
      Ok(())
    }
  }

  pub fn from_ym(year: isize, month: isize, index: usize, start: usize) -> Self {
    Self::new(year, month, index, start).unwrap()
  }

  pub fn get_lunar_month(&self) -> LunarMonth {
    LunarMonth::from_ym(self.get_year(), self.get_month())
  }

  pub fn get_first_day(&self) -> LunarDay {
    let first_day: LunarDay = LunarDay::from_ymd(self.get_year(), self.get_month(), 1);
    first_day.next(self.get_index() as isize * 7 - self.index_of((first_day.get_week().get_index() as isize) - (self.get_start() as isize), 7) as isize)
  }

  pub fn get_days(&self) -> Vec<LunarDay> {
    let mut l: Vec<LunarDay> = Vec::new();
    let d: LunarDay = self.get_first_day();
    let n: LunarDay = d.clone();
    l.push(d);
    for i in 1..7 {
      l.push(n.next(i));
    }
    l
  }
}

impl Display for LunarWeek {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.get_lunar_month(), self.get_name())
  }
}

impl PartialEq for LunarWeek {
  fn eq(&self, other: &Self) -> bool {
    self.get_first_day() == other.get_first_day()
  }
}

impl Eq for LunarWeek {}

/// 农历日名称
pub static LUNAR_DAY_NAMES: [&str; 30] = ["初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八", "初九", "初十", "十一", "十二", "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十", "廿一", "廿二", "廿三", "廿四", "廿五", "廿六", "廿七", "廿八", "廿九", "三十"];

/// 农历日
#[derive(Debug, Clone)]
pub struct LunarDay {
  parent: DayUnit,
}

impl Tyme for LunarDay {
  fn next(&self, n: isize) -> Self {
    if n == 0 {
      self.clone()
    } else {
      self.get_solar_day().next(n).get_lunar_day()
    }
  }
}

impl Culture for LunarDay {
  fn get_name(&self) -> String {
    LUNAR_DAY_NAMES[self.get_day() - 1].to_string()
  }
}

impl Deref for LunarDay {
  type Target = DayUnit;

  fn deref(&self) -> &Self::Target {
    &self.parent
  }
}

impl DerefMut for LunarDay {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.parent
  }
}

impl LunarDay {
  pub fn new(year: isize, month: isize, day: usize) -> Result<Self, String> {
    Self::validate(year, month, day)?;
    Ok(Self {
      parent: DayUnit::new(year, month, day as isize)
    })
  }

  pub fn validate(year: isize, month: isize, day: usize) -> Result<(), String> {
    if day < 1 {
      Err(format!("illegal lunar day {}", day))
    } else {
      let m: LunarMonth = LunarMonth::from_ym(year, month);
      if day > m.get_day_count() {
        return Err(format!("illegal day {} in {}", day, m))
      }
      Ok(())
    }
  }

  pub fn from_ymd(year: isize, month: isize, day: usize) -> Self {
    Self::new(year, month, day).unwrap()
  }

  /// 农历月
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::lunar::{LunarDay, LunarMonth};
  ///
  /// // 正月
  /// let lunar_month: LunarMonth = LunarDay::from_ymd(2023, 1, 1).get_lunar_month();
  /// ```
  pub fn get_lunar_month(&self) -> LunarMonth {
    LunarMonth::from_ym(self.get_year(), self.get_month())
  }

  /// 日
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::lunar::LunarDay;
  ///
  /// // 1
  /// let day: usize = LunarDay::from_ymd(2023, 1, 1).get_day();
  /// ```
  pub fn get_day(&self) -> usize {
    self.parent.get_day() as usize
  }

  pub fn get_week(&self) -> Week {
    self.get_solar_day().get_week()
  }

  pub fn is_before(&self, target: LunarDay) -> bool {
    let a_year: isize = self.get_year();
    let b_year: isize = target.get_year();
    if a_year != b_year {
      return a_year < b_year;
    }
    let a_month: isize = self.get_month();
    let b_month: isize = target.get_month();
    if a_month != b_month {
      return a_month.abs() < b_month.abs();
    }
    self.get_day() < target.get_day()
  }

  pub fn is_after(&self, target: LunarDay) -> bool {
    let a_year: isize = self.get_year();
    let b_year: isize = target.get_year();
    if a_year != b_year {
      return a_year > b_year;
    }
    let a_month: isize = self.get_month();
    let b_month: isize = target.get_month();
    if a_month != b_month {
      return a_month.abs() >= b_month.abs();
    }
    self.get_day() > target.get_day()
  }

  /// 当天的年干支（立春换）
  #[deprecated(since = "1.3.0", note = "please use SixtyCycleDay.get_year() instead")]
  pub fn get_year_sixty_cycle(&self) -> SixtyCycle {
    self.get_sixty_cycle_day().get_year()
  }

  /// 当天的月干支（节气换）
  #[deprecated(since = "1.3.0", note = "please use SixtyCycleDay.get_month() instead")]
  pub fn get_month_sixty_cycle(&self) -> SixtyCycle {
    self.get_sixty_cycle_day().get_month()
  }

  /// 干支
  pub fn get_sixty_cycle(&self) -> SixtyCycle {
    let offset: isize = self.get_lunar_month().get_first_julian_day().next((self.get_day() as isize) - 12).get_day() as isize;
    SixtyCycle::from_name(format!("{}{}", HeavenStem::from_index(offset).get_name(), EarthBranch::from_index(offset).get_name()).as_str())
  }

  /// 建除十二值神
  pub fn get_duty(&self) -> Duty {
    self.get_sixty_cycle_day().get_duty()
  }

  /// 太岁方位
  pub fn get_jupiter_direction(&self) -> Direction {
    let index: isize = self.get_sixty_cycle().get_index() as isize;
    if index % 12 < 6 {
      return Element::from_index(index / 12).get_direction();
    }
    self.get_lunar_month().get_lunar_year().get_jupiter_direction()
  }

  /// 月相第几天
  pub fn get_phase_day(&self) -> PhaseDay {
    let today: SolarDay = self.get_solar_day();
    let m: LunarMonth = self.get_lunar_month().next(1);
    let mut p: Phase = Phase::from_index(m.get_year(), m.get_month_with_leap(), 0);
    let mut d: SolarDay = p.get_solar_day();
    while d.is_after(today) {
      p = p.next(-1);
      d = p.get_solar_day();
    }
    PhaseDay::new(p, today.subtract(d) as usize)
  }

  /// 月相
  pub fn get_phase(&self) -> Phase {
    self.get_phase_day().get_phase()
  }

  /// 六曜
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::culture::star::six::SixStar;
  /// use tyme4rs::tyme::lunar::LunarDay;
  ///
  /// let six_star: SixStar = LunarDay::from_ymd(2023, 1, 1).get_six_star();
  /// ```
  pub fn get_six_star(&self) -> SixStar {
    SixStar::from_index((self.get_month() + self.get_day() as isize - 2) % 6)
  }

  /// 公历日
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::lunar::LunarDay;
  /// use tyme4rs::tyme::solar::SolarDay;
  ///
  /// // 农历日转公历日
  /// let solar_day: SolarDay = LunarDay::from_ymd(2023, 1, 1).get_solar_day();
  /// ```
  pub fn get_solar_day(&self) -> SolarDay {
    self.get_lunar_month().get_first_julian_day().next(self.get_day() as isize - 1).get_solar_day()
  }

  /// 干支日
  pub fn get_sixty_cycle_day(&self) -> SixtyCycleDay {
    self.get_solar_day().get_sixty_cycle_day()
  }

  /// 黄道黑道十二神
  pub fn get_twelve_star(&self) -> TwelveStar {
    self.get_sixty_cycle_day().get_twelve_star()
  }

  /// 二十八宿
  pub fn get_twenty_eight_star(&self) -> TwentyEightStar {
    TwentyEightStar::from_index([10, 18, 26, 6, 14, 22, 2][self.get_solar_day().get_week().get_index()]).next(-7 * self.get_sixty_cycle().get_earth_branch().get_index() as isize)
  }

  /// 逐日胎神
  pub fn get_fetus_day(&self) -> FetusDay {
    FetusDay::from_lunar_day(self.clone())
  }

  /// 农历传统节日
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::festival::LunarFestival;
  /// use tyme4rs::tyme::lunar::LunarDay;
  ///
  /// let festival: Option<LunarFestival> = LunarDay::from_ymd(2024, 1, 1).get_festival();
  /// ```
  pub fn get_festival(&self) -> Option<LunarFestival> {
    LunarFestival::from_ymd(self.get_year(), self.get_month(), self.get_day())
  }

  /// 九星
  pub fn get_nine_star(&self) -> NineStar {
    let solar: SolarDay = self.get_solar_day();
    let dong_zhi: SolarTerm = SolarTerm::from_index(solar.get_year(), 0);
    let dong_zhi_solar: SolarDay = dong_zhi.get_solar_day();
    let xia_zhi_solar: SolarDay = dong_zhi.next(12).get_solar_day();
    let dong_zhi_solar2: SolarDay = dong_zhi.next(24).get_solar_day();
    let dong_zhi_index: isize = dong_zhi_solar.get_lunar_day().get_sixty_cycle().get_index() as isize;
    let xia_zhi_index: isize = xia_zhi_solar.get_lunar_day().get_sixty_cycle().get_index() as isize;
    let dong_zhi_index2: isize = dong_zhi_solar2.get_lunar_day().get_sixty_cycle().get_index() as isize;
    let solar_shun_bai: SolarDay = dong_zhi_solar.next(if dong_zhi_index > 29 { 60 - dong_zhi_index } else { -dong_zhi_index });
    let solar_shun_bai2: SolarDay = dong_zhi_solar2.next(if dong_zhi_index2 > 29 { 60 - dong_zhi_index2 } else { -dong_zhi_index2 });
    let solar_ni_zi: SolarDay = xia_zhi_solar.next(if xia_zhi_index > 29 { 60 - xia_zhi_index } else { -xia_zhi_index });
    let mut offset: isize = 0;
    if !solar.is_before(solar_shun_bai) && solar.is_before(solar_ni_zi) {
      offset = solar.subtract(solar_shun_bai);
    } else if !solar.is_before(solar_ni_zi) && solar.is_before(solar_shun_bai2) {
      offset = 8 - solar.subtract(solar_ni_zi);
    } else if !solar.is_before(solar_shun_bai2) {
      offset = solar.subtract(solar_shun_bai2);
    } else if solar.is_before(solar_shun_bai) {
      offset = 8 + solar_shun_bai.subtract(solar);
    }
    NineStar::from_index(offset)
  }

  pub fn get_hours(&self) -> Vec<LunarHour> {
    let mut l: Vec<LunarHour> = Vec::new();
    let y: isize = self.get_year();
    let m: isize = self.get_month();
    let d: usize = self.get_day();
    l.push(LunarHour::from_ymd_hms(y, m, d, 0, 0, 0));
    for i in (0..24).step_by(2) {
      l.push(LunarHour::from_ymd_hms(y, m, d, i + 1, 0, 0));
    }
    l
  }

  pub fn get_gods(&self) -> Vec<God> {
    self.get_sixty_cycle_day().get_gods()
  }

  /// 宜
  pub fn get_recommends(&self) -> Vec<Taboo> {
    self.get_sixty_cycle_day().get_recommends()
  }

  /// 忌
  pub fn get_avoids(&self) -> Vec<Taboo> {
    self.get_sixty_cycle_day().get_avoids()
  }

  /// 小六壬
  pub fn get_minor_ren(&self) -> MinorRen {
    self.get_lunar_month().get_minor_ren().next(self.get_day() as isize - 1)
  }

  /// 三柱
  pub fn get_three_pillars(&self) -> ThreePillars {
    self.get_sixty_cycle_day().get_three_pillars()
  }
}

impl Display for LunarDay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.get_lunar_month(), self.get_name())
  }
}

impl PartialEq for LunarDay {
  fn eq(&self, other: &Self) -> bool {
    self.get_year() == other.get_year() && self.get_month() == other.get_month() && self.get_day() == other.get_day()
  }
}

impl Eq for LunarDay {}

lazy_static! {
  static ref EIGHT_CHAR_PROVIDER: Arc<Mutex<Box<dyn EightCharProvider + Sync + Send + 'static>>> = Arc::new(Mutex::new(Box::new(DefaultEightCharProvider::new())));
}

/// 农历时辰
#[derive(Debug, Clone)]
pub struct LunarHour {
  parent: SecondUnit,
}

impl Tyme for LunarHour {
  fn next(&self, n: isize) -> Self {
    if n == 0 {
      self.clone()
    } else {
      let h: isize = (self.get_hour() as isize) + n * 2;
      let diff: isize = if h < 0 { -1 } else { 1 };
      let mut hour: isize = h.abs();
      let mut days: isize = hour / 24 * diff;
      hour = (hour % 24) * diff;
      if hour < 0 {
        hour += 24;
        days -= 1;
      }
      let d: LunarDay = self.get_lunar_day().next(days);
      Self::from_ymd_hms(d.get_year(), d.get_month(), d.get_day(), hour as usize, self.get_minute(), self.get_second())
    }
  }
}

impl Culture for LunarHour {
  fn get_name(&self) -> String {
    format!("{}时", EarthBranch::from_index(self.get_index_in_day() as isize).get_name())
  }
}

impl Deref for LunarHour {
  type Target = SecondUnit;

  fn deref(&self) -> &Self::Target {
    &self.parent
  }
}

impl DerefMut for LunarHour {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.parent
  }
}

impl LunarHour {
  pub fn new(year: isize, month: isize, day: usize, hour: usize, minute: usize, second: usize) -> Result<Self, String> {
    Self::validate(year, month, day, hour, minute, second)?;
    Ok(Self {
      parent: SecondUnit::new(year, month, day as isize, hour, minute, second),
    })
  }

  pub fn validate(year: isize, month: isize, day: usize, hour: usize, minute: usize, second: usize) -> Result<(), String> {
    SecondUnit::validate(hour, minute, second)?;
    LunarDay::validate(year, month, day)?;
    Ok(())
  }

  pub fn from_ymd_hms(year: isize, month: isize, day: usize, hour: usize, minute: usize, second: usize) -> Self {
    Self::new(year, month, day, hour, minute, second).unwrap()
  }

  pub fn get_lunar_day(&self) -> LunarDay {
    LunarDay::from_ymd(self.get_year(), self.get_month(), self.get_day())
  }

  pub fn get_day(&self) -> usize {
    self.parent.get_day() as usize
  }

  pub fn get_index_in_day(&self) -> usize {
    (self.get_hour() + 1) / 2
  }

  pub fn is_before(&self, target: LunarHour) -> bool {
    let a_day: LunarDay = self.get_lunar_day();
    let b_day: LunarDay = target.get_lunar_day();
    if a_day != b_day {
      return a_day.is_before(b_day);
    }
    let a_hour: usize = self.get_hour();
    let b_hour: usize = target.get_hour();
    if a_hour != b_hour {
      return a_hour < b_hour;
    }
    let a_minute: usize = self.get_minute();
    let b_minute: usize = target.get_minute();
    if a_minute != b_minute { a_minute < b_minute } else { self.get_second() < target.get_second() }
  }

  pub fn is_after(&self, target: LunarHour) -> bool {
    let a_day: LunarDay = self.get_lunar_day();
    let b_day: LunarDay = target.get_lunar_day();
    if a_day != b_day {
      return a_day.is_after(b_day);
    }
    let a_hour: usize = self.get_hour();
    let b_hour: usize = target.get_hour();
    if a_hour != b_hour {
      return a_hour > b_hour;
    }
    let a_minute: usize = self.get_minute();
    let b_minute: usize = target.get_minute();
    if a_minute != b_minute { a_minute > b_minute } else { self.get_second() > target.get_second() }
  }

  #[deprecated(since = "1.3.0", note = "please use SixtyCycleHour.get_year() instead")]
  pub fn get_year_sixty_cycle(&self) -> SixtyCycle {
    self.clone().get_sixty_cycle_hour().get_year()
  }

  #[deprecated(since = "1.3.0", note = "please use SixtyCycleHour.get_month() instead")]
  pub fn get_month_sixty_cycle(&self) -> SixtyCycle {
    self.clone().get_sixty_cycle_hour().get_month()
  }

  #[deprecated(since = "1.3.0", note = "please use SixtyCycleHour.get_day() instead")]
  pub fn get_day_sixty_cycle(&self) -> SixtyCycle {
    self.clone().get_sixty_cycle_hour().get_day()
  }

  pub fn get_sixty_cycle(&self) -> SixtyCycle {
    let earth_branch_index: isize = self.get_index_in_day() as isize % 12;
    let mut d: SixtyCycle = self.get_lunar_day().get_sixty_cycle();
    if self.get_hour() >= 23 {
      d = d.next(1);
    }
    SixtyCycle::from_name(format!("{}{}", HeavenStem::from_index(d.get_heaven_stem().get_index() as isize % 5 * 2 + earth_branch_index).get_name(), EarthBranch::from_index(earth_branch_index).get_name()).as_str())
  }

  pub fn get_solar_time(&self) -> SolarTime {
    let d: SolarDay = self.get_lunar_day().get_solar_day();
    SolarTime::from_ymd_hms(d.get_year(), d.get_month(), d.get_day(), self.get_hour(), self.get_minute(), self.get_second())
  }

  pub fn get_sixty_cycle_hour(&self) -> SixtyCycleHour {
    self.get_solar_time().get_sixty_cycle_hour()
  }

  pub fn get_eight_char(&self) -> EightChar {
    EIGHT_CHAR_PROVIDER.lock().unwrap().get_eight_char(self.clone())
  }

  pub fn get_nine_star(&self) -> NineStar {
    let day: LunarDay = self.get_lunar_day();
    let solar: SolarDay = day.get_solar_day();
    let dong_zhi: SolarTerm = SolarTerm::from_index(solar.get_year(), 0);
    let earth_branch_index: isize = self.get_index_in_day() as isize % 12;
    let mut index = [8, 5, 2][day.get_sixty_cycle().get_earth_branch().get_index() % 3];
    if !solar.is_before(dong_zhi.get_julian_day().get_solar_day()) && solar.is_before(dong_zhi.next(12).get_julian_day().get_solar_day()) {
      index = 8 + earth_branch_index - index;
    } else {
      index -= earth_branch_index;
    }
    NineStar::from_index(index)
  }

  pub fn get_twelve_star(&self) -> TwelveStar {
    TwelveStar::from_index(self.get_sixty_cycle().get_earth_branch().get_index() as isize + (8 - self.get_sixty_cycle_hour().get_day().get_earth_branch().get_index() as isize % 6) * 2)
  }

  pub fn get_recommends(&self) -> Vec<Taboo> {
    Taboo::get_hour_recommends(self.get_sixty_cycle_hour().get_day(), self.get_sixty_cycle())
  }

  pub fn get_avoids(&self) -> Vec<Taboo> {
    Taboo::get_hour_avoids(self.get_sixty_cycle_hour().get_day(), self.get_sixty_cycle())
  }

  /// 小六壬
  pub fn get_minor_ren(&self) -> MinorRen {
    self.get_lunar_day().get_minor_ren().next(self.get_index_in_day() as isize)
  }
}

impl Display for LunarHour {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}时", self.get_lunar_day(), self.get_sixty_cycle().get_name())
  }
}

impl PartialEq for LunarHour {
  fn eq(&self, other: &Self) -> bool {
    self.get_lunar_day() == other.get_lunar_day() && self.get_hour() == other.get_hour() && self.get_minute() == other.get_minute() && self.get_second() == other.get_second()
  }
}

impl Eq for LunarHour {}

#[cfg(test)]
mod tests {
  use crate::tyme::culture::star::twenty_eight::TwentyEightStar;
  use crate::tyme::lunar::{LunarDay, LunarHour, LunarMonth, LunarYear};
  use crate::tyme::solar::SolarDay;
  use crate::tyme::{Culture, Tyme};

  #[test]
  fn test1() {
    assert_eq!("1年1月1日", LunarDay::from_ymd(0, 11, 18).get_solar_day().to_string());
  }

  #[test]
  fn test2() {
    assert_eq!("9999年12月31日", LunarDay::from_ymd(9999, 12, 2).get_solar_day().to_string());
  }

  #[test]
  fn test3() {
    assert_eq!("1905年2月4日", LunarDay::from_ymd(1905, 1, 1).get_solar_day().to_string());
  }

  #[test]
  fn test4() {
    assert_eq!("2039年1月23日", LunarDay::from_ymd(2038, 12, 29).get_solar_day().to_string());
  }

  #[test]
  fn test5() {
    assert_eq!("1500年1月31日", LunarDay::from_ymd(1500, 1, 1).get_solar_day().to_string());
  }

  #[test]
  fn test6() {
    assert_eq!("1501年1月18日", LunarDay::from_ymd(1500, 12, 29).get_solar_day().to_string());
  }

  #[test]
  fn test7() {
    assert_eq!("1582年10月4日", LunarDay::from_ymd(1582, 9, 18).get_solar_day().to_string());
  }

  #[test]
  fn test8() {
    assert_eq!("1582年10月15日", LunarDay::from_ymd(1582, 9, 19).get_solar_day().to_string());
  }

  #[test]
  fn test9() {
    assert_eq!("2020年1月6日", LunarDay::from_ymd(2019, 12, 12).get_solar_day().to_string());
  }

  #[test]
  fn test10() {
    assert_eq!("2033年12月22日", LunarDay::from_ymd(2033, -11, 1).get_solar_day().to_string());
  }

  #[test]
  fn test11() {
    assert_eq!("2021年7月16日", LunarDay::from_ymd(2021, 6, 7).get_solar_day().to_string());
  }

  #[test]
  fn test12() {
    assert_eq!("2034年2月19日", LunarDay::from_ymd(2034, 1, 1).get_solar_day().to_string());
  }

  #[test]
  fn test13() {
    assert_eq!("2034年1月20日", LunarDay::from_ymd(2033, 12, 1).get_solar_day().to_string());
  }

  #[test]
  fn test14() {
    assert_eq!("7013年12月24日", LunarDay::from_ymd(7013, -11, 4).get_solar_day().to_string());
  }

  #[test]
  fn test15() {
    assert_eq!("己亥", LunarDay::from_ymd(2023, 8, 24).get_sixty_cycle().to_string());
  }

  #[test]
  fn test16() {
    assert_eq!("癸酉", LunarDay::from_ymd(1653, 1, 6).get_sixty_cycle().to_string());
  }

  #[test]
  fn test17() {
    assert_eq!("农历庚寅年二月初二", LunarDay::from_ymd(2010, 1, 1).next(31).to_string());
  }

  #[test]
  fn test18() {
    assert_eq!("农历壬辰年闰四月初一", LunarDay::from_ymd(2012, 3, 1).next(60).to_string());
  }

  #[test]
  fn test19() {
    assert_eq!("农历壬辰年闰四月廿九", LunarDay::from_ymd(2012, 3, 1).next(88).to_string());
  }

  #[test]
  fn test20() {
    assert_eq!("农历壬辰年五月初一", LunarDay::from_ymd(2012, 3, 1).next(89).to_string());
  }

  #[test]
  fn test21() {
    assert_eq!("2020年4月23日", LunarDay::from_ymd(2020, 4, 1).get_solar_day().to_string());
  }

  #[test]
  fn test22() {
    assert_eq!("甲辰", LunarDay::from_ymd(2024, 1, 1).get_lunar_month().get_lunar_year().get_sixty_cycle().get_name());
  }

  #[test]
  fn test23() {
    assert_eq!("癸卯", LunarDay::from_ymd(2023, 12, 30).get_lunar_month().get_lunar_year().get_sixty_cycle().get_name());
  }

  #[test]
  fn test24() {
    let d: LunarDay = LunarDay::from_ymd(2020, 4, 13);
    let star: TwentyEightStar = d.get_twenty_eight_star();
    assert_eq!("南", star.get_zone().get_name());
    assert_eq!("朱雀", star.get_zone().get_beast().get_name());
    assert_eq!("翼", star.get_name());
    assert_eq!("火", star.get_seven_star().get_name());
    assert_eq!("蛇", star.get_animal().get_name());
    assert_eq!("凶", star.get_luck().get_name());
    assert_eq!("阳天", star.get_land().get_name());
    assert_eq!("东南", star.get_land().get_direction().get_name());
  }

  #[test]
  fn test25() {
    let d: LunarDay = LunarDay::from_ymd(2023, 9, 28);
    let star: TwentyEightStar = d.get_twenty_eight_star();
    assert_eq!("南", star.get_zone().get_name());
    assert_eq!("朱雀", star.get_zone().get_beast().get_name());
    assert_eq!("柳", star.get_name());
    assert_eq!("土", star.get_seven_star().get_name());
    assert_eq!("獐", star.get_animal().get_name());
    assert_eq!("凶", star.get_luck().get_name());
    assert_eq!("炎天", star.get_land().get_name());
    assert_eq!("南", star.get_land().get_direction().get_name());
  }

  #[test]
  fn test26() {
    let lunar: LunarDay = LunarDay::from_ymd(2005, 11, 23);
    assert_eq!("戊子", lunar.get_lunar_month().get_sixty_cycle().get_name());
    assert_eq!("戊子", lunar.get_sixty_cycle_day().get_month().get_name());
  }

  #[test]
  fn test27() {
    let h: LunarHour = LunarHour::from_ymd_hms(2020, -4, 5, 23, 0, 0);
    assert_eq!("子时", h.get_name());
    assert_eq!("农历庚子年闰四月初五戊子时", h.to_string());
  }

  #[test]
  fn test28() {
    let h: LunarHour = LunarHour::from_ymd_hms(2020, -4, 5, 0, 59, 0);
    assert_eq!("子时", h.get_name());
    assert_eq!("农历庚子年闰四月初五丙子时", h.to_string());
  }

  #[test]
  fn test29() {
    let h: LunarHour = LunarHour::from_ymd_hms(2020, -4, 5, 1, 0, 0);
    assert_eq!("丑时", h.get_name());
    assert_eq!("农历庚子年闰四月初五丁丑时", h.to_string());
  }

  #[test]
  fn test30() {
    let h: LunarHour = LunarHour::from_ymd_hms(2020, -4, 5, 21, 30, 0);
    assert_eq!("亥时", h.get_name());
    assert_eq!("农历庚子年闰四月初五丁亥时", h.to_string());
  }

  #[test]
  fn test31() {
    let h: LunarHour = LunarHour::from_ymd_hms(2020, -4, 2, 23, 30, 0);
    assert_eq!("子时", h.get_name());
    assert_eq!("农历庚子年闰四月初二壬子时", h.to_string());
  }

  #[test]
  fn test32() {
    let h: LunarHour = LunarHour::from_ymd_hms(2020, 4, 28, 23, 30, 0);
    assert_eq!("子时", h.get_name());
    assert_eq!("农历庚子年四月廿八甲子时", h.to_string());
  }

  #[test]
  fn test33() {
    let h: LunarHour = LunarHour::from_ymd_hms(2020, 4, 29, 0, 0, 0);
    assert_eq!("子时", h.get_name());
    assert_eq!("农历庚子年四月廿九甲子时", h.to_string());
  }

  #[test]
  fn test34() {
    let h: LunarHour = LunarHour::from_ymd_hms(2023, 11, 14, 23, 0, 0);
    assert_eq!("甲子", h.get_sixty_cycle().get_name());

    assert_eq!("己未", h.get_sixty_cycle_hour().get_day().get_name());
    assert_eq!("戊午", h.get_lunar_day().get_sixty_cycle().get_name());
    assert_eq!("农历癸卯年十一月十四", h.get_lunar_day().to_string());

    assert_eq!("甲子", h.get_sixty_cycle_hour().get_month().get_name());
    assert_eq!("农历癸卯年十一月", h.get_lunar_day().get_lunar_month().to_string());
    assert_eq!("甲子", h.get_lunar_day().get_lunar_month().get_sixty_cycle().get_name());

    assert_eq!("癸卯", h.get_sixty_cycle_hour().get_year().get_name());
    assert_eq!("农历癸卯年", h.get_lunar_day().get_lunar_month().get_lunar_year().to_string());
    assert_eq!("癸卯", h.get_lunar_day().get_lunar_month().get_lunar_year().get_sixty_cycle().get_name());
  }

  #[test]
  fn test35() {
    let h: LunarHour = LunarHour::from_ymd_hms(2023, 11, 14, 6, 0, 0);
    assert_eq!("乙卯", h.get_sixty_cycle().get_name());

    assert_eq!("戊午", h.get_sixty_cycle_hour().get_day().get_name());
    assert_eq!("戊午", h.get_lunar_day().get_sixty_cycle().get_name());
    assert_eq!("农历癸卯年十一月十四", h.get_lunar_day().to_string());

    assert_eq!("甲子", h.get_sixty_cycle_hour().get_month().get_name());
    assert_eq!("农历癸卯年十一月", h.get_lunar_day().get_lunar_month().to_string());
    assert_eq!("甲子", h.get_lunar_day().get_lunar_month().get_sixty_cycle().get_name());

    assert_eq!("癸卯", h.get_sixty_cycle_hour().get_year().get_name());
    assert_eq!("农历癸卯年", h.get_lunar_day().get_lunar_month().get_lunar_year().to_string());
    assert_eq!("癸卯", h.get_lunar_day().get_lunar_month().get_lunar_year().get_sixty_cycle().get_name());
  }

  #[test]
  fn test36() {
    assert_eq!("七月", LunarMonth::from_ym(2359, 7).get_name());
  }

  /**
   * 闰月
   */
  #[test]
  fn test37() {
    assert_eq!("闰七月", LunarMonth::from_ym(2359, -7).get_name());
  }

  #[test]
  fn test38() {
    assert_eq!(29, LunarMonth::from_ym(2023, 6).get_day_count());
  }

  #[test]
  fn test39() {
    assert_eq!(30, LunarMonth::from_ym(2023, 7).get_day_count());
  }

  #[test]
  fn test40() {
    assert_eq!(30, LunarMonth::from_ym(2023, 8).get_day_count());
  }

  #[test]
  fn test41() {
    assert_eq!(29, LunarMonth::from_ym(2023, 9).get_day_count());
  }

  #[test]
  fn test42() {
    assert_eq!("2023年10月15日", LunarMonth::from_ym(2023, 9).get_first_julian_day().get_solar_day().to_string());
  }

  #[test]
  fn test43() {
    assert_eq!("甲寅", LunarMonth::from_ym(2023, 1).get_sixty_cycle().get_name());
  }

  #[test]
  fn test44() {
    assert_eq!("乙卯", LunarMonth::from_ym(2023, -2).get_sixty_cycle().get_name());
  }

  #[test]
  fn test45() {
    assert_eq!("丙辰", LunarMonth::from_ym(2023, 3).get_sixty_cycle().get_name());
  }

  #[test]
  fn test46() {
    assert_eq!("丙寅", LunarMonth::from_ym(2024, 1).get_sixty_cycle().get_name());
  }

  #[test]
  fn test47() {
    assert_eq!("乙丑", LunarMonth::from_ym(2023, 12).get_sixty_cycle().get_name());
  }

  #[test]
  fn test48() {
    assert_eq!("壬寅", LunarMonth::from_ym(2022, 1).get_sixty_cycle().get_name());
  }

  #[test]
  fn test49() {
    assert_eq!("闰十二月", LunarMonth::from_ym(37, -12).get_name());
  }

  #[test]
  fn test50() {
    assert_eq!("闰十二月", LunarMonth::from_ym(5552, -12).get_name());
  }

  #[test]
  fn test51() {
    assert_eq!("农历戊子年十二月", LunarMonth::from_ym(2008, 11).next(1).to_string());
  }

  #[test]
  fn test52() {
    assert_eq!("农历己丑年正月", LunarMonth::from_ym(2008, 11).next(2).to_string());
  }

  #[test]
  fn test53() {
    assert_eq!("农历己丑年五月", LunarMonth::from_ym(2008, 11).next(6).to_string());
  }

  #[test]
  fn test54() {
    assert_eq!("农历己丑年闰五月", LunarMonth::from_ym(2008, 11).next(7).to_string());
  }

  #[test]
  fn test55() {
    assert_eq!("农历己丑年六月", LunarMonth::from_ym(2008, 11).next(8).to_string());
  }

  #[test]
  fn test56() {
    assert_eq!("农历庚寅年正月", LunarMonth::from_ym(2008, 11).next(15).to_string());
  }

  #[test]
  fn test57() {
    assert_eq!("农历戊子年十一月", LunarMonth::from_ym(2008, 12).next(-1).to_string());
  }

  #[test]
  fn test58() {
    assert_eq!("农历戊子年十一月", LunarMonth::from_ym(2009, 1).next(-2).to_string());
  }

  #[test]
  fn test59() {
    assert_eq!("农历戊子年十一月", LunarMonth::from_ym(2009, 5).next(-6).to_string());
  }

  #[test]
  fn test60() {
    assert_eq!("农历戊子年十一月", LunarMonth::from_ym(2009, -5).next(-7).to_string());
  }

  #[test]
  fn test61() {
    assert_eq!("农历戊子年十一月", LunarMonth::from_ym(2009, 6).next(-8).to_string());
  }

  #[test]
  fn test62() {
    assert_eq!("农历戊子年十一月", LunarMonth::from_ym(2010, 1).next(-15).to_string());
  }

  #[test]
  fn test63() {
    assert_eq!(29, LunarMonth::from_ym(2012, -4).get_day_count());
  }

  #[test]
  fn test64() {
    assert_eq!("壬戌", LunarMonth::from_ym(2023, 9).get_sixty_cycle().to_string());
  }

  #[test]
  fn test65() {
    let d: LunarDay = SolarDay::from_ymd(2023, 10, 7).get_lunar_day();
    assert_eq!("辛酉", d.get_lunar_month().get_sixty_cycle().to_string());
    assert_eq!("辛酉", d.get_sixty_cycle_day().get_month().to_string());
  }

  #[test]
  fn test66() {
    let d: LunarDay = SolarDay::from_ymd(2023, 10, 8).get_lunar_day();
    assert_eq!("辛酉", d.get_lunar_month().get_sixty_cycle().to_string());
    assert_eq!("壬戌", d.get_sixty_cycle_day().get_month().to_string());
  }

  #[test]
  fn test67() {
    let d: LunarDay = SolarDay::from_ymd(2023, 10, 15).get_lunar_day();
    assert_eq!("九月", d.get_lunar_month().get_name());
    assert_eq!("壬戌", d.get_lunar_month().get_sixty_cycle().to_string());
    assert_eq!("壬戌", d.get_sixty_cycle_day().get_month().to_string());
  }

  #[test]
  fn test68() {
    let d: LunarDay = SolarDay::from_ymd(2023, 11, 7).get_lunar_day();
    assert_eq!("壬戌", d.get_lunar_month().get_sixty_cycle().to_string());
    assert_eq!("壬戌", d.get_sixty_cycle_day().get_month().to_string());
  }

  #[test]
  fn test69() {
    let d: LunarDay = SolarDay::from_ymd(2023, 11, 8).get_lunar_day();
    assert_eq!("壬戌", d.get_lunar_month().get_sixty_cycle().to_string());
    assert_eq!("癸亥", d.get_sixty_cycle_day().get_month().to_string());
  }

  #[test]
  fn test70() {
    // 2023年闰2月
    let m: LunarMonth = LunarMonth::from_ym(2023, 12);
    assert_eq!("农历癸卯年十二月", m.to_string());
    assert_eq!("农历癸卯年十一月", m.next(-1).to_string());
    assert_eq!("农历癸卯年十月", m.next(-2).to_string());
  }

  #[test]
  fn test71() {
    // 2023年闰2月
    let m: LunarMonth = LunarMonth::from_ym(2023, 3);
    assert_eq!("农历癸卯年三月", m.to_string());
    assert_eq!("农历癸卯年闰二月", m.next(-1).to_string());
    assert_eq!("农历癸卯年二月", m.next(-2).to_string());
    assert_eq!("农历癸卯年正月", m.next(-3).to_string());
    assert_eq!("农历壬寅年十二月", m.next(-4).to_string());
    assert_eq!("农历壬寅年十一月", m.next(-5).to_string());
  }

  #[test]
  fn test72() {
    let d: LunarDay = SolarDay::from_ymd(1983, 2, 15).get_lunar_day();
    assert_eq!("甲寅", d.get_lunar_month().get_sixty_cycle().to_string());
    assert_eq!("甲寅", d.get_sixty_cycle_day().get_month().to_string());
  }

  #[test]
  fn test73() {
    let d: LunarDay = SolarDay::from_ymd(2023, 10, 30).get_lunar_day();
    assert_eq!("壬戌", d.get_lunar_month().get_sixty_cycle().to_string());
    assert_eq!("壬戌", d.get_sixty_cycle_day().get_month().to_string());
  }

  #[test]
  fn test74() {
    let d: LunarDay = SolarDay::from_ymd(2023, 10, 19).get_lunar_day();
    assert_eq!("壬戌", d.get_lunar_month().get_sixty_cycle().to_string());
    assert_eq!("壬戌", d.get_sixty_cycle_day().get_month().to_string());
  }

  #[test]
  fn test75() {
    let m: LunarMonth = LunarMonth::from_ym(2023, 11);
    assert_eq!("农历癸卯年十一月", m.to_string());
    assert_eq!("甲子", m.get_sixty_cycle().to_string());
  }

  #[test]
  fn test76() {
    assert_eq!("庚申", LunarDay::from_ymd(2018, 6, 26).get_sixty_cycle_day().get_month().to_string());
  }

  #[test]
  fn test77() {
    assert_eq!("辛丑", LunarMonth::from_ym(1991, 12).get_sixty_cycle().to_string());
  }

  #[test]
  fn test78() {
    assert_eq!("农历癸卯年", LunarYear::from_year(2023).get_name());
  }

  #[test]
  fn test79() {
    assert_eq!("农历戊申年", LunarYear::from_year(2023).next(5).get_name());
  }

  #[test]
  fn test80() {
    assert_eq!("农历戊戌年", LunarYear::from_year(2023).next(-5).get_name());
  }

  /**
   * 农历年的干支
   */
  #[test]
  fn test81() {
    assert_eq!("庚子", LunarYear::from_year(2020).get_sixty_cycle().get_name());
  }

  /**
   * 农历年的生肖(农历年.干支.地支.生肖)
   */
  #[test]
  fn test82() {
    assert_eq!("虎", LunarYear::from_year(1986).get_sixty_cycle().get_earth_branch().get_zodiac().get_name());
  }

  #[test]
  fn test83() {
    assert_eq!(12, LunarYear::from_year(151).get_leap_month());
  }

  #[test]
  fn test84() {
    assert_eq!(1, LunarYear::from_year(2357).get_leap_month());
  }

  #[test]
  fn test85() {
    let y: LunarYear = LunarYear::from_year(2023);
    assert_eq!("癸卯", y.get_sixty_cycle().get_name());
    assert_eq!("兔", y.get_sixty_cycle().get_earth_branch().get_zodiac().get_name());
  }

  #[test]
  fn test86() {
    assert_eq!("上元", LunarYear::from_year(1864).get_twenty().get_sixty().get_name());
  }

  #[test]
  fn test87() {
    assert_eq!("上元", LunarYear::from_year(1923).get_twenty().get_sixty().get_name());
  }

  #[test]
  fn test88() {
    assert_eq!("中元", LunarYear::from_year(1924).get_twenty().get_sixty().get_name());
  }

  #[test]
  fn test89() {
    assert_eq!("中元", LunarYear::from_year(1983).get_twenty().get_sixty().get_name());
  }

  #[test]
  fn test90() {
    assert_eq!("下元", LunarYear::from_year(1984).get_twenty().get_sixty().get_name());
  }

  #[test]
  fn test91() {
    assert_eq!("下元", LunarYear::from_year(2043).get_twenty().get_sixty().get_name());
  }

  #[test]
  fn test92() {
    assert_eq!("一运", LunarYear::from_year(1864).get_twenty().get_name());
  }

  #[test]
  fn test93() {
    assert_eq!("一运", LunarYear::from_year(1883).get_twenty().get_name());
  }

  #[test]
  fn test94() {
    assert_eq!("二运", LunarYear::from_year(1884).get_twenty().get_name());
  }

  #[test]
  fn test95() {
    assert_eq!("二运", LunarYear::from_year(1903).get_twenty().get_name());
  }

  #[test]
  fn test96() {
    assert_eq!("三运", LunarYear::from_year(1904).get_twenty().get_name());
  }

  #[test]
  fn test97() {
    assert_eq!("三运", LunarYear::from_year(1923).get_twenty().get_name());
  }

  #[test]
  fn test98() {
    assert_eq!("八运", LunarYear::from_year(2004).get_twenty().get_name());
  }

  #[test]
  fn test99() {
    let year: LunarYear = LunarYear::from_year(1);
    assert_eq!("六运", year.get_twenty().get_name());
    assert_eq!("中元", year.get_twenty().get_sixty().get_name());
  }

  #[test]
  fn test100() {
    let year: LunarYear = LunarYear::from_year(1863);
    assert_eq!("九运", year.get_twenty().get_name());
    assert_eq!("下元", year.get_twenty().get_sixty().get_name());
  }
}
