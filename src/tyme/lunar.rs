use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use lazy_static::lazy_static;
use crate::tyme::culture::{Direction, Duty, Phase, Twenty, Week};
use crate::tyme::sixtycycle::{EarthBranch, HeavenStem, SixtyCycle};
use crate::tyme::{AbstractCulture, AbstractTyme, Culture, LoopTyme, Tyme};
use crate::tyme::culture::eightchar::EightChar;
use crate::tyme::culture::fetus::FetusDay;
use crate::tyme::culture::star::nine::NineStar;
use crate::tyme::culture::star::six::SixStar;
use crate::tyme::culture::star::twelve::TwelveStar;
use crate::tyme::culture::star::twenty_eight::TwentyEightStar;
use crate::tyme::festival::LunarFestival;
use crate::tyme::jd::{J2000, JulianDay};
use crate::tyme::solar::{SolarDay, SolarMonth, SolarTerm, SolarTime};
use crate::tyme::util::ShouXingUtil;

lazy_static! {
  static ref LEAP_MONTH_YEAR: HashMap<usize, Vec<isize>> = {
    let mut map: HashMap<usize, Vec<isize>> = HashMap::new();
    let chars: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_@";
    let months :Vec<&str> = "080b0r0j0j0j0C0j0j0C0j0j0j0C0j0C0j0C0F0j0V0V0V0u0j0j0C0j0j0j0j0V0C0j1v0u0C0V1v0C0b080u110u0C0j0C1v9K1v2z0j1vmZbl1veN3s1v0V0C2S1v0V0C2S2o0C0j1Z1c2S1v0j1c0j2z1v0j1c0j392H0b2_2S0C0V0j1c0j2z0C0C0j0j1c0j0N250j0C0j0b081n080b0C0C0C1c0j0N,0r1v1c1v0V0V0F0V0j0C0j0C0j0V0j0u1O0j0C0V0j0j0j0V0b080u0r0u080b0j0j0C0V0C0V0j0b080V0u080b0j0j0u0j1v0u080b1c0j080b0j0V0j0j0V0C0N1v0j1c0j0j1v2g1v420j1c0j2z1v0j1v5Q9z1v4l0j1vfn1v420j9z4l1v1v2S1c0j1v2S3s1v0V0C2S1v1v2S1c0j1v2S2_0b0j2_2z0j1c0j,0z0j0j0j0C0j0j0C0j0j0j0C0j0C0j0j0j0j0m0j0C0j0j0C0j0j0j0j0b0V0j0j0C0j0j0j0j0V0j0j0j0V0b0V0V0C0V0C0j0j0b080u110u0V0C0j0N0j0b080b080b0j0r0b0r0b0j0j0j0j0C0j0b0r0C0j0b0j0C0C0j0j0j0j0j0j0j0j0j0b110j0b0j0j0j0C0j0C0j0j0j0j0b080b080b0V080b080b0j0j0j0j0j0j0V0j0j0u1v0j0j0j0C0j0j0j0V0C0N1c0j0C0C0j0j0j1n080b0j0V0C0j0C0C2g0j1c0j0j1v2g1v0j0j1v7N0j1c0j3L0j0j1v5Q1Z5Q1v4lfn1v420j1v5Q1Z5Q1v4l1v2z1v,0H140r0N0r140r0u0r0V171c11140C0j0u110j0u0j1v0j0C0j0j0j0b080V0u080b0C1v0j0j0j0C0j0b080V0j0j0b080b0j0j0j0j0b080b0C080j0b080b0j0j0j0j0j0j0b080j0b080C0b080b080b080b0j0j0j0j080b0j0C0j0j0j0b0j0j080C0b0j0j0j0j0j0j0b08080b0j0C0j0j0j0b0j0j0K0b0j0C0j0j0j0b080b080j0C0b0j080b080b0j0j0j0j080b0j0b0r0j0j0j0b0j0C0r0b0j0j0j0j0j0j0j0b080j0b0r0C0j0b0j0j0j0r0b0j0C0j0j0j0u0r0b0C0j080b0j0j0j0j0j0j0j1c0j0b0j0j0j0C0j0j0j0j0j0j0j0b080j1c0u0j0j0j0C0j1c0j0u0j1c0j0j0j0j0j0j0j0j1c0j0u1v0j0j0V0j0j2g0j0j0j0C1v0C1G0j0j0V0C1Z1O0j0V0j0j2g1v0j0j0V0C2g5x1v4l1v421O7N0V0C4l1v2S1c0j1v2S2_,050b080C0j0j0j0C0j0j0C0j0j0j0C0j0C0j0C030j0j0j0j0j0j0j0j0j0C0j0b080u0V080b0j0j0V0j0j0j0j0j0j0j0j0j0V0N0j0C0C0j0j0j0j0j0j0j0j1c0j0u0j1v0j0j0j0j0j0b080b080j0j0j0b080b080b080b080b0j0j0j080b0j0b080j0j0j0j0b080b0j0j0r0b080b0b080j0j0j0j0b080b080j0b080j0b080b080b080b080b0j0j0r0b0j0b080j0j0j0j0b080b0j0j0C080b0b080j0j0j0j0j0j0j0b080u080j0j0b0j0j0j0C0j0b080j0j0j0j0b080b080b080b0C080b080b080b0j0j0j0j0j0j0b0C080j0j0b0j0j0j0C0j0b080j0j0C0b080b080j0b0j0j0C080b0j0j0j0j0j0j0b0j0j080C0b0j080b0j0j0j0j0j0j0j0C0j0j0j0b0j0j0C080b0j0j0j0j0j0j0b080b080b0K0b080b080b0j0j0j0j0j0j0j0C0j0j0u0j0j0V0j080b0j0C0j0j0j0b0j0r0C0b0j0j0j0j0j0j0j0j0j0C0j0b080b080b0j0C0C0j0C0j0j0j0u110u0j0j0j0j0j0j0j0j0C0j0j0u0j1c0j0j0j0j0j0j0j0j0V0C0u0j0C0C0V0C1Z0j0j0j0C0j0j0j1v0u0j1c0j0j0j0C0j0j2g0j1c1v0C1Z0V0j4l0j0V0j0j2g0j1v0j1v2S1c7N1v,0w0j1c0j0V0j0j0V0V0V0j0m0V0j0C1c140j0j0j0C0V0C0j1v0j0N0j0C0j0j0j0V0j0j1v0N0j0j0V0j0j0j0j0j0j080b0j0j0j0j0j0j0j080b0j0C0j0j0j0b0j0j080u080b0j0j0j0j0j0j0b080b080b080C0b0j080b080b0j0j0j0j080b0j0C0j0j0j0b0j0j080u080b0j0j0j0j0j0j0b080b080b080b0r0b0j080b080b0j0j0j0j080b0j0b0r0j0j0b080b0j0j080b0j080b0j080b080b0j0j0j0j0j0b080b0r0C0b080b0j0j0j0j080b0b080b080j0j0j0b080b080b080b0j0j0j0j080b0j0b080j0j0j0j0b080b0j0j0r0b080b0j0j0j0j0j0b080b080j0b0r0b080j0b080b0j0j0j0j080b0j0b080j0j0j0j0b080b0j080b0r0b0j080b080b0j0j0j0j0j0b080b0r0C0b080b0j0j0j0j0j0j0b080j0j0j0b080b080b080b0j0j0j0r0b0j0b080j0j0j0j0b080b0r0b0r0b0j080b080b0j0j0j0j0j0j0b0r0j0j0j0b0j0j0j0j080b0j0b080j0j0j0j0b080b080b0j0r0b0j080b0j0j0j0j0j0j0j0b0r0C0b0j0j0j0j0j0j0j080b0j0C0j0j0j0b0j0C0r0b0j0j0j0j0j0j0b080b080u0r0b0j080b0j0j0j0j0j0j0j0b0r0C0u0j0j0j0C0j080b0j0C0j0j0j0u110b0j0j0j0j0j0j0j0j0j0C0j0b080b0j0j0C0C0j0C0j0j0j0b0j1c0j080b0j0j0j0j0j0j0V0j0j0u0j1c0j0j0j0C0j0j2g0j0j0j0C0j0j0V0j0b080b1c0C0V0j0j2g0j0j0V0j0j1c0j1Z0j0j0C0C0j1v,160j0j0V0j1c0j0C0j0C0j1f0j0V0C0j0j0C0j0j0j1G080b080u0V080b0j0j0V0j1v0j0u0j1c0j0j0j0C0j0j0j0C0C0j1D0b0j080b0j0j0j0j0C0j0b0r0C0j0b0j0C0C0j0j0j0j0j0j0j0j0j0b0r0b0r0j0b0j0j0j0C0j0b0r0j0j0j0b080b080j0b0C0j080b080b0j0j0j0j0j0j0b0C080j0j0b0j0j0j0C0j0b080j0j0j0j0b080b080j0b0C0r0j0b0j0j0j0j0j0j0b0C080j0j0b0j0j0j0C0j0j0j0j0C0j0j0b080b0j0j0C080b0j0j0j0j0j0j0b080b080b080C0b080b080b080b0j0j0j0j0j0b080C0j0j0b080b0j0j0C080b0j0j0j0j0j0j0b080j0b0C080j0j0b0j0j0j0j0j0j0b080j0b080C0b080b080b080b0j0j0j0j080b0j0C0j0j0b080b0j0j0C080b0j0j0j0j0j0j0b080j0b080u080j0j0b0j0j0j0j0j0j0b080C0j0j0b080b0j0j0C0j0j080b0j0j0j0j0j0b080b0C0r0b080b0j0j0j0j0j0j0b080j0b080u080b080b080b0j0j0j0C0j0b080j0j0j0j0b0j0j0j0C0j0j080b0j0j0j0j0j0b080b0C0r0b080b0j0j0j0j0j0j0b080j0b0r0b080b080b080b0j0j0j0r0b0j0b0r0j0j0j0b0j0j0j0r0b0j080b0j0j0j0j0j0j0j0b0r0C0b0j0j0j0j0j0j0j0b080j0C0u080b080b0j0j0j0r0b0j0C0C0j0b0j110b0j080b0j0j0j0j0j0j0u0r0C0b0j0j0j0j0j0j0j0j0j0C0j0j0j0b0j1c0j0C0j0j0j0b0j0814080b080b0j0j0j0j0j0j1c0j0u0j0j0V0j0j0j0j0j0j0j0u110u0j0j0j,020b0r0C0j0j0j0C0j0j0V0j0j0j0j0j0C0j1f0j0C0j0V1G0j0j0j0j0V0C0j0C1v0u0j0j0j0V0j0j0C0j0j0j1v0N0C0V0j0j0j0K0C250b0C0V0j0j0V0j0j2g0C0V0j0j0C0j0j0b081v0N0j0j0V0V0j0j0u0j1c0j080b0j0j0j0j0j0j0V0j0j0u0j0j0V0j0j0j0C0j0b080b080V0b0j080b0j0j0j0j0j0j0j0b0r0C0j0b0j0j0j0C0j080b0j0j0j0j0j0j0u0r0C0u0j0j0j0j0j0j0b080j0C0j0b080b080b0j0C0j080b0j0j0j0j0j0j0b080b110b0j0j0j0j0j0j0j0j0j0b0r0j0j0j0b0j0j0j0r0b0j0b080j0j0j0j0b080b080b080b0r0b0j080b080b0j0j0j0j0j0j0b0r0C0b080b0j0j0j0j080b0j0b080j0j0j0j0b080b080b0j0j0j0r0b0j0j0j0j0j0j0b080b0j080C0b0j080b080b0j0j0j0j080b0j0b0r0C0b080b0j0j0j0j080b0j0j0j0j0j0b080b080b080b0j0j080b0r0b0j0j0j0j0j0j0b0j0j080C0b0j080b080b0j0j0j0j0j0b080C0j0j0b080b0j0j0C0j0b080j0j0j0j0b080b080b080b0C0C080b0j0j0j0j0j0j0b0C0C080b080b080b0j0j0j0j0j0j0b0C080j0j0b0j0j0j0C0j0b080j0b080j0j0b080b080b080b0C0r0b0j0j0j0j0j0j0b080b0r0b0r0b0j080b080b0j0j0j0j0j0j0b0r0C0j0b0j0j0j0j0j0j0b080j0C0j0b080j0b0j0j0K0b0j0C0j0j0j0b080b0j0K0b0j080b0j0j0j0j0j0j0V0j0j0b0j0j0j0C0j0j0j0j,0l0C0K0N0r0N0j0r1G0V0m0j0V1c0C0j0j0j0j1O0N110u0j0j0j0C0j0j0V0C0j0u110u0j0j0j0C0j0j0j0C0C0j250j1c2S1v1v0j5x2g0j1c0j0j1c2z0j1c0j0j1c0j0N1v0V0C1v0C0b0C0V0j0j0C0j0C1v0u0j0C0C0j0j0j0C0j0j0j0u110u0j0j0j0C0j0C0C0C0b080b0j0C0j080b0j0C0j0j0j0u110u0j0j0j0C0j0j0j0C0j0j0j0u0C0r0u0j0j0j0j0j0j0b0r0b0V080b080b0j0C0j0j0j0V0j0j0b0j0j0j0C0j0j0j0j0j0j0j0b080j0b0C0r0j0b0j0j0j0C0j0b0r0b0r0j0b080b080b0j0C0j0j0j0j0j0j0j0j0b0j0C0r0b0j0j0j0j0j0j0b080b080j0b0r0b0r0j0b0j0j0j0j080b0j0b0r0j0j0j0b080b080b0j0j0j0j080b0j0j0j0j0j0j0b0j0j0j0r0b0j0j0j0j0j0j0b080b080b080b0r0C0b080b0j0j0j0j0j0b080b0r0C0b080b080b080b0j0j0j0j080b0j0C0j0j0j0b0j0j0C080b0j0j0j0j0j0j0b080j0b0C080j0j0b0j0j0j0j0j0j0b0r0b080j0j0b080b080b0j0j0j0j0j0j0b080j0j0j0j0b0j0j0j0r0b0j0b080j0j0j0j0j0b080b080b0C0r0b0j0j0j0j0j0j0b080b080j0C0b0j080b080b0j0j0j0j0j0j,0a0j0j0j0j0C0j0j0C0j0C0C0j0j0j0j0j0j0j0m0C0j0j0j0j0u080j0j0j1n0j0j0j0j0C0j0j0j0V0j0j0j1c0u0j0C0V0j0j0V0j0j1v0N0C0V2o1v1O2S2o141v0j1v4l0j1c0j1v2S2o0C0u1v0j0C0C2S1v0j1c0j0j1v0N251c0j1v0b1c1v1n1v0j0j0V0j0j1v0N1v0C0V0j0j1v0b0C0j0j0V1c0j0u0j1c0j0j0j0j0j0j0j0j1c0j0u0j0j0V0j0j0j0j0j0j0b080u110u0j0j0j0j0j0j1c0j0b0j080b0j0C0j0j0j0V0j0j0u0C0V0j0j0j0C0j0b080j1c0j0b0j0j0j0C0j0C0j0j0j0b080b080b0j0C0j080b0j0j0j0j0j0j0j0b0C0r0u0j0j0j0j0j0j0b080j0b0r0C0j0b0j0j0j0r0b0j0b0r0j0j0j0b080b080b0j0r0b0j080b0j0j0j0j0j0j0b0j0r0C0b0j0j0j0j0j0j0b080j0j0C0j0j0b080b0j0j0j0j0j0j0j0j0j0j0b080b080b080b0C0j0j080b0j0j0j0j0j0j0b0j0j0C080b0j0j0j0j0j0j0j0j0b0C080j0j0b0j0j0j0j0j,0n0Q0j1c14010q0V1c171k0u0r140V0j0j1c0C0N1O0j0V0j0j0j1c0j0u110u0C0j0C0V0C0j0j0b671v0j1v5Q1O2S2o2S1v4l1v0j1v2S2o0C1Z0j0C0C1O141v0j1c0j2z1O0j0V0j0j1v0b2H390j1c0j0V0C2z0j1c0j1v2g0C0V0j1O0b0j0j0V0C1c0j0u0j1c0j0j0j0j0j0j0j0j1c0N0j0j0V0j0j0C0j0j0b081v0u0j0j0j0C0j1c0N0j0j0C0j0j0j0C0j0j0j0u0C0r0u0j0j0j0C0j0b080j1c0j0b0j0C0C0j0C0C0j0b080b080u0C0j080b0j0C0j0j0j0u110u0j0j0j0j0j0j0j0j0C0C0j0b0j0j0j0C0j0C0C0j0b080b080b0j0C0j080b0j0C0j0j0j0b0j110b0j0j0j0j0j,0B0j0V0j0j0C0j0j0j0C0j0C0j0j0C0j0m0j0j0j0j0C0j0C0j0j0u0j1c0j0j0C0C0j0j0j0j0j0j0j0j0u110N0j0j0V0C0V0j0b081n080b0CrU1O5e2SbX2_1Z0V2o141v0j0C0C0j2z1v0j1c0j7N1O420j1c0j1v2S1c0j1v2S2_0b0j0V0j0j1v0N1v0j0j1c0j1v140j0V0j0j0C0C0b080u1v0C0V0u110u0j0j0j0C0j0j0j0C0C0N0C0V0j0j0C0j0j0b080u110u0C0j0C0u0r0C0u080b0j0j0C0j0j0j".split(",").collect();
    for i in 0..12 {
      let mut n: isize = 0;
      let m: &str = months[i];
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
      map.insert(i + 1, l);
    }
    map
  };
}

