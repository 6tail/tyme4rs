use std::fmt::{Display, Formatter};

use crate::tyme::{AbstractCulture, AbstractCultureDay, AbstractTyme, Culture, LoopTyme, Tyme};
use crate::tyme::culture::{Constellation, Week};
use crate::tyme::culture::dog::{Dog, DogDay};
use crate::tyme::culture::nine::{Nine, NineDay};
use crate::tyme::culture::phenology::{Phenology, PhenologyDay};
use crate::tyme::culture::plumrain::{PlumRain, PlumRainDay};
use crate::tyme::festival::SolarFestival;
use crate::tyme::holiday::LegalHoliday;
use crate::tyme::jd::{J2000, JulianDay};
use crate::tyme::lunar::{LunarDay, LunarHour, LunarMonth};
use crate::tyme::util::ShouXingUtil;

/// 公历年
#[derive(Debug, Copy, Clone)]
pub struct SolarYear {
  /// 年
  year: isize,
}

impl Tyme for SolarYear {
  fn next(&self, n: isize) -> Self {
    Self::from_year(self.year + n)
  }
}

impl Culture for SolarYear {
  fn get_name(&self) -> String {
    format!("{}年", self.year)
  }
}

impl SolarYear {
  pub fn new(year: isize) -> Result<Self, String> {
    if year < 1 || year > 9999 {
      Err(format!("illegal solar year: {}", year))
    } else {
      Ok(Self {
        year
      })
    }
  }

  pub fn from_year(year: isize) -> Self {
    Self::new(year).unwrap()
  }

  /// 年
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::SolarYear;
  ///
  /// // 2023
  /// let year: isize = SolarYear::from_year(2023).get_year();
  /// ```
  pub fn get_year(&self) -> isize {
    self.year
  }

  /// 当年总天数
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::SolarYear;
  ///
  /// // 365
  /// let day_count: usize = SolarYear::from_year(2023).get_day_count();
  /// ```
  pub fn get_day_count(&self) -> usize {
    if 1582 == self.year {
      355
    } else {
      if self.is_leap() {
        366
      } else {
        365
      }
    }
  }

  /// 是否闰年
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::SolarYear;
  ///
  /// // false
  /// let leap: bool = SolarYear::from_year(2023).is_leap();
  /// ```
  pub fn is_leap(&self) -> bool {
    if self.year < 1600 {
      self.year % 4 == 0
    } else {
      (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
    }
  }

  /// 公历月列表
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarMonth, SolarYear};
  ///
  /// // 1-12月
  /// let months: Vec<SolarMonth> = SolarYear::from_year(2023).get_months();
  /// ```
  pub fn get_months(&self) -> Vec<SolarMonth> {
    let mut l: Vec<SolarMonth> = Vec::new();
    for i in 1..13 {
      l.push(SolarMonth::from_ym(self.year, i));
    }
    l
  }

  /// 公历季度列表
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarSeason, SolarYear};
  ///
  /// // 1,2,3,4季度
  /// let seasons: Vec<SolarSeason> = SolarYear::from_year(2023).get_seasons();
  /// ```
  pub fn get_seasons(&self) -> Vec<SolarSeason> {
    let mut l: Vec<SolarSeason> = Vec::new();
    for i in 0..4 {
      l.push(SolarSeason::from_index(self.year, i));
    }
    l
  }

  /// 公历半年列表
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarHalfYear, SolarYear};
  ///
  /// // 上半年，下半年
  /// let half_years: Vec<SolarHalfYear> = SolarYear::from_year(2023).get_half_years();
  /// ```
  pub fn get_half_years(&self) -> Vec<SolarHalfYear> {
    let mut l: Vec<SolarHalfYear> = Vec::new();
    for i in 0..2 {
      l.push(SolarHalfYear::from_index(self.year, i));
    }
    l
  }
}

impl Display for SolarYear {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for SolarYear {
  fn eq(&self, other: &Self) -> bool {
    self.get_year() == other.get_year()
  }
}

impl Eq for SolarYear {}

/// 公历半年名称
pub static SOLAR_HALF_YEAR_NAMES: [&str; 2] = ["上半年", "下半年"];

/// 公历半年
#[derive(Debug, Copy, Clone)]
pub struct SolarHalfYear {
  /// 公历年
  year: SolarYear,
  /// 索引，0-1
  index: usize,
}

impl Tyme for SolarHalfYear {
  fn next(&self, n: isize) -> Self {
    if n == 0 {
      return self.clone();
    }
    let mut i: isize = self.index as isize + n;
    let mut y: isize = self.year.get_year() + i / 2;
    i %= 2;
    if i < 0 {
      i += 2;
      y -= 1;
    }
    Self::from_index(y, i as usize)
  }
}

impl Culture for SolarHalfYear {
  fn get_name(&self) -> String {
    SOLAR_HALF_YEAR_NAMES[self.index].to_string()
  }
}

impl SolarHalfYear {
  pub fn new(year: isize, index: usize) -> Result<Self, String> {
    if index > 1 {
      Err(format!("illegal solar half year index: {}", index))
    } else {
      Ok(Self {
        year: SolarYear::from_year(year),
        index,
      })
    }
  }

  pub fn from_index(year: isize, index: usize) -> Self {
    Self::new(year, index).unwrap()
  }

  /// 公历年
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarHalfYear, SolarYear};
  ///
  /// // 2023年上半年
  /// let half_year: SolarHalfYear = SolarHalfYear::from_index(2023, 0);
  ///
  /// // 2023年
  /// let year: SolarYear = half_year.get_solar_year();
  /// ```
  pub fn get_solar_year(&self) -> SolarYear {
    self.year
  }

  /// 年
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarHalfYear};
  ///
  /// // 2023年上半年
  /// let half_year: SolarHalfYear = SolarHalfYear::from_index(2023, 0);
  ///
  /// // 2023
  /// let year: isize = half_year.get_year();
  /// ```
  pub fn get_year(&self) -> isize {
    self.year.get_year()
  }

  /// 索引
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarHalfYear, SolarYear};
  ///
  /// // 2023年上半年
  /// let half_year: SolarHalfYear = SolarHalfYear::from_index(2023, 0);
  ///
  /// // 0
  /// let index: usize = half_year.get_index();
  /// ```
  pub fn get_index(&self) -> usize {
    self.index
  }

  /// 公历月列表
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarMonth, SolarHalfYear};
  ///
  /// // 1,2,3,4月
  /// let months: Vec<SolarMonth> = SolarHalfYear::from_index(2023, 0).get_months();
  /// ```
  pub fn get_months(&self) -> Vec<SolarMonth> {
    let mut l: Vec<SolarMonth> = Vec::new();
    let y: isize = self.get_year();
    for i in 1..7 {
      l.push(SolarMonth::from_ym(y, self.index * 6 + i));
    }
    l
  }

  /// 公历季度列表
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarSeason, SolarHalfYear};
  ///
  /// // 1,2季度
  /// let seasons: Vec<SolarSeason> = SolarHalfYear::from_index(2023, 0).get_seasons();
  /// ```
  pub fn get_seasons(&self) -> Vec<SolarSeason> {
    let mut l: Vec<SolarSeason> = Vec::new();
    let y: isize = self.get_year();
    for i in 0..2 {
      l.push(SolarSeason::from_index(y, self.index * 2 + i));
    }
    l
  }
}

impl Display for SolarHalfYear {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.year, self.get_name())
  }
}

impl PartialEq for SolarHalfYear {
  fn eq(&self, other: &Self) -> bool {
    self.get_year() == other.get_year() && self.get_index() == other.get_index()
  }
}

impl Eq for SolarHalfYear {}

/// 公历季度名称
pub static SOLAR_SEASON_NAMES: [&str; 4] = ["一季度", "二季度", "三季度", "四季度"];

/// 公历季度
#[derive(Debug, Copy, Clone)]
pub struct SolarSeason {
  /// 公历年
  year: SolarYear,
  /// 索引，0-1
  index: usize,
}

impl Tyme for SolarSeason {
  fn next(&self, n: isize) -> Self {
    if n == 0 {
      return self.clone();
    }
    let mut i: isize = self.index as isize + n;
    let mut y: isize = self.year.get_year() + i / 4;
    i %= 4;
    if i < 0 {
      i += 4;
      y -= 1;
    }
    Self::from_index(y, i as usize)
  }
}

