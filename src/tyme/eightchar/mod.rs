use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::tyme::{Culture, Tyme};
use crate::tyme::culture::Duty;
use crate::tyme::eightchar::provider::{ChildLimitProvider, DefaultChildLimitProvider};
use crate::tyme::enums::{Gender, YinYang};
use crate::tyme::lunar::LunarYear;
use crate::tyme::sixtycycle::{EarthBranch, HeavenStem, SixtyCycle, SixtyCycleYear};
use crate::tyme::solar::{SolarDay, SolarTerm, SolarTime};

pub mod provider;

/// 八字
#[derive(Debug, Clone)]
pub struct EightChar {
  year: SixtyCycle,
  month: SixtyCycle,
  day: SixtyCycle,
  hour: SixtyCycle,
}

impl Culture for EightChar {
  fn get_name(&self) -> String {
    format!("{} {} {} {}", self.year, self.month, self.day, self.hour)
  }
}

impl EightChar {
  pub fn new(year: &str, month: &str, day: &str, hour: &str) -> Self {
    Self {
      year: SixtyCycle::from_name(year),
      month: SixtyCycle::from_name(month),
      day: SixtyCycle::from_name(day),
      hour: SixtyCycle::from_name(hour),
    }
  }

  pub fn from_sixty_cycle(year: SixtyCycle, month: SixtyCycle, day: SixtyCycle, hour: SixtyCycle) -> Self {
    Self {
      year,
      month,
      day,
      hour,
    }
  }

  pub fn get_year(&self) -> SixtyCycle {
    self.year.clone()
  }

  pub fn get_month(&self) -> SixtyCycle {
    self.month.clone()
  }

  pub fn get_day(&self) -> SixtyCycle {
    self.day.clone()
  }

  pub fn get_hour(&self) -> SixtyCycle {
    self.hour.clone()
  }

  pub fn get_fetal_origin(&self) -> SixtyCycle {
    SixtyCycle::from_name(format!("{}{}", self.month.get_heaven_stem().next(1).get_name(), self.month.get_earth_branch().next(3).get_name()).as_str())
  }

  pub fn get_fetal_breath(&self) -> SixtyCycle {
    SixtyCycle::from_name(format!("{}{}", self.day.get_heaven_stem().next(5).get_name(), EarthBranch::from_index(13 - (self.day.get_earth_branch().get_index() as isize)).get_name()).as_str())
  }

  pub fn get_own_sign(&self) -> SixtyCycle {
    let mut m: isize = self.month.get_earth_branch().get_index() as isize - 1;
    if m < 1 {
      m += 12;
    }
    let mut h: isize = self.hour.get_earth_branch().get_index() as isize - 1;
    if h < 1 {
      h += 12;
    }
    let mut offset: isize = m + h;
    offset = if offset >= 14 { 26 } else { 14 } - offset;
    SixtyCycle::from_name(format!("{}{}", HeavenStem::from_index(((self.year.get_heaven_stem().get_index() as isize) + 1) * 2 + offset - 1).get_name(), EarthBranch::from_index(offset + 1).get_name()).as_str())
  }

  pub fn get_body_sign(&self) -> SixtyCycle {
    let mut m: isize = self.month.get_earth_branch().get_index() as isize - 1;
    if m < 1 {
      m += 12;
    }
    let h: isize = self.hour.get_earth_branch().get_index() as isize + 1;
    let mut offset: isize = m + h;
    if offset > 12 {
      offset -= 12;
    }
    SixtyCycle::from_name(format!("{}{}", HeavenStem::from_index(((self.year.get_heaven_stem().get_index() as isize) + 1) * 2 + offset - 1).get_name(), EarthBranch::from_index(offset + 1).get_name()).as_str())
  }

  #[deprecated(since = "1.3.0", note = "please use SixtyCycleDay.get_duty() instead")]
  pub fn get_duty(&self) -> Duty {
    Duty::from_index((self.day.get_earth_branch().get_index() as isize) - (self.month.get_earth_branch().get_index() as isize))
  }