/// 农历年
#[derive(Debug, Copy, Clone)]
pub struct LunarYear {
  /// 年
  year: isize,
}

impl Tyme for LunarYear {
  fn next(&self, n: isize) -> Result<Self, String> {
    Self::from_year(self.year + n)
  }
}

impl Culture for LunarYear {
  fn get_name(&self) -> String {
    format!("农历{}年", self.get_sixty_cycle())
  }
}

impl LunarYear {
  pub fn from_year(year: isize) -> Result<Self, String> {
    if year < -1 || year > 9999 {
      Err(format!("illegal lunar year: {}", year))
    } else {
      Ok(Self {
        year
      })
    }
  }

  pub fn get_year(&self) -> isize {
    self.year
  }

  pub fn get_day_count(&self) -> usize {
    let mut n: usize = 0;
    for m in self.get_months() {
      n += m.get_day_count();
    }
    n
  }

  pub fn get_months(&self) -> Vec<LunarMonth> {
    let mut l: Vec<LunarMonth> = Vec::new();
    let mut m: LunarMonth = LunarMonth::from_ym(self.year, 1).unwrap();
    while m.get_year().get_year() == self.year {
      l.push(m);
      m = m.next(1).unwrap();
    }
    l
  }

  pub fn get_leap_month(&self) -> usize {
    if self.year == -1 {
      return 11;
    }
    for (key, value) in LEAP_MONTH_YEAR.clone().into_iter() {
      if value.contains(&self.year) {
        return key;
      }
    }
    return 0;
  }

  pub fn get_sixty_cycle(&self) -> SixtyCycle {
    SixtyCycle::from_index(self.year - 4)
  }

  pub fn get_twenty(&self) -> Twenty {
    Twenty::from_index(((self.year as f64 - 1864.0) / 20.0).floor() as isize)
  }

  pub fn get_jupiter_direction(&self) -> Direction {
    Direction::from_index([0, 7, 7, 2, 3, 3, 8, 1, 1, 6, 0, 0][self.get_sixty_cycle().get_earth_branch().get_index()])
  }

  pub fn get_nine_star(&self) -> NineStar {
    NineStar::from_index(63 + self.get_twenty().get_sixty().get_index() as isize * 3 - self.get_sixty_cycle().get_index() as isize)
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
  /// 农历年
  year: LunarYear,
  month: usize,
  leap: bool,
  day_count: usize,
  index_in_year: usize,
  first_julian_day: JulianDay,
}

impl Tyme for LunarMonth {
  fn next(&self, n: isize) -> Result<Self, String> {
    if n == 0 {
      return Self::from_ym(self.year.get_year(), self.get_month_with_leap());
    }
    let mut m: isize = self.index_in_year as isize + 1 + n;
    let mut y: LunarYear = self.year;
    let mut leap_month: usize = y.get_leap_month();
    let mut month_size: isize = if leap_month > 0 { 13 } else { 12 };
    let forward: bool = n > 0;
    let add: isize = if forward { 1 } else { -1 };
    while if forward { m > month_size } else { m <= 0 } {
      if forward {
        m -= month_size;
      }
      y = y.next(add).unwrap();
      leap_month = y.get_leap_month();
      month_size = if leap_month > 0 { 13 } else { 12 };
      if !forward {
        m += month_size;
      }
    }
    let mut leap: bool = false;
    if leap_month > 0 {
      if m == leap_month as isize + 1 {
        leap = true;
      }
      if m > leap_month as isize {
        m -= 1;
      }
    }
    let month: isize = if leap { -m } else { m };
    return Self::from_ym(y.get_year(), month);
  }
}

impl Culture for LunarMonth {
  fn get_name(&self) -> String {
    let leap: &str = if self.leap { "闰" } else { "" };
    format!("{}{}", leap, LUNAR_MONTH_NAMES[self.month - 1])
  }
}

impl LunarMonth {
  pub fn from_ym(year: isize, month: isize) -> Result<Self, String> {
    let current_year: LunarYear = LunarYear::from_year(year).unwrap();
    let current_leap_month: usize = current_year.get_leap_month();
    if month == 0 || month > 12 || month < -12 {
      return Err(format!("illegal lunar month: {}", month));
    }
    let leap: bool = month < 0;
    let m: usize = month.abs() as usize;
    if leap && m != current_leap_month {
      return Err(format!("illegal leap month {} in lunar year {}", m, year));
    }

    // 冬至
    let dong_zhi: SolarTerm = SolarTerm::from_index(year, 0);
    let dong_zhi_jd: f64 = dong_zhi.get_cursory_julian_day();

    // 冬至前的初一，今年首朔的日月黄经差
    let mut w: f64 = ShouXingUtil::calc_shuo(dong_zhi_jd);
    if w > dong_zhi_jd {
      w -= 29.53;
    }

    // 计算正月初一的偏移
    let prev_year: LunarYear = LunarYear::from_year(year - 1).unwrap();
    let prev_leap_month: usize = prev_year.get_leap_month();

    // 正常情况正月初一为第3个朔日，但有些特殊的
    let mut offset: f64 = 2.0;
    if year > 8 && year < 24 {
      offset = 1.0;
    } else if prev_leap_month > 10 && year != 239 && year != 240 {
      offset = 3.0;
    }

    // 位于当年的索引
    let mut index: usize = m - 1;
    if leap || (current_leap_month > 0 && m > current_leap_month) {
      index += 1;
    }
    let index_in_year: usize = index;

    // 本月初一
    w += 29.5306 * (offset + index as f64);
    let first_day: f64 = ShouXingUtil::calc_shuo(w);
    let first_julian_day: JulianDay = JulianDay::from_julian_day(J2000 + first_day).unwrap();
    // 本月天数 = 下月初一 - 本月初一
    let day_count: usize = (ShouXingUtil::calc_shuo(w + 29.5306) - first_day) as usize;

    Ok(Self {
      year: current_year,
      month: m,
      leap,
      day_count,
      index_in_year,
      first_julian_day,
    })
  }

  pub fn get_year(&self) -> LunarYear {
    self.year
  }

  pub fn get_month(&self) -> usize {
    self.month
  }

  pub fn get_month_with_leap(&self) -> isize {
    match self.leap {
      false => self.month as isize,
      _ => -(self.month as isize)
    }
  }

  pub fn get_day_count(&self) -> usize {
    self.day_count
  }

  pub fn get_index_in_year(&self) -> usize {
    self.index_in_year
  }

  pub fn get_season(&self) -> LunarSeason {
    LunarSeason::from_index(self.month as isize - 1)
  }

  pub fn get_first_julian_day(&self) -> JulianDay {
    self.first_julian_day
  }

  pub fn is_leap(&self) -> bool {
    self.leap
  }

  pub fn get_week_count(&self, start: usize) -> usize {
    ((AbstractCulture::new().index_of((self.first_julian_day.get_week().get_index() as isize) - (start as isize), 7) + self.get_day_count()) as f64 / 7.0).ceil() as usize
  }

  pub fn get_days(&self) -> Vec<LunarDay> {
    let mut l: Vec<LunarDay> = Vec::new();
    let size: usize = self.get_day_count();
    let y: isize = self.year.get_year();
    let m: isize = self.get_month_with_leap();
    for i in 0..size {
      l.push(LunarDay::from_ymd(y, m, i + 1).unwrap());
    }
    l
  }

  pub fn get_weeks(&self, start: usize) -> Vec<LunarWeek> {
    let mut l: Vec<LunarWeek> = Vec::new();
    let size: usize = self.get_week_count(start);
    let y: isize = self.year.get_year();
    let m: isize = self.get_month_with_leap();
    for i in 0..size {
      l.push(LunarWeek::from_ym(y, m, i, start).unwrap());
    }
    l
  }

  pub fn get_sixty_cycle(&self) -> SixtyCycle {
    SixtyCycle::from_name(format!("{}{}", HeavenStem::from_index((self.year.get_sixty_cycle().get_heaven_stem().get_index() as isize + 1) * 2 + self.index_in_year as isize).get_name(), EarthBranch::from_index(self.index_in_year as isize + 2).get_name()).as_str()).unwrap()
  }

  pub fn get_jupiter_direction(&self) -> Direction {
    let sixty_cycle: SixtyCycle = self.get_sixty_cycle();
    let n: isize = [7, -1, 1, 3][sixty_cycle.get_earth_branch().next(-2).unwrap().get_index() % 4];
    match n {
      -1 => sixty_cycle.get_heaven_stem().get_direction(),
      _ => Direction::from_index(n)
    }
  }

