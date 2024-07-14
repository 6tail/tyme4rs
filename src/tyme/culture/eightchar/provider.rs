use crate::tyme::culture::eightchar::ChildLimitInfo;
use crate::tyme::solar::{SolarMonth, SolarTerm, SolarTime};
use crate::tyme::Tyme;

pub trait ChildLimitProvider {
  fn get_info(&self, birth_time: SolarTime, term: SolarTerm) -> ChildLimitInfo;
}

/// 默认的童限计算
#[derive(Debug, Copy, Clone)]
pub struct DefaultChildLimitProvider {}

impl DefaultChildLimitProvider {
  pub fn new() -> Self {
    Self {}
  }
}

impl ChildLimitProvider for DefaultChildLimitProvider {
  fn get_info(&self, birth_time: SolarTime, term: SolarTerm) -> ChildLimitInfo {
    // 出生时刻和节令时刻相差的秒数
    let mut seconds: usize = term.get_julian_day().get_solar_time().subtract(birth_time).abs() as usize;
    // 3天 = 1年，3天=60*60*24*3秒=259200秒 = 1年
    let year: usize = seconds / 259200;
    seconds %= 259200;
    // 1天 = 4月，1天=60*60*24秒=86400秒 = 4月，85400秒/4=21600秒 = 1月
    let month: usize = seconds / 21600;
    seconds %= 21600;
    // 1时 = 5天，1时=60*60秒=3600秒 = 5天，3600秒/5=720秒 = 1天
    let day: usize = seconds / 720;
    seconds %= 720;
    // 1分 = 2时，60秒 = 2时，60秒/2=30秒 = 1时
    let hour: usize = seconds / 30;
    seconds %= 30;
    // 1秒 = 2分，1秒/2=0.5秒 = 1分
    let minute: usize = seconds * 2;

    let mut d: usize = birth_time.get_day() + day;
    let mut h: usize = birth_time.get_hour() + hour;
    let mut mi: usize = birth_time.get_minute() + minute;
    h += mi / 60;
    mi %= 60;
    d += h / 24;
    h %= 24;

    let mut sm: SolarMonth = SolarMonth::from_ym(birth_time.get_year() + year as isize, birth_time.get_month()).unwrap().next(month as isize).unwrap();

    let dc: usize = sm.get_day_count();
    if d > dc {
      d -= dc;
      sm = sm.next(1).unwrap();
    }

    ChildLimitInfo {
      start_time: birth_time,
      end_time: SolarTime::from_ymd_hms(sm.get_year(), sm.get_month(), d, h, mi, birth_time.get_second()).unwrap(),
      year_count: year,
      month_count: month,
      day_count: day,
      hour_count: hour,
      minute_count: minute,
    }
  }
}

/// 元亨利贞的童限计算
#[derive(Debug, Copy, Clone)]
pub struct China95ChildLimitProvider {}

impl China95ChildLimitProvider {
  pub fn new() -> Self {
    Self {}
  }
}

impl ChildLimitProvider for China95ChildLimitProvider {
  fn get_info(&self, birth_time: SolarTime, term: SolarTerm) -> ChildLimitInfo {
    // 出生时刻和节令时刻相差的分钟数
    let mut minutes: usize = term.get_julian_day().get_solar_time().subtract(birth_time).abs() as usize / 60;
    let year: usize = minutes / 4320;
    minutes %= 4320;
    let month: usize = minutes / 360;
    minutes %= 360;
    let day: usize = minutes / 12;

    let mut sm: SolarMonth = SolarMonth::from_ym(birth_time.get_year() + year as isize, birth_time.get_month()).unwrap().next(month as isize).unwrap();

    let mut d: usize = birth_time.get_day() + day;
    let dc: usize = sm.get_day_count();
    if d > dc {
      d -= dc;
      sm = sm.next(1).unwrap();
    }

    ChildLimitInfo {
      start_time: birth_time,
      end_time: SolarTime::from_ymd_hms(sm.get_year(), sm.get_month(), d, birth_time.get_hour(), birth_time.get_minute(), birth_time.get_second()).unwrap(),
      year_count: year,
      month_count: month,
      day_count: day,
      hour_count: 0,
      minute_count: 0,
    }
  }
}
