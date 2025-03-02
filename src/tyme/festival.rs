use std::fmt::{Display, Formatter};
use std::str::FromStr;

use regex::Regex;

use crate::tyme::{AbstractCulture, Culture, Tyme};
use crate::tyme::enums::FestivalType;
use crate::tyme::lunar::{LunarDay};
use crate::tyme::solar::{SolarDay, SolarTerm};

pub static SOLAR_FESTIVAL_NAMES: [&str; 10] = ["元旦", "三八妇女节", "植树节", "五一劳动节", "五四青年节", "六一儿童节", "建党节", "八一建军节", "教师节", "国庆节"];
pub static SOLAR_FESTIVAL_DATA: &str = "@00001011950@01003081950@02003121979@03005011950@04005041950@05006011950@06007011941@07008011933@08009101985@09010011950";

/// 公历现代节日
#[derive(Debug, Copy, Clone)]
pub struct SolarFestival {
  /// 类型
  festival_type: FestivalType,
  /// 公历日
  day: SolarDay,
  /// 索引
  index: usize,
  /// 起始年
  start_year: isize,
}

impl Culture for SolarFestival {
  fn get_name(&self) -> String {
    SOLAR_FESTIVAL_NAMES[self.index].to_string()
  }
}

impl SolarFestival {
  pub fn from_ymd(year: isize, month: usize, day: usize) -> Option<Self> {
    let reg: Regex = Regex::new(format!("{}{:0>two$}{:0>two$}{}", r"@\d{2}0", month, day, r"\d+", two = 2).as_str()).unwrap();
    if reg.is_match(SOLAR_FESTIVAL_DATA) {
      let data: &str = reg.find(SOLAR_FESTIVAL_DATA).unwrap().as_str();
      let dy: &str = &data[8..data.len()];
      let start_year: isize = isize::from_str(dy).unwrap();
      if year < start_year {
        return None;
      }
      let day: SolarDay = SolarDay::from_ymd(year, month, day);
      let di: &str = &data[1..3];
      let index: usize = usize::from_str(di).unwrap();
      Some(Self {
        festival_type: FestivalType::DAY,
        day,
        index,
        start_year,
      })
    } else {
      None
    }
  }

  pub fn from_index(year: isize, index: usize) -> Option<Self> {
    if index >= SOLAR_FESTIVAL_NAMES.len() {
      return None;
    }
    let reg: Regex = Regex::new(format!("{}{:0>two$}{}", r"@", index, r"\d+", two = 2).as_str()).unwrap();
    if reg.is_match(SOLAR_FESTIVAL_DATA) {
      let data: &str = reg.find(SOLAR_FESTIVAL_DATA).unwrap().as_str();
      let dt: usize = (data.chars().nth(3).unwrap() as usize) - ('0' as usize);
      let festival_type: FestivalType = FestivalType::from_code(dt).unwrap();
      if festival_type != FestivalType::DAY {
        return None;
      }
      let dy: &str = &data[8..data.len()];
      let start_year: isize = isize::from_str(dy).unwrap();
      if year < start_year {
        return None;
      }
      let dm: &str = &data[4..6];
      let dd: &str = &data[6..8];
      let month: usize = usize::from_str(dm).unwrap();
      let day: usize = usize::from_str(dd).unwrap();

      let day: SolarDay = SolarDay::from_ymd(year, month, day);
      let di: &str = &data[1..3];
      let index: usize = usize::from_str(di).unwrap();
      Some(Self {
        festival_type: FestivalType::DAY,
        day,
        index,
        start_year,
      })
    } else {
      None
    }
  }

  pub fn get_type(&self) -> FestivalType {
    self.festival_type.clone()
  }

  pub fn get_index(&self) -> usize {
    self.index
  }

  pub fn get_day(&self) -> SolarDay {
    self.day
  }

  pub fn get_start_year(&self) -> isize {
    self.start_year
  }

  pub fn next(&self, n: isize) -> Option<Self> {
    let size: isize = SOLAR_FESTIVAL_NAMES.len() as isize;
    let i: isize = self.get_index() as isize + n;
    Self::from_index((self.day.get_year() * size + i) / size, AbstractCulture::new().index_of(i, size as usize))
  }
}

impl Display for SolarFestival {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", self.get_day(), self.get_name())
  }
}