impl Culture for SolarSeason {
  fn get_name(&self) -> String {
    SOLAR_SEASON_NAMES[self.index].to_string()
  }
}

impl SolarSeason {
  pub fn new(year: isize, index: usize) -> Result<Self, String> {
    if index > 3 {
      Err(format!("illegal solar season index: {}", index))
    } else {
      Ok(Self {
        year: SolarYear::from_year(year),
        index,
      })
    }
  }

  pub fn from_index(year: isize, index: usize) -> Self {
    Self::new(year, index).unwrap()
  }

  /// 公历年
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarSeason, SolarYear};
  ///
  /// // 2023年一季度
  /// let season: SolarSeason = SolarSeason::from_index(2023, 0);
  ///
  /// // 2023年
  /// let year: SolarYear = season.get_solar_year();
  /// ```
  pub fn get_solar_year(&self) -> SolarYear {
    self.year
  }

  /// 年
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarSeason};
  ///
  /// // 2023年一季度
  /// let season: SolarSeason = SolarSeason::from_index(2023, 0);
  ///
  /// // 2023
  /// let year: isize = season.get_year();
  /// ```
  pub fn get_year(&self) -> isize {
    self.year.get_year()
  }

  /// 索引
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarSeason};
  ///
  /// // 2023年一季度
  /// let season: SolarSeason = SolarSeason::from_index(2023, 0);
  ///
  /// // 0
  /// let index: usize = season.get_index();
  /// ```
  pub fn get_index(&self) -> usize {
    self.index
  }

  /// 公历月列表
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarSeason, SolarMonth};
  ///
  /// // 2023年一季度
  /// let season: SolarSeason = SolarSeason::from_index(2023, 0);
  ///
  /// // 1,2,3月
  /// let months: Vec<SolarMonth> = season.get_months();
  /// ```
  pub fn get_months(&self) -> Vec<SolarMonth> {
    let mut l: Vec<SolarMonth> = Vec::new();
    let y: isize = self.get_year();
    for i in 1..4 {
      l.push(SolarMonth::from_ym(y, self.index * 3 + i));
    }
    l
  }
}

impl Display for SolarSeason {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.year, self.get_name())
  }
}

impl PartialEq for SolarSeason {
  fn eq(&self, other: &Self) -> bool {
    self.get_year() == other.get_year() && self.get_index() == other.get_index()
  }
}

impl Eq for SolarSeason {}

/// 公历月名称
pub static SOLAR_MONTH_NAMES: [&str; 12] = ["1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月", "11月", "12月"];

/// 公历每月天数
pub static SOLAR_MONTH_DAYS: [usize; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// 公历月
#[derive(Debug, Copy, Clone)]
pub struct SolarMonth {
  parent: AbstractTyme,
  /// 公历年
  year: SolarYear,
  /// 月
  month: usize,
}

impl Tyme for SolarMonth {
  fn next(&self, n: isize) -> Self {
    if n == 0 {
      self.clone()
    } else {
      let mut m: isize = (self.month as isize) + n;
      let mut y: isize = self.year.get_year() + m / 12;
      m %= 12;
      if m < 1 {
        m += 12;
        y -= 1
      }
      Self::from_ym(y, m as usize)
    }
  }
}

impl Culture for SolarMonth {
  fn get_name(&self) -> String {
    SOLAR_MONTH_NAMES[self.get_index_in_year()].to_string()
  }
}

impl SolarMonth {
  pub fn new(year: isize, month: usize) -> Result<Self, String> {
    if month < 1 || month > 12 {
      Err(format!("illegal solar month: {}", month))
    } else {
      Ok(Self {
        parent: AbstractTyme::new(),
        year: SolarYear::from_year(year),
        month,
      })
    }
  }

  pub fn from_ym(year: isize, month: usize) -> Self {
    Self::new(year, month).unwrap()
  }

  /// 公历年
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarMonth, SolarYear};
  ///
  /// // 2023年1月
  /// let month: SolarMonth = SolarMonth::from_ym(2023, 1);
  ///
  /// // 2023年
  /// let year: SolarYear = month.get_solar_year();
  /// ```
  pub fn get_solar_year(&self) -> SolarYear {
    self.year
  }

  /// 年
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarMonth};
  ///
  /// // 2023年1月
  /// let month: SolarMonth = SolarMonth::from_ym(2023, 1);
  ///
  /// // 2023
  /// let year: isize = month.get_year();
  /// ```
  pub fn get_year(&self) -> isize {
    self.year.get_year()
  }

  /// 月
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarMonth};
  ///
  /// // 2023年1月
  /// let month: SolarMonth = SolarMonth::from_ym(2023, 1);
  ///
  /// // 1
  /// let m: usize = month.get_month();
  /// ```
  pub fn get_month(&self) -> usize {
    self.month
  }

  /// 当月天数
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarMonth};
  ///
  /// // 2023年1月
  /// let month: SolarMonth = SolarMonth::from_ym(2023, 1);
  ///
  /// // 31
  /// let count: usize = month.get_day_count();
  /// ```
  pub fn get_day_count(&self) -> usize {
    if 1582 == self.get_year() && 10 == self.month {
      21
    } else {
      let mut d: usize = SOLAR_MONTH_DAYS[self.get_index_in_year()];
      //公历闰年2月多一天
      if 2 == self.month && self.year.is_leap() {
        d += 1
      }
      d
    }
  }

  /// 位于当年的月索引
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarMonth};
  ///
  /// // 2023年1月
  /// let month: SolarMonth = SolarMonth::from_ym(2023, 1);
  ///
  /// // 0
  /// let index: usize = month.get_index_in_year();
  /// ```
  pub fn get_index_in_year(&self) -> usize {
    self.month - 1
  }

  /// 当月周数
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarMonth};
  ///
  /// // 2024年6月
  /// let month: SolarMonth = SolarMonth::from_ym(2024, 6);
  ///
  /// // 6
  /// let count: usize = month.get_week_count(0);
  /// ```
  pub fn get_week_count(&self, start: usize) -> usize {
    let culture: AbstractCulture = self.parent.into();
    ((culture.index_of(SolarDay::from_ymd(self.get_year(), self.month, 1).get_week().get_index() as isize - start as isize, 7) + self.get_day_count()) as f64 / 7f64).ceil() as usize
  }

  /// 公历季度
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarMonth, SolarSeason};
  ///
  /// // 2023年1月
  /// let month: SolarMonth = SolarMonth::from_ym(2023, 1);
  ///
  /// // 一季度
  /// let season: SolarSeason = month.get_season();
  /// ```
  pub fn get_season(&self) -> SolarSeason {
    SolarSeason::from_index(self.get_year(), self.get_index_in_year() / 3)
  }

  /// 公历周列表
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarMonth, SolarWeek};
  ///
  /// // 2024年6月
  /// let month: SolarMonth = SolarMonth::from_ym(2024, 6);
  ///
  /// // 第一周到第六周
  /// let weeks: Vec<SolarWeek> = month.get_weeks(0);
  /// ```
  pub fn get_weeks(&self, start: usize) -> Vec<SolarWeek> {
    let y: isize = self.get_year();
    let mut l: Vec<SolarWeek> = Vec::new();
    for i in 0..self.get_week_count(start) {
      l.push(SolarWeek::from_ym(y, self.month, i, start));
    }
    l
  }

  /// 公历日列表
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarDay, SolarMonth};
  ///
  /// // 2024年6月
  /// let month: SolarMonth = SolarMonth::from_ym(2024, 6);
  ///
  /// // 1日到30日
  /// let days: Vec<SolarDay> = month.get_days();
  /// ```
  pub fn get_days(&self) -> Vec<SolarDay> {
    let y: isize = self.get_year();
    let mut l: Vec<SolarDay> = Vec::new();
    for i in 1..self.get_day_count() + 1 {
      l.push(SolarDay::from_ymd(y, self.month, i));
    }
    l
  }
}

impl Display for SolarMonth {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.year, self.get_name())
  }
}

impl PartialEq for SolarMonth {
  fn eq(&self, other: &Self) -> bool {
    self.get_year() == other.get_year() && self.get_month() == other.get_month()
  }
}

impl Eq for SolarMonth {}

impl Into<AbstractTyme> for SolarMonth {
  fn into(self) -> AbstractTyme {
    self.parent
  }
}

/// 公历周名称
pub static SOLAR_WEEK_NAMES: [&str; 6] = ["第一周", "第二周", "第三周", "第四周", "第五周", "第六周"];

/// 公历周
#[derive(Debug, Clone)]
pub struct SolarWeek {
  parent: AbstractTyme,
  /// 公历月
  month: SolarMonth,
  /// 索引，0-6
  index: usize,
  /// 起始星期
  start: Week,
}

impl Tyme for SolarWeek {
  fn next(&self, n: isize) -> Self {
    if n == 0 {
      self.clone()
    } else {
      let mut d: isize = (self.index as isize) + n;
      let mut m: SolarMonth = self.month;
      let start_index: usize = self.start.get_index();
      if n > 0 {
        let mut week_count: isize = m.get_week_count(start_index) as isize;
        while d >= week_count {
          d -= week_count;
          m = m.next(1);
          if SolarDay::from_ymd(m.get_year(), m.get_month(), 1).get_week() != self.start {
            d += 1;
          }
          week_count = m.get_week_count(start_index) as isize;
        }
      } else {
        while d < 0 {
          if SolarDay::from_ymd(m.get_year(), m.get_month(), 1).get_week() != self.start {
            d -= 1;
          }
          m = m.next(-1);
          d += m.get_week_count(start_index) as isize;
        }
      }
      Self::from_ym(m.get_year(), m.get_month(), d as usize, start_index)
    }
  }
}

impl Culture for SolarWeek {
  fn get_name(&self) -> String {
    SOLAR_WEEK_NAMES[self.index].to_string()
  }
}

impl SolarWeek {
  pub fn new(year: isize, month: usize, index: usize, start: usize) -> Result<Self, String> {
    if index > 5 {
      Err(format!("illegal solar week index: {}", index))
    } else if start > 6 {
      Err(format!("illegal solar week start: {}", start))
    } else {
      let m: SolarMonth = SolarMonth::from_ym(year, month);
      if index >= m.get_week_count(start) {
        Err(format!("illegal solar week index: {} in month: {}", index, m))
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

  pub fn from_ym(year: isize, month: usize, index: usize, start: usize) -> Self {
    Self::new(year, month, index, start).unwrap()
  }

  /// 公历月
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarMonth, SolarWeek};
  ///
  /// // 2023年1月第一周
  /// let week: SolarWeek = SolarWeek::from_ym(2023, 1, 0, 0);
  ///
  /// // 2023年1月
  /// let m: SolarMonth = week.get_solar_month();
  /// ```
  pub fn get_solar_month(&self) -> SolarMonth {
    self.month
  }

  /// 年
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarWeek};
  ///
  /// // 2023年1月第一周
  /// let week: SolarWeek = SolarWeek::from_ym(2023, 1, 0, 0);
  ///
  /// // 2023
  /// let y: isize = week.get_year();
  /// ```
  pub fn get_year(&self) -> isize {
    self.month.get_year()
  }

  /// 月
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarWeek};
  ///
  /// // 2023年1月第一周
  /// let week: SolarWeek = SolarWeek::from_ym(2023, 1, 0, 0);
  ///
  /// // 1
  /// let y: usize = week.get_month();
  /// ```
  pub fn get_month(&self) -> usize {
    self.month.get_month()
  }

  pub fn get_index(&self) -> usize {
    self.index
  }

  pub fn get_start(&self) -> Week {
    self.start.clone()
  }

  pub fn get_first_day(&self) -> SolarDay {
    let first_day: SolarDay = SolarDay::from_ymd(self.get_year(), self.get_month(), 1);
    let parent: AbstractTyme = self.parent.into();
    let culture: AbstractCulture = parent.into();
    first_day.next(self.index as isize * 7 - culture.index_of((first_day.get_week().get_index() as isize) - (self.start.get_index() as isize), 7) as isize)
  }

  pub fn get_days(&self) -> Vec<SolarDay> {
    let mut l: Vec<SolarDay> = Vec::new();
    let d: SolarDay = self.get_first_day();
    l.push(d);
    for i in 1..7 {
      l.push(d.next(i));
    }
    l
  }

  /// 位于当年的索引（从0开始）
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarWeek};
  ///
  /// // 0
  /// let index: usize = SolarWeek::from_ym(2000, 1, 0, 0).get_index_in_year();
  /// ```
  pub fn get_index_in_year(&self) -> usize {
    let mut i: usize = 0;
    let first_day: SolarDay = self.get_first_day();
    // 今年第1周
    let mut w: SolarWeek = SolarWeek::from_ym(self.get_year(), 1, 0, self.start.get_index());
    while w.get_first_day() != first_day {
      w = w.next(1);
      i += 1;
    }
    i
  }
}

impl Display for SolarWeek {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.month, self.get_name())
  }
}

impl PartialEq for SolarWeek {
  fn eq(&self, other: &Self) -> bool {
    self.get_first_day() == other.get_first_day()
  }
}

impl Eq for SolarWeek {}

impl Into<AbstractTyme> for SolarWeek {
  fn into(self) -> AbstractTyme {
    self.parent
  }
}

/// 公历日名称
pub static SOLAR_DAY_NAMES: [&str; 31] = ["1日", "2日", "3日", "4日", "5日", "6日", "7日", "8日", "9日", "10日", "11日", "12日", "13日", "14日", "15日", "16日", "17日", "18日", "19日", "20日", "21日", "22日", "23日", "24日", "25日", "26日", "27日", "28日", "29日", "30日", "31日"];

/// 公历日
#[derive(Debug, Copy, Clone)]
pub struct SolarDay {
  /// 公历月
  month: SolarMonth,
  /// 日
  day: usize,
}

impl Tyme for SolarDay {
  fn next(&self, n: isize) -> Self {
    self.get_julian_day().next(n).get_solar_day()
  }
}

impl Culture for SolarDay {
  fn get_name(&self) -> String {
    SOLAR_DAY_NAMES[self.day - 1].to_string()
  }
}

impl SolarDay {
  pub fn new(year: isize, month: usize, day: usize) -> Result<Self, String> {
    let m: SolarMonth = SolarMonth::from_ym(year, month);
    if day < 1 {
      Err(format!("illegal solar day: {}-{}-{}", year, month, day))
    } else if 1582 == year && 10 == month {
      if (day > 4 && day < 15) || day > 31 {
        Err(format!("illegal solar day: {}-{}-{}", year, month, day))
      } else {
        Ok(Self {
          month: m,
          day,
        })
      }
    } else if day > m.get_day_count() {
      Err(format!("illegal solar day: {}-{}-{}", year, month, day))
    } else {
      Ok(Self {
        month: m,
        day,
      })
    }
  }

  pub fn from_ymd(year: isize, month: usize, day: usize) -> Self {
    Self::new(year, month, day).unwrap()
  }