  pub fn get_nine_star(&self) -> NineStar {
    NineStar::from_index(27 - self.year.get_sixty_cycle().get_earth_branch().get_index() as isize % 3 * 3 - self.get_sixty_cycle().get_earth_branch().get_index() as isize)
  }
}

impl Display for LunarMonth {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.year.to_string(), self.get_name())
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

impl Tyme for LunarSeason {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
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

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(LUNAR_SEASON_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for LunarSeason {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for LunarSeason {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
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
  parent: AbstractTyme,
  /// 农历月
  month: LunarMonth,
  /// 索引，0-6
  index: usize,
  /// 起始星期
  start: Week,
}

impl Tyme for LunarWeek {
  fn next(&self, n: isize) -> Result<Self, String> {
    if n == 0 {
      Ok(self.clone())
    } else {
      let mut d: isize = (self.index as isize) + n;
      let mut m: LunarMonth = self.month;
      let start_index: usize = self.start.get_index();
      let mut weeks_in_month: isize = m.get_week_count(start_index) as isize;
      let forward: bool = n > 0;
      let add: isize = if forward { 1 } else { -1 };
      while if forward { d >= weeks_in_month } else { d < 0 } {
        if forward {
          d -= weeks_in_month;
        } else {
          if LunarDay::from_ymd(m.get_year().get_year(), m.get_month_with_leap(), 1).unwrap().get_week() != self.start {
            d += add;
          }
        }
        m = m.next(add).unwrap();
        if forward {
          if LunarDay::from_ymd(m.get_year().get_year(), m.get_month_with_leap(), 1).unwrap().get_week() != self.start {
            d += add;
          }
        }
        weeks_in_month = m.get_week_count(start_index) as isize;
        if !forward {
          d += weeks_in_month;
        }
      }
      Self::from_ym(m.get_year().get_year(), m.get_month_with_leap(), d as usize, start_index)
    }
  }
}

impl Culture for LunarWeek {
  fn get_name(&self) -> String {
    LUNAR_WEEK_NAMES[self.index].to_string()
  }
}

impl LunarWeek {
  pub fn from_ym(year: isize, month: isize, index: usize, start: usize) -> Result<Self, String> {
    if index > 5 {
      Err(format!("illegal lunar week index: {}", index))
    } else if start > 6 {
      Err(format!("illegal lunar week start: {}", start))
    } else {
      let m: LunarMonth = LunarMonth::from_ym(year, month).unwrap();
      if index >= m.get_week_count(start) {
        Err(format!("illegal lunar week index: {} in month: {}", index, m))
      } else {
        Ok(Self {
          parent: AbstractTyme::new(),
          month: m,
          index,
          start: Week::from_index(start as isize),
        })
      }
    }
  }

  pub fn get_month(&self) -> LunarMonth {
    self.month
  }

  pub fn get_index(&self) -> usize {
    self.index
  }

  pub fn get_start(&self) -> Week {
    self.start.clone()
  }

  pub fn get_first_day(&self) -> LunarDay {
    let m: LunarMonth = self.get_month();
    let first_day: LunarDay = LunarDay::from_ymd(m.get_year().get_year(), m.get_month_with_leap(), 1).unwrap();
    let parent: AbstractTyme = self.parent.into();
    let culture: AbstractCulture = parent.into();
    first_day.next(self.index as isize * 7 - culture.index_of((first_day.get_week().get_index() as isize) - (self.start.get_index() as isize), 7) as isize).unwrap()
  }

  pub fn get_days(&self) -> Vec<LunarDay> {
    let mut l: Vec<LunarDay> = Vec::new();
    let d: LunarDay = self.get_first_day();
    l.push(d);
    for i in 1..7 {
      l.push(d.next(i).unwrap());
    }
    l
  }
}

impl Display for LunarWeek {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.month, self.get_name())
  }
}

impl PartialEq for LunarWeek {
  fn eq(&self, other: &Self) -> bool {
    self.get_month() == other.get_month() && self.get_index() == other.get_index() && self.get_start() == other.get_start()
  }
}

impl Eq for LunarWeek {}

impl Into<AbstractTyme> for LunarWeek {
  fn into(self) -> AbstractTyme {
    self.parent
  }
}

/// 农历日名称
pub static LUNAR_DAY_NAMES: [&str; 30] = ["初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八", "初九", "初十", "十一", "十二", "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十", "廿一", "廿二", "廿三", "廿四", "廿五", "廿六", "廿七", "廿八", "廿九", "三十"];

/// 农历日
#[derive(Debug, Copy, Clone)]
pub struct LunarDay {
  /// 农历月
  month: LunarMonth,
  /// 日
  day: usize,
}

impl Tyme for LunarDay {
  fn next(&self, n: isize) -> Result<Self, String> {
    if n == 0 {
      Ok(self.clone())
    } else {
      let mut d: isize = (self.day as isize) + n;
      let mut m: LunarMonth = self.month;
      let mut days_in_month: isize = m.get_day_count() as isize;
      let forward: bool = n > 0;
      let add: isize = if forward { 1 } else { -1 };
      while if forward { d > days_in_month } else { d <= 0 } {
        if forward {
          d -= days_in_month;
        }
        m = m.next(add).unwrap();
        days_in_month = m.get_day_count() as isize;
        if !forward {
          d += days_in_month;
        }
      }
      Self::from_ymd(m.get_year().get_year(), m.get_month_with_leap(), d as usize)
    }
  }
}

impl Culture for LunarDay {
  fn get_name(&self) -> String {
    LUNAR_DAY_NAMES[self.day - 1].to_string()
  }
}

impl LunarDay {
  pub fn from_ymd(year: isize, month: isize, day: usize) -> Result<Self, String> {
    let m: LunarMonth = LunarMonth::from_ym(year, month).unwrap();
    if day < 1 || day > m.get_day_count() {
      Err(format!("illegal day {} in {}", day, m))
    } else {
      Ok(Self {
        month: m,
        day,
      })
    }
  }

  /// 农历月
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::lunar::{LunarDay, LunarMonth};
  ///
  /// // 正月
  /// let lunar_month: LunarMonth = LunarDay::from_ymd(2023, 1, 1).unwrap().get_month();
  /// ```
  pub fn get_month(&self) -> LunarMonth {
    self.month
  }

  /// 日
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::lunar::LunarDay;
  ///
  /// // 1
  /// let day: usize = LunarDay::from_ymd(2023, 1, 1).unwrap().get_day();
  /// ```
  pub fn get_day(&self) -> usize {
    self.day
  }

  pub fn get_week(&self) -> Week {
    self.get_solar_day().get_week()
  }

  pub fn is_before(&self, target: LunarDay) -> bool {
    let b_month: LunarMonth = target.get_month();
    let a_year: isize = self.month.get_year().get_year();
    let b_year: isize = b_month.get_year().get_year();
    if a_year != b_year {
      return a_year < b_year;
    }
    if self.month.get_month() != b_month.get_month() {
      return self.month.get_month() < b_month.get_month();
    }
    if self.month.is_leap() && !b_month.is_leap() {
      return false;
    }
    self.day < target.get_day()
  }

  pub fn is_after(&self, target: LunarDay) -> bool {
    let b_month: LunarMonth = target.get_month();
    let a_year: isize = self.month.get_year().get_year();
    let b_year: isize = b_month.get_year().get_year();
    if a_year != b_year {
      return a_year > b_year;
    }
    if self.month.get_month() != b_month.get_month() {
      return self.month.get_month() > b_month.get_month();
    }
    if self.month.is_leap() && !b_month.is_leap() {
      return true;
    }
    self.day > target.get_day()
  }

  pub fn get_year_sixty_cycle(&self) -> SixtyCycle {
    let solar_day: SolarDay = self.get_solar_day();
    let solar_year: isize = solar_day.get_month().get_year().get_year();
    let spring_solar_day: SolarDay = SolarTerm::from_index(solar_year, 3).get_julian_day().get_solar_day();
    let lunar_year: LunarYear = self.month.get_year();
    let year: isize = lunar_year.get_year();
    let mut sixty_cycle: SixtyCycle = lunar_year.get_sixty_cycle();
    if year == solar_year {
      if solar_day.is_before(spring_solar_day) {
        sixty_cycle = sixty_cycle.next(-1).unwrap();
      }
    } else if year < solar_year {
      if !solar_day.is_before(spring_solar_day) {
        sixty_cycle = sixty_cycle.next(1).unwrap();
      }
    }
    sixty_cycle
  }

  pub fn get_month_sixty_cycle(&self) -> SixtyCycle {
    let solar_day: SolarDay = self.get_solar_day();
    let year: isize = solar_day.get_month().get_year().get_year();
    let term: SolarTerm = solar_day.get_term();
    let mut index: isize = (term.get_index() as isize) - 3;
    if index < 0 && term.get_julian_day().get_solar_day().is_after(SolarTerm::from_index(year, 3).get_julian_day().get_solar_day()) {
      index += 24;
    }
    LunarMonth::from_ym(year, 1).unwrap().get_sixty_cycle().next((index as f64 / 2.0).floor() as isize).unwrap()
  }

  pub fn get_sixty_cycle(&self) -> SixtyCycle {
    let offset: isize = self.month.get_first_julian_day().next((self.day as isize) - 12).unwrap().get_day() as isize;
    SixtyCycle::from_name(format!("{}{}", HeavenStem::from_index(offset).get_name(), EarthBranch::from_index(offset).get_name()).as_str()).unwrap()
  }

  pub fn get_duty(&self) -> Duty {
    Duty::from_index(self.get_sixty_cycle().get_earth_branch().get_index() as isize - self.get_month_sixty_cycle().get_earth_branch().get_index() as isize)
  }

  pub fn get_jupiter_direction(&self) -> Direction {
    let index: usize = self.get_sixty_cycle().get_index();
    if index % 12 < 6 {
      return Direction::from_index([2, 8, 4, 6, 0][index / 12]);
    }
    self.month.get_year().get_jupiter_direction()
  }

  pub fn get_phase(&self) -> Phase {
    Phase::from_index(self.day as isize - 1)
  }

  /// 六曜
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::culture::star::six::SixStar;
  /// use tyme4rs::tyme::lunar::LunarDay;
  ///
  /// let six_star: SixStar = LunarDay::from_ymd(2023, 1, 1).unwrap().get_six_star();
  /// ```
  pub fn get_six_star(&self) -> SixStar {
    SixStar::from_index((self.month.get_month() as isize + self.day as isize - 2) % 6)
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
  /// let solar_day: SolarDay = LunarDay::from_ymd(2023, 1, 1).unwrap().get_solar_day();
  /// ```
  pub fn get_solar_day(&self) -> SolarDay {
    self.month.get_first_julian_day().next((self.day as isize) - 1).unwrap().get_solar_day()
  }

  pub fn get_twelve_star(&self) -> TwelveStar {
    TwelveStar::from_index(self.get_sixty_cycle().get_earth_branch().get_index() as isize + (8 - self.get_month_sixty_cycle().get_earth_branch().get_index() as isize % 6) * 2)
  }

  pub fn get_twenty_eight_star(&self) -> TwentyEightStar {
    TwentyEightStar::from_index([10, 18, 26, 6, 14, 22, 2][self.get_solar_day().get_week().get_index()]).next(-7 * self.get_sixty_cycle().get_earth_branch().get_index() as isize).unwrap()
  }

  pub fn get_fetus_day(&self) -> FetusDay {
    FetusDay::from_lunar_day(*self)
  }

  /// 农历传统节日
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::festival::LunarFestival;
  /// use tyme4rs::tyme::lunar::LunarDay;
  ///
  /// let festival: Option<LunarFestival> = LunarDay::from_ymd(2024, 1, 1).unwrap().get_festival();
  /// ```
  pub fn get_festival(&self) -> Option<LunarFestival> {
    LunarFestival::from_ymd(self.month.get_year().get_year(), self.month.get_month(), self.day)
  }

  pub fn get_nine_star(&self) -> NineStar {
    let solar: SolarDay = self.get_solar_day();
    let dong_zhi: SolarTerm = SolarTerm::from_index(solar.get_month().get_year().get_year(), 0);
    let xia_zhi: SolarTerm = dong_zhi.next(12).unwrap();
    let dong_zhi2: SolarTerm = dong_zhi.next(24).unwrap();
    let dong_zhi_solar: SolarDay = dong_zhi.get_julian_day().get_solar_day();
    let xia_zhi_solar: SolarDay = xia_zhi.get_julian_day().get_solar_day();
    let dong_zhi_solar2: SolarDay = dong_zhi2.get_julian_day().get_solar_day();
    let dong_zhi_index: isize = dong_zhi_solar.get_lunar_day().get_sixty_cycle().get_index() as isize;
    let xia_zhi_index: isize = xia_zhi_solar.get_lunar_day().get_sixty_cycle().get_index() as isize;
    let dong_zhi_index2: isize = dong_zhi_solar2.get_lunar_day().get_sixty_cycle().get_index() as isize;
    let solar_shun_bai: SolarDay = dong_zhi_solar.next(if dong_zhi_index > 29 { 60 - dong_zhi_index } else { -dong_zhi_index }).unwrap();
    let solar_shun_bai2: SolarDay = dong_zhi_solar2.next(if dong_zhi_index2 > 29 { 60 - dong_zhi_index2 } else { -dong_zhi_index2 }).unwrap();
    let solar_ni_zi: SolarDay = xia_zhi_solar.next(if xia_zhi_index > 29 { 60 - xia_zhi_index } else { -xia_zhi_index }).unwrap();
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
    let y: isize = self.month.get_year().get_year();
    let m: isize = self.month.get_month() as isize;
    l.push(LunarHour::from_ymd_hms(y, m, self.day, 0, 0, 0).unwrap());
    for i in (0..24).step_by(2) {
      l.push(LunarHour::from_ymd_hms(y, m, self.day, i + 1, 0, 0).unwrap());
    }
    l
  }
}

impl Display for LunarDay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.month, self.get_name())
  }
}

impl PartialEq for LunarDay {
  fn eq(&self, other: &Self) -> bool {
    self.get_month().eq(&other.get_month()) && self.get_day() == other.get_day()
  }
}

impl Eq for LunarDay {}

/// 农历时辰
#[derive(Debug, Copy, Clone)]
pub struct LunarHour {
  /// 农历日
  day: LunarDay,
  /// 时
  hour: usize,
  minute: usize,
  second: usize,
}

impl Tyme for LunarHour {
  fn next(&self, n: isize) -> Result<Self, String> {
    if n == 0 {
      Ok(self.clone())
    } else {
      let h: isize = (self.hour as isize) + n * 2;
      let diff: isize = if h < 0 { -1 } else { 1 };
      let mut hour: isize = h.abs();
      let mut days: isize = hour / 24 * diff;
      hour = (hour % 24) * diff;
      if hour < 0 {
        hour += 24;
        days -= 1;
      }
      let d: LunarDay = self.day.next(days).unwrap();
      let m: LunarMonth = d.get_month();
      Self::from_ymd_hms(m.get_year().get_year(), m.get_month_with_leap(), d.get_day(), hour as usize, self.minute, self.second)
    }
  }
}

impl Culture for LunarHour {
  fn get_name(&self) -> String {
    format!("{}时", EarthBranch::from_index(self.get_index_in_day() as isize).get_name())
  }
}

impl LunarHour {
  pub fn from_ymd_hms(year: isize, month: isize, day: usize, hour: usize, minute: usize, second: usize) -> Result<Self, String> {
    if hour > 23 {
      Err(format!("illegal hour: {}", hour))
    } else if minute > 59 {
      Err(format!("illegal minute: {}", minute))
    } else if second > 59 {
      Err(format!("illegal second: {}", second))
    } else {
      Ok(Self {
        day: LunarDay::from_ymd(year, month, day).unwrap(),
        hour,
        minute,
        second,
      })
    }
  }

  pub fn get_day(&self) -> LunarDay {
    self.day
  }

  pub fn get_hour(&self) -> usize {
    self.hour
  }

  pub fn get_minute(&self) -> usize {
    self.minute
  }

  pub fn get_second(&self) -> usize {
    self.second
  }

  pub fn get_index_in_day(&self) -> usize {
    (self.hour + 1) / 2
  }

  pub fn is_before(&self, target: LunarHour) -> bool {
    if self.day != target.get_day() {
      return self.day.is_before(target.get_day());
    }
    if self.hour != target.get_hour() {
      return self.hour < target.get_hour();
    }
    if self.minute != target.get_minute() { self.minute < target.get_minute() } else { self.second < target.get_second() }
  }

  pub fn is_after(&self, target: LunarHour) -> bool {
    if self.day != target.get_day() {
      return self.day.is_after(target.get_day());
    }
    if self.hour != target.get_hour() {
      return self.hour > target.get_hour();
    }
    if self.minute != target.get_minute() { self.minute > target.get_minute() } else { self.second > target.get_second() }
  }

