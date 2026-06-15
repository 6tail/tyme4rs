use crate::tyme::{AbstractCulture, AbstractTyme};
use std::ops::{Deref, DerefMut};

/// 年
#[derive(Debug, Copy, Clone)]
pub struct YearUnit {
    parent: AbstractTyme,
    /// 年
    year: isize,
}

impl Deref for YearUnit {
    type Target = AbstractTyme;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for YearUnit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl YearUnit {
    pub fn new(year: isize) -> Self {
        Self {
            parent: AbstractTyme::new(),
            year,
        }
    }

    /// 年
    pub fn get_year(&self) -> isize {
        self.year
    }

    /// 用于比较大小的索引
    pub fn get_compare_index(&self) -> isize {
        self.year * 10000
    }
}

/// 月
#[derive(Debug, Copy, Clone)]
pub struct MonthUnit {
    /// 年
    parent: YearUnit,
    /// 月
    month: isize,
}

impl MonthUnit {
    pub fn new(year: isize, month: isize) -> Self {
        Self {
            parent: YearUnit::new(year),
            month,
        }
    }

    /// 月
    pub fn get_month(&self) -> isize {
        self.month
    }

    /// 用于比较大小的索引
    pub fn get_compare_index(&self) -> isize {
        let m: isize;
        if self.month > 0 {
            m = self.month * 2;
        } else {
            m = -self.month * 2 + 1;
        }
        self.parent.get_compare_index() + m * 100
    }
}

impl Deref for MonthUnit {
    type Target = YearUnit;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for MonthUnit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

/// 日
#[derive(Debug, Copy, Clone)]
pub struct DayUnit {
    /// 月
    parent: MonthUnit,
    /// 日
    day: isize,
}

impl DayUnit {
    pub fn new(year: isize, month: isize, day: isize) -> Self {
        Self {
            parent: MonthUnit::new(year, month),
            day,
        }
    }

    /// 日
    pub fn get_day(&self) -> isize {
        self.day
    }

    /// 用于比较大小的索引
    pub fn get_compare_index(&self) -> isize {
        self.parent.get_compare_index() + self.day
    }
}

impl Deref for DayUnit {
    type Target = MonthUnit;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for DayUnit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

/// 秒
#[derive(Debug, Copy, Clone)]
pub struct SecondUnit {
    /// 日
    parent: DayUnit,
    /// 时
    hour: usize,
    /// 分
    minute: usize,
    /// 秒
    second: usize,
}

impl SecondUnit {
    pub fn new(
        year: isize,
        month: isize,
        day: isize,
        hour: usize,
        minute: usize,
        second: usize,
    ) -> Self {
        Self {
            parent: DayUnit::new(year, month, day),
            hour,
            minute,
            second,
        }
    }

    pub fn validate(hour: usize, minute: usize, second: usize) -> Result<(), String> {
        AbstractCulture::validate_range(hour as isize, 0, 23, "hour")?;
        AbstractCulture::validate_range(minute as isize, 0, 59, "minute")?;
        AbstractCulture::validate_range(second as isize, 0, 59, "second")
    }

    /// 时
    pub fn get_hour(&self) -> usize {
        self.hour
    }

    /// 分
    pub fn get_minute(&self) -> usize {
        self.minute
    }

    /// 秒
    pub fn get_second(&self) -> usize {
        self.second
    }

    /// 当天秒数
    pub fn get_seconds_in_day(&self) -> usize {
        self.hour * 3600 + self.minute * 60 + self.second
    }

    /// 用于比较大小的索引
    pub fn get_compare_index(&self) -> isize {
        self.parent.get_compare_index() * 86400 + self.get_seconds_in_day() as isize
    }
}

impl Deref for SecondUnit {
    type Target = DayUnit;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for SecondUnit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

/// 周名称
pub static WEEK_UNIT_NAMES: [&str; 6] =
    ["第一周", "第二周", "第三周", "第四周", "第五周", "第六周"];

/// 周
#[derive(Debug, Copy, Clone)]
pub struct WeekUnit {
    /// 月
    parent: MonthUnit,
    /// 索引，0-6
    index: usize,
    /// 起始星期
    start: usize,
}

impl WeekUnit {
    pub fn new(year: isize, month: isize, index: usize, start: usize) -> Self {
        Self {
            parent: MonthUnit::new(year, month),
            index,
            start,
        }
    }

    pub fn validate(index: usize, start: usize) -> Result<(), String> {
        AbstractCulture::validate_range(index as isize, 0, 5, "week index")?;
        AbstractCulture::validate_range(start as isize, 0, 6, "week start")
    }

    /// 索引，0-6
    pub fn get_index(&self) -> usize {
        self.index
    }

    /// 起始星期
    pub fn get_start(&self) -> usize {
        self.start
    }
}

impl Deref for WeekUnit {
    type Target = MonthUnit;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for WeekUnit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}