impl PartialEq for SolarFestival {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for SolarFestival {}

pub static LUNAR_FESTIVAL_NAMES: [&str; 13] = ["春节", "元宵节", "龙头节", "上巳节", "清明节", "端午节", "七夕节", "中元节", "中秋节", "重阳节", "冬至节", "腊八节", "除夕"];
pub static LUNAR_FESTIVAL_DATA: &str = "@0000101@0100115@0200202@0300303@04107@0500505@0600707@0700715@0800815@0900909@10124@1101208@122";

/// 农历传统节日（依据国家标准《农历的编算和颁行》GB/T 33661-2017）
#[derive(Debug, Clone)]
pub struct LunarFestival {
  /// 类型
  festival_type: FestivalType,
  /// 农历日
  day: LunarDay,
  /// 索引
  index: usize,
  /// 节气
  solar_term: Option<SolarTerm>,
}

impl Culture for LunarFestival {
  fn get_name(&self) -> String {
    LUNAR_FESTIVAL_NAMES[self.index].to_string()
  }
}

impl LunarFestival {
  pub fn from_ymd(year: isize, month: isize, day: usize) -> Option<Self> {
    let mut reg: Regex = Regex::new(format!("{}{:0>two$}{:0>two$}", r"@\d{2}0", month, day, two = 2).as_str()).unwrap();
    if reg.is_match(LUNAR_FESTIVAL_DATA) {
      let data: &str = reg.find(LUNAR_FESTIVAL_DATA).unwrap().as_str();
      let day: LunarDay = LunarDay::from_ymd(year, month, day);
      let di: &str = &data[1..3];
      let index: usize = usize::from_str(di).unwrap();
      return Some(Self {
        festival_type: FestivalType::DAY,
        day,
        index,
        solar_term: None,
      });
    }
    reg = Regex::new(r"@\d{2}1\d{2}").unwrap();
    if reg.is_match(LUNAR_FESTIVAL_DATA) {
      let data: &str = reg.find(LUNAR_FESTIVAL_DATA).unwrap().as_str();
      let di: &str = &data[4..data.len()];
      let term_index: usize = usize::from_str(di).unwrap();
      let solar_term: SolarTerm = SolarTerm::from_index(year, term_index as isize);
      let di: &str = &data[1..3];
      let index: usize = usize::from_str(di).unwrap();
      let lunar_day: LunarDay = solar_term.get_julian_day().get_solar_day().get_lunar_day();
      if lunar_day.get_year() == year && lunar_day.get_month() == month && lunar_day.get_day() == day {
        return Some(Self {
          festival_type: FestivalType::TERM,
          day: lunar_day,
          index,
          solar_term: Some(solar_term),
        });
      }
    }
    reg = Regex::new(r"@\d{2}2").unwrap();
    if reg.is_match(LUNAR_FESTIVAL_DATA) {
      let data: &str = reg.find(LUNAR_FESTIVAL_DATA).unwrap().as_str();
      let di: &str = &data[1..3];
      let index: usize = usize::from_str(di).unwrap();

      let lunar_day: LunarDay = LunarDay::from_ymd(year, month, day);
      let next_day: LunarDay = lunar_day.next(1);
      if next_day.get_month() == 1 && next_day.get_day() == 1 {
        return Some(Self {
          festival_type: FestivalType::EVE,
          day: lunar_day,
          index,
          solar_term: None,
        });
      }
    }
    None
  }

  pub fn from_index(year: isize, index: usize) -> Option<Self> {
    if index >= LUNAR_FESTIVAL_NAMES.len() {
      return None;
    }
    let reg: Regex = Regex::new(format!("{}{:0>two$}{}", r"@", index, r"\d+", two = 2).as_str()).unwrap();
    if reg.is_match(LUNAR_FESTIVAL_DATA) {
      let data: &str = reg.find(LUNAR_FESTIVAL_DATA).unwrap().as_str();
      let dt: usize = (data.chars().nth(3).unwrap() as usize) - ('0' as usize);
      let festival_type: FestivalType = FestivalType::from_code(dt).unwrap();
      return match festival_type {
        FestivalType::DAY => {
          let dm: &str = &data[4..6];
          let dd: &str = &data[6..8];
          let month: usize = usize::from_str(dm).unwrap();
          let day: usize = usize::from_str(dd).unwrap();
          let di: &str = &data[1..3];
          let index: usize = usize::from_str(di).unwrap();
          Some(Self {
            festival_type: FestivalType::DAY,
            day: LunarDay::from_ymd(year, month as isize, day),
            index,
            solar_term: None,
          })
        }
        FestivalType::TERM => {
          let ti: &str = &data[4..data.len()];
          let term_index: usize = usize::from_str(ti).unwrap();
          let solar_term: SolarTerm = SolarTerm::from_index(year, term_index as isize);
          let di: &str = &data[1..3];
          let index: usize = usize::from_str(di).unwrap();
          let lunar_day: LunarDay = solar_term.get_julian_day().get_solar_day().get_lunar_day();
          Some(Self {
            festival_type: FestivalType::TERM,
            day: lunar_day,
            index,
            solar_term: Some(solar_term),
          })
        }
        FestivalType::EVE => {
          let di: &str = &data[1..3];
          let index: usize = usize::from_str(di).unwrap();
          Some(Self {
            festival_type: FestivalType::EVE,
            day: LunarDay::from_ymd(year + 1, 1, 1).next(-1),
            index,
            solar_term: None,
          })
        }
      };
    }
    None
  }