  pub fn get_year_sixty_cycle(&self) -> SixtyCycle {
    let solar_time: SolarTime = self.get_solar_time();
    let solar_year: isize = self.day.get_solar_day().get_month().get_year().get_year();
    let spring_solar_time: SolarTime = SolarTerm::from_index(solar_year, 3).get_julian_day().get_solar_time();
    let lunar_year: LunarYear = self.day.get_month().get_year();
    let year: isize = lunar_year.get_year();
    let mut sixty_cycle: SixtyCycle = lunar_year.get_sixty_cycle();
    if year == solar_year {
      if solar_time.is_before(spring_solar_time) {
        sixty_cycle = sixty_cycle.next(-1).unwrap();
      }
    } else if year < solar_year {
      if !solar_time.is_before(spring_solar_time) {
        sixty_cycle = sixty_cycle.next(1).unwrap();
      }
    }
    sixty_cycle
  }

  pub fn get_month_sixty_cycle(&self) -> SixtyCycle {
    let solar_time: SolarTime = self.get_solar_time();
    let year: isize = solar_time.get_day().get_month().get_year().get_year();
    let term: SolarTerm = solar_time.get_term();
    let mut index: isize = (term.get_index() as isize) - 3;
    if index < 0 && term.get_julian_day().get_solar_day().is_after(SolarTerm::from_index(year, 3).get_julian_day().get_solar_day()) {
      index += 24;
    }
    LunarMonth::from_ym(year, 1).unwrap().get_sixty_cycle().next((index as f64 / 2.0).floor() as isize).unwrap()
  }

  pub fn get_day_sixty_cycle(&self) -> SixtyCycle {
    let d: SixtyCycle = self.day.get_sixty_cycle();
    return if self.hour < 23 { d } else { d.next(1).unwrap() };
  }

  pub fn get_sixty_cycle(&self) -> SixtyCycle {
    let earth_branch_index: isize = self.get_index_in_day() as isize % 12;
    let heaven_stem_index: isize = self.get_day_sixty_cycle().get_heaven_stem().get_index() as isize % 5 * 2 + earth_branch_index;
    SixtyCycle::from_name(format!("{}{}", HeavenStem::from_index(heaven_stem_index).get_name(), EarthBranch::from_index(earth_branch_index).get_name()).as_str()).unwrap()
  }

  pub fn get_solar_time(&self) -> SolarTime {
    let d: SolarDay = self.day.get_solar_day();
    let m: SolarMonth = d.get_month();
    SolarTime::from_ymd_hms(m.get_year().get_year(), m.get_month(), d.get_day(), self.hour, self.minute, self.second).unwrap()
  }

  pub fn get_eight_char(&self) -> EightChar {
    EightChar::from_sixty_cycle(self.get_year_sixty_cycle(), self.get_month_sixty_cycle(), self.get_day_sixty_cycle(), self.get_sixty_cycle())
  }

  pub fn get_nine_star(&self) -> NineStar {
    let solar: SolarDay = self.day.get_solar_day();
    let dong_zhi: SolarTerm = SolarTerm::from_index(solar.get_month().get_year().get_year(), 0);
    let xia_zhi: SolarTerm = dong_zhi.next(12).unwrap();
    let asc: bool = !solar.is_before(dong_zhi.get_julian_day().get_solar_day()) && solar.is_before(xia_zhi.get_julian_day().get_solar_day());
    let mut start: isize = [8, 5, 2][self.day.get_sixty_cycle().get_earth_branch().get_index() % 3];
    if asc {
      start = 8 - start;
    }
    let earth_branch_index: isize = self.get_index_in_day() as isize % 12;
    NineStar::from_index(start + if asc { earth_branch_index } else { -earth_branch_index })
  }

  pub fn get_twelve_star(&self) -> TwelveStar {
    TwelveStar::from_index(self.get_sixty_cycle().get_earth_branch().get_index() as isize + (8 - self.get_day_sixty_cycle().get_earth_branch().get_index() as isize % 6) * 2)
  }
}

impl Display for LunarHour {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}时", self.day.to_string(), self.get_sixty_cycle().get_name())
  }
}

impl PartialEq for LunarHour {
  fn eq(&self, other: &Self) -> bool {
    self.get_day() == other.get_day() && self.get_hour() == other.get_hour() && self.get_minute() == other.get_minute() && self.get_second() == other.get_second()
  }
}

impl Eq for LunarHour {}

#[cfg(test)]
mod tests {
  use crate::tyme::lunar::{LunarDay, LunarHour, LunarMonth, LunarYear};
  use crate::tyme::{Culture, Tyme};
  use crate::tyme::culture::star::twenty_eight::TwentyEightStar;
  use crate::tyme::solar::SolarDay;

  #[test]
  fn test1() {
    assert_eq!("1年1月1日", LunarDay::from_ymd(0, 11, 18).unwrap().get_solar_day().to_string());
  }

  #[test]
  fn test2() {
    assert_eq!("9999年12月31日", LunarDay::from_ymd(9999, 12, 2).unwrap().get_solar_day().to_string());
  }

  #[test]
  fn test3() {
    assert_eq!("1905年2月4日", LunarDay::from_ymd(1905, 1, 1).unwrap().get_solar_day().to_string());
  }

  #[test]
  fn test4() {
    assert_eq!("2039年1月23日", LunarDay::from_ymd(2038, 12, 29).unwrap().get_solar_day().to_string());
  }

  #[test]
  fn test5() {
    assert_eq!("1500年1月31日", LunarDay::from_ymd(1500, 1, 1).unwrap().get_solar_day().to_string());
  }

  #[test]
  fn test6() {
    assert_eq!("1501年1月18日", LunarDay::from_ymd(1500, 12, 29).unwrap().get_solar_day().to_string());
  }

  #[test]
  fn test7() {
    assert_eq!("1582年10月4日", LunarDay::from_ymd(1582, 9, 18).unwrap().get_solar_day().to_string());
  }

  #[test]
  fn test8() {
    assert_eq!("1582年10月15日", LunarDay::from_ymd(1582, 9, 19).unwrap().get_solar_day().to_string());
  }

  #[test]
  fn test9() {
    assert_eq!("2020年1月6日", LunarDay::from_ymd(2019, 12, 12).unwrap().get_solar_day().to_string());
  }

  #[test]
  fn test10() {
    assert_eq!("2033年12月22日", LunarDay::from_ymd(2033, -11, 1).unwrap().get_solar_day().to_string());
  }

  #[test]
  fn test11() {
    assert_eq!("2021年7月16日", LunarDay::from_ymd(2021, 6, 7).unwrap().get_solar_day().to_string());
  }

  #[test]
  fn test12() {
    assert_eq!("2034年2月19日", LunarDay::from_ymd(2034, 1, 1).unwrap().get_solar_day().to_string());
  }

  #[test]
  fn test13() {
    assert_eq!("2034年1月20日", LunarDay::from_ymd(2033, 12, 1).unwrap().get_solar_day().to_string());
  }

  #[test]
  fn test14() {
    assert_eq!("7013年12月24日", LunarDay::from_ymd(7013, -11, 4).unwrap().get_solar_day().to_string());
  }

  #[test]
  fn test15() {
    assert_eq!("己亥", LunarDay::from_ymd(2023, 8, 24).unwrap().get_sixty_cycle().to_string());
  }

  #[test]
  fn test16() {
    assert_eq!("癸酉", LunarDay::from_ymd(1653, 1, 6).unwrap().get_sixty_cycle().to_string());
  }

  #[test]
  fn test17() {
    assert_eq!("农历庚寅年二月初二", LunarDay::from_ymd(2010, 1, 1).unwrap().next(31).unwrap().to_string());
  }

  #[test]
  fn test18() {
    assert_eq!("农历壬辰年闰四月初一", LunarDay::from_ymd(2012, 3, 1).unwrap().next(60).unwrap().to_string());
  }

  #[test]
  fn test19() {
    assert_eq!("农历壬辰年闰四月廿九", LunarDay::from_ymd(2012, 3, 1).unwrap().next(88).unwrap().to_string());
  }

  #[test]
  fn test20() {
    assert_eq!("农历壬辰年五月初一", LunarDay::from_ymd(2012, 3, 1).unwrap().next(89).unwrap().to_string());
  }

  #[test]
  fn test21() {
    assert_eq!("2020年4月23日", LunarDay::from_ymd(2020, 4, 1).unwrap().get_solar_day().to_string());
  }

  #[test]
  fn test22() {
    assert_eq!("甲辰", LunarDay::from_ymd(2024, 1, 1).unwrap().get_month().get_year().get_sixty_cycle().get_name());
  }

  #[test]
  fn test23() {
    assert_eq!("癸卯", LunarDay::from_ymd(2023, 12, 30).unwrap().get_month().get_year().get_sixty_cycle().get_name());
  }

