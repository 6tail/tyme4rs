use crate::tyme::enums::EventType;
use crate::tyme::lunar::LunarDay;
use crate::tyme::lunar::LunarMonth;
use crate::tyme::solar::{SolarDay, SolarMonth, SolarTerm};
use crate::tyme::{AbstractCulture, Culture, Tyme};
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::iter::Iterator;
use std::ops::{Deref, DerefMut};
use std::string::ToString;
use std::sync::{Mutex, MutexGuard};

pub static EVENT_MANAGER_DATA: Mutex<String> = Mutex::new(String::new());
pub static EVENT_MANAGER_REGEX: &str = "(@[0-9A-Za-z_]{8})";

lazy_static! {
    static ref EVENT_MANAGER_CHARS: Vec<char> =
        "0123456789ABCDEFGHIJKLMNOPQRSTU_VWXYZabcdefghijklmnopqrstuvwxyz"
            .chars()
            .collect();
}

/// 事件
#[derive(Debug, Clone, Eq)]
pub struct Event {
    parent: AbstractCulture,
    /// 名称
    name: String,
    /// 数据
    data: String,
}

impl Deref for Event {
    type Target = AbstractCulture;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for Event {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Culture for Event {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

impl Event {
    fn new(name: &str, data: &str) -> Result<Self, String> {
        Self::validate(data)?;
        Ok(Self {
            parent: AbstractCulture::new(),
            name: name.to_string(),
            data: data.to_string(),
        })
    }

    pub fn validate(data: &str) -> Result<(), String> {
        if data.len() != 9 {
            Err(format!("illegal event data: {}", data))
        } else {
            Ok(())
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        let reg: Regex = Regex::new(format!("{}({})", EVENT_MANAGER_REGEX, name).as_str()).unwrap();
        if let Some(caps) = reg.captures(EVENT_MANAGER_DATA.lock().unwrap().as_str()) {
            let data: String = caps.get(1).unwrap().as_str().to_string();
            return Some(Self {
                parent: AbstractCulture::new(),
                name: name.to_string(),
                data,
            });
        }
        None
    }

    pub fn builder() -> EventBuilder {
        EventBuilder::new()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_data(&self) -> String {
        self.data.clone()
    }

    pub fn get_type(&self) -> EventType {
        let t: char = self.data.chars().nth(1).unwrap();
        let i: usize = EVENT_MANAGER_CHARS.iter().position(|&c| c == t).unwrap();
        EventType::from_code(i).unwrap()
    }

    pub fn get_start_year(&self) -> isize {
        let mut n: isize = 0;
        let size: isize = EVENT_MANAGER_CHARS.len() as isize;
        let chars: Vec<char> = self.data.chars().collect();
        for i in 0..3 {
            let t: char = chars[6 + i];
            n = n * size + EVENT_MANAGER_CHARS.iter().position(|&c| c == t).unwrap() as isize;
        }
        n
    }

    pub fn all() -> Vec<Event> {
        let mut l: Vec<Self> = Vec::new();
        let reg: Regex = Regex::new(format!("{}(.[^@]+)", EVENT_MANAGER_REGEX).as_str()).unwrap();
        for caps in reg.captures_iter(EVENT_MANAGER_DATA.lock().unwrap().as_str()) {
            let data: String = caps.get(1).unwrap().as_str().to_string();
            let name: String = caps.get(2).unwrap().as_str().to_string();
            l.push(Self {
                parent: AbstractCulture::new(),
                name,
                data,
            });
        }
        l
    }

    pub fn from_solar_day(d: SolarDay) -> Vec<Self> {
        let mut l: Vec<Self> = Vec::new();
        for e in Self::all() {
            if let Some(t) = e.get_solar_day(d.get_year()) {
                if d == t {
                    l.push(e);
                }
            }
        }
        l
    }

    pub fn get_solar_day(&self, year: isize) -> Option<SolarDay> {
        let t: EventType = self.get_type();
        if year < self.get_start_year() {
            return None;
        }
        let d: Option<SolarDay> = match t {
            EventType::SolarDay => self.get_solar_day_by_solar_day(year),
            EventType::LunarDay => self.get_solar_day_by_lunar_day(year),
            EventType::SolarWeek => self.get_solar_day_by_week(year),
            EventType::TermDay => self.get_solar_day_by_term(year),
            EventType::TermHs => self.get_solar_day_by_term_heaven_stem(year),
            EventType::TermEb => self.get_solar_day_by_term_earth_branch(year),
        };
        d?;
        let chars: Vec<char> = self.data.chars().collect();
        let offset: isize = EVENT_MANAGER_CHARS
            .iter()
            .position(|&c| c == chars[5])
            .unwrap() as isize
            - 31;
        if offset != 0 {
            return Some(d.unwrap().next(offset));
        }
        d
    }

    fn get_solar_day_by_solar_day(&self, year: isize) -> Option<SolarDay> {
        let mut y: isize = year;
        let chars: Vec<char> = self.data.chars().collect();
        let mut m: usize = EVENT_MANAGER_CHARS
            .iter()
            .position(|&c| c == chars[2])
            .unwrap()
            - 31;
        if m > 12 {
            m = 1;
            y += 1;
        }
        let d: usize = EVENT_MANAGER_CHARS
            .iter()
            .position(|&c| c == chars[3])
            .unwrap()
            - 31;
        let delay: isize = EVENT_MANAGER_CHARS
            .iter()
            .position(|&c| c == chars[4])
            .unwrap() as isize
            - 31;
        let month: SolarMonth = SolarMonth::from_ym(y, m);
        let last_day: usize = month.get_day_count();
        if d > last_day {
            if 0 == delay {
                return None;
            } else if delay < 0 {
                return Some(SolarDay::from_ymd(y, m, (d as isize + delay) as usize));
            }
            return Some(SolarDay::from_ymd(y, m, last_day).next(delay));
        }
        Some(SolarDay::from_ymd(y, m, d))
    }

    fn get_solar_day_by_lunar_day(&self, year: isize) -> Option<SolarDay> {
        let mut y: isize = year;
        let chars: Vec<char> = self.data.chars().collect();
        let mut m: isize = EVENT_MANAGER_CHARS
            .iter()
            .position(|&c| c == chars[2])
            .unwrap() as isize
            - 31;
        if m > 12 {
            m = 1;
            y += 1;
        }
        let d: usize = EVENT_MANAGER_CHARS
            .iter()
            .position(|&c| c == chars[3])
            .unwrap()
            - 31;
        let delay: isize = EVENT_MANAGER_CHARS
            .iter()
            .position(|&c| c == chars[4])
            .unwrap() as isize
            - 31;
        let month: LunarMonth = LunarMonth::from_ym(y, m);
        let last_day: usize = month.get_day_count();
        if d > last_day {
            if 0 == delay {
                return None;
            } else if delay < 0 {
                return Some(
                    LunarDay::from_ymd(y, m, (d as isize + delay) as usize).get_solar_day(),
                );
            }
            return Some(
                LunarDay::from_ymd(y, m, last_day)
                    .get_solar_day()
                    .next(delay),
            );
        }
        Some(LunarDay::from_ymd(y, m, d).get_solar_day())
    }

    fn get_solar_day_by_week(&self, year: isize) -> Option<SolarDay> {
        let chars: Vec<char> = self.data.chars().collect();
        // 第几个星期
        let n: isize = EVENT_MANAGER_CHARS
            .iter()
            .position(|&c| c == chars[3])
            .unwrap() as isize
            - 31;
        if n == 0 {
            return None;
        }
        let m: SolarMonth = SolarMonth::from_ym(
            year,
            (EVENT_MANAGER_CHARS
                .iter()
                .position(|&c| c == chars[2])
                .unwrap() as isize
                - 31) as usize,
        );
        // 星期几
        let w: isize = EVENT_MANAGER_CHARS
            .iter()
            .position(|&c| c == chars[4])
            .unwrap() as isize
            - 31;
        if n > 0 {
            // 当月第1天
            let d: SolarDay = m.get_first_day();
            // 往后找第几个星期几
            return Some(d.next(d.get_week().steps_to(w) as isize + 7 * n - 7));
        }
        // 当月最后一天
        let d: SolarDay = SolarDay::from_ymd(year, m.get_month(), m.get_day_count());
        // 往前找第几个星期几
        Some(d.next(d.get_week().steps_back_to(w) + 7 * n + 7))
    }

    fn get_solar_day_by_term(&self, year: isize) -> Option<SolarDay> {
        let chars: Vec<char> = self.data.chars().collect();
        let offset: isize = EVENT_MANAGER_CHARS
            .iter()
            .position(|&c| c == chars[4])
            .unwrap() as isize
            - 31;
        let d: SolarDay = SolarTerm::from_index(
            year,
            EVENT_MANAGER_CHARS
                .iter()
                .position(|&c| c == chars[2])
                .unwrap() as isize
                - 31,
        )
        .get_solar_day();
        if offset != 0 {
            return Some(d.next(offset));
        }
        Some(d)
    }

    fn get_solar_day_by_term_heaven_stem(&self, year: isize) -> Option<SolarDay> {
        let d: SolarDay = self.get_solar_day_by_term(year)?;
        let chars: Vec<char> = self.data.chars().collect();
        Some(
            d.next(
                d.get_lunar_day()
                    .get_sixty_cycle()
                    .get_heaven_stem()
                    .steps_to(
                        EVENT_MANAGER_CHARS
                            .iter()
                            .position(|&c| c == chars[3])
                            .unwrap() as isize
                            - 31,
                    ) as isize,
            ),
        )
    }

    fn get_solar_day_by_term_earth_branch(&self, year: isize) -> Option<SolarDay> {
        let d: SolarDay = self.get_solar_day_by_term(year)?;
        let chars: Vec<char> = self.data.chars().collect();
        Some(
            d.next(
                d.get_lunar_day()
                    .get_sixty_cycle()
                    .get_earth_branch()
                    .steps_to(
                        EVENT_MANAGER_CHARS
                            .iter()
                            .position(|&c| c == chars[3])
                            .unwrap() as isize
                            - 31,
                    ) as isize,
            ),
        )
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_name())
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

/// 事件构造器
#[derive(Debug, Clone)]
pub struct EventBuilder {
    /// 事件名称
    name: String,
    /// 事件数据
    data: [char; 9],
}

impl EventBuilder {
    fn new() -> Self {
        Self {
            name: String::new(),
            data: ['@', '_', '_', '_', '_', '_', '0', '0', '0'],
        }
    }

    fn encode_type(t: EventType) -> char {
        if let Some(c) = EVENT_MANAGER_CHARS.get(t.get_code()) {
            *c
        } else {
            '_'
        }
    }

    fn content(mut self, t: EventType, a: isize, b: isize, c: isize) -> Self {
        self.data[1] = Self::encode_type(t);
        self.data[2] = *EVENT_MANAGER_CHARS.get((31 + a) as usize).unwrap();
        self.data[3] = *EVENT_MANAGER_CHARS.get((31 + b) as usize).unwrap();
        self.data[4] = *EVENT_MANAGER_CHARS.get((31 + c) as usize).unwrap();
        self
    }

    fn term(self, t: EventType, a: isize, b: isize, c: isize) -> Self {
        let n: isize = if c.abs() > 31 { c.signum() * 31 } else { 0 };
        self.content(t, a, b, c - n).offset(n)
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn solar_day(self, solar_month: usize, solar_day: usize, delay_days: isize) -> Self {
        self.content(
            EventType::SolarDay,
            solar_month as isize,
            solar_day as isize,
            delay_days,
        )
    }

    pub fn lunar_day(self, lunar_month: isize, lunar_day: usize, delay_days: isize) -> Self {
        self.content(
            EventType::LunarDay,
            lunar_month,
            lunar_day as isize,
            delay_days,
        )
    }

    pub fn solar_week(self, solar_month: usize, week_index: isize, week: usize) -> Self {
        self.content(
            EventType::SolarWeek,
            solar_month as isize,
            week_index,
            week as isize,
        )
    }

    pub fn term_day(self, term_index: usize, delay_days: isize) -> Self {
        self.term(EventType::TermDay, term_index as isize, 0, delay_days)
    }

    pub fn term_heaven_stem(
        self,
        term_index: usize,
        heaven_stem_index: usize,
        delay_days: isize,
    ) -> Self {
        self.term(
            EventType::TermHs,
            term_index as isize,
            heaven_stem_index as isize,
            delay_days,
        )
    }

    pub fn term_earth_branch(
        self,
        term_index: usize,
        earth_branch_index: usize,
        delay_days: isize,
    ) -> Self {
        self.term(
            EventType::TermEb,
            term_index as isize,
            earth_branch_index as isize,
            delay_days,
        )
    }

    pub fn start_year(mut self, year: isize) -> Self {
        let size: usize = EVENT_MANAGER_CHARS.len();
        let mut n: isize = year;
        for i in 0..3 {
            self.data[8 - i] = *EVENT_MANAGER_CHARS
                .get((n % size as isize) as usize)
                .unwrap();
            n /= size as isize;
        }
        self
    }

    pub fn offset(mut self, days: isize) -> Self {
        self.data[5] = *EVENT_MANAGER_CHARS.get((31 + days) as usize).unwrap();
        self
    }

    pub fn build(self) -> Event {
        let s: String = self.data.iter().collect();
        Event::new(self.name.as_str(), s.as_str()).unwrap()
    }
}

/// 事件管理器
pub struct EventManager {}

impl EventManager {
    pub fn delete(name: &str) {
        let reg: Regex = Regex::new(format!("{}{}", EVENT_MANAGER_REGEX, name).as_str()).unwrap();
        let mut s: MutexGuard<String> = EVENT_MANAGER_DATA.lock().unwrap();
        let str: &str = s.as_str();
        *s = reg.replace_all(str, "").to_string();
    }

    fn save_or_update(name: &str, data: &str) {
        let reg: Regex = Regex::new(format!("{}{}", EVENT_MANAGER_REGEX, name).as_str()).unwrap();
        let mut s: MutexGuard<String> = EVENT_MANAGER_DATA.lock().unwrap();
        let str: &str = s.as_str();
        if reg.is_match(str) {
            *s = reg.replace_all(str, data).to_string();
        } else {
            *s = str.to_owned() + data;
        }
    }

    pub fn update_data(name: &str, data: &str) -> Result<(), String> {
        Event::validate(data)?;
        Self::save_or_update(name, data);
        Ok(())
    }

    pub fn update(name: &str, e: Event) {
        let s: String = e.get_name();
        let mut n: &str = s.as_str();
        if n.is_empty() {
            n = name;
        }
        Self::save_or_update(name, format!("{}{}", e.get_data(), n).as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::tyme::event::{Event, EventManager};
    use crate::tyme::solar::SolarDay;

    fn setup() {
        // 公历现代节日
        // *EVENT_MANAGER_DATA.lock().unwrap() = "@0VV__0Ux公历现代节日:元旦@0Xc__0Ux公历现代节日:三八妇女节@0Xg__0_Q公历现代节日:植树节@0ZV__0Ux公历现代节日:五一劳动节@0ZY__0Ux公历现代节日:五四青年节@0aV__0Ux公历现代节日:六一儿童节@0bV__0Uo公历现代节日:建党节@0cV__0Ug公历现代节日:八一建军节@0de__0_V公历现代节日:教师节@0eV__0Ux公历现代节日:国庆节".to_string();
        EventManager::update(
            "公历现代节日:元旦",
            Event::builder().solar_day(1, 1, 0).start_year(1950).build(),
        );
        EventManager::update(
            "公历现代节日:三八妇女节",
            Event::builder().solar_day(3, 8, 0).start_year(1950).build(),
        );
        EventManager::update(
            "公历现代节日:植树节",
            Event::builder()
                .solar_day(3, 12, 0)
                .start_year(1979)
                .build(),
        );
        EventManager::update(
            "公历现代节日:五一劳动节",
            Event::builder().solar_day(5, 1, 0).start_year(1950).build(),
        );
        EventManager::update(
            "公历现代节日:五四青年节",
            Event::builder().solar_day(5, 4, 0).start_year(1950).build(),
        );
        EventManager::update(
            "公历现代节日:六一儿童节",
            Event::builder().solar_day(6, 1, 0).start_year(1950).build(),
        );
        EventManager::update(
            "公历现代节日:建党节",
            Event::builder().solar_day(7, 1, 0).start_year(1941).build(),
        );
        EventManager::update(
            "公历现代节日:八一建军节",
            Event::builder().solar_day(8, 1, 0).start_year(1933).build(),
        );
        EventManager::update(
            "公历现代节日:教师节",
            Event::builder()
                .solar_day(9, 10, 0)
                .start_year(1985)
                .build(),
        );
        EventManager::update(
            "公历现代节日:国庆节",
            Event::builder()
                .solar_day(10, 1, 0)
                .start_year(1950)
                .build(),
        );

        // 农历传统节日
        // *EVENT_MANAGER_DATA.lock().unwrap() = "@2VV__000农历传统节日:春节@2Vj__000农历传统节日:元宵节@2WW__000农历传统节日:龙头节@2XX__000农历传统节日:上巳节@3b___000农历传统节日:清明节@2ZZ__000农历传统节日:端午节@2bb__000农历传统节日:七夕节@2bj__000农历传统节日:中元节@2cj__000农历传统节日:中秋节@2dd__000农历传统节日:重阳节@3s___000农历传统节日:冬至节@2gc__000农历传统节日:腊八节@2hV_U000农历传统节日:除夕".to_string();
        EventManager::update(
            "农历传统节日:春节",
            Event::builder().lunar_day(1, 1, 0).build(),
        );
        EventManager::update(
            "农历传统节日:元宵节",
            Event::builder().lunar_day(1, 15, 0).build(),
        );
        EventManager::update(
            "农历传统节日:龙头节",
            Event::builder().lunar_day(2, 2, 0).build(),
        );
        EventManager::update(
            "农历传统节日:上巳节",
            Event::builder().lunar_day(3, 3, 0).build(),
        );
        EventManager::update(
            "农历传统节日:清明节",
            Event::builder().term_day(7, 0).build(),
        );
        EventManager::update(
            "农历传统节日:端午节",
            Event::builder().lunar_day(5, 5, 0).build(),
        );
        EventManager::update(
            "农历传统节日:七夕节",
            Event::builder().lunar_day(7, 7, 0).build(),
        );
        EventManager::update(
            "农历传统节日:中元节",
            Event::builder().lunar_day(7, 15, 0).build(),
        );
        EventManager::update(
            "农历传统节日:中秋节",
            Event::builder().lunar_day(8, 15, 0).build(),
        );
        EventManager::update(
            "农历传统节日:重阳节",
            Event::builder().lunar_day(9, 9, 0).build(),
        );
        EventManager::update(
            "农历传统节日:冬至节",
            Event::builder().term_day(24, 0).build(),
        );
        EventManager::update(
            "农历传统节日:腊八节",
            Event::builder().lunar_day(12, 8, 0).build(),
        );
        EventManager::update(
            "农历传统节日:除夕",
            Event::builder().lunar_day(13, 1, 0).offset(-1).build(),
        );

        EventManager::update(
            "情人节",
            Event::builder()
                .solar_day(2, 14, 0)
                .start_year(270)
                .build(),
        );
        EventManager::update(
            "国际消费者权益日",
            Event::builder()
                .solar_day(3, 15, 0)
                .start_year(1983)
                .build(),
        );
        EventManager::update(
            "愚人节",
            Event::builder().solar_day(4, 1, 0).start_year(1564).build(),
        );
        EventManager::update(
            "万圣夜",
            Event::builder().solar_day(10, 31, 0).start_year(600).build(),
        );
        EventManager::update(
            "万圣节",
            Event::builder().solar_day(11, 1, 0).start_year(600).build(),
        );
        EventManager::update(
            "平安夜",
            Event::builder().solar_day(12, 24, 0).start_year(336).build(),
        );
        EventManager::update(
            "圣诞节",
            Event::builder().solar_day(12, 25, 0).start_year(336).build(),
        );

        EventManager::update(
            "全国中小学生安全教育日",
            Event::builder()
                .solar_week(3, -1, 1)
                .start_year(1996)
                .build(),
        );
        EventManager::update(
            "母亲节",
            Event::builder()
                .solar_week(5, 2, 0)
                .start_year(1914)
                .build(),
        );
        EventManager::update(
            "父亲节",
            Event::builder()
                .solar_week(6, 3, 0)
                .start_year(1972)
                .build(),
        );
        EventManager::update(
            "感恩节",
            Event::builder()
                .solar_week(11, 4, 4)
                .start_year(1941)
                .build(),
        );

        // 清明前1天
        EventManager::update("寒食节", Event::builder().term_day(7, -1).build());
        // 立春后第5个戊日
        EventManager::update("春社", Event::builder().term_heaven_stem(3, 4, 40).build());
        // 立秋后第5个戊日
        EventManager::update("秋社", Event::builder().term_heaven_stem(15, 4, 40).build());

        // 夏至后第3个庚日
        EventManager::update("入伏", Event::builder().term_heaven_stem(12, 6, 20).build());
        // 夏至后第4个庚日
        EventManager::update("中伏", Event::builder().term_heaven_stem(12, 6, 30).build());
        // 立秋后第1个庚日
        EventManager::update("末伏", Event::builder().term_heaven_stem(15, 6, 0).build());

        // 芒种后第1个丙日
        EventManager::update("入梅", Event::builder().term_heaven_stem(11, 2, 0).build());
        // 小暑后第1个未日
        EventManager::update("出梅", Event::builder().term_earth_branch(13, 7, 0).build());

        // 如果没有2月29，则倒推1天
        EventManager::update(
            "公历生日",
            Event::builder()
                .solar_day(2, 29, -1)
                .start_year(2004)
                .build(),
        );

        EventManager::update(
            "农历生日",
            Event::builder()
                .lunar_day(4, 21, 0)
                .start_year(1986)
                .build(),
        );
    }

    #[test]
    fn test1() {
        setup();
        let mut e: Event = Event::from_name("公历生日").unwrap();

        let mut d: SolarDay = e.get_solar_day(2008).unwrap();
        assert_eq!("2008年2月29日", d.to_string());

        // 2005年没有2月29，按最初设置的，没有就倒推1天
        d = e.get_solar_day(2005).unwrap();
        assert_eq!("2005年2月28日", d.to_string());

        e = Event::from_name("农历生日").unwrap();

        d = e.get_solar_day(2026).unwrap();
        assert_eq!("2026年6月6日", d.to_string());
    }

    #[test]
    fn test2() {
        setup();
        let e: Event = Event::from_name("国际消费者权益日").unwrap();

        let d: SolarDay = e.get_solar_day(2026).unwrap();
        assert_eq!("2026年3月15日", d.to_string());
    }

    #[test]
    fn test3() {
        setup();
        let e: Event = Event::from_name("全国中小学生安全教育日").unwrap();

        let d: SolarDay = e.get_solar_day(2024).unwrap();
        assert_eq!("2024年3月25日", d.to_string());
    }
}
