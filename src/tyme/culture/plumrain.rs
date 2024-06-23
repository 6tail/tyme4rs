use std::fmt::{Display, Formatter};

use crate::tyme::{AbstractCulture, AbstractCultureDay, AbstractTyme, Culture, LoopTyme, Tyme};

pub static PLUM_RAIN_NAMES: [&str; 2] = ["入梅", "出梅"];

/// 梅雨
#[derive(Debug, Clone)]
pub struct PlumRain {
  parent: LoopTyme,
}

impl Tyme for PlumRain {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for PlumRain {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl PlumRain {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(PLUM_RAIN_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(PLUM_RAIN_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for PlumRain {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for PlumRain {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for PlumRain {}

impl Into<LoopTyme> for PlumRain {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

/// 梅雨天
#[derive(Debug, Clone)]
pub struct PlumRainDay {
  parent: AbstractCultureDay,
  plum_rain: PlumRain,
}

impl Culture for PlumRainDay {
  fn get_name(&self) -> String {
    self.plum_rain.get_name()
  }
}

impl PlumRainDay {
  pub fn new(plum_rain: PlumRain, day_index: usize) -> Self {
    let loop_tyme: LoopTyme = plum_rain.clone().into();
    let abstract_tyme: AbstractTyme = loop_tyme.into();
    let culture: AbstractCulture = abstract_tyme.into();
    Self {
      parent: AbstractCultureDay::new(culture, day_index),
      plum_rain,
    }
  }

  pub fn get_plum_rain(&self) -> PlumRain {
    self.plum_rain.clone()
  }

  pub fn get_day_index(&self) -> usize {
    self.parent.get_day_index()
  }
}

impl Display for PlumRainDay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.plum_rain.get_index() == 0 {
      write!(f, "{}第{}天", self.get_name(), self.parent.get_day_index() + 1)
    } else {
      write!(f, "{}", self.plum_rain.get_name())
    }
  }
}

impl PartialEq for PlumRainDay {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for PlumRainDay {}

impl Into<AbstractCultureDay> for PlumRainDay {
  fn into(self) -> AbstractCultureDay {
    self.parent
  }
}

#[cfg(test)]
mod tests {
  use crate::tyme::Culture;
  use crate::tyme::culture::plumrain::PlumRainDay;
  use crate::tyme::solar::SolarDay;

  #[test]
  fn test1() {
    assert_eq!(true, SolarDay::from_ymd(2024, 6, 10).unwrap().get_plum_rain_day().is_none());
  }

  #[test]
  fn test2() {
    let d: PlumRainDay = SolarDay::from_ymd(2024, 6, 11).unwrap().get_plum_rain_day().unwrap();
    assert_eq!("入梅", d.get_name());
    assert_eq!("入梅", d.get_plum_rain().to_string());
    assert_eq!("入梅第1天", d.to_string());
  }

  #[test]
  fn test3() {
    let d: PlumRainDay = SolarDay::from_ymd(2024, 7, 6).unwrap().get_plum_rain_day().unwrap();
    assert_eq!("出梅", d.get_name());
    assert_eq!("出梅", d.get_plum_rain().to_string());
    assert_eq!("出梅", d.to_string());
  }

  #[test]
  fn test4() {
    let d: PlumRainDay = SolarDay::from_ymd(2024, 7, 5).unwrap().get_plum_rain_day().unwrap();
    assert_eq!("入梅", d.get_name());
    assert_eq!("入梅", d.get_plum_rain().to_string());
    assert_eq!("入梅第25天", d.to_string());
  }
}