  #[test]
  fn test24() {
    let d: LunarDay = LunarDay::from_ymd(2020, 4, 13).unwrap();
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
    let d: LunarDay = LunarDay::from_ymd(2023, 9, 28).unwrap();
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
    let lunar: LunarDay = LunarDay::from_ymd(2005, 11, 23).unwrap();
    assert_eq!("戊子", lunar.get_month().get_sixty_cycle().get_name());
    assert_eq!("戊子", lunar.get_month_sixty_cycle().get_name());
  }

  #[test]
  fn test27() {
    let h: LunarHour = LunarHour::from_ymd_hms(2020, -4, 5, 23, 0, 0).unwrap();
    assert_eq!("子时", h.get_name());
    assert_eq!("农历庚子年闰四月初五戊子时", h.to_string());
  }

  #[test]
  fn test28() {
    let h: LunarHour = LunarHour::from_ymd_hms(2020, -4, 5, 0, 59, 0).unwrap();
    assert_eq!("子时", h.get_name());
    assert_eq!("农历庚子年闰四月初五丙子时", h.to_string());
  }

  #[test]
  fn test29() {
    let h: LunarHour = LunarHour::from_ymd_hms(2020, -4, 5, 1, 0, 0).unwrap();
    assert_eq!("丑时", h.get_name());
    assert_eq!("农历庚子年闰四月初五丁丑时", h.to_string());
  }

  #[test]
  fn test30() {
    let h: LunarHour = LunarHour::from_ymd_hms(2020, -4, 5, 21, 30, 0).unwrap();
    assert_eq!("亥时", h.get_name());
    assert_eq!("农历庚子年闰四月初五丁亥时", h.to_string());
  }

  #[test]
  fn test31() {
    let h: LunarHour = LunarHour::from_ymd_hms(2020, -4, 2, 23, 30, 0).unwrap();
    assert_eq!("子时", h.get_name());
    assert_eq!("农历庚子年闰四月初二壬子时", h.to_string());
  }

  #[test]
  fn test32() {
    let h: LunarHour = LunarHour::from_ymd_hms(2020, 4, 28, 23, 30, 0).unwrap();
    assert_eq!("子时", h.get_name());
    assert_eq!("农历庚子年四月廿八甲子时", h.to_string());
  }

  #[test]
  fn test33() {
    let h: LunarHour = LunarHour::from_ymd_hms(2020, 4, 29, 0, 0, 0).unwrap();
    assert_eq!("子时", h.get_name());
    assert_eq!("农历庚子年四月廿九甲子时", h.to_string());
  }

  #[test]
  fn test34() {
    let h: LunarHour = LunarHour::from_ymd_hms(2023, 11, 14, 23, 0, 0).unwrap();
    assert_eq!("甲子", h.get_sixty_cycle().get_name());

    assert_eq!("己未", h.get_day_sixty_cycle().get_name());
    assert_eq!("戊午", h.get_day().get_sixty_cycle().get_name());
    assert_eq!("农历癸卯年十一月十四", h.get_day().to_string());

    assert_eq!("甲子", h.get_month_sixty_cycle().get_name());
    assert_eq!("农历癸卯年十一月", h.get_day().get_month().to_string());
    assert_eq!("乙丑", h.get_day().get_month().get_sixty_cycle().get_name());

    assert_eq!("癸卯", h.get_year_sixty_cycle().get_name());
    assert_eq!("农历癸卯年", h.get_day().get_month().get_year().to_string());
    assert_eq!("癸卯", h.get_day().get_month().get_year().get_sixty_cycle().get_name());
  }

  #[test]
  fn test35() {
    let h: LunarHour = LunarHour::from_ymd_hms(2023, 11, 14, 6, 0, 0).unwrap();
    assert_eq!("乙卯", h.get_sixty_cycle().get_name());

    assert_eq!("戊午", h.get_day_sixty_cycle().get_name());
    assert_eq!("戊午", h.get_day().get_sixty_cycle().get_name());
    assert_eq!("农历癸卯年十一月十四", h.get_day().to_string());

    assert_eq!("甲子", h.get_month_sixty_cycle().get_name());
    assert_eq!("农历癸卯年十一月", h.get_day().get_month().to_string());
    assert_eq!("乙丑", h.get_day().get_month().get_sixty_cycle().get_name());

    assert_eq!("癸卯", h.get_year_sixty_cycle().get_name());
    assert_eq!("农历癸卯年", h.get_day().get_month().get_year().to_string());
    assert_eq!("癸卯", h.get_day().get_month().get_year().get_sixty_cycle().get_name());
  }

  #[test]
  fn test36() {
    assert_eq!("七月", LunarMonth::from_ym(2359, 7).unwrap().get_name());
  }

  /**
   * 闰月
   */
  #[test]
  fn test37() {
    assert_eq!("闰七月", LunarMonth::from_ym(2359, -7).unwrap().get_name());
  }

  #[test]
  fn test38() {
    assert_eq!(29, LunarMonth::from_ym(2023, 6).unwrap().get_day_count());
  }

  #[test]
  fn test39() {
    assert_eq!(30, LunarMonth::from_ym(2023, 7).unwrap().get_day_count());
  }

  #[test]
  fn test40() {
    assert_eq!(30, LunarMonth::from_ym(2023, 8).unwrap().get_day_count());
  }

  #[test]
  fn test41() {
    assert_eq!(29, LunarMonth::from_ym(2023, 9).unwrap().get_day_count());
  }

  #[test]
  fn test42() {
    assert_eq!("2023年10月15日", LunarMonth::from_ym(2023, 9).unwrap().get_first_julian_day().get_solar_day().to_string());
  }

  #[test]
  fn test43() {
    assert_eq!("甲寅", LunarMonth::from_ym(2023, 1).unwrap().get_sixty_cycle().get_name());
  }

  #[test]
  fn test44() {
    assert_eq!("丙辰", LunarMonth::from_ym(2023, -2).unwrap().get_sixty_cycle().get_name());
  }

  #[test]
  fn test45() {
    assert_eq!("丁巳", LunarMonth::from_ym(2023, 3).unwrap().get_sixty_cycle().get_name());
  }

  #[test]
  fn test46() {
    assert_eq!("丙寅", LunarMonth::from_ym(2024, 1).unwrap().get_sixty_cycle().get_name());
  }

  #[test]
  fn test47() {
    assert_eq!("丙寅", LunarMonth::from_ym(2023, 12).unwrap().get_sixty_cycle().get_name());
  }

  #[test]
  fn test48() {
    assert_eq!("壬寅", LunarMonth::from_ym(2022, 1).unwrap().get_sixty_cycle().get_name());
  }

  #[test]
  fn test49() {
    assert_eq!("闰十二月", LunarMonth::from_ym(37, -12).unwrap().get_name());
  }

  #[test]
  fn test50() {
    assert_eq!("闰十二月", LunarMonth::from_ym(5552, -12).unwrap().get_name());
  }

  #[test]
  fn test51() {
    assert_eq!("农历戊子年十二月", LunarMonth::from_ym(2008, 11).unwrap().next(1).unwrap().to_string());
  }

  #[test]
  fn test52() {
    assert_eq!("农历己丑年正月", LunarMonth::from_ym(2008, 11).unwrap().next(2).unwrap().to_string());
  }

  #[test]
  fn test53() {
    assert_eq!("农历己丑年五月", LunarMonth::from_ym(2008, 11).unwrap().next(6).unwrap().to_string());
  }

  #[test]
  fn test54() {
    assert_eq!("农历己丑年闰五月", LunarMonth::from_ym(2008, 11).unwrap().next(7).unwrap().to_string());
  }

  #[test]
  fn test55() {
    assert_eq!("农历己丑年六月", LunarMonth::from_ym(2008, 11).unwrap().next(8).unwrap().to_string());
  }

  #[test]
  fn test56() {
    assert_eq!("农历庚寅年正月", LunarMonth::from_ym(2008, 11).unwrap().next(15).unwrap().to_string());
  }

  #[test]
  fn test57() {
    assert_eq!("农历戊子年十一月", LunarMonth::from_ym(2008, 12).unwrap().next(-1).unwrap().to_string());
  }

  #[test]
  fn test58() {
    assert_eq!("农历戊子年十一月", LunarMonth::from_ym(2009, 1).unwrap().next(-2).unwrap().to_string());
  }

  #[test]
  fn test59() {
    assert_eq!("农历戊子年十一月", LunarMonth::from_ym(2009, 5).unwrap().next(-6).unwrap().to_string());
  }

  #[test]
  fn test60() {
    assert_eq!("农历戊子年十一月", LunarMonth::from_ym(2009, -5).unwrap().next(-7).unwrap().to_string());
  }

  #[test]
  fn test61() {
    assert_eq!("农历戊子年十一月", LunarMonth::from_ym(2009, 6).unwrap().next(-8).unwrap().to_string());
  }

