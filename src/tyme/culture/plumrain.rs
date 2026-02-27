use crate::tyme::{AbstractCulture, AbstractCultureDay, AbstractTyme, Culture, LoopTyme, Tyme};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

pub static PLUM_RAIN_NAMES: [&str; 2] = ["入梅", "出梅"];

/// 梅雨
#[derive(Debug, Clone)]
pub struct PlumRain {
    parent: LoopTyme,
}

impl Deref for PlumRain {
    type Target = LoopTyme;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for PlumRain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Tyme for PlumRain {
    fn next(&self, n: isize) -> Self {
        Self::from_index(self.parent.next_index(n) as isize)
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
            parent: LoopTyme::from_index(
                PLUM_RAIN_NAMES
                    .to_vec()
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
                index,
            ),
        }
    }

    pub fn from_name(name: &str) -> Self {
        Self {
            parent: LoopTyme::from_name(
                PLUM_RAIN_NAMES
                    .to_vec()
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
                name,
            ),
        }
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

impl From<PlumRain> for LoopTyme {
    fn from(val: PlumRain) -> Self {
        val.parent
    }
}

/// 梅雨天
#[derive(Debug, Clone)]
pub struct PlumRainDay {
    parent: AbstractCultureDay,
    plum_rain: PlumRain,
}

impl Deref for PlumRainDay {
    type Target = AbstractCultureDay;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for PlumRainDay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
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
}

impl Display for PlumRainDay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.plum_rain.get_index() == 0 {
            write!(f, "{}第{}天", self.get_name(), self.get_day_index() + 1)
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

impl From<PlumRainDay> for AbstractCultureDay {
    fn from(val: PlumRainDay) -> Self {
        val.parent
    }
}

#[cfg(test)]
mod tests {
    use crate::tyme::culture::plumrain::PlumRainDay;
    use crate::tyme::solar::SolarDay;
    use crate::tyme::Culture;

    #[test]
    fn test1() {
        assert!(SolarDay::from_ymd(2024, 6, 10)
            .get_plum_rain_day()
            .is_none());
    }

    #[test]
    fn test2() {
        let d: PlumRainDay = SolarDay::from_ymd(2024, 6, 11).get_plum_rain_day().unwrap();
        assert_eq!("入梅", d.get_name());
        assert_eq!("入梅", d.get_plum_rain().to_string());
        assert_eq!("入梅第1天", d.to_string());
    }

    #[test]
    fn test3() {
        let d: PlumRainDay = SolarDay::from_ymd(2024, 7, 6).get_plum_rain_day().unwrap();
        assert_eq!("出梅", d.get_name());
        assert_eq!("出梅", d.get_plum_rain().to_string());
        assert_eq!("出梅", d.to_string());
    }

    #[test]
    fn test4() {
        let d: PlumRainDay = SolarDay::from_ymd(2024, 7, 5).get_plum_rain_day().unwrap();
        assert_eq!("入梅", d.get_name());
        assert_eq!("入梅", d.get_plum_rain().to_string());
        assert_eq!("入梅第25天", d.to_string());
    }
}