  pub fn get_solar_times(&self, start_year: isize, end_year: isize) -> Vec<SolarTime> {
    let mut l: Vec<SolarTime> = Vec::new();
    // 月地支距寅月的偏移值
    let mut m: isize = self.month.get_earth_branch().next(-2).get_index() as isize;
    // 月天干要一致
    if HeavenStem::from_index((self.year.get_heaven_stem().get_index() as isize + 1) * 2 + m) != self.month.get_heaven_stem() {
      return l;
    }
    // 1年的立春是辛酉，序号57
    let mut y: isize = self.year.next(-57).get_index() as isize + 1;
    // 节令偏移值
    m *= 2;
    // 时辰地支转时刻
    let h: usize = self.hour.get_earth_branch().get_index() * 2;
    let mut hours: Vec<usize> = vec![];
    hours.push(0);
    if h == 0 {
      hours.push(23);
    }
    let base_year: isize = start_year - 1;
    if base_year > y {
      y += 60 * ((base_year - y) as f64 / 60.0).ceil() as isize;
    }
    while y <= end_year {
      // 立春为寅月的开始
      let mut term: SolarTerm = SolarTerm::from_index(y, 3);
      // 节令推移，年干支和月干支就都匹配上了
      if m > 0 {
        term = term.next(m);
      }
      let solar_time: SolarTime = term.get_julian_day().get_solar_time();
      if solar_time.get_year() >= start_year {
        // 日干支和节令干支的偏移值
        let mut solar_day: SolarDay = solar_time.get_solar_day();
        let d: isize = self.day.next(-(solar_day.get_lunar_day().get_sixty_cycle().get_index() as isize)).get_index() as isize;
        if d > 0 {
          // 从节令推移天数
          solar_day = solar_day.next(d);
        }
        for &hour in hours.iter() {
          let mut mi: usize = 0;
          let mut s: usize = 0;
          if d == 0 && hour == solar_time.get_hour() {
            // 如果正好是节令当天，且小时和节令的小时数相等的极端情况，把分钟和秒钟带上
            mi = solar_time.get_minute();
            s = solar_time.get_second();
          }
          let time: SolarTime = SolarTime::from_ymd_hms(solar_day.get_year(), solar_day.get_month(), solar_day.get_day(), hour, mi, s);
          // 验证一下
          if time.get_lunar_hour().get_eight_char() == *self {
            l.push(time);
          }
        }
      }
      y += 60;
    }
    l
  }
}

impl Display for EightChar {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for EightChar {
  fn eq(&self, other: &Self) -> bool {
    self.get_name() == other.get_name()
  }
}

impl Eq for EightChar {}

/// 童限信息
#[derive(Debug, Clone)]
pub struct ChildLimitInfo {
  start_time: SolarTime,
  end_time: SolarTime,
  year_count: usize,
  month_count: usize,
  day_count: usize,
  hour_count: usize,
  minute_count: usize,
}

impl ChildLimitInfo {
  pub fn new(start_time: SolarTime, end_time: SolarTime, year_count: usize, month_count: usize, day_count: usize, hour_count: usize, minute_count: usize) -> Self {
    Self {
      start_time,
      end_time,
      year_count,
      month_count,
      day_count,
      hour_count,
      minute_count,
    }
  }

  pub fn get_start_time(&self) -> SolarTime {
    self.start_time
  }

  pub fn get_end_time(&self) -> SolarTime {
    self.end_time
  }

  pub fn get_year_count(&self) -> usize {
    self.year_count
  }

  pub fn get_month_count(&self) -> usize {
    self.month_count
  }

  pub fn get_day_count(&self) -> usize {
    self.day_count
  }

  pub fn get_hour_count(&self) -> usize {
    self.hour_count
  }

  pub fn get_minute_count(&self) -> usize {
    self.minute_count
  }
}

impl PartialEq for ChildLimitInfo {
  fn eq(&self, other: &Self) -> bool {
    self.get_start_time() == other.get_start_time() && self.get_end_time() == other.get_end_time() && self.get_year_count() == other.get_year_count() && self.get_month_count() == other.get_month_count() && self.get_day_count() == other.get_day_count() && self.get_hour_count() == other.get_hour_count() && self.get_minute_count() == other.get_minute_count()
  }
}

impl Eq for ChildLimitInfo {}

lazy_static! {
  static ref CHILD_LIMIT_PROVIDER: Arc<Mutex<Box<dyn ChildLimitProvider + Sync + Send + 'static>>> = Arc::new(Mutex::new(Box::new(DefaultChildLimitProvider::new())));
}

/// 童限（从出生到起运的时间段）
#[derive(Debug, Clone)]
pub struct ChildLimit {
  eight_char: EightChar,
  gender: Gender,
  forward: bool,
  info: ChildLimitInfo,
}

impl ChildLimit {
  pub fn from_solar_time(birth_time: SolarTime, gender: Gender) -> Self {
    let eight_char: EightChar = birth_time.get_lunar_hour().get_eight_char();
    // 阳男阴女顺推，阴男阳女逆推
    let yang: bool = YinYang::YANG == eight_char.get_year().get_heaven_stem().get_yin_yang();
    let man: bool = Gender::MAN == gender;
    let forward: bool = (yang && man) || (!yang && !man);
    let mut term: SolarTerm = birth_time.get_term();
    if !term.is_jie() {
      term = term.next(-1);
    }
    if forward {
      term = term.next(2);
    }
    let info: ChildLimitInfo = CHILD_LIMIT_PROVIDER.lock().unwrap().get_info(birth_time, term);

    Self {
      eight_char,
      gender,
      forward,
      info,
    }
  }

