use crate::tyme::jd::JulianDay;
use crate::tyme::solar::SolarDay;
use crate::tyme::unit::{DayUnit, MonthUnit, YearUnit};
use crate::tyme::{AbstractCulture, Culture, Tyme};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

/// 回历年
#[derive(Debug, Copy, Clone)]
pub struct HijriYear {
    parent: YearUnit,
}

impl Tyme for HijriYear {
    fn next(&self, n: isize) -> Self {
        Self::from_year(self.get_year() + n)
    }
}

impl Culture for HijriYear {
    fn get_name(&self) -> String {
        format!("{}年", self.get_year())
    }
}

impl Deref for HijriYear {
    type Target = YearUnit;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for HijriYear {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl HijriYear {
    pub fn new(year: isize) -> Result<Self, String> {
        Self::validate(year)?;
        Ok(Self {
            parent: YearUnit::new(year),
        })
    }

    pub fn validate(year: isize) -> Result<(), String> {
        AbstractCulture::validate_range(year, -640, 9666, "hijri year")
    }

    pub fn from_year(year: isize) -> Self {
        Self::new(year).unwrap()
    }

    /// 当年总天数
    pub fn get_day_count(&self) -> usize {
        if self.is_leap() {
            355
        } else {
            354
        }
    }

    /// 是否闰年
    pub fn is_leap(&self) -> bool {
        let i: usize = self.index_of(self.get_year() - 1, 30);
        i == 1
            || i == 4
            || i == 6
            || i == 9
            || i == 12
            || i == 15
            || i == 17
            || i == 20
            || i == 23
            || i == 25
            || i == 28
    }

    /// 回历月列表
    ///
    /// # 示例
    ///
    /// ```
    /// use tyme4rs::tyme::hijri::{HijriMonth, HijriYear};
    ///
    /// // 1-12月
    /// let months: Vec<HijriMonth> = HijriYear::from_year(2023).get_months();
    /// ```
    pub fn get_months(&self) -> Vec<HijriMonth> {
        let mut l: Vec<HijriMonth> = Vec::new();
        let y: isize = self.get_year();
        for i in 1..13 {
            l.push(HijriMonth::from_ym(y, i));
        }
        l
    }

    /// 首月
    pub fn get_first_month(&self) -> HijriMonth {
        HijriMonth::from_ym(self.get_year(), 1)
    }
}

impl Display for HijriYear {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_name())
    }
}

impl PartialEq for HijriYear {
    fn eq(&self, other: &Self) -> bool {
        self.get_year() == other.get_year()
    }
}

impl Eq for HijriYear {}

/// 回历月名称
pub static HIJRI_MONTH_NAMES: [&str; 12] = [
    "穆哈兰姆月",
    "色法尔月",
    "赖比尔·敖外鲁月",
    "赖比尔·阿色尼月",
    "主马达·敖外鲁月",
    "主马达·阿色尼月",
    "赖哲卜月",
    "舍尔邦月",
    "赖买丹月",
    "闪瓦鲁月",
    "都尔喀尔德月",
    "都尔黑哲月",
];

/// 回历月
#[derive(Debug, Copy, Clone)]
pub struct HijriMonth {
    parent: MonthUnit,
}

impl Tyme for HijriMonth {
    fn next(&self, n: isize) -> Self {
        let i: isize = self.get_month() as isize - 1 + n;
        Self::from_ym((self.get_year() * 12 + i) / 12, self.index_of(i, 12) + 1)
    }
}

impl Culture for HijriMonth {
    fn get_name(&self) -> String {
        HIJRI_MONTH_NAMES[self.get_index_in_year()].to_string()
    }
}

impl Deref for HijriMonth {
    type Target = MonthUnit;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for HijriMonth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl HijriMonth {
    pub fn new(year: isize, month: usize) -> Result<Self, String> {
        Self::validate(year, month)?;
        Ok(Self {
            parent: MonthUnit::new(year, month as isize),
        })
    }

    pub fn validate(year: isize, month: usize) -> Result<(), String> {
        AbstractCulture::validate_range(month as isize, 1, 12, "hijri month")?;
        HijriYear::validate(year)
    }

    pub fn from_ym(year: isize, month: usize) -> Self {
        Self::new(year, month).unwrap()
    }

    /// 回历年
    pub fn get_hijri_year(&self) -> HijriYear {
        HijriYear::new(self.get_year()).unwrap()
    }

    /// 月
    pub fn get_month(&self) -> usize {
        self.parent.get_month() as usize
    }

    /// 当月天数
    pub fn get_day_count(&self) -> usize {
        let mut d: usize;
        let m: usize = self.get_month();
        if m % 2 == 0 {
            d = 29;
        } else {
            d = 30;
        }
        // 闰年第12月30天
        if 12 == m && self.get_hijri_year().is_leap() {
            d += 1
        }
        d
    }

    /// 位于当年的月索引
    pub fn get_index_in_year(&self) -> usize {
        self.get_month() - 1
    }

    /// 回历日列表
    pub fn get_days(&self) -> Vec<HijriDay> {
        let y: isize = self.get_year();
        let mut l: Vec<HijriDay> = Vec::new();
        for i in 1..self.get_day_count() + 1 {
            l.push(HijriDay::from_ymd(y, self.get_month(), i));
        }
        l
    }

    /// 本月第1天
    pub fn get_first_day(&self) -> HijriDay {
        HijriDay::from_ymd(self.get_year(), self.get_month(), 1)
    }
}

impl Display for HijriMonth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.get_hijri_year(), self.get_name())
    }
}

impl PartialEq for HijriMonth {
    fn eq(&self, other: &Self) -> bool {
        self.get_year() == other.get_year() && self.get_month() == other.get_month()
    }
}

