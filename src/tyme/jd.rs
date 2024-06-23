use std::fmt::{Display, Formatter};
use crate::tyme::{Culture, Tyme};
use crate::tyme::culture::Week;
use crate::tyme::solar::{SolarDay, SolarTime};

/// 2000年儒略日数(2000-1-1 12:00:00 UTC)
pub static J2000: f64 = 2451545.0;

/// 儒略日
#[derive(Debug, Copy, Clone)]
pub struct JulianDay {
  /// 儒略日
  day: f64,
}

impl Tyme for JulianDay {
  fn next(&self, n: isize) -> Result<Self, String> {
    Self::from_julian_day(self.day + n as f64)
  }
}

impl Culture for JulianDay {
  fn get_name(&self) -> String {
    format!("{}", self.day)
  }
}

impl JulianDay {
  pub fn from_julian_day(day: f64) -> Result<Self, String> {
    Ok(Self {
      day
    })
  }

  pub fn from_ymd_hms(year: isize, month: usize, day: usize, hour: usize, minute: usize, second: usize) -> Result<Self, String> {
    let d: f64 = (day as f64) + ((second as f64 * 1.0 / 60.0 + (minute as f64)) / 60.0 + (hour as f64)) / 24.0;
    let mut n: isize = 0;
    let g: bool = year * 372 + (month as isize) * 31 + (d as isize) >= 588829;
    let mut m: usize = month;
    let mut y: isize = year;
    if m <= 2 {
      m += 12;
      y -= 1;
    }
    if g {
      n = ((y as f64) / 100.0) as isize;
      n = 2 - n + (((n as f64) / 4.0) as isize);
    }
    Self::from_julian_day((((365.25 * ((y + 4716) as f64)) as isize) as f64) + (((30.6001 * (m + 1) as f64) as isize) as f64) + d + (n as f64) - 1524.5)
  }

  pub fn get_day(&self) -> f64 {
    self.day
  }

  pub fn get_week(&self) -> Week {
    Week::from_index((self.day + 0.5) as isize + 7000001)
  }

  pub fn get_solar_day(&self) -> SolarDay {
    let mut d: isize = (self.day + 0.5) as isize;
    let mut f: f64 = self.day + 0.5 - (d as f64);

    if d >= 2299161 {
      let c: isize = (((d as f64) - 1867216.25) / 36524.25) as isize;
      d += 1 + c - ((c as f64) / 4.0) as isize;
    }
    d += 1524;
    let mut year: isize = (((d as f64) - 122.1) / 365.25) as isize;
    d -= (365.25 * (year as f64)) as isize;
    let mut month: isize = ((d as f64) / 30.601) as isize;
    d -= (30.601 * (month as f64)) as isize;
    let mut day: isize = d;
    if month > 13 {
      month -= 13;
      year -= 4715;
    } else {
      month -= 1;
      year -= 4716;
    }
    f *= 24.0;
    let mut hour: isize = f as isize;

    f -= hour as f64;
    f *= 60.0;
    let mut minute: isize = f as isize;

    f -= minute as f64;
    f *= 60.0;
    let second: isize = f.round() as isize;
    if second > 59 {
      minute += 1
    }
    if minute > 59 {
      hour += 1
    }
    if hour > 23 {
      day += 1
    }
    SolarDay::from_ymd(year, month as usize, day as usize).unwrap()
  }

  pub fn get_solar_time(&self) -> SolarTime {
    let mut d: isize = (self.day + 0.5) as isize;
    let mut f: f64 = self.day + 0.5 - (d as f64);

    if d >= 2299161 {
      let c: isize = (((d as f64) - 1867216.25) / 36524.25) as isize;
      d += 1 + c - ((c as f64) / 4.0) as isize;
    }
    d += 1524;
    let mut year: isize = (((d as f64) - 122.1) / 365.25) as isize;
    d -= (365.25 * (year as f64)) as isize;
    let mut month: isize = ((d as f64) / 30.601) as isize;
    d -= (30.601 * (month as f64)) as isize;
    let mut day: isize = d;
    if month > 13 {
      month -= 13;
      year -= 4715;
    } else {
      month -= 1;
      year -= 4716;
    }
    f *= 24.0;
    let mut hour: isize = f as isize;

    f -= hour as f64;
    f *= 60.0;
    let mut minute: isize = f as isize;

    f -= minute as f64;
    f *= 60.0;
    let mut second: isize = f.round() as isize;
    if second > 59 {
      second -= 60;
      minute += 1
    }
    if minute > 59 {
      minute -= 60;
      hour += 1
    }
    if hour > 23 {
      hour -= 24;
      day += 1
    }
    SolarTime::from_ymd_hms(year, month as usize, day as usize, hour as usize, minute as usize, second as usize).unwrap()
  }

  /// 儒略日相减
  ///
  /// # 示例
  ///
  /// ```
  /// use tyme4rs::tyme::solar::SolarDay;
  ///
  /// let v: f64 = SolarDay::from_ymd(2023, 1, 10).unwrap().get_julian_day().subtract(SolarDay::from_ymd(2023, 1, 1).unwrap().get_julian_day());
  /// ```
  pub fn subtract(&self, target: JulianDay) -> f64 {
    self.day - target.get_day()
  }
}

impl Display for JulianDay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for JulianDay {
  fn eq(&self, other: &Self) -> bool {
    self.get_day() == other.get_day()
  }
}

impl Eq for JulianDay {}

#[cfg(test)]
mod tests {
  use crate::tyme::solar::{SolarDay};

  #[test]
  fn test1() {
    assert_eq!("2023年1月1日", SolarDay::from_ymd(2023, 1, 1).unwrap().get_julian_day().get_solar_day().to_string());
  }
}
