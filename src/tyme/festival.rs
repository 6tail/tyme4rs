use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

use crate::tyme::enums::EventType;
use crate::tyme::event::Event;
use crate::tyme::lunar::LunarDay;
use crate::tyme::solar::{SolarDay, SolarTerm, SolarTermDay};
use crate::tyme::unit::DayUnit;
use crate::tyme::{AbstractTyme, Culture, Tyme};

/// 节日抽象
#[derive(Debug, Clone)]
pub struct AbstractFestival {
    parent: AbstractTyme,
    /// 索引
    index: usize,
    /// 日
    day: DayUnit,
    /// 事件
    event: Event,
}

impl Deref for AbstractFestival {
    type Target = AbstractTyme;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for AbstractFestival {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Culture for AbstractFestival {
    fn get_name(&self) -> String {
        self.event.get_name()
    }
}

impl AbstractFestival {
    pub fn new(index: usize, event: Event, day: DayUnit) -> Self {
        Self {
            parent: AbstractTyme::new(),
            index,
            day,
            event,
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_day(&self) -> DayUnit {
        self.day
    }
}

impl Display for AbstractFestival {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.get_day(), self.get_name())
    }
}

impl PartialEq for AbstractFestival {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for AbstractFestival {}

pub static SOLAR_FESTIVAL_NAMES: [&str; 10] = [
    "元旦",
    "妇女节",
    "植树节",
    "劳动节",
    "青年节",
    "儿童节",
    "建党节",
    "建军节",
    "教师节",
    "国庆节",
];
pub static SOLAR_FESTIVAL_DATA: &str =
    "0VV__0Ux0Xc__0Ux0Xg__0_Q0ZV__0Ux0ZY__0Ux0aV__0Ux0bV__0Uo0cV__0Ug0de__0_V0eV__0Ux";

/// 公历现代节日
#[derive(Debug, Clone)]
pub struct SolarFestival {
    parent: AbstractFestival,
}

impl Deref for SolarFestival {
    type Target = AbstractFestival;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for SolarFestival {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Culture for SolarFestival {
    fn get_name(&self) -> String {
        SOLAR_FESTIVAL_NAMES[self.index].to_string()
    }
}

impl SolarFestival {
    pub fn new(index: usize, event: Event, day: SolarDay) -> Self {
        Self {
            parent: AbstractFestival::new(
                index,
                event,
                DayUnit::new(
                    day.get_year(),
                    day.get_month() as isize,
                    day.get_day() as isize,
                ),
            ),
        }
    }

    pub fn from_ymd(year: isize, month: usize, day: usize) -> Option<Self> {
        let d: SolarDay = SolarDay::from_ymd(year, month, day);
        for (i, name) in SOLAR_FESTIVAL_NAMES.iter().enumerate() {
            let start: usize = i * 8;
            let e: Event = Event::new(
                name,
                format!("@{}", &SOLAR_FESTIVAL_DATA[start..start + 8]).as_str(),
            )
            .unwrap();
            let m: usize = e.get_value(2) as usize;
            let day: usize = e.get_value(3) as usize;
            if d.get_year() >= e.get_start_year() && d.get_month() == m && d.get_day() == day {
                return Some(Self::new(i, e, d));
            }
        }
        None
    }

    pub fn from_index(year: isize, index: usize) -> Option<Self> {
        if index >= SOLAR_FESTIVAL_NAMES.len() {
            return None;
        }
        let start: usize = index * 8;
        let e: Event = Event::new(
            SOLAR_FESTIVAL_NAMES[index],
            format!("@{}", &SOLAR_FESTIVAL_DATA[start..start + 8]).as_str(),
        )
        .unwrap();
        if year < e.get_start_year() {
            return None;
        }
        let m: usize = e.get_value(2) as usize;
        let d: usize = e.get_value(3) as usize;
        Some(Self::new(index, e, SolarDay::from_ymd(year, m, d)))
    }

    pub fn get_day(&self) -> SolarDay {
        let m: usize = self.day.get_month() as usize;
        let d: usize = self.day.get_day() as usize;
        SolarDay::from_ymd(self.day.get_year(), m, d)
    }

    pub fn get_start_year(&self) -> isize {
        self.event.get_start_year()
    }

    pub fn next(&self, n: isize) -> Option<Self> {
        let size: isize = SOLAR_FESTIVAL_NAMES.len() as isize;
        let i: isize = self.get_index() as isize + n;
        Self::from_index(
            (self.day.get_year() * size + i) / size,
            self.index_of(i, size as usize),
        )
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

pub static LUNAR_FESTIVAL_NAMES: [&str; 13] = [
    "春节",
    "元宵节",
    "龙头节",
    "上巳节",
    "清明节",
    "端午节",
    "七夕节",
    "中元节",
    "中秋节",
    "重阳节",
    "冬至节",
    "腊八节",
    "除夕",
];
pub static LUNAR_FESTIVAL_DATA: &str = "2VV__0002Vj__0002WW__0002XX__0003b___0002ZZ__0002bb__0002bj__0002cj__0002dd__0003s___0002gc__0002hV_U000";

/// 农历传统节日（依据国家标准《农历的编算和颁行》GB/T 33661-2017）
#[derive(Debug, Clone)]
pub struct LunarFestival {
    parent: AbstractFestival,
}

impl Deref for LunarFestival {
    type Target = AbstractFestival;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for LunarFestival {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Culture for LunarFestival {
    fn get_name(&self) -> String {
        LUNAR_FESTIVAL_NAMES[self.index].to_string()
    }
}

impl LunarFestival {
    pub fn new(index: usize, event: Event, day: LunarDay) -> Self {
        Self {
            parent: AbstractFestival::new(
                index,
                event,
                DayUnit::new(day.get_year(), day.get_month(), day.get_day() as isize),
            ),
        }
    }

    pub fn from_ymd(year: isize, month: isize, day: usize) -> Option<Self> {
        let d: LunarDay = LunarDay::from_ymd(year, month, day);
        for (i, name) in LUNAR_FESTIVAL_NAMES.iter().enumerate() {
            let start: usize = i * 8;
            let e: Event = Event::new(
                name,
                format!("@{}", &LUNAR_FESTIVAL_DATA[start..start + 8]).as_str(),
            )
            .unwrap();
            match e.get_type() {
                EventType::LunarDay => {
                    let offset: isize = e.get_value(5);
                    if 0 == offset {
                        if d.get_month() == e.get_value(2) && d.get_day() == e.get_value(3) as usize
                        {
                            return Some(LunarFestival::new(i, e, d));
                        }
                    } else {
                        let m: (isize, isize) = e.get_month(d.get_year());
                        let next: LunarDay = d.next(-offset);
                        if next.get_year() == m.0
                            && next.get_month() == m.1
                            && next.get_day() == e.get_value(3) as usize
                        {
                            return Some(LunarFestival::new(i, e, d));
                        }
                    }
                }
                EventType::TermDay => {
                    let term: SolarTermDay = d.get_solar_day().get_term_day();
                    if term.day_index == 0
                        && term.get_solar_term().index == e.get_value(2) as usize % 24
                    {
                        return Some(LunarFestival::new(i, e, d));
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn from_index(year: isize, index: usize) -> Option<Self> {
        if index >= LUNAR_FESTIVAL_NAMES.len() {
            return None;
        }
        let start: usize = index * 8;
        let e: Event = Event::new(
            LUNAR_FESTIVAL_NAMES[index],
            format!("@{}", &LUNAR_FESTIVAL_DATA[start..start + 8]).as_str(),
        )
        .unwrap();
        match e.get_type() {
            EventType::LunarDay => {
                let m: (isize, isize) = e.get_month(year);
                let mut d: LunarDay = LunarDay::from_ymd(m.0, m.1, e.get_value(3) as usize);
                let offset: isize = e.get_value(5);
                if 0 != offset {
                    d = d.next(offset);
                }
                Some(LunarFestival::new(index, e, d))
            }
            EventType::TermDay => {
                let offset: isize = e.get_value(2);
                Some(LunarFestival::new(
                    index,
                    e,
                    SolarTerm::from_index(year, offset)
                        .get_solar_day()
                        .get_lunar_day(),
                ))
            }
            _ => None,
        }
    }

    pub fn get_day(&self) -> LunarDay {
        LunarDay::from_ymd(
            self.day.get_year(),
            self.day.get_month(),
            self.day.get_day() as usize,
        )
    }

    pub fn get_solar_term(&self) -> Option<SolarTerm> {
        let t: SolarTermDay = self.get_day().get_solar_day().get_term_day();
        if t.get_day_index() == 0 {
            Some(t.get_solar_term())
        } else {
            None
        }
    }

    pub fn next(&self, n: isize) -> Option<Self> {
        let size: isize = LUNAR_FESTIVAL_NAMES.len() as isize;
        let i: isize = self.get_index() as isize + n;
        Self::from_index(
            (self.get_day().get_year() * size + i) / size,
            self.index_of(i, size as usize),
        )
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
        assert_eq!(
            "农历壬寅年十一月廿九 冬至节",
            f.next(-3).unwrap().to_string()
        );
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
        let n: SolarFestival = f.unwrap();
        assert_eq!("2024年5月1日 劳动节", n.next(13).unwrap().to_string());
        assert_eq!("2022年8月1日 建军节", n.next(-3).unwrap().to_string());
    }

    #[test]
    fn test6() {
        let f: Option<SolarFestival> = SolarFestival::from_index(2023, 0);
        assert_eq!(false, f.is_none());
        assert_eq!(
            "2022年3月8日 妇女节",
            f.unwrap().next(-9).unwrap().to_string()
        );
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
        assert_eq!("2021年5月4日 青年节", f.unwrap().to_string());
    }

    #[test]
    fn test9() {
        let f: Option<SolarFestival> = SolarDay::from_ymd(1939, 5, 4).get_festival();
        assert_eq!(true, f.is_none());
    }
}