impl Eq for HijriMonth {}

/// 回历日名称
pub static HIJRI_DAY_NAMES: [&str; 30] = [
    "1日", "2日", "3日", "4日", "5日", "6日", "7日", "8日", "9日", "10日", "11日", "12日", "13日",
    "14日", "15日", "16日", "17日", "18日", "19日", "20日", "21日", "22日", "23日", "24日", "25日",
    "26日", "27日", "28日", "29日", "30日",
];

/// 回历日
#[derive(Debug, Copy, Clone)]
pub struct HijriDay {
    parent: DayUnit,
}

impl Tyme for HijriDay {
    fn next(&self, n: isize) -> Self {
        self.get_solar_day().next(n).get_hijri_day()
    }
}

impl Culture for HijriDay {
    fn get_name(&self) -> String {
        HIJRI_DAY_NAMES[self.get_day() - 1].to_string()
    }
}

impl Deref for HijriDay {
    type Target = DayUnit;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for HijriDay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl HijriDay {
    pub fn new(year: isize, month: usize, day: usize) -> Result<Self, String> {
        Self::validate(year, month, day)?;
        Ok(Self {
            parent: DayUnit::new(year, month as isize, day as isize),
        })
    }

    pub fn validate(year: isize, month: usize, day: usize) -> Result<(), String> {
        if day < 1 || day > HijriMonth::from_ym(year, month).get_day_count() {
            Err(format!("illegal hijri day: {}-{}-{}", year, month, day))
        } else {
            Ok(())
        }
    }

    pub fn from_ymd(year: isize, month: usize, day: usize) -> Self {
        Self::new(year, month, day).unwrap()
    }

    /// 回历月
    pub fn get_hijri_month(&self) -> HijriMonth {
        HijriMonth::new(self.get_year(), self.get_month()).unwrap()
    }

    /// 月
    pub fn get_month(&self) -> usize {
        self.parent.get_month() as usize
    }

    /// 日
    pub fn get_day(&self) -> usize {
        self.parent.get_day() as usize
    }

    /// 儒略日
    pub fn get_julian_day(&self) -> JulianDay {
        let y: isize = self.get_year();
        let m: isize = self.get_month() as isize;
        JulianDay::from_julian_day(
            self.floor_div(11 * y + 3, 30) as f64 + 354.0 * y as f64 + 30.0 * m as f64
                - self.floor_div(m - 1, 2) as f64
                + self.get_day() as f64
                + 1948055.0,
        )
    }

    pub fn is_before(&self, target: HijriDay) -> bool {
        self.get_compare_index() < target.get_compare_index()
    }

    pub fn is_after(&self, target: HijriDay) -> bool {
        self.get_compare_index() > target.get_compare_index()
    }

    /// 位于当年的索引
    pub fn get_index_in_year(&self) -> usize {
        self.subtract(Self::from_ymd(self.get_year(), 1, 1)) as usize
    }

    /// 回历日相减
    pub fn subtract(&self, target: HijriDay) -> isize {
        self.get_julian_day().subtract(target.get_julian_day()) as isize
    }

    /// 公历日
    pub fn get_solar_day(&self) -> SolarDay {
        SolarDay::from_ymd(622, 7, 16).next(self.subtract(Self::from_ymd(1, 1, 1)))
    }
}

impl Display for HijriDay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.get_hijri_month(), self.get_name())
    }
}

impl PartialEq for HijriDay {
    fn eq(&self, other: &Self) -> bool {
        self.get_year() == other.get_year()
            && self.get_month() == other.get_month()
            && self.get_day() == other.get_day()
    }
}

impl Eq for HijriDay {}

#[cfg(test)]
mod tests {
    use crate::tyme::hijri::{HijriDay, HijriYear};
    use crate::tyme::solar::SolarDay;

    #[test]
    fn test0() {
        assert_eq!(false, HijriYear::from_year(1).is_leap());
        assert_eq!(true, HijriYear::from_year(2).is_leap());
        assert_eq!(false, HijriYear::from_year(0).is_leap());
        assert_eq!(true, HijriYear::from_year(-1).is_leap());
    }

    #[test]
    fn test1() {
        assert_eq!(
            "1年穆哈兰姆月1日",
            SolarDay::from_ymd(622, 7, 16).get_hijri_day().to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            "1447年都尔喀尔德月26日",
            SolarDay::from_ymd(2026, 5, 13).get_hijri_day().to_string()
        );
        assert_eq!(
            "2026年5月13日",
            HijriDay::from_ymd(1447, 11, 26).get_solar_day().to_string()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            "-538年都尔黑哲月12日",
            SolarDay::from_ymd(100, 7, 8).get_hijri_day().to_string()
        );
        assert_eq!(
            "100年7月8日",
            HijriDay::from_ymd(-538, 12, 12).get_solar_day().to_string()
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            "0年都尔黑哲月29日",
            SolarDay::from_ymd(622, 7, 15).get_hijri_day().to_string()
        );
        assert_eq!(
            "622年7月15日",
            HijriDay::from_ymd(0, 12, 29).get_solar_day().to_string()
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            "-640年主马达·敖外鲁月16日",
            SolarDay::from_ymd(1, 1, 1).get_hijri_day().to_string()
        );
        assert_eq!(
            "1年1月1日",
            HijriDay::from_ymd(-640, 5, 16).get_solar_day().to_string()
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            "9666年赖比尔·阿色尼月2日",
            SolarDay::from_ymd(9999, 12, 31).get_hijri_day().to_string()
        );
        assert_eq!(
            "9999年12月31日",
            HijriDay::from_ymd(9666, 4, 2).get_solar_day().to_string()
        );
    }
}
