use crate::tyme::AbstractTyme;
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
        if hour > 23 {
            Err(format!("illegal hour: {}", hour))
        } else if minute > 59 {
            Err(format!("illegal minute: {}", minute))
        } else if second > 59 {
            Err(format!("illegal second: {}", second))
        } else {
            Ok(())
        }
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
        if index > 5 {
            Err(format!("illegal week index: {}", index))
        } else if start > 6 {
            Err(format!("illegal week start: {}", start))
        } else {
            Ok(())
        }
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