  pub fn get_eight_char(&self) -> EightChar {
    self.eight_char.clone()
  }

  pub fn get_gender(&self) -> Gender {
    self.gender
  }

  pub fn is_forward(&self) -> bool {
    self.forward
  }

  pub fn get_year_count(&self) -> usize {
    self.info.get_year_count()
  }

  pub fn get_month_count(&self) -> usize {
    self.info.get_month_count()
  }

  pub fn get_day_count(&self) -> usize {
    self.info.get_day_count()
  }

  pub fn get_hour_count(&self) -> usize {
    self.info.get_hour_count()
  }

  pub fn get_minute_count(&self) -> usize {
    self.info.get_minute_count()
  }

  pub fn get_start_time(&self) -> SolarTime {
    self.info.get_start_time()
  }

  pub fn get_end_time(&self) -> SolarTime {
    self.info.get_end_time()
  }

  /// 起始大运
  pub fn get_start_decade_fortune(&self) -> DecadeFortune {
    DecadeFortune::from_child_limit(self.clone(), 0)
  }

  /// 所属大运
  pub fn get_decade_fortune(&self) -> DecadeFortune {
    DecadeFortune::from_child_limit(self.clone(), -1)
  }

  pub fn get_start_fortune(&self) -> Fortune {
    Fortune::from_child_limit(self.clone(), 0)
  }

  #[deprecated(since = "1.3.0", note = "please use get_end_sixty_cycle_year() instead")]
  pub fn get_end_lunar_year(&self) -> LunarYear {
    LunarYear::from_year(self.get_start_time().get_lunar_hour().get_year() + self.get_end_time().get_year() - self.get_start_time().get_year())
  }

  /// 开始(即出生)干支年
  pub fn get_start_sixty_cycle_year(&self) -> SixtyCycleYear {
    SixtyCycleYear::from_year(self.get_start_time().get_year())
  }

  /// 结束(即起运)干支年
  pub fn get_end_sixty_cycle_year(&self) -> SixtyCycleYear {
    SixtyCycleYear::from_year(self.get_end_time().get_year())
  }

  /// 开始年龄
  pub fn get_start_age(&self) -> usize {
    1
  }

  /// 结束年龄
  pub fn get_end_age(&self) -> usize {
    let n: isize = self.get_end_sixty_cycle_year().get_year() - self.get_start_sixty_cycle_year().get_year();
    if n > 1 {
      return n as usize;
    }
    1
  }
}

impl PartialEq for ChildLimit {
  fn eq(&self, other: &Self) -> bool {
    self.get_start_time() == other.get_start_time() && self.get_gender() == other.get_gender()
  }
}

impl Eq for ChildLimit {}

/// 大运（10年1大运）
#[derive(Debug, Clone)]
pub struct DecadeFortune {
  child_limit: ChildLimit,
  index: isize,
}

impl Culture for DecadeFortune {
  fn get_name(&self) -> String {
    self.get_sixty_cycle().get_name()
  }
}

impl Tyme for DecadeFortune {
  fn next(&self, n: isize) -> Self {
    Self::new(self.get_child_limit(), self.index + n)
  }
}

impl DecadeFortune {
  pub fn new(child_limit: ChildLimit, index: isize) -> Self {
    Self {
      child_limit,
      index,
    }
  }

  pub fn from_child_limit(child_limit: ChildLimit, index: isize) -> Self {
    Self::new(child_limit, index)
  }

  pub fn get_child_limit(&self) -> ChildLimit {
    self.child_limit.clone()
  }

  pub fn get_index(&self) -> isize {
    self.index
  }

  pub fn get_start_age(&self) -> isize {
    self.child_limit.get_end_sixty_cycle_year().get_year() - self.child_limit.get_start_sixty_cycle_year().get_year() + 1 + self.index * 10
  }

  pub fn get_end_age(&self) -> isize {
    self.get_start_age() + 9
  }

  #[deprecated(since = "1.3.0", note = "please use get_start_sixty_cycle_year() instead")]
  pub fn get_start_lunar_year(&self) -> LunarYear {
    LunarYear::from_year(self.child_limit.get_start_time().get_lunar_hour().get_year() + self.child_limit.get_end_time().get_year() - self.child_limit.get_start_time().get_year()).next(self.index * 10)
  }