  /// 公历月
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarDay, SolarMonth};
  /// // 2000年1月
  /// let month: SolarMonth = SolarDay::from_ymd(2000, 1, 29).get_solar_month();
  /// ```
  pub fn get_solar_month(&self) -> SolarMonth {
    self.month
  }

  /// 年
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::SolarDay;
  ///
  /// // 2000
  /// let y: isize = SolarDay::from_ymd(2000, 1, 29).get_year();
  /// ```
  pub fn get_year(&self) -> isize {
    self.month.get_year()
  }

  /// 月
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::SolarDay;
  ///
  /// // 1
  /// let m: usize = SolarDay::from_ymd(2000, 1, 29).get_month();
  /// ```
  pub fn get_month(&self) -> usize {
    self.month.get_month()
  }

  /// 日
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::SolarDay;
  ///
  /// // 29
  /// let day: usize = SolarDay::from_ymd(2000, 1, 29).get_day();
  /// ```
  pub fn get_day(&self) -> usize {
    self.day
  }

  /// 星期
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::culture::Week;
  /// use tyme4rs::tyme::solar::SolarDay;
  ///
  /// let week: Week = SolarDay::from_ymd(1986, 5, 29).get_week();
  /// ```
  pub fn get_week(&self) -> Week {
    self.get_julian_day().get_week()
  }

  /// 公历周
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarDay, SolarWeek};
  ///
  /// // 2000年1月第一周
  /// let week: SolarWeek = SolarDay::from_ymd(2000, 1, 1).get_solar_week(0);
  /// ```
  pub fn get_solar_week(&self, start: usize) -> SolarWeek {
    let y: isize = self.get_year();
    let m: usize = self.get_month();
    SolarWeek::from_ym(y, m, ((self.day + SolarDay::from_ymd(y, m, 1).get_week().next(-(start as isize)).get_index()) as f64 / 7.0).ceil() as usize - 1, start)
  }

  /// 节气
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarDay, SolarTerm};
  ///
  /// let term: SolarTerm = SolarDay::from_ymd(1986, 5, 29).get_term();
  /// ```
  pub fn get_term(&self) -> SolarTerm {
    self.get_term_day().get_solar_term()
  }

  /// 节气第几天
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarDay, SolarTerm, SolarTermDay};
  ///
  /// let term_day: SolarTermDay = SolarDay::from_ymd(2023, 12, 7).get_term_day();
  ///
  /// // 大雪
  /// let term: SolarTerm = term_day.get_solar_term();
  ///
  /// // 0
  /// let index: usize = term_day.get_day_index();
  /// ```
  pub fn get_term_day(&self) -> SolarTermDay {
    let mut y: isize = self.get_year();
    let mut i: usize = self.get_month() * 2;
    if i == 24 {
      y += 1;
      i = 0;
    }
    let mut term: SolarTerm = SolarTerm::from_index(y, i as isize);
    let mut day: SolarDay = term.get_julian_day().get_solar_day();
    while self.is_before(day) {
      term = term.next(-1);
      day = term.get_julian_day().get_solar_day();
    }
    SolarTermDay::new(term, self.subtract(day) as usize)
  }

  /// 儒略日
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::jd::JulianDay;
  /// use tyme4rs::tyme::solar::SolarDay;
  ///
  /// let julian_day: JulianDay = SolarDay::from_ymd(2023, 12, 7).get_julian_day();
  /// ```
  pub fn get_julian_day(&self) -> JulianDay {
    JulianDay::from_ymd_hms(self.get_year(), self.get_month(), self.day, 0, 0, 0)
  }

  pub fn is_before(&self, target: SolarDay) -> bool {
    let a_year: isize = self.get_year();
    let b_year: isize = target.get_year();
    if a_year != b_year {
      return a_year < b_year;
    }
    let a_month: usize = self.get_month();
    let b_month: usize = target.get_month();
    if a_month != b_month { a_month < b_month } else { self.day < target.get_day() }
  }

  pub fn is_after(&self, target: SolarDay) -> bool {
    let a_year: isize = self.get_year();
    let b_year: isize = target.get_year();
    if a_year != b_year {
      return a_year > b_year;
    }
    let a_month: usize = self.get_month();
    let b_month: usize = target.get_month();
    if a_month != b_month { a_month > b_month } else { self.day > target.get_day() }
  }

  /// 位于当年的索引
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::SolarDay;
  ///
  /// // 9
  /// let index_in_year: usize = SolarDay::from_ymd(2023, 1, 10).get_index_in_year();
  /// ```
  pub fn get_index_in_year(&self) -> usize {
    self.subtract(Self::from_ymd(self.get_year(), 1, 1)) as usize
  }

  /// 公历日相减
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::SolarDay;
  ///
  /// // 9
  /// let v: isize = SolarDay::from_ymd(2023, 1, 10).subtract(SolarDay::from_ymd(2023, 1, 1));
  /// ```
  pub fn subtract(&self, target: SolarDay) -> isize {
    (self.get_julian_day().subtract(target.get_julian_day())) as isize
  }

  /// 农历日
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::lunar::LunarDay;
  /// use tyme4rs::tyme::solar::SolarDay;
  ///
  /// // 公历日转农历日
  /// let lunar_day: LunarDay = SolarDay::from_ymd(1986, 5, 29).get_lunar_day();
  /// ```
  pub fn get_lunar_day(&self) -> LunarDay {
    let mut m: LunarMonth = LunarMonth::from_ym(self.get_year(), self.get_month() as isize);
    let mut days: isize = self.subtract(m.get_first_julian_day().get_solar_day());
    while days < 0 {
      m = m.next(-1);
      days += m.get_day_count() as isize;
    }
    LunarDay::from_ymd(m.get_year(), m.get_month_with_leap(), (days + 1) as usize)
  }

  /// 星座
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::culture::Constellation;
  /// use tyme4rs::tyme::solar::SolarDay;
  ///
  /// let constellation: Constellation = SolarDay::from_ymd(2023, 9, 12).get_constellation();
  /// ```
  pub fn get_constellation(&self) -> Constellation {
    let mut index: isize = 11;
    let y: usize = self.get_month() * 100 + self.day;
    if y >= 321 && y <= 419 {
      index = 0;
    } else if y >= 420 && y <= 520 {
      index = 1;
    } else if y >= 521 && y <= 621 {
      index = 2;
    } else if y >= 622 && y <= 722 {
      index = 3;
    } else if y >= 723 && y <= 822 {
      index = 4;
    } else if y >= 823 && y <= 922 {
      index = 5;
    } else if y >= 923 && y <= 1023 {
      index = 6;
    } else if y >= 1024 && y <= 1122 {
      index = 7;
    } else if y >= 1123 && y <= 1221 {
      index = 8;
    } else if y >= 1222 || y <= 119 {
      index = 9;
    } else if y <= 218 {
      index = 10;
    }
    Constellation::from_index(index)
  }

  /// 三伏天
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::culture::dog::DogDay;
  /// use tyme4rs::tyme::solar::SolarDay;
  ///
  /// let dog_day: Option<DogDay> = SolarDay::from_ymd(2023, 9, 12).get_dog_day();
  /// ```
  pub fn get_dog_day(&self) -> Option<DogDay> {
    let xia_zhi: SolarTerm = SolarTerm::from_index(self.get_year(), 12);
    // 第1个庚日
    let mut start: SolarDay = xia_zhi.get_julian_day().get_solar_day();
    let mut add: isize = 6 - start.get_lunar_day().get_sixty_cycle().get_heaven_stem().get_index() as isize;
    if add < 0 {
      add += 10;
    }
    // 第3个庚日，即初伏第1天
    add += 20;
    start = start.next(add);
    let mut days: isize = self.subtract(start);
    // 初伏以前
    if days < 0 {
      return None;
    }
    if days < 10 {
      return Some(DogDay::new(Dog::from_index(0), days as usize));
    }
    // 第4个庚日，中伏第1天
    start = start.next(10);
    days = self.subtract(start);
    if days < 10 {
      return Some(DogDay::new(Dog::from_index(1), days as usize));
    }
    // 第5个庚日，中伏第11天或末伏第1天
    start = start.next(10);
    days = self.subtract(start);
    // 立秋
    if xia_zhi.next(3).get_julian_day().get_solar_day().is_after(start) {
      if days < 10 {
        return Some(DogDay::new(Dog::from_index(1), days as usize + 10));
      }
      start = start.next(10);
      days = self.subtract(start);
    }
    if days < 10 {
      return Some(DogDay::new(Dog::from_index(2), days as usize));
    }
    None
  }

  /// 数九天
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::culture::nine::NineDay;
  /// use tyme4rs::tyme::solar::SolarDay;
  ///
  /// let nine_day: Option<NineDay> = SolarDay::from_ymd(2023, 12, 26).get_nine_day();
  /// ```
  pub fn get_nine_day(&self) -> Option<NineDay> {
    let year: isize = self.get_year();
    let mut start: SolarDay = SolarTerm::from_index(year + 1, 0).get_julian_day().get_solar_day();
    if self.is_before(start) {
      start = SolarTerm::from_index(year, 0).get_julian_day().get_solar_day();
    }
    let end: SolarDay = start.next(81);
    if self.is_before(start) || !self.is_before(end) {
      return None;
    }
    let days: isize = self.subtract(start);
    Some(NineDay::new(Nine::from_index(days / 9), days as usize % 9))
  }

  /// 七十二候
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::culture::phenology::PhenologyDay;
  /// use tyme4rs::tyme::solar::SolarDay;
  ///
  /// let phenology_day: PhenologyDay = SolarDay::from_ymd(2023, 12, 26).get_phenology_day();
  /// ```
  pub fn get_phenology_day(&self) -> PhenologyDay {
    let term: SolarTerm = self.get_term();
    let mut day_index: isize = self.subtract(term.get_julian_day().get_solar_day());
    let mut index: isize = day_index / 5;
    if index > 2 {
      index = 2;
    }
    day_index -= index * 5;
    PhenologyDay::new(Phenology::from_index(term.get_index() as isize * 3 + index), day_index as usize)
  }

  pub fn get_plum_rain_day(&self) -> Option<PlumRainDay> {
    // 芒种
    let grain_in_ear: SolarTerm = SolarTerm::from_index(self.get_year(), 11);
    let mut start: SolarDay = grain_in_ear.get_julian_day().get_solar_day();
    let mut add: isize = 2 - start.get_lunar_day().get_sixty_cycle().get_heaven_stem().get_index() as isize;
    if add < 0 {
      add += 10;
    }
    // 芒种后的第1个丙日
    start = start.next(add);

    // 小暑
    let slight_heat: SolarTerm = grain_in_ear.next(2);
    let mut end: SolarDay = slight_heat.get_julian_day().get_solar_day();
    add = 7 - end.get_lunar_day().get_sixty_cycle().get_earth_branch().get_index() as isize;
    if add < 0 {
      add += 12;
    }
    // 小暑后的第1个未日
    end = end.next(add);

    if self.is_before(start) || self.is_after(end) {
      return None;
    }
    Some(if self.eq(&end) { PlumRainDay::new(PlumRain::from_index(1), 0) } else { PlumRainDay::new(PlumRain::from_index(0), self.subtract(start) as usize)})
  }

  /// 法定假日
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::holiday::LegalHoliday;
  /// use tyme4rs::tyme::solar::SolarDay;
  ///
  /// let legal_holiday: Option<LegalHoliday> = SolarDay::from_ymd(2024, 10, 1).get_legal_holiday();
  /// ```
  pub fn get_legal_holiday(&self) -> Option<LegalHoliday> {
    LegalHoliday::from_ymd(self.get_year(), self.get_month(), self.day)
  }

  /// 公历现代节日
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::festival::SolarFestival;
  /// use tyme4rs::tyme::solar::SolarDay;
  ///
  /// let festival: Option<SolarFestival> = SolarDay::from_ymd(2024, 10, 1).get_festival();
  /// ```
  pub fn get_festival(&self) -> Option<SolarFestival> {
    SolarFestival::from_ymd(self.get_year(), self.get_month(), self.day)
  }
}

impl Display for SolarDay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.month, self.get_name())
  }
}

impl PartialEq for SolarDay {
  fn eq(&self, other: &Self) -> bool {
    self.get_year() == other.get_year() && self.get_month() == other.get_month() && self.get_day() == other.get_day()
  }
}

impl Eq for SolarDay {}

/// 公历时刻
#[derive(Debug, Copy, Clone)]
pub struct SolarTime {
  /// 公历日
  day: SolarDay,
  /// 时
  hour: usize,
  /// 分
  minute: usize,
  /// 秒
  second: usize,
}

impl Tyme for SolarTime {
  fn next(&self, n: isize) -> Self {
    if n == 0 {
      self.clone()
    } else {
      let mut ts: isize = (self.second as isize) + n;
      let mut tm: isize = (self.minute as isize) + ts / 60;
      ts %= 60;
      if ts < 0 {
        ts += 60;
        tm -= 1;
      }
      let mut th: isize = (self.hour as isize) + tm / 60;
      tm %= 60;
      if tm < 0 {
        tm += 60;
        th -= 1;
      }
      let mut td: isize = th / 24;
      th %= 24;
      if th < 0 {
        th += 24;
        td -= 1;
      }

      let d: SolarDay = self.day.next(td);
      return Self::from_ymd_hms(d.get_year(), d.get_month(), d.get_day(), th as usize, tm as usize, ts as usize);
    }
  }
}

impl Culture for SolarTime {
  fn get_name(&self) -> String {
    format!("{:0>2}:{:0>2}:{:0>2}", self.hour, self.minute, self.second)
  }
}

impl SolarTime {
  pub fn new(year: isize, month: usize, day: usize, hour: usize, minute: usize, second: usize) -> Result<Self, String> {
    if hour > 23 {
      Err(format!("illegal hour: {}", hour))
    } else if minute > 59 {
      Err(format!("illegal minute: {}", minute))
    } else if second > 59 {
      Err(format!("illegal second: {}", second))
    } else {
      Ok(Self {
        day: SolarDay::from_ymd(year, month, day),
        hour,
        minute,
        second,
      })
    }
  }

  pub fn from_ymd_hms(year: isize, month: usize, day: usize, hour: usize, minute: usize, second: usize) -> Self {
    Self::new(year, month, day, hour, minute, second).unwrap()
  }

  /// 公历日
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarDay, SolarTime};
  ///
  /// // 2000年1月29日
  /// let day: SolarDay = SolarTime::from_ymd_hms(2000, 1, 29, 12, 0, 0).get_solar_day();
  /// ```
  pub fn get_solar_day(&self) -> SolarDay {
    self.day
  }

  /// 年
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarTime};
  ///
  /// // 2000
  /// let y: isize = SolarTime::from_ymd_hms(2000, 1, 29, 12, 0, 0).get_year();
  /// ```
  pub fn get_year(&self) -> isize {
    self.day.get_year()
  }

  /// 月
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarTime};
  ///
  /// // 1
  /// let m: usize = SolarTime::from_ymd_hms(2000, 1, 29, 12, 0, 0).get_month();
  /// ```
  pub fn get_month(&self) -> usize {
    self.day.get_month()
  }

  /// 日
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarTime};
  ///
  /// // 29
  /// let day: usize = SolarTime::from_ymd_hms(2000, 1, 29, 12, 0, 0).get_day();
  /// ```
  pub fn get_day(&self) -> usize {
    self.day.get_day()
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

  pub fn is_before(&self, target: SolarTime) -> bool {
    if self.day != target.get_solar_day() {
      return self.day.is_before(target.get_solar_day());
    }
    if self.hour != target.get_hour() {
      return self.hour < target.get_hour();
    }
    if self.minute != target.get_minute() { self.minute < target.get_minute() } else { self.second < target.get_second() }
  }

  pub fn is_after(&self, target: SolarTime) -> bool {
    if self.day != target.get_solar_day() {
      return self.day.is_after(target.get_solar_day());
    }
    if self.hour != target.get_hour() {
      return self.hour > target.get_hour();
    }
    if self.minute != target.get_minute() { self.minute > target.get_minute() } else { self.second > target.get_second() }
  }

  /// 节气
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarTerm, SolarTime};
  ///
  /// let term: SolarTerm = SolarTime::from_ymd_hms(2023, 12, 7, 13, 20, 0).get_term();
  /// ```
  pub fn get_term(&self) -> SolarTerm {
    let mut y: isize = self.get_year();
    let mut i: usize = self.get_month() * 2;
    if i == 24 {
      y += 1;
      i = 0;
    }
    let mut term: SolarTerm = SolarTerm::from_index(y, i as isize);
    while self.is_before(term.get_julian_day().get_solar_time()) {
      term = term.next(-1);
    }
    term
  }

  /// 儒略日
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::jd::JulianDay;
  /// use tyme4rs::tyme::solar::SolarTime;
  ///
  /// let jd: JulianDay = SolarTime::from_ymd_hms(2023, 12, 7, 13, 20, 0).get_julian_day();
  /// ```
  pub fn get_julian_day(&self) -> JulianDay {
    JulianDay::from_ymd_hms(self.get_year(), self.get_month(), self.get_day(), self.hour, self.minute, self.second)
  }

  pub fn subtract(&self, target: SolarTime) -> isize {
    let mut days: isize = self.day.subtract(target.get_solar_day());
    let cs: usize = self.hour * 3600 + self.minute * 60 + self.second;
    let ts: usize = target.get_hour() * 3600 + target.get_minute() * 60 + target.get_second();
    let mut seconds: isize = cs as isize - ts as isize;
    if seconds < 0 {
      seconds += 86400;
      days -= 1;
    }
    seconds += days * 86400;
    return seconds;
  }

  pub fn get_lunar_hour(&self) -> LunarHour {
    let d: LunarDay = self.day.get_lunar_day();
    LunarHour::from_ymd_hms(d.get_year(), d.get_month(), d.get_day(), self.hour, self.minute, self.second)
  }
}

impl Display for SolarTime {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", self.day, self.get_name())
  }
}

impl PartialEq for SolarTime {
  fn eq(&self, other: &Self) -> bool {
    self.get_solar_day() == other.get_solar_day() && self.get_hour() == other.get_hour() && self.get_minute() == other.get_minute() && self.get_second() == other.get_second()
  }
}

impl Eq for SolarTime {}

/// 节气名称
pub static SOLAR_TERM_NAMES: [&str; 24] = ["冬至", "小寒", "大寒", "立春", "雨水", "惊蛰", "春分", "清明", "谷雨", "立夏", "小满", "芒种", "夏至", "小暑", "大暑", "立秋", "处暑", "白露", "秋分", "寒露", "霜降", "立冬", "小雪", "大雪"];

/// 节气
#[derive(Debug, Clone)]
pub struct SolarTerm {
  parent: LoopTyme,
  /// 粗略的儒略日
  cursory_julian_day: f64,
}

impl Tyme for SolarTerm {
  fn next(&self, n: isize) -> Self {
    Self::from_cursory_julian_day(self.cursory_julian_day + 15.2184 * (n as f64), self.parent.next_index(n) as isize)
  }
}

impl Culture for SolarTerm {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl SolarTerm {
  pub fn from_index(year: isize, index: isize) -> Self {
    let parent: LoopTyme = LoopTyme::from_index(SOLAR_TERM_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index);
    let jd: f64 = ((year as f64 - 2000.0) * 365.2422 + 180.0).floor();
    // 355是2000.12冬至，得到较靠近jd的冬至估计值
    let mut w: f64 = ((jd - 355.0 + 183.0) / 365.2422).floor() * 365.2422 + 355.0;
    if ShouXingUtil::calc_qi(w) > jd {
      w -= 365.2422;
    }
    Self {
      parent,
      cursory_julian_day: ShouXingUtil::calc_qi(w + 15.2184 * index as f64),
    }
  }

  pub fn new(year: isize, name: &str) -> Result<Self, String> {
    let parent: LoopTyme = LoopTyme::from_name(SOLAR_TERM_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name);
    let jd: f64 = (((year - 2000) as f64) * 365.2422 + 180.0).floor();
    // 355是2000.12冬至，得到较靠近jd的冬至估计值
    let mut w: f64 = ((jd - 355.0 + 183.0) / 365.2422).floor() * 365.2422 + 355.0;
    if ShouXingUtil::calc_qi(w) > jd {
      w -= 365.2422;
    }
    let index: usize = parent.get_index();
    Ok(Self {
      parent,
      cursory_julian_day: ShouXingUtil::calc_qi(w + 15.2184 * (index as f64)),
    })
  }

  pub fn from_name(year: isize, name: &str) -> Self {
    Self::new(year, name).unwrap()
  }

  fn from_cursory_julian_day(cursory_julian_day: f64, index: isize) -> Self {
    let parent: LoopTyme = LoopTyme::from_index(SOLAR_TERM_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index);
    Self {
      parent,
      cursory_julian_day,
    }
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  /// 是否节令
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarDay, SolarTerm};
  ///
  /// let term: SolarTerm = SolarDay::from_ymd(2024, 10, 1).get_term();
  ///
  /// let is_jie: bool = term.is_jie();
  /// ```
  pub fn is_jie(&self) -> bool {
    self.get_index() % 2 == 1
  }

  /// 是否气令
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarDay, SolarTerm};
  ///
  /// let term: SolarTerm = SolarDay::from_ymd(2024, 10, 1).get_term();
  ///
  /// let is_jie: bool = term.is_qi();
  /// ```
  pub fn is_qi(&self) -> bool {
    self.get_index() % 2 == 0
  }

  /// 儒略日
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::jd::JulianDay;
  /// use tyme4rs::tyme::solar::{SolarDay, SolarTerm};
  ///
  /// let term: SolarTerm = SolarDay::from_ymd(2024, 10, 1).get_term();
  ///
  /// let julian_day: JulianDay = term.get_julian_day();
  /// ```
  pub fn get_julian_day(&self) -> JulianDay {
    JulianDay::from_julian_day(ShouXingUtil::qi_accurate2(self.cursory_julian_day) + J2000)
  }

  /// 粗略的儒略日
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::jd::JulianDay;
  /// use tyme4rs::tyme::solar::{SolarDay, SolarTerm};
  ///
  /// let term: SolarTerm = SolarDay::from_ymd(2024, 10, 1).get_term();
  ///
  /// let cursory_julian_day: f64 = term.get_cursory_julian_day();
  /// ```
  pub fn get_cursory_julian_day(&self) -> f64 {
    self.cursory_julian_day
  }
}

impl Display for SolarTerm {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for SolarTerm {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for SolarTerm {}

impl Into<LoopTyme> for SolarTerm {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

/// 节气第几天
#[derive(Debug, Clone)]
pub struct SolarTermDay {
  parent: AbstractCultureDay,
  /// 节气
  solar_term: SolarTerm,
}

impl Culture for SolarTermDay {
  fn get_name(&self) -> String {
    self.solar_term.get_name()
  }
}

impl SolarTermDay {
  pub fn new(solar_term: SolarTerm, day_index: usize) -> Self {
    let loop_tyme: LoopTyme = solar_term.clone().into();
    let abstract_tyme: AbstractTyme = loop_tyme.into();
    let culture: AbstractCulture = abstract_tyme.into();
    Self {
      parent: AbstractCultureDay::new(culture, day_index),
      solar_term,
    }
  }

  /// 节气
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarDay, SolarTerm, SolarTermDay};
  ///
  /// let term_day: SolarTermDay = SolarDay::from_ymd(2024, 10, 1).get_term_day();
  ///
  /// let term: SolarTerm = term_day.get_solar_term();
  /// ```
  pub fn get_solar_term(&self) -> SolarTerm {
    self.solar_term.clone()
  }

  /// 天索引
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::{SolarDay, SolarTerm, SolarTermDay};
  ///
  /// // 大雪第1天
  /// let term_day: SolarTermDay = SolarDay::from_ymd(2023, 12, 7).get_term_day();
  ///
  /// // 0
  /// let day_index: usize = term_day.get_day_index();
  /// ```
  pub fn get_day_index(&self) -> usize {
    self.parent.get_day_index()
  }
}

impl Display for SolarTermDay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}第{}天", self.get_name(), self.parent.get_day_index() + 1)
  }
}

impl PartialEq for SolarTermDay {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for SolarTermDay {}

impl Into<AbstractCultureDay> for SolarTermDay {
  fn into(self) -> AbstractCultureDay {
    self.parent
  }
}

#[cfg(test)]
mod tests {
  use crate::tyme::{Culture, Tyme};
  use crate::tyme::lunar::LunarWeek;
  use crate::tyme::solar::{SolarDay, SolarHalfYear, SolarMonth, SolarSeason, SolarTerm, SolarTime, SolarWeek, SolarYear};

  #[test]
  fn test0() {
    assert_eq!("1日", SolarDay::from_ymd(2023, 1, 1).get_name());
    assert_eq!("2023年1月1日", SolarDay::from_ymd(2023, 1, 1).to_string());
  }

  #[test]
  fn test1() {
    assert_eq!("29日", SolarDay::from_ymd(2000, 2, 29).get_name());
    assert_eq!("2000年2月29日", SolarDay::from_ymd(2000, 2, 29).to_string());
  }

  #[test]
  fn test2() {
    assert_eq!(0, SolarDay::from_ymd(2023, 1, 1).get_index_in_year());
    assert_eq!(364, SolarDay::from_ymd(2023, 12, 31).get_index_in_year());
    assert_eq!(365, SolarDay::from_ymd(2020, 12, 31).get_index_in_year());
  }

  #[test]
  fn test3() {
    assert_eq!(0, SolarDay::from_ymd(2023, 1, 1).subtract(SolarDay::from_ymd(2023, 1, 1)));
    assert_eq!(1, SolarDay::from_ymd(2023, 1, 2).subtract(SolarDay::from_ymd(2023, 1, 1)));
    assert_eq!(-1, SolarDay::from_ymd(2023, 1, 1).subtract(SolarDay::from_ymd(2023, 1, 2)));
    assert_eq!(31, SolarDay::from_ymd(2023, 2, 1).subtract(SolarDay::from_ymd(2023, 1, 1)));
    assert_eq!(-31, SolarDay::from_ymd(2023, 1, 1).subtract(SolarDay::from_ymd(2023, 2, 1)));
    assert_eq!(365, SolarDay::from_ymd(2024, 1, 1).subtract(SolarDay::from_ymd(2023, 1, 1)));
    assert_eq!(-365, SolarDay::from_ymd(2023, 1, 1).subtract(SolarDay::from_ymd(2024, 1, 1)));
    assert_eq!(1, SolarDay::from_ymd(1582, 10, 15).subtract(SolarDay::from_ymd(1582, 10, 4)));
  }

  #[test]
  fn test4() {
    assert_eq!("1582年10月4日", SolarDay::from_ymd(1582, 10, 15).next(-1).to_string());
  }

  #[test]
  fn test5() {
    assert_eq!("2000年3月1日", SolarDay::from_ymd(2000, 2, 28).next(2).to_string());
  }

  #[test]
  fn test6() {
    assert_eq!("农历庚子年闰四月初二", SolarDay::from_ymd(2020, 5, 24).get_lunar_day().to_string());
  }

  #[test]
  fn test7() {
    assert_eq!(31, SolarDay::from_ymd(2020, 5, 24).subtract(SolarDay::from_ymd(2020, 4, 23)));
  }

  #[test]
  fn test8() {
    assert_eq!("农历丙子年十一月十二", SolarDay::from_ymd(16, 11, 30).get_lunar_day().to_string());
  }

  #[test]
  fn test9() {
    assert_eq!("霜降", SolarDay::from_ymd(2023, 10, 27).get_term().to_string());
  }

  #[test]
  fn test10() {
    assert_eq!("豺乃祭兽第4天", SolarDay::from_ymd(2023, 10, 27).get_phenology_day().to_string());
  }

  #[test]
  fn test11() {
    assert_eq!("初候", SolarDay::from_ymd(2023, 10, 27).get_phenology_day().get_phenology().get_three_phenology().to_string());
  }

  #[test]
  fn test22() {
    assert_eq!("甲辰", SolarDay::from_ymd(2024, 2, 10).get_lunar_day().get_lunar_month().get_lunar_year().get_sixty_cycle().get_name());
  }

  #[test]
  fn test23() {
    assert_eq!("癸卯", SolarDay::from_ymd(2024, 2, 9).get_lunar_day().get_lunar_month().get_lunar_year().get_sixty_cycle().get_name());
  }

  #[test]
  fn test24() {
    assert_eq!("上半年", SolarHalfYear::from_index(2023, 0).get_name());
    assert_eq!("2023年上半年", SolarHalfYear::from_index(2023, 0).to_string());
  }

  #[test]
  fn test25() {
    assert_eq!("下半年", SolarHalfYear::from_index(2023, 1).get_name());
    assert_eq!("2023年下半年", SolarHalfYear::from_index(2023, 1).to_string());
  }

  #[test]
  fn test26() {
    assert_eq!("下半年", SolarHalfYear::from_index(2023, 0).next(1).get_name());
    assert_eq!("2023年下半年", SolarHalfYear::from_index(2023, 0).next(1).to_string());
  }

  #[test]
  fn test27() {
    assert_eq!("上半年", SolarHalfYear::from_index(2023, 0).next(2).get_name());
    assert_eq!("2024年上半年", SolarHalfYear::from_index(2023, 0).next(2).to_string());
  }

  #[test]
  fn test28() {
    assert_eq!("上半年", SolarHalfYear::from_index(2023, 0).next(-2).get_name());
    assert_eq!("2022年上半年", SolarHalfYear::from_index(2023, 0).next(-2).to_string());
  }

  #[test]
  fn test29() {
    assert_eq!("2021年上半年", SolarHalfYear::from_index(2023, 0).next(-4).to_string());
    assert_eq!("2021年下半年", SolarHalfYear::from_index(2023, 0).next(-3).to_string());
  }

  #[test]
  fn test30() {
    let m: SolarMonth = SolarMonth::from_ym(2019, 5);
    assert_eq!("5月", m.get_name());
    assert_eq!("2019年5月", m.to_string());
  }

  #[test]
  fn test31() {
    let m: SolarMonth = SolarMonth::from_ym(2023, 1);
    assert_eq!(5, m.get_week_count(0));
    assert_eq!(6, m.get_week_count(1));
    assert_eq!(6, m.get_week_count(2));
    assert_eq!(5, m.get_week_count(3));
    assert_eq!(5, m.get_week_count(4));
    assert_eq!(5, m.get_week_count(5));
    assert_eq!(5, m.get_week_count(6));
  }

  #[test]
  fn test32() {
    let m: SolarMonth = SolarMonth::from_ym(2023, 2);
    assert_eq!(5, m.get_week_count(0));
    assert_eq!(5, m.get_week_count(1));
    assert_eq!(5, m.get_week_count(2));
    assert_eq!(4, m.get_week_count(3));
    assert_eq!(5, m.get_week_count(4));
    assert_eq!(5, m.get_week_count(5));
    assert_eq!(5, m.get_week_count(6));
  }

  #[test]
  fn test33() {
    let m: SolarMonth = SolarMonth::from_ym(2023, 10).next(1);
    assert_eq!("11月", m.get_name());
    assert_eq!("2023年11月", m.to_string());
  }

  #[test]
  fn test34() {
    let m: SolarMonth = SolarMonth::from_ym(2023, 10);
    assert_eq!("2023年12月", m.next(2).to_string());
    assert_eq!("2024年1月", m.next(3).to_string());
    assert_eq!("2023年5月", m.next(-5).to_string());
    assert_eq!("2023年1月", m.next(-9).to_string());
    assert_eq!("2022年12月", m.next(-10).to_string());
    assert_eq!("2025年10月", m.next(24).to_string());
    assert_eq!("2021年10月", m.next(-24).to_string());
  }

  #[test]
  fn test35() {
    let season: SolarSeason = SolarSeason::from_index(2023, 0);
    assert_eq!("2023年一季度", season.to_string());
    assert_eq!("2021年四季度", season.next(-5).to_string());
  }

  #[test]
  fn test36() {
    // 冬至在去年，2022-12-22 05:48:11
    let dong_zhi: SolarTerm = SolarTerm::from_name(2023, "冬至");
    assert_eq!("冬至", dong_zhi.get_name());
    assert_eq!(0, dong_zhi.get_index());
    // 公历日
    assert_eq!("2022年12月22日", dong_zhi.get_julian_day().get_solar_day().to_string());

    // 冬至顺推23次，就是大雪 2023-12-07 17:32:55
    let da_xue: SolarTerm = dong_zhi.next(23);
    assert_eq!("大雪", da_xue.get_name());
    assert_eq!(23, da_xue.get_index());
    assert_eq!("2023年12月7日", da_xue.get_julian_day().get_solar_day().to_string());

    // 冬至逆推2次，就是上一年的小雪 2022-11-22 16:20:28
    let xiao_xue: SolarTerm = dong_zhi.next(-2);
    assert_eq!("小雪", xiao_xue.get_name());
    assert_eq!(22, xiao_xue.get_index());
    assert_eq!("2022年11月22日", xiao_xue.get_julian_day().get_solar_day().to_string());

    // 冬至顺推24次，就是下一个冬至 2023-12-22 11:27:20
    let dong_zhi2: SolarTerm = dong_zhi.next(24);
    assert_eq!("冬至", dong_zhi2.get_name());
    assert_eq!(0, dong_zhi2.get_index());
    assert_eq!("2023年12月22日", dong_zhi2.get_julian_day().get_solar_day().to_string());
  }

  #[test]
  fn test37() {
    // 公历2023年的雨水，2023-02-19 06:34:16
    let jq: SolarTerm = SolarTerm::from_name(2023, "雨水");
    assert_eq!("雨水", jq.get_name());
    assert_eq!(4, jq.get_index());
  }