  pub fn get_type(&self) -> FestivalType {
    self.festival_type.clone()
  }

  pub fn get_index(&self) -> usize {
    self.index
  }

  pub fn get_day(&self) -> LunarDay {
    self.day
  }

  pub fn get_solar_term(&self) -> Option<SolarTerm> {
    self.solar_term.clone()
  }

  pub fn next(&self, n: isize) -> Option<Self> {
    let size: isize = LUNAR_FESTIVAL_NAMES.len() as isize;
    let i: isize = self.get_index() as isize + n;
    Self::from_index((self.get_day().get_year() * size + i) / size, AbstractCulture::new().index_of(i, size as usize))
  }
}

impl Display for LunarFestival {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", self.get_day(), self.get_name())
  }
}

impl PartialEq for LunarFestival {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for LunarFestival {}

#[cfg(test)]
mod tests {
  use crate::tyme::festival::{LunarFestival, SolarFestival};
  use crate::tyme::lunar::LunarDay;
  use crate::tyme::solar::SolarDay;

  #[test]
  fn test1() {
    let f: LunarFestival = LunarFestival::from_index(2023, 0).unwrap();
    assert_eq!("农历甲辰年正月初一 春节", f.next(13).unwrap().to_string());
    assert_eq!("农历壬寅年十一月廿九 冬至节", f.next(-3).unwrap().to_string());
  }

  #[test]
  fn test2() {
    let f: LunarFestival = LunarFestival::from_index(2023, 0).unwrap();
    assert_eq!("农历壬寅年三月初五 清明节", f.next(-9).unwrap().to_string());
  }

  #[test]
  fn test3() {
    let f: LunarFestival = LunarDay::from_ymd(2010, 1, 15).get_festival().unwrap();
    assert_eq!("农历庚寅年正月十五 元宵节", f.to_string());
  }

  #[test]
  fn test4() {
    let f: LunarFestival = LunarDay::from_ymd(2021, 12, 29).get_festival().unwrap();
    assert_eq!("农历辛丑年十二月廿九 除夕", f.to_string());
  }

  #[test]
  fn test5() {
    let f: Option<SolarFestival> = SolarFestival::from_index(2023, 0);
    assert_eq!(false, f.is_none());
    assert_eq!("2024年5月1日 五一劳动节", f.unwrap().next(13).unwrap().to_string());
    assert_eq!("2022年8月1日 八一建军节", f.unwrap().next(-3).unwrap().to_string());
  }

  #[test]
  fn test6() {
    let f: Option<SolarFestival> = SolarFestival::from_index(2023, 0);
    assert_eq!(false, f.is_none());
    assert_eq!("2022年3月8日 三八妇女节", f.unwrap().next(-9).unwrap().to_string());
  }

  #[test]
  fn test7() {
    let f: Option<SolarFestival> = SolarDay::from_ymd(2010, 1, 1).get_festival();
    assert_eq!(false, f.is_none());
    assert_eq!("2010年1月1日 元旦", f.unwrap().to_string());
  }

  #[test]
  fn test8() {
    let f: Option<SolarFestival> = SolarDay::from_ymd(2021, 5, 4).get_festival();
    assert_eq!(false, f.is_none());
    assert_eq!("2021年5月4日 五四青年节", f.unwrap().to_string());
  }

  #[test]
  fn test9() {
    let f: Option<SolarFestival> = SolarDay::from_ymd(1939, 5, 4).get_festival();
    assert_eq!(true, f.is_none());
  }
}