  /// 开始干支年
  pub fn get_start_sixty_cycle_year(&self) -> SixtyCycleYear {
    self.child_limit.get_end_sixty_cycle_year().next(self.index * 10)
  }

  #[deprecated(since = "1.3.0", note = "please use get_end_sixty_cycle_year() instead")]
  pub fn get_end_lunar_year(&self) -> LunarYear {
    LunarYear::from_year(self.child_limit.get_start_time().get_lunar_hour().get_year() + self.child_limit.get_end_time().get_year() - self.child_limit.get_start_time().get_year()).next(self.index * 10 + 9)
  }

  /// 结束干支年
  pub fn get_end_sixty_cycle_year(&self) -> SixtyCycleYear {
    self.get_start_sixty_cycle_year().next(9)
  }

  pub fn get_sixty_cycle(&self) -> SixtyCycle {
    let n: isize = self.index + 1;
    self.child_limit.get_eight_char().get_month().next(if self.child_limit.is_forward() { n } else { -n })
  }

  pub fn get_start_fortune(&self) -> Fortune {
    Fortune::from_child_limit(self.child_limit.clone(), self.index * 10)
  }
}

impl PartialEq for DecadeFortune {
  fn eq(&self, other: &Self) -> bool {
    self.get_child_limit() == other.get_child_limit() && self.get_index() == other.get_index()
  }
}

impl Eq for DecadeFortune {}

/// 小运
#[derive(Debug, Clone)]
pub struct Fortune {
  child_limit: ChildLimit,
  index: isize,
}

impl Culture for Fortune {
  fn get_name(&self) -> String {
    self.get_sixty_cycle().get_name()
  }
}

impl Tyme for Fortune {
  fn next(&self, n: isize) -> Self {
    Self::new(self.get_child_limit(), self.get_index() + n)
  }
}

impl Fortune {
  pub fn new(child_limit: ChildLimit, index: isize) -> Self {
    Self {
      child_limit,
      index,
    }
  }

  pub fn from_child_limit(child_limit: ChildLimit, index: isize) -> Self {
    Self::new(child_limit, index)
  }

  pub fn get_child_limit(&self) -> ChildLimit {
    self.child_limit.clone()
  }

  pub fn get_index(&self) -> isize {
    self.index
  }

  pub fn get_age(&self) -> isize {
    self.child_limit.get_end_sixty_cycle_year().get_year() - self.child_limit.get_start_sixty_cycle_year().get_year() + 1 + self.index
  }

  #[deprecated(since = "1.3.0", note = "please use get_sixty_cycle_year() instead")]
  pub fn get_lunar_year(&self) -> LunarYear {
    LunarYear::from_year(self.child_limit.get_start_time().get_lunar_hour().get_year() + self.child_limit.get_end_time().get_year() - self.child_limit.get_start_time().get_year()).next(self.index)
  }

  /// 干支年
  pub fn get_sixty_cycle_year(&self) -> SixtyCycleYear {
    self.child_limit.get_end_sixty_cycle_year().next(self.index)
  }

  pub fn get_sixty_cycle(&self) -> SixtyCycle {
    let n: isize = self.get_age();
    self.child_limit.get_eight_char().get_hour().next(if self.child_limit.is_forward() { n } else { -n })
  }
}

impl PartialEq for Fortune {
  fn eq(&self, other: &Self) -> bool {
    self.get_child_limit() == other.get_child_limit() && self.get_index() == other.get_index()
  }
}

impl Eq for Fortune {}

#[cfg(test)]
mod tests {
  use std::sync::MutexGuard;
  use crate::tyme::eightchar::{CHILD_LIMIT_PROVIDER, ChildLimit};
  use crate::tyme::eightchar::provider::{ChildLimitProvider, DefaultChildLimitProvider};
  use crate::tyme::enums::Gender;
  use crate::tyme::solar::SolarTime;

  #[test]
  fn test0() {
    // 动态切换童限实现
    {
      let mut provider: MutexGuard<Box<dyn ChildLimitProvider + Sync + Send + 'static>> = CHILD_LIMIT_PROVIDER.lock().unwrap();
      *provider = Box::new(DefaultChildLimitProvider::new());
    }

    let d: ChildLimit = ChildLimit::from_solar_time(SolarTime::from_ymd_hms(1989, 12, 31, 23, 7, 17), Gender::MAN);
    assert_eq!("1998年3月1日 19:47:17", d.get_end_time().to_string());
  }
}