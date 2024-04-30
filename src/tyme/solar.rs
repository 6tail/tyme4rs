use std::fmt::{Display, Formatter};
use crate::tyme::{AbstractCulture, AbstractTyme, Culture, LoopTyme, Tyme};
use crate::tyme::culture::{Constellation, Week};
use crate::tyme::culture::dog::{Dog, DogDay};
use crate::tyme::culture::nine::{Nine, NineDay};
use crate::tyme::culture::phenology::{Phenology, PhenologyDay};
use crate::tyme::festival::SolarFestival;
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
  fn next(&self, n: isize) -> Result<Self, String> {
    Self::from_year(self.year + n)
  }
}

impl Culture for SolarYear {
  fn get_name(&self) -> String {
    format!("{}年", self.year)
  }
}

impl SolarYear {
  pub fn from_year(year: isize) -> Result<Self, String> {
    if year < 1 || year > 9999 {
      Err(format!("illegal solar year: {}", year))
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

  pub fn is_leap(&self) -> bool {
    if self.year < 1600 {
      self.year % 4 == 0
    } else {
      (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
    }
  }

  pub fn get_months(&self) -> Vec<SolarMonth> {
    let mut l: Vec<SolarMonth> = Vec::new();
    for i in 0..12 {
      l.push(SolarMonth::from_ym(self.year, i + 1).unwrap());
    }
    l
  }

  pub fn get_seasons(&self) -> Vec<SolarSeason> {
    let mut l: Vec<SolarSeason> = Vec::new();
    for i in 0..4 {
      l.push(SolarSeason::from_index(self.year, i).unwrap());
    }
    l
  }

  pub fn get_half_years(&self) -> Vec<SolarHalfYear> {
    let mut l: Vec<SolarHalfYear> = Vec::new();
    for i in 0..2 {
      l.push(SolarHalfYear::from_index(self.year, i).unwrap());
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
  fn next(&self, n: isize) -> Result<Self, String> {
    if n == 0 {
      return Ok(self.clone());
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
  pub fn from_index(year: isize, index: usize) -> Result<Self, String> {
    let y: SolarYear = SolarYear::from_year(year).unwrap();
    if index > 1 {
      Err(format!("illegal solar half year index: {}", index))
    } else {
      Ok(Self {
        year: y,
        index,
      })
    }
  }

  pub fn get_year(&self) -> SolarYear {
    self.year
  }

  pub fn get_index(&self) -> usize {
    self.index
  }

  pub fn get_months(&self) -> Vec<SolarMonth> {
    let mut l: Vec<SolarMonth> = Vec::new();
    let y: isize = self.year.get_year();
    for i in 0..6 {
      l.push(SolarMonth::from_ym(y, self.index * 6 + i + 1).unwrap());
    }
    l
  }

  pub fn get_seasons(&self) -> Vec<SolarSeason> {
    let mut l: Vec<SolarSeason> = Vec::new();
    let y: isize = self.year.get_year();
    for i in 0..2 {
      l.push(SolarSeason::from_index(y, self.index * 2 + i).unwrap());
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
  fn next(&self, n: isize) -> Result<Self, String> {
    if n == 0 {
      return Ok(self.clone());
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
  pub fn from_index(year: isize, index: usize) -> Result<Self, String> {
    let y: SolarYear = SolarYear::from_year(year).unwrap();
    if index > 3 {
      Err(format!("illegal solar season index: {}", index))
    } else {
      Ok(Self {
        year: y,
        index,
      })
    }
  }

  pub fn get_year(&self) -> SolarYear {
    self.year
  }

  pub fn get_index(&self) -> usize {
    self.index
  }

  pub fn get_months(&self) -> Vec<SolarMonth> {
    let mut l: Vec<SolarMonth> = Vec::new();
    let y: isize = self.year.get_year();
    for i in 0..3 {
      l.push(SolarMonth::from_ym(y, self.index * 3 + i + 1).unwrap());
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
  fn next(&self, n: isize) -> Result<Self, String> {
    if n == 0 {
      Ok(self.clone())
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
  pub fn from_ym(year: isize, month: usize) -> Result<Self, String> {
    let y: SolarYear = SolarYear::from_year(year).unwrap();
    if month < 1 || month > 12 {
      Err(format!("illegal solar month: {}", month))
    } else {
      Ok(Self {
        parent: AbstractTyme::new(),
        year: y,
        month,
      })
    }
  }

  pub fn get_year(&self) -> SolarYear {
    self.year
  }

  pub fn get_month(&self) -> usize {
    self.month
  }

  pub fn get_day_count(&self) -> usize {
    if 1582 == self.year.get_year() && 10 == self.month {
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

  pub fn get_index_in_year(&self) -> usize {
    self.month - 1
  }

  pub fn get_week_count(&self, start: usize) -> usize {
    let culture: AbstractCulture = self.parent.into();
    ((culture.index_of(SolarDay::from_ymd(self.year.get_year(), self.month, 1).unwrap().get_week().get_index() as isize - start as isize, 7) + self.get_day_count()) as f64 / 7f64).ceil() as usize
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
  fn next(&self, n: isize) -> Result<Self, String> {
    if n == 0 {
      Ok(self.clone())
    } else {
      let mut d: isize = (self.index as isize) + n;
      let mut m: SolarMonth = self.month;
      let start_index: usize = self.start.get_index();
      let mut weeks_in_month: isize = m.get_week_count(start_index) as isize;
      let forward: bool = n > 0;
      let add: isize = if forward { 1 } else { -1 };
      while if forward { d >= weeks_in_month } else { d < 0 } {
        if forward {
          d -= weeks_in_month;
        }
        if !forward {
          if SolarDay::from_ymd(m.get_year().get_year(), m.get_month(), 1).unwrap().get_week() != self.start {
            d += add;
          }
        }
        m = m.next(add).unwrap();
        if forward {
          if SolarDay::from_ymd(m.get_year().get_year(), m.get_month(), 1).unwrap().get_week() != self.start {
            d += add;
          }
        }
        weeks_in_month = m.get_week_count(start_index) as isize;
        if !forward {
          d += weeks_in_month;
        }
      }
      Self::from_ym(m.get_year().get_year(), m.get_month(), d as usize, start_index)
    }
  }
}

impl Culture for SolarWeek {
  fn get_name(&self) -> String {
    SOLAR_WEEK_NAMES[self.index].to_string()
  }
}

impl SolarWeek {
  pub fn from_ym(year: isize, month: usize, index: usize, start: usize) -> Result<Self, String> {
    if index > 5 {
      Err(format!("illegal solar week index: {}", index))
    } else if start > 6 {
      Err(format!("illegal solar week start: {}", start))
    } else {
      let m: SolarMonth = SolarMonth::from_ym(year, month).unwrap();
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

  pub fn get_month(&self) -> SolarMonth {
    self.month
  }

  pub fn get_index(&self) -> usize {
    self.index
  }

  pub fn get_start(&self) -> Week {
    self.start.clone()
  }

  pub fn get_first_day(&self) -> SolarDay {
    let m: SolarMonth = self.get_month();
    let first_day: SolarDay = SolarDay::from_ymd(m.get_year().get_year(), m.get_month(), 1).unwrap();
    let parent: AbstractTyme = self.parent.into();
    let culture: AbstractCulture = parent.into();
    first_day.next(self.index as isize * 7 - culture.index_of((first_day.get_week().get_index() as isize) - (self.start.get_index() as isize), 7) as isize).unwrap()
  }

  pub fn get_days(&self) -> Vec<SolarDay> {
    let mut l: Vec<SolarDay> = Vec::new();
    let d: SolarDay = self.get_first_day();
    l.push(d);
    for i in 1..7 {
      l.push(d.next(i).unwrap());
    }
    l
  }

  pub fn get_index_in_year(&self) -> usize {
    let mut i: usize = 0;
    // 今年第1周
    let mut w: SolarWeek = SolarWeek::from_ym(self.month.get_year().get_year(), 1, 0, self.start.get_index()).unwrap();
    while w != *self {
      w = w.next(1).unwrap();
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
    self.get_month() == other.get_month() && self.get_index() == other.get_index() && self.get_start() == other.get_start()
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
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(self.get_julian_day().next(n).unwrap().get_solar_day())
  }
}

impl Culture for SolarDay {
  fn get_name(&self) -> String {
    SOLAR_DAY_NAMES[self.day - 1].to_string()
  }
}

impl SolarDay {
  pub fn from_ymd(year: isize, month: usize, day: usize) -> Result<Self, String> {
    let m: SolarMonth = SolarMonth::from_ym(year, month).unwrap();
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

  pub fn get_month(&self) -> SolarMonth {
    self.month
  }

  pub fn get_day(&self) -> usize {
    self.day
  }

  pub fn get_week(&self) -> Week {
    self.get_julian_day().get_week()
  }

  pub fn get_solar_week(&self, start: usize) -> SolarWeek {
    let y: isize = self.month.get_year().get_year();
    let m: usize = self.month.get_month();
    SolarWeek::from_ym(y, m, ((self.day + SolarDay::from_ymd(y, m, 1).unwrap().get_week().next(-(start as isize)).unwrap().get_index()) as f64 / 7.0).ceil() as usize - 1, start).unwrap()
  }

  pub fn get_term(&self) -> SolarTerm {
    let mut term: SolarTerm = SolarTerm::from_index(self.month.get_year().get_year() + 1, 0);
    while self.is_before(term.get_julian_day().get_solar_day()) {
      term = term.next(-1).unwrap();
    }
    term
  }

  pub fn get_julian_day(&self) -> JulianDay {
    JulianDay::from_ymd_hms(self.month.get_year().get_year(), self.month.get_month(), self.day, 0, 0, 0).unwrap()
  }

  pub fn is_before(&self, target: SolarDay) -> bool {
    let a_year: isize = self.month.get_year().get_year();
    let target_month: SolarMonth = target.get_month();
    let b_year: isize = target_month.get_year().get_year();
    if a_year == b_year {
      let a_month: usize = self.month.get_month();
      let b_month: usize = target_month.get_month();
      return if a_month == b_month { self.day < target.get_day() } else { a_month < b_month };
    }
    a_year < b_year
  }

  pub fn is_after(&self, target: SolarDay) -> bool {
    let a_year: isize = self.month.get_year().get_year();
    let target_month: SolarMonth = target.get_month();
    let b_year: isize = target_month.get_year().get_year();
    if a_year == b_year {
      let a_month: usize = self.month.get_month();
      let b_month: usize = target_month.get_month();
      return if a_month == b_month { self.day > target.get_day() } else { a_month > b_month };
    }
    a_year > b_year
  }

  pub fn get_index_in_year(&self) -> usize {
    let m: usize = self.month.get_month();
    let y: isize = self.month.get_year().get_year();
    let mut days: usize = 0;
    for i in 1..m {
      days += SolarMonth::from_ym(y, i).unwrap().get_day_count();
    }
    let mut d: usize = self.day;
    if 1582 == y && 10 == m {
      if d >= 15 {
        d -= 10;
      }
    }
    return days + d - 1;
  }

  pub fn subtract(&self, target: SolarDay) -> isize {
    (self.get_julian_day().get_day() - target.get_julian_day().get_day()) as isize
  }

  pub fn get_lunar_day(&self) -> LunarDay {
    let mut m: LunarMonth = LunarMonth::from_ym(self.month.get_year().get_year(), self.month.get_month() as isize).unwrap().next(-3).unwrap();
    let mut days: isize = self.subtract(m.get_first_julian_day().get_solar_day());
    while days >= m.get_day_count() as isize {
      m = m.next(1).unwrap();
      days = self.subtract(m.get_first_julian_day().get_solar_day());
    }
    LunarDay::from_ymd(m.get_year().get_year(), m.get_month_with_leap(), (days + 1) as usize).unwrap()
  }

  pub fn get_constellation(&self) -> Constellation {
    let mut index: isize = 11;
    let y: usize = self.get_month().get_month() * 100 + self.day;
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

  pub fn get_dog_day(&self) -> Option<DogDay> {
    let xia_zhi: SolarTerm = SolarTerm::from_index(self.month.get_year().get_year(), 12);
    // 第1个庚日
    let mut start: SolarDay = xia_zhi.get_julian_day().get_solar_day();
    let mut add: isize = 6 - start.get_lunar_day().get_sixty_cycle().get_heaven_stem().get_index() as isize;
    if add < 0 {
      add += 10;
    }
    // 第3个庚日，即初伏第1天
    add += 20;
    start = start.next(add).unwrap();
    let mut days: isize = self.subtract(start);
    // 初伏以前
    if days < 0 {
      return None;
    }
    if days < 10 {
      return Some(DogDay::new(Dog::from_index(0), days as usize));
    }
    // 第4个庚日，中伏第1天
    start = start.next(10).unwrap();
    days = self.subtract(start);
    if days < 10 {
      return Some(DogDay::new(Dog::from_index(1), days as usize));
    }
    // 第5个庚日，中伏第11天或末伏第1天
    start = start.next(10).unwrap();
    days = self.subtract(start);
    // 立秋
    if xia_zhi.next(3).unwrap().get_julian_day().get_solar_day().is_after(start) {
      if days < 10 {
        return Some(DogDay::new(Dog::from_index(1), days as usize + 10));
      }
      start = start.next(10).unwrap();
      days = self.subtract(start);
    }
    if days < 10 {
      return Some(DogDay::new(Dog::from_index(2), days as usize));
    }
    None
  }

  pub fn get_nine_day(&self) -> Option<NineDay> {
    let year: isize = self.month.get_year().get_year();
    let mut start: SolarDay = SolarTerm::from_index(year + 1, 0).get_julian_day().get_solar_day();
    if self.is_before(start) {
      start = SolarTerm::from_index(year, 0).get_julian_day().get_solar_day();
    }
    let end: SolarDay = start.next(81).unwrap();
    if self.is_before(start) || !self.is_before(end) {
      return None;
    }
    let days: isize = self.subtract(start);
    Some(NineDay::new(Nine::from_index(days / 9), days as usize % 9))
  }

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

  pub fn get_festival(&self) -> Option<SolarFestival> {
    let m: SolarMonth = self.get_month();
    SolarFestival::from_ymd(m.get_year().get_year(), m.get_month(), self.day)
  }
}

impl Display for SolarDay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.month, self.get_name())
  }
}

impl PartialEq for SolarDay {
  fn eq(&self, other: &Self) -> bool {
    self.get_month() == other.get_month() && self.get_day() == other.get_day()
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
  fn next(&self, n: isize) -> Result<Self, String> {
    if n == 0 {
      Ok(self.clone())
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

      let d: SolarDay = self.day.next(td).unwrap();
      let m: SolarMonth = d.get_month();
      return Self::from_ymd_hms(m.get_year().get_year(), m.get_month(), d.get_day(), th as usize, tm as usize, ts as usize);
    }
  }
}

impl Culture for SolarTime {
  fn get_name(&self) -> String {
    format!("{:0>2}:{:0>2}:{:0>2}", self.hour, self.minute, self.second)
  }
}

impl SolarTime {
  pub fn from_ymd_hms(year: isize, month: usize, day: usize, hour: usize, minute: usize, second: usize) -> Result<Self, String> {
    if hour > 23 {
      Err(format!("illegal hour: {}", hour))
    } else if minute > 59 {
      Err(format!("illegal minute: {}", minute))
    } else if second > 59 {
      Err(format!("illegal second: {}", second))
    } else {
      Ok(Self {
        day: SolarDay::from_ymd(year, month, day).unwrap(),
        hour,
        minute,
        second,
      })
    }
  }

  pub fn get_day(&self) -> SolarDay {
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

  pub fn is_before(&self, target: SolarTime) -> bool {
    if self.day != target.get_day() {
      return self.day.is_before(target.get_day());
    }
    let b_hour: usize = target.get_hour();
    if self.hour == b_hour {
      let b_minute: usize = target.get_minute();
      return if self.minute == b_minute { self.second < target.get_second() } else { self.minute < b_minute };
    }
    self.hour < b_hour
  }

  pub fn is_after(&self, target: SolarTime) -> bool {
    if self.day != target.get_day() {
      return self.day.is_after(target.get_day());
    }
    let b_hour: usize = target.get_hour();
    if self.hour == b_hour {
      let b_minute: usize = target.get_minute();
      return if self.minute == b_minute { self.second > target.get_second() } else { self.minute > b_minute };
    }
    self.hour > b_hour
  }

  pub fn get_term(&self) -> SolarTerm {
    let mut term: SolarTerm = SolarTerm::from_index(self.day.get_month().get_year().get_year() + 1, 0);
    while self.is_before(term.get_julian_day().get_solar_time()) {
      term = term.next(-1).unwrap();
    }
    term
  }

  pub fn get_julian_day(&self) -> JulianDay {
    let month: SolarMonth = self.day.get_month();
    JulianDay::from_ymd_hms(month.get_year().get_year(), month.get_month(), self.day.get_day(), self.hour, self.minute, self.second).unwrap()
  }

  pub fn subtract(&self, target: SolarTime) -> isize {
    let mut days: isize = self.day.subtract(target.get_day());
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
    let m: LunarMonth = d.get_month();
    LunarHour::from_ymd_hms(m.get_year().get_year(), m.get_month_with_leap(), d.get_day(), self.hour, self.minute, self.second).unwrap()
  }
}

impl Display for SolarTime {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", self.day, self.get_name())
  }
}

impl PartialEq for SolarTime {
  fn eq(&self, other: &Self) -> bool {
    self.get_day() == other.get_day() && self.get_hour() == other.get_hour() && self.get_minute() == other.get_minute() && self.get_second() == other.get_second()
  }
}

impl Eq for SolarTime {}

pub static SOLAR_TERM_NAMES: [&str; 24] = ["冬至", "小寒", "大寒", "立春", "雨水", "惊蛰", "春分", "清明", "谷雨", "立夏", "小满", "芒种", "夏至", "小暑", "大暑", "立秋", "处暑", "白露", "秋分", "寒露", "霜降", "立冬", "小雪", "大雪"];

/// 节气
#[derive(Debug, Clone)]
pub struct SolarTerm {
  parent: LoopTyme,
  cursory_julian_day: f64,
}

impl Tyme for SolarTerm {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_cursory_julian_day(self.cursory_julian_day + 15.2184 * (n as f64), self.parent.next_index(n) as isize))
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

  pub fn from_name(year: isize, name: &str) -> Result<Self, String> {
    let parent: LoopTyme = LoopTyme::from_name(SOLAR_TERM_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap();
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

  pub fn is_jie(&self) -> bool {
    self.get_index() % 2 == 1
  }

  pub fn is_qi(&self) -> bool {
    self.get_index() % 2 == 0
  }

  pub fn get_julian_day(&self) -> JulianDay {
    JulianDay::from_julian_day(ShouXingUtil::qi_accurate2(self.cursory_julian_day) + J2000).unwrap()
  }

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

#[cfg(test)]
mod tests {
  use crate::tyme::{Culture, Tyme};
  use crate::tyme::lunar::LunarWeek;
  use crate::tyme::solar::{SolarDay, SolarHalfYear, SolarMonth, SolarSeason, SolarTerm, SolarTime, SolarWeek, SolarYear};

  #[test]
  fn test0() {
    assert_eq!("1日", SolarDay::from_ymd(2023, 1, 1).unwrap().get_name());
    assert_eq!("2023年1月1日", SolarDay::from_ymd(2023, 1, 1).unwrap().to_string());
  }

  #[test]
  fn test1() {
    assert_eq!("29日", SolarDay::from_ymd(2000, 2, 29).unwrap().get_name());
    assert_eq!("2000年2月29日", SolarDay::from_ymd(2000, 2, 29).unwrap().to_string());
  }

  #[test]
  fn test2() {
    assert_eq!(0, SolarDay::from_ymd(2023, 1, 1).unwrap().get_index_in_year());
    assert_eq!(364, SolarDay::from_ymd(2023, 12, 31).unwrap().get_index_in_year());
    assert_eq!(365, SolarDay::from_ymd(2020, 12, 31).unwrap().get_index_in_year());
  }

  #[test]
  fn test3() {
    assert_eq!(0, SolarDay::from_ymd(2023, 1, 1).unwrap().subtract(SolarDay::from_ymd(2023, 1, 1).unwrap()));
    assert_eq!(1, SolarDay::from_ymd(2023, 1, 2).unwrap().subtract(SolarDay::from_ymd(2023, 1, 1).unwrap()));
    assert_eq!(-1, SolarDay::from_ymd(2023, 1, 1).unwrap().subtract(SolarDay::from_ymd(2023, 1, 2).unwrap()));
    assert_eq!(31, SolarDay::from_ymd(2023, 2, 1).unwrap().subtract(SolarDay::from_ymd(2023, 1, 1).unwrap()));
    assert_eq!(-31, SolarDay::from_ymd(2023, 1, 1).unwrap().subtract(SolarDay::from_ymd(2023, 2, 1).unwrap()));
    assert_eq!(365, SolarDay::from_ymd(2024, 1, 1).unwrap().subtract(SolarDay::from_ymd(2023, 1, 1).unwrap()));
    assert_eq!(-365, SolarDay::from_ymd(2023, 1, 1).unwrap().subtract(SolarDay::from_ymd(2024, 1, 1).unwrap()));
    assert_eq!(1, SolarDay::from_ymd(1582, 10, 15).unwrap().subtract(SolarDay::from_ymd(1582, 10, 4).unwrap()));
  }

  #[test]
  fn test4() {
    assert_eq!("1582年10月4日", SolarDay::from_ymd(1582, 10, 15).unwrap().next(-1).unwrap().to_string());
  }

  #[test]
  fn test5() {
    assert_eq!("2000年3月1日", SolarDay::from_ymd(2000, 2, 28).unwrap().next(2).unwrap().to_string());
  }

  #[test]
  fn test6() {
    assert_eq!("农历庚子年闰四月初二", SolarDay::from_ymd(2020, 5, 24).unwrap().get_lunar_day().to_string());
  }

  #[test]
  fn test7() {
    assert_eq!(31, SolarDay::from_ymd(2020, 5, 24).unwrap().subtract(SolarDay::from_ymd(2020, 4, 23).unwrap()));
  }

  #[test]
  fn test8() {
    assert_eq!("农历丙子年十一月十二", SolarDay::from_ymd(16, 11, 30).unwrap().get_lunar_day().to_string());
  }

  #[test]
  fn test9() {
    assert_eq!("霜降", SolarDay::from_ymd(2023, 10, 27).unwrap().get_term().to_string());
  }

  #[test]
  fn test10() {
    assert_eq!("豺乃祭兽第4天", SolarDay::from_ymd(2023, 10, 27).unwrap().get_phenology_day().to_string());
  }

  #[test]
  fn test11() {
    assert_eq!("初候", SolarDay::from_ymd(2023, 10, 27).unwrap().get_phenology_day().get_phenology().get_three_phenology().to_string());
  }

  #[test]
  fn test22() {
    assert_eq!("甲辰", SolarDay::from_ymd(2024, 2, 10).unwrap().get_lunar_day().get_month().get_year().get_sixty_cycle().get_name());
  }

  #[test]
  fn test23() {
    assert_eq!("癸卯", SolarDay::from_ymd(2024, 2, 9).unwrap().get_lunar_day().get_month().get_year().get_sixty_cycle().get_name());
  }

  #[test]
  fn test24() {
    assert_eq!("上半年", SolarHalfYear::from_index(2023, 0).unwrap().get_name());
    assert_eq!("2023年上半年", SolarHalfYear::from_index(2023, 0).unwrap().to_string());
  }

  #[test]
  fn test25() {
    assert_eq!("下半年", SolarHalfYear::from_index(2023, 1).unwrap().get_name());
    assert_eq!("2023年下半年", SolarHalfYear::from_index(2023, 1).unwrap().to_string());
  }

  #[test]
  fn test26() {
    assert_eq!("下半年", SolarHalfYear::from_index(2023, 0).unwrap().next(1).unwrap().get_name());
    assert_eq!("2023年下半年", SolarHalfYear::from_index(2023, 0).unwrap().next(1).unwrap().to_string());
  }

  #[test]
  fn test27() {
    assert_eq!("上半年", SolarHalfYear::from_index(2023, 0).unwrap().next(2).unwrap().get_name());
    assert_eq!("2024年上半年", SolarHalfYear::from_index(2023, 0).unwrap().next(2).unwrap().to_string());
  }

  #[test]
  fn test28() {
    assert_eq!("上半年", SolarHalfYear::from_index(2023, 0).unwrap().next(-2).unwrap().get_name());
    assert_eq!("2022年上半年", SolarHalfYear::from_index(2023, 0).unwrap().next(-2).unwrap().to_string());
  }

  #[test]
  fn test29() {
    assert_eq!("2021年上半年", SolarHalfYear::from_index(2023, 0).unwrap().next(-4).unwrap().to_string());
    assert_eq!("2021年下半年", SolarHalfYear::from_index(2023, 0).unwrap().next(-3).unwrap().to_string());
  }

  #[test]
  fn test30() {
    let m: SolarMonth = SolarMonth::from_ym(2019, 5).unwrap();
    assert_eq!("5月", m.get_name());
    assert_eq!("2019年5月", m.to_string());
  }

  #[test]
  fn test31() {
    let m: SolarMonth = SolarMonth::from_ym(2023, 1).unwrap();
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
    let m: SolarMonth = SolarMonth::from_ym(2023, 2).unwrap();
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
    let m: SolarMonth = SolarMonth::from_ym(2023, 10).unwrap().next(1).unwrap();
    assert_eq!("11月", m.get_name());
    assert_eq!("2023年11月", m.to_string());
  }

  #[test]
  fn test34() {
    let m: SolarMonth = SolarMonth::from_ym(2023, 10).unwrap();
    assert_eq!("2023年12月", m.next(2).unwrap().to_string());
    assert_eq!("2024年1月", m.next(3).unwrap().to_string());
    assert_eq!("2023年5月", m.next(-5).unwrap().to_string());
    assert_eq!("2023年1月", m.next(-9).unwrap().to_string());
    assert_eq!("2022年12月", m.next(-10).unwrap().to_string());
    assert_eq!("2025年10月", m.next(24).unwrap().to_string());
    assert_eq!("2021年10月", m.next(-24).unwrap().to_string());
  }

  #[test]
  fn test35() {
    let season: SolarSeason = SolarSeason::from_index(2023, 0).unwrap();
    assert_eq!("2023年一季度", season.to_string());
    assert_eq!("2021年四季度", season.next(-5).unwrap().to_string());
  }

  #[test]
  fn test36() {
    // 冬至在去年，2022-12-22 05:48:11
    let dong_zhi: SolarTerm = SolarTerm::from_name(2023, "冬至").unwrap();
    assert_eq!("冬至", dong_zhi.get_name());
    assert_eq!(0, dong_zhi.get_index());
    // 公历日
    assert_eq!("2022年12月22日", dong_zhi.get_julian_day().get_solar_day().to_string());

    // 冬至顺推23次，就是大雪 2023-12-07 17:32:55
    let da_xue: SolarTerm = dong_zhi.next(23).unwrap();
    assert_eq!("大雪", da_xue.get_name());
    assert_eq!(23, da_xue.get_index());
    assert_eq!("2023年12月7日", da_xue.get_julian_day().get_solar_day().to_string());

    // 冬至逆推2次，就是上一年的小雪 2022-11-22 16:20:28
    let xiao_xue: SolarTerm = dong_zhi.next(-2).unwrap();
    assert_eq!("小雪", xiao_xue.get_name());
    assert_eq!(22, xiao_xue.get_index());
    assert_eq!("2022年11月22日", xiao_xue.get_julian_day().get_solar_day().to_string());

    // 冬至顺推24次，就是下一个冬至 2023-12-22 11:27:20
    let dong_zhi2: SolarTerm = dong_zhi.next(24).unwrap();
    assert_eq!("冬至", dong_zhi2.get_name());
    assert_eq!(0, dong_zhi2.get_index());
    assert_eq!("2023年12月22日", dong_zhi2.get_julian_day().get_solar_day().to_string());
  }

  #[test]
  fn test37() {
    // 公历2023年的雨水，2023-02-19 06:34:16
    let jq: SolarTerm = SolarTerm::from_name(2023, "雨水").unwrap();
    assert_eq!("雨水", jq.get_name());
    assert_eq!(4, jq.get_index());
  }

  #[test]
  fn test38() {
    // 公历2023年的大雪，2023-12-07 17:32:55
    let jq: SolarTerm = SolarTerm::from_name(2023, "大雪").unwrap();
    assert_eq!("大雪", jq.get_name());
    // 索引
    assert_eq!(23, jq.get_index());
    // 公历
    assert_eq!("2023年12月7日", jq.get_julian_day().get_solar_day().to_string());
    // 农历
    assert_eq!("农历癸卯年十月廿五", jq.get_julian_day().get_solar_day().get_lunar_day().to_string());
    // 推移
    assert_eq!("雨水", jq.next(5).unwrap().get_name());
  }

  #[test]
  fn test39() {
    assert_eq!("寒露", SolarDay::from_ymd(2023, 10, 10).unwrap().get_term().get_name());
  }

  #[test]
  fn test40() {
    let time: SolarTime = SolarTime::from_ymd_hms(2023, 1, 1, 13, 5, 20).unwrap();
    assert_eq!("13:05:20", time.get_name());
    assert_eq!("13:04:59", time.next(-21).unwrap().get_name());
  }

  #[test]
  fn test41() {
    let time: SolarTime = SolarTime::from_ymd_hms(2023, 1, 1, 13, 5, 20).unwrap();
    assert_eq!("13:05:20", time.get_name());
    assert_eq!("14:06:01", time.next(3641).unwrap().get_name());
  }

  #[test]
  fn test42() {
    assert_eq!("2023年", SolarYear::from_year(2023).unwrap().get_name());
  }

  #[test]
  fn test43() {
    assert_eq!(false, SolarYear::from_year(2023).unwrap().is_leap());
  }

  #[test]
  fn test44() {
    assert_eq!(true, SolarYear::from_year(1500).unwrap().is_leap());
  }

  #[test]
  fn test45() {
    assert_eq!(false, SolarYear::from_year(1700).unwrap().is_leap());
  }

  #[test]
  fn test46() {
    assert_eq!(365, SolarYear::from_year(2023).unwrap().get_day_count());
  }

  #[test]
  fn test47() {
    assert_eq!("2028年", SolarYear::from_year(2023).unwrap().next(5).unwrap().get_name());
  }

  #[test]
  fn test48() {
    assert_eq!("2018年", SolarYear::from_year(2023).unwrap().next(-5).unwrap().get_name());
  }

  #[test]
  fn test49() {
    assert_eq!("一", SolarDay::from_ymd(1582, 10, 1).unwrap().get_week().get_name());
  }

  #[test]
  fn test50() {
    assert_eq!("五", SolarDay::from_ymd(1582, 10, 15).unwrap().get_week().get_name());
  }

  #[test]
  fn test51() {
    assert_eq!(2, SolarDay::from_ymd(2023, 10, 31).unwrap().get_week().get_index());
  }

  #[test]
  fn test52() {
    let w: SolarWeek = SolarWeek::from_ym(2023, 10, 0, 0).unwrap();
    assert_eq!("第一周", w.get_name());
    assert_eq!("2023年10月第一周", w.to_string());
  }

  #[test]
  fn test53() {
    let w: SolarWeek = SolarWeek::from_ym(2023, 10, 4, 0).unwrap();
    assert_eq!("第五周", w.get_name());
    assert_eq!("2023年10月第五周", w.to_string());
  }

  #[test]
  fn test54() {
    let w: SolarWeek = SolarWeek::from_ym(2023, 10, 5, 1).unwrap();
    assert_eq!("第六周", w.get_name());
    assert_eq!("2023年10月第六周", w.to_string());
  }

  #[test]
  fn test55() {
    let w: SolarWeek = SolarWeek::from_ym(2023, 10, 0, 0).unwrap().next(4).unwrap();
    assert_eq!("第五周", w.get_name());
    assert_eq!("2023年10月第五周", w.to_string());
  }

  #[test]
  fn test56() {
    let w: SolarWeek = SolarWeek::from_ym(2023, 10, 0, 0).unwrap().next(5).unwrap();
    assert_eq!("第二周", w.get_name());
    assert_eq!("2023年11月第二周", w.to_string());
  }

  #[test]
  fn test57() {
    let w: SolarWeek = SolarWeek::from_ym(2023, 10, 0, 0).unwrap().next(-1).unwrap();
    assert_eq!("第五周", w.get_name());
    assert_eq!("2023年9月第五周", w.to_string());
  }

  #[test]
  fn test58() {
    let w: SolarWeek = SolarWeek::from_ym(2023, 10, 0, 0).unwrap().next(-5).unwrap();
    assert_eq!("第一周", w.get_name());
    assert_eq!("2023年9月第一周", w.to_string());
  }

  #[test]
  fn test59() {
    let w: SolarWeek = SolarWeek::from_ym(2023, 10, 0, 0).unwrap().next(-6).unwrap();
    assert_eq!("第四周", w.get_name());
    assert_eq!("2023年8月第四周", w.to_string());
  }

  #[test]
  fn test60() {
    let solar: SolarDay = SolarDay::from_ymd(1582, 10, 1).unwrap();
    assert_eq!(1, solar.get_week().get_index());
  }

  #[test]
  fn test61() {
    let solar: SolarDay = SolarDay::from_ymd(1582, 10, 15).unwrap();
    assert_eq!(5, solar.get_week().get_index());
  }

  #[test]
  fn test62() {
    let solar: SolarDay = SolarDay::from_ymd(1129, 11, 17).unwrap();
    assert_eq!(0, solar.get_week().get_index());
  }

  #[test]
  fn test63() {
    let solar: SolarDay = SolarDay::from_ymd(1129, 11, 1).unwrap();
    assert_eq!(5, solar.get_week().get_index());
  }

  #[test]
  fn test64() {
    let solar: SolarDay = SolarDay::from_ymd(8, 11, 1).unwrap();
    assert_eq!(4, solar.get_week().get_index());
  }

  #[test]
  fn test65() {
    let solar: SolarDay = SolarDay::from_ymd(1582, 9, 30).unwrap();
    assert_eq!(0, solar.get_week().get_index());
  }

  #[test]
  fn test66() {
    let solar: SolarDay = SolarDay::from_ymd(1582, 1, 1).unwrap();
    assert_eq!(1, solar.get_week().get_index());
  }

  #[test]
  fn test67() {
    let solar: SolarDay = SolarDay::from_ymd(1500, 2, 29).unwrap();
    assert_eq!(6, solar.get_week().get_index());
  }

  #[test]
  fn test68() {
    let solar: SolarDay = SolarDay::from_ymd(9865, 7, 26).unwrap();
    assert_eq!(3, solar.get_week().get_index());
  }

  #[test]
  fn test69() {
    let week: LunarWeek = LunarWeek::from_ym(2023, 1, 0, 2).unwrap();
    assert_eq!("农历癸卯年正月第一周", week.to_string());
    assert_eq!("农历壬寅年十二月廿六", week.get_first_day().to_string());
  }

  #[test]
  fn test70() {
    let week: SolarWeek = SolarWeek::from_ym(2023, 1, 0, 2).unwrap();
    assert_eq!("2023年1月第一周", week.to_string());
    assert_eq!("2022年12月27日", week.get_first_day().to_string());
  }

  #[test]
  fn test71() {
    let start: usize = 0;
    let mut week: SolarWeek = SolarWeek::from_ym(2024, 2, 2, start).unwrap();
    assert_eq!("2024年2月第三周", week.to_string());
    assert_eq!(6, week.get_index_in_year());

    week = SolarDay::from_ymd(2024, 2, 11).unwrap().get_solar_week(start);
    assert_eq!("2024年2月第三周", week.to_string());

    week = SolarDay::from_ymd(2024, 2, 17).unwrap().get_solar_week(start);
    assert_eq!("2024年2月第三周", week.to_string());

    week = SolarDay::from_ymd(2024, 2, 10).unwrap().get_solar_week(start);
    assert_eq!("2024年2月第二周", week.to_string());

    week = SolarDay::from_ymd(2024, 2, 18).unwrap().get_solar_week(start);
    assert_eq!("2024年2月第四周", week.to_string());
  }
}