  #[test]
  fn test38() {
    // 公历2023年的大雪，2023-12-07 17:32:55
    let jq: SolarTerm = SolarTerm::from_name(2023, "大雪");
    assert_eq!("大雪", jq.get_name());
    // 索引
    assert_eq!(23, jq.get_index());
    // 公历
    assert_eq!("2023年12月7日", jq.get_julian_day().get_solar_day().to_string());
    // 农历
    assert_eq!("农历癸卯年十月廿五", jq.get_julian_day().get_solar_day().get_lunar_day().to_string());
    // 推移
    assert_eq!("雨水", jq.next(5).get_name());
  }

  #[test]
  fn test39() {
    assert_eq!("寒露", SolarDay::from_ymd(2023, 10, 10).get_term().get_name());
  }

  #[test]
  fn test40() {
    let time: SolarTime = SolarTime::from_ymd_hms(2023, 1, 1, 13, 5, 20);
    assert_eq!("13:05:20", time.get_name());
    assert_eq!("13:04:59", time.next(-21).get_name());
  }

  #[test]
  fn test41() {
    let time: SolarTime = SolarTime::from_ymd_hms(2023, 1, 1, 13, 5, 20);
    assert_eq!("13:05:20", time.get_name());
    assert_eq!("14:06:01", time.next(3641).get_name());
  }

  #[test]
  fn test42() {
    assert_eq!("2023年", SolarYear::from_year(2023).get_name());
  }

  #[test]
  fn test43() {
    assert_eq!(false, SolarYear::from_year(2023).is_leap());
  }

  #[test]
  fn test44() {
    assert_eq!(true, SolarYear::from_year(1500).is_leap());
  }

  #[test]
  fn test45() {
    assert_eq!(false, SolarYear::from_year(1700).is_leap());
  }

  #[test]
  fn test46() {
    assert_eq!(365, SolarYear::from_year(2023).get_day_count());
  }

  #[test]
  fn test47() {
    assert_eq!("2028年", SolarYear::from_year(2023).next(5).get_name());
  }

  #[test]
  fn test48() {
    assert_eq!("2018年", SolarYear::from_year(2023).next(-5).get_name());
  }

  #[test]
  fn test49() {
    assert_eq!("一", SolarDay::from_ymd(1582, 10, 1).get_week().get_name());
  }

  #[test]
  fn test50() {
    assert_eq!("五", SolarDay::from_ymd(1582, 10, 15).get_week().get_name());
  }

  #[test]
  fn test51() {
    assert_eq!(2, SolarDay::from_ymd(2023, 10, 31).get_week().get_index());
  }

  #[test]
  fn test52() {
    let w: SolarWeek = SolarWeek::from_ym(2023, 10, 0, 0);
    assert_eq!("第一周", w.get_name());
    assert_eq!("2023年10月第一周", w.to_string());
  }

  #[test]
  fn test53() {
    let w: SolarWeek = SolarWeek::from_ym(2023, 10, 4, 0);
    assert_eq!("第五周", w.get_name());
    assert_eq!("2023年10月第五周", w.to_string());
  }

  #[test]
  fn test54() {
    let w: SolarWeek = SolarWeek::from_ym(2023, 10, 5, 1);
    assert_eq!("第六周", w.get_name());
    assert_eq!("2023年10月第六周", w.to_string());
  }

  #[test]
  fn test55() {
    let w: SolarWeek = SolarWeek::from_ym(2023, 10, 0, 0).next(4);
    assert_eq!("第五周", w.get_name());
    assert_eq!("2023年10月第五周", w.to_string());
  }

  #[test]
  fn test56() {
    let w: SolarWeek = SolarWeek::from_ym(2023, 10, 0, 0).next(5);
    assert_eq!("第二周", w.get_name());
    assert_eq!("2023年11月第二周", w.to_string());
  }

  #[test]
  fn test57() {
    let w: SolarWeek = SolarWeek::from_ym(2023, 10, 0, 0).next(-1);
    assert_eq!("第五周", w.get_name());
    assert_eq!("2023年9月第五周", w.to_string());
  }

  #[test]
  fn test58() {
    let w: SolarWeek = SolarWeek::from_ym(2023, 10, 0, 0).next(-5);
    assert_eq!("第一周", w.get_name());
    assert_eq!("2023年9月第一周", w.to_string());
  }

  #[test]
  fn test59() {
    let w: SolarWeek = SolarWeek::from_ym(2023, 10, 0, 0).next(-6);
    assert_eq!("第四周", w.get_name());
    assert_eq!("2023年8月第四周", w.to_string());
  }

  #[test]
  fn test60() {
    let solar: SolarDay = SolarDay::from_ymd(1582, 10, 1);
    assert_eq!(1, solar.get_week().get_index());
  }

  #[test]
  fn test61() {
    let solar: SolarDay = SolarDay::from_ymd(1582, 10, 15);
    assert_eq!(5, solar.get_week().get_index());
  }

  #[test]
  fn test62() {
    let solar: SolarDay = SolarDay::from_ymd(1129, 11, 17);
    assert_eq!(0, solar.get_week().get_index());
  }

  #[test]
  fn test63() {
    let solar: SolarDay = SolarDay::from_ymd(1129, 11, 1);
    assert_eq!(5, solar.get_week().get_index());
  }

  #[test]
  fn test64() {
    let solar: SolarDay = SolarDay::from_ymd(8, 11, 1);
    assert_eq!(4, solar.get_week().get_index());
  }

  #[test]
  fn test65() {
    let solar: SolarDay = SolarDay::from_ymd(1582, 9, 30);
    assert_eq!(0, solar.get_week().get_index());
  }

  #[test]
  fn test66() {
    let solar: SolarDay = SolarDay::from_ymd(1582, 1, 1);
    assert_eq!(1, solar.get_week().get_index());
  }

  #[test]
  fn test67() {
    let solar: SolarDay = SolarDay::from_ymd(1500, 2, 29);
    assert_eq!(6, solar.get_week().get_index());
  }

  #[test]
  fn test68() {
    let solar: SolarDay = SolarDay::from_ymd(9865, 7, 26);
    assert_eq!(3, solar.get_week().get_index());
  }

  #[test]
  fn test69() {
    let week: LunarWeek = LunarWeek::from_ym(2023, 1, 0, 2);
    assert_eq!("农历癸卯年正月第一周", week.to_string());
    assert_eq!("农历壬寅年十二月廿六", week.get_first_day().to_string());
  }

  #[test]
  fn test70() {
    let week: SolarWeek = SolarWeek::from_ym(2023, 1, 0, 2);
    assert_eq!("2023年1月第一周", week.to_string());
    assert_eq!("2022年12月27日", week.get_first_day().to_string());
  }

  #[test]
  fn test71() {
    let start: usize = 0;
    let mut week: SolarWeek = SolarWeek::from_ym(2024, 2, 2, start);
    assert_eq!("2024年2月第三周", week.to_string());
    assert_eq!(6, week.get_index_in_year());

    week = SolarDay::from_ymd(2024, 2, 11).get_solar_week(start);
    assert_eq!("2024年2月第三周", week.to_string());

    week = SolarDay::from_ymd(2024, 2, 17).get_solar_week(start);
    assert_eq!("2024年2月第三周", week.to_string());

    week = SolarDay::from_ymd(2024, 2, 10).get_solar_week(start);
    assert_eq!("2024年2月第二周", week.to_string());

    week = SolarDay::from_ymd(2024, 2, 18).get_solar_week(start);
    assert_eq!("2024年2月第四周", week.to_string());
  }

  #[test]
  fn test72() {
    // 大雪当天
    assert_eq!("大雪第1天", SolarDay::from_ymd(2023, 12, 7).get_term_day().to_string());
    // 天数索引
    assert_eq!(0, SolarDay::from_ymd(2023, 12, 7).get_term_day().get_day_index());

    assert_eq!("大雪第2天", SolarDay::from_ymd(2023, 12, 8).get_term_day().to_string());
    assert_eq!("大雪第15天", SolarDay::from_ymd(2023, 12, 21).get_term_day().to_string());

    assert_eq!("冬至第1天", SolarDay::from_ymd(2023, 12, 22).get_term_day().to_string());
  }
}