  #[test]
  fn test62() {
    assert_eq!("农历戊子年十一月", LunarMonth::from_ym(2010, 1).unwrap().next(-15).unwrap().to_string());
  }

  #[test]
  fn test63() {
    assert_eq!(29, LunarMonth::from_ym(2012, -4).unwrap().get_day_count());
  }

  #[test]
  fn test64() {
    assert_eq!("癸亥", LunarMonth::from_ym(2023, 9).unwrap().get_sixty_cycle().to_string());
  }

  #[test]
  fn test65() {
    let d: LunarDay = SolarDay::from_ymd(2023, 10, 7).unwrap().get_lunar_day();
    assert_eq!("壬戌", d.get_month().get_sixty_cycle().to_string());
    assert_eq!("辛酉", d.get_month_sixty_cycle().to_string());
  }

  #[test]
  fn test66() {
    let d: LunarDay = SolarDay::from_ymd(2023, 10, 8).unwrap().get_lunar_day();
    assert_eq!("壬戌", d.get_month().get_sixty_cycle().to_string());
    assert_eq!("壬戌", d.get_month_sixty_cycle().to_string());
  }

  #[test]
  fn test67() {
    let d: LunarDay = SolarDay::from_ymd(2023, 10, 15).unwrap().get_lunar_day();
    assert_eq!("九月", d.get_month().get_name());
    assert_eq!("癸亥", d.get_month().get_sixty_cycle().to_string());
    assert_eq!("壬戌", d.get_month_sixty_cycle().to_string());
  }

  #[test]
  fn test68() {
    let d: LunarDay = SolarDay::from_ymd(2023, 11, 7).unwrap().get_lunar_day();
    assert_eq!("癸亥", d.get_month().get_sixty_cycle().to_string());
    assert_eq!("壬戌", d.get_month_sixty_cycle().to_string());
  }

  #[test]
  fn test69() {
    let d: LunarDay = SolarDay::from_ymd(2023, 11, 8).unwrap().get_lunar_day();
    assert_eq!("癸亥", d.get_month().get_sixty_cycle().to_string());
    assert_eq!("癸亥", d.get_month_sixty_cycle().to_string());
  }

  #[test]
  fn test70() {
    // 2023年闰2月
    let m: LunarMonth = LunarMonth::from_ym(2023, 12).unwrap();
    assert_eq!("农历癸卯年十二月", m.to_string());
    assert_eq!("农历癸卯年十一月", m.next(-1).unwrap().to_string());
    assert_eq!("农历癸卯年十月", m.next(-2).unwrap().to_string());
  }

  #[test]
  fn test71() {
    // 2023年闰2月
    let m: LunarMonth = LunarMonth::from_ym(2023, 3).unwrap();
    assert_eq!("农历癸卯年三月", m.to_string());
    assert_eq!("农历癸卯年闰二月", m.next(-1).unwrap().to_string());
    assert_eq!("农历癸卯年二月", m.next(-2).unwrap().to_string());
    assert_eq!("农历癸卯年正月", m.next(-3).unwrap().to_string());
    assert_eq!("农历壬寅年十二月", m.next(-4).unwrap().to_string());
    assert_eq!("农历壬寅年十一月", m.next(-5).unwrap().to_string());
  }

  #[test]
  fn test72() {
    let d: LunarDay = SolarDay::from_ymd(1983, 2, 15).unwrap().get_lunar_day();
    assert_eq!("甲寅", d.get_month().get_sixty_cycle().to_string());
    assert_eq!("甲寅", d.get_month_sixty_cycle().to_string());
  }

  #[test]
  fn test73() {
    let d: LunarDay = SolarDay::from_ymd(2023, 10, 30).unwrap().get_lunar_day();
    assert_eq!("癸亥", d.get_month().get_sixty_cycle().to_string());
    assert_eq!("壬戌", d.get_month_sixty_cycle().to_string());
  }

  #[test]
  fn test74() {
    let d: LunarDay = SolarDay::from_ymd(2023, 10, 19).unwrap().get_lunar_day();
    assert_eq!("癸亥", d.get_month().get_sixty_cycle().to_string());
    assert_eq!("壬戌", d.get_month_sixty_cycle().to_string());
  }

  #[test]
  fn test75() {
    let m: LunarMonth = LunarMonth::from_ym(2023, 11).unwrap();
    assert_eq!("农历癸卯年十一月", m.to_string());
    assert_eq!("乙丑", m.get_sixty_cycle().to_string());
  }

  #[test]
  fn test76() {
    assert_eq!("庚申", LunarDay::from_ymd(2018, 6, 26).unwrap().get_month_sixty_cycle().to_string());
  }

  #[test]
  fn test77() {
    assert_eq!("辛丑", LunarMonth::from_ym(1991, 12).unwrap().get_sixty_cycle().to_string());
  }

  #[test]
  fn test78() {
    assert_eq!("农历癸卯年", LunarYear::from_year(2023).unwrap().get_name());
  }

  #[test]
  fn test79() {
    assert_eq!("农历戊申年", LunarYear::from_year(2023).unwrap().next(5).unwrap().get_name());
  }

  #[test]
  fn test80() {
    assert_eq!("农历戊戌年", LunarYear::from_year(2023).unwrap().next(-5).unwrap().get_name());
  }

  /**
   * 农历年的干支
   */
  #[test]
  fn test81() {
    assert_eq!("庚子", LunarYear::from_year(2020).unwrap().get_sixty_cycle().get_name());
  }

  /**
   * 农历年的生肖(农历年.干支.地支.生肖)
   */
  #[test]
  fn test82() {
    assert_eq!("虎", LunarYear::from_year(1986).unwrap().get_sixty_cycle().get_earth_branch().get_zodiac().get_name());
  }

  #[test]
  fn test83() {
    assert_eq!(12, LunarYear::from_year(151).unwrap().get_leap_month());
  }

  #[test]
  fn test84() {
    assert_eq!(1, LunarYear::from_year(2357).unwrap().get_leap_month());
  }

  #[test]
  fn test85() {
    let y: LunarYear = LunarYear::from_year(2023).unwrap();
    assert_eq!("癸卯", y.get_sixty_cycle().get_name());
    assert_eq!("兔", y.get_sixty_cycle().get_earth_branch().get_zodiac().get_name());
  }

  #[test]
  fn test86() {
    assert_eq!("上元", LunarYear::from_year(1864).unwrap().get_twenty().get_sixty().get_name());
  }

  #[test]
  fn test87() {
    assert_eq!("上元", LunarYear::from_year(1923).unwrap().get_twenty().get_sixty().get_name());
  }

  #[test]
  fn test88() {
    assert_eq!("中元", LunarYear::from_year(1924).unwrap().get_twenty().get_sixty().get_name());
  }

  #[test]
  fn test89() {
    assert_eq!("中元", LunarYear::from_year(1983).unwrap().get_twenty().get_sixty().get_name());
  }

  #[test]
  fn test90() {
    assert_eq!("下元", LunarYear::from_year(1984).unwrap().get_twenty().get_sixty().get_name());
  }

  #[test]
  fn test91() {
    assert_eq!("下元", LunarYear::from_year(2043).unwrap().get_twenty().get_sixty().get_name());
  }

  #[test]
  fn test92() {
    assert_eq!("一运", LunarYear::from_year(1864).unwrap().get_twenty().get_name());
  }

  #[test]
  fn test93() {
    assert_eq!("一运", LunarYear::from_year(1883).unwrap().get_twenty().get_name());
  }

  #[test]
  fn test94() {
    assert_eq!("二运", LunarYear::from_year(1884).unwrap().get_twenty().get_name());
  }

  #[test]
  fn test95() {
    assert_eq!("二运", LunarYear::from_year(1903).unwrap().get_twenty().get_name());
  }

  #[test]
  fn test96() {
    assert_eq!("三运", LunarYear::from_year(1904).unwrap().get_twenty().get_name());
  }

  #[test]
  fn test97() {
    assert_eq!("三运", LunarYear::from_year(1923).unwrap().get_twenty().get_name());
  }

  #[test]
  fn test98() {
    assert_eq!("八运", LunarYear::from_year(2004).unwrap().get_twenty().get_name());
  }

  #[test]
  fn test99() {
    let year: LunarYear = LunarYear::from_year(1).unwrap();
    assert_eq!("六运", year.get_twenty().get_name());
    assert_eq!("中元", year.get_twenty().get_sixty().get_name());
  }

  #[test]
  fn test100() {
    let year: LunarYear = LunarYear::from_year(1863).unwrap();
    assert_eq!("九运", year.get_twenty().get_name());
    assert_eq!("下元", year.get_twenty().get_sixty().get_name());
  }
}
