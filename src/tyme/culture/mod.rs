use std::fmt::{Display, Formatter};

use crate::tyme::{Culture, LoopTyme, Tyme};

pub static ANIMAL_NAMES: [&str; 28] = ["蛟", "龙", "貉", "兔", "狐", "虎", "豹", "獬", "牛", "蝠", "鼠", "燕", "猪", "獝", "狼", "狗", "彘", "鸡", "乌", "猴", "猿", "犴", "羊", "獐", "马", "鹿", "蛇", "蚓"];

/// 动物
#[derive(Debug, Clone)]
pub struct Animal {
  parent: LoopTyme,
}

impl Tyme for Animal {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Animal {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Animal {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(ANIMAL_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(ANIMAL_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for Animal {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Animal {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Animal {}

impl Into<LoopTyme> for Animal {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static BEAST_NAMES: [&str; 4] = ["青龙", "玄武", "白虎", "朱雀"];

/// 神兽
#[derive(Debug, Clone)]
pub struct Beast {
  parent: LoopTyme,
}

impl Tyme for Beast {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Beast {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Beast {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(BEAST_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(BEAST_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for Beast {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Beast {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Beast {}

impl Into<LoopTyme> for Beast {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static CONSTELLATION_NAMES: [&str; 12] = ["白羊", "金牛", "双子", "巨蟹", "狮子", "处女", "天秤", "天蝎", "射手", "摩羯", "水瓶", "双鱼"];

/// 星座
#[derive(Debug, Clone)]
pub struct Constellation {
  parent: LoopTyme,
}

impl Tyme for Constellation {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Constellation {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Constellation {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(CONSTELLATION_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(CONSTELLATION_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for Constellation {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Constellation {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Constellation {}

impl Into<LoopTyme> for Constellation {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static DIRECTION_NAMES: [&str; 9] = ["北", "西南", "东", "东南", "中", "西北", "西", "东北", "南"];

/// 方位
#[derive(Debug, Clone)]
pub struct Direction {
  parent: LoopTyme,
}

impl Tyme for Direction {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Direction {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Direction {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(DIRECTION_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(DIRECTION_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for Direction {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Direction {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Direction {}

impl Into<LoopTyme> for Direction {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static DUTY_NAMES: [&str; 12] = ["建", "除", "满", "平", "定", "执", "破", "危", "成", "收", "开", "闭"];

/// 建除十二值神
#[derive(Debug, Clone)]
pub struct Duty {
  parent: LoopTyme,
}

impl Tyme for Duty {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Duty {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Duty {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(DUTY_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(DUTY_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for Duty {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Duty {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Duty {}

impl Into<LoopTyme> for Duty {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static ELEMENT_NAMES: [&str; 5] = ["木", "火", "土", "金", "水"];

/// 五行
#[derive(Debug, Clone)]
pub struct Element {
  parent: LoopTyme,
}

impl Tyme for Element {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Element {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Element {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(ELEMENT_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(ELEMENT_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  /// 我生者（生）
  pub fn get_reinforce(&self) -> Self {
    self.next(1).unwrap()
  }

  /// 我克者（克）
  pub fn get_restrain(&self) -> Self {
    self.next(2).unwrap()
  }

  /// 生我者（泄）
  pub fn get_reinforced(&self) -> Self {
    self.next(-1).unwrap()
  }

  /// 克我者（耗）
  pub fn get_restrained(&self) -> Self {
    self.next(-2).unwrap()
  }
}

impl Display for Element {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Element {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Element {}

impl Into<LoopTyme> for Element {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static LAND_NAMES: [&str; 9] = ["玄天", "朱天", "苍天", "阳天", "钧天", "幽天", "颢天", "变天", "炎天"];

/// 九野
#[derive(Debug, Clone)]
pub struct Land {
  parent: LoopTyme,
}

impl Tyme for Land {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Land {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Land {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(LAND_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(LAND_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  /// 方位
  pub fn get_direction(&self) -> Direction {
    return Direction::from_index(self.get_index() as isize);
  }
}

impl Display for Land {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Land {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Land {}

impl Into<LoopTyme> for Land {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static LUCK_NAMES: [&str; 2] = ["吉", "凶"];

/// 吉凶
#[derive(Debug, Clone)]
pub struct Luck {
  parent: LoopTyme,
}

impl Tyme for Luck {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Luck {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Luck {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(LUCK_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(LUCK_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for Luck {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Luck {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Luck {}

impl Into<LoopTyme> for Luck {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static PHASE_NAMES: [&str; 30] = ["朔月", "既朔月", "蛾眉新月", "蛾眉新月", "蛾眉月", "夕月", "上弦月", "上弦月", "九夜月", "宵月", "宵月", "宵月", "渐盈凸月", "小望月", "望月", "既望月", "立待月", "居待月", "寝待月", "更待月", "渐亏凸月", "下弦月", "下弦月", "有明月", "有明月", "蛾眉残月", "蛾眉残月", "残月", "晓月", "晦月"];

/// 月相
#[derive(Debug, Clone)]
pub struct Phase {
  parent: LoopTyme,
}

impl Tyme for Phase {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Phase {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Phase {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(PHASE_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(PHASE_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for Phase {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Phase {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Phase {}

impl Into<LoopTyme> for Phase {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static SIXTY_NAMES: [&str; 3] = ["上元", "中元", "下元"];

/// 元（60年=1元）
#[derive(Debug, Clone)]
pub struct Sixty {
  parent: LoopTyme,
}

impl Tyme for Sixty {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Sixty {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Sixty {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(SIXTY_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(SIXTY_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for Sixty {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Sixty {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Sixty {}

impl Into<LoopTyme> for Sixty {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static SOUND_NAMES: [&str; 30] = ["海中金", "炉中火", "大林木", "路旁土", "剑锋金", "山头火", "涧下水", "城头土", "白蜡金", "杨柳木", "泉中水", "屋上土", "霹雳火", "松柏木", "长流水", "沙中金", "山下火", "平地木", "壁上土", "金箔金", "覆灯火", "天河水", "大驿土", "钗钏金", "桑柘木", "大溪水", "沙中土", "天上火", "石榴木", "大海水"];

/// 纳音
#[derive(Debug, Clone)]
pub struct Sound {
  parent: LoopTyme,
}

impl Tyme for Sound {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Sound {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Sound {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(SOUND_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(SOUND_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for Sound {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Sound {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Sound {}

impl Into<LoopTyme> for Sound {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static TEN_NAMES: [&str; 6] = ["甲子", "甲戌", "甲申", "甲午", "甲辰", "甲寅"];

/// 旬
#[derive(Debug, Clone)]
pub struct Ten {
  parent: LoopTyme,
}

impl Tyme for Ten {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Ten {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Ten {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(TEN_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(TEN_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for Ten {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Ten {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Ten {}

impl Into<LoopTyme> for Ten {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static TERRAIN_NAMES: [&str; 12] = ["长生", "沐浴", "冠带", "临官", "帝旺", "衰", "病", "死", "墓", "绝", "胎", "养"];

/// 地势(长生十二神)
#[derive(Debug, Clone)]
pub struct Terrain {
  parent: LoopTyme,
}

impl Tyme for Terrain {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Terrain {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Terrain {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(TERRAIN_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(TERRAIN_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for Terrain {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Terrain {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Terrain {}

impl Into<LoopTyme> for Terrain {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static TWENTY_NAMES: [&str; 9] = ["一运", "二运", "三运", "四运", "五运", "六运", "七运", "八运", "九运"];

/// 运（20年=1运，3运=1元）
#[derive(Debug, Clone)]
pub struct Twenty {
  parent: LoopTyme,
}

impl Tyme for Twenty {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Twenty {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Twenty {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(TWENTY_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(TWENTY_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  pub fn get_sixty(&self) -> Sixty {
    Sixty::from_index((self.get_index() / 3) as isize)
  }
}

impl Display for Twenty {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Twenty {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Twenty {}

impl Into<LoopTyme> for Twenty {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static WEEK_NAMES: [&str; 7] = ["日", "一", "二", "三", "四", "五", "六"];

/// 星期
#[derive(Debug, Clone)]
pub struct Week {
  parent: LoopTyme,
}

impl Tyme for Week {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Week {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Week {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(WEEK_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(WEEK_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for Week {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Week {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Week {}

impl Into<LoopTyme> for Week {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static ZODIAC_NAMES: [&str; 12] = ["鼠", "牛", "虎", "兔", "龙", "蛇", "马", "羊", "猴", "鸡", "狗", "猪"];

/// 生肖
#[derive(Debug, Clone)]
pub struct Zodiac {
  parent: LoopTyme,
}

impl Tyme for Zodiac {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Zodiac {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Zodiac {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(ZODIAC_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(ZODIAC_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for Zodiac {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Zodiac {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Zodiac {}

impl Into<LoopTyme> for Zodiac {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static ZONE_NAMES: [&str; 4] = ["东", "北", "西", "南"];

/// 宫
#[derive(Debug, Clone)]
pub struct Zone {
  parent: LoopTyme,
}

impl Tyme for Zone {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Zone {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Zone {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(ZONE_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(ZONE_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  pub fn get_direction(&self) -> Direction {
    Direction::from_name(self.get_name().as_str()).unwrap()
  }

  pub fn get_beast(&self) -> Beast {
    Beast::from_index(self.get_index() as isize)
  }
}

impl Display for Zone {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Zone {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Zone {}

impl Into<LoopTyme> for Zone {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub mod dog;
pub mod nine;
pub mod fetus;
pub mod peng_zu;
pub mod phenology;
pub mod star;
pub mod eightchar;
pub mod plumrain;

#[cfg(test)]
mod tests {
  use crate::tyme::Culture;
  use crate::tyme::culture::{Animal, Beast, Constellation, Direction, Duty, Element, Land, Luck, Phase};
  use crate::tyme::culture::dog::DogDay;
  use crate::tyme::sixtycycle::{EarthBranch, HeavenStem};
  use crate::tyme::solar::SolarDay;

  #[test]
  fn test1() {
    assert_eq!("龙", Animal::from_name("龙").unwrap().get_name());
  }

  #[test]
  fn test2() {
    assert_eq!("龙", Animal::from_index(1).get_name());
    assert_eq!("龙", Animal::from_index(1).to_string());
    assert_eq!(1, Animal::from_index(1).get_index());
    assert_eq!(28, Animal::from_index(1).get_size());
  }

  #[test]
  fn test3() {
    assert_eq!("青龙", Beast::from_index(0).get_name());
  }

  #[test]
  fn test4() {
    assert_eq!("白羊", Constellation::from_index(0).get_name());
  }

  #[test]
  fn test5() {
    assert_eq!("北", Direction::from_index(0).get_name());
  }

  #[test]
  fn test6() {
    assert_eq!("建", Duty::from_index(0).get_name());
  }

  #[test]
  fn test7() {
    assert_eq!("木", Element::from_index(0).get_name());
  }

  #[test]
  fn test8() {
    assert_eq!(Element::from_name("木").unwrap(), Element::from_name("金").unwrap().get_restrain());
  }

  #[test]
  fn test9() {
    assert_eq!(Element::from_name("火").unwrap(), Element::from_name("土").unwrap().get_reinforced());
  }

  #[test]
  fn test10() {
    assert_eq!("玄天", Land::from_index(0).get_name());
  }

  #[test]
  fn test11() {
    assert_eq!("吉", Luck::from_index(0).get_name());
  }

  #[test]
  fn test12() {
    assert_eq!("朔月", Phase::from_index(0).get_name());
  }

  #[test]
  fn test13() {
    assert_eq!("白羊", SolarDay::from_ymd(2020, 3, 21).unwrap().get_constellation().get_name());
    assert_eq!("白羊", SolarDay::from_ymd(2020, 4, 19).unwrap().get_constellation().get_name());
  }

  #[test]
  fn test14() {
    assert_eq!("金牛", SolarDay::from_ymd(2020, 4, 20).unwrap().get_constellation().get_name());
    assert_eq!("金牛", SolarDay::from_ymd(2020, 5, 20).unwrap().get_constellation().get_name());
  }

  #[test]
  fn test15() {
    assert_eq!("双子", SolarDay::from_ymd(2020, 5, 21).unwrap().get_constellation().get_name());
    assert_eq!("双子", SolarDay::from_ymd(2020, 6, 21).unwrap().get_constellation().get_name());
  }

  #[test]
  fn test16() {
    assert_eq!("巨蟹", SolarDay::from_ymd(2020, 6, 22).unwrap().get_constellation().get_name());
    assert_eq!("巨蟹", SolarDay::from_ymd(2020, 7, 22).unwrap().get_constellation().get_name());
  }

  #[test]
  fn test17() {
    assert_eq!("东南", SolarDay::from_ymd(2021, 11, 13).unwrap().get_lunar_day().get_sixty_cycle().get_heaven_stem().get_mascot_direction().get_name());
  }

  #[test]
  fn test18() {
    assert_eq!("东南", SolarDay::from_ymd(2024, 1, 1).unwrap().get_lunar_day().get_sixty_cycle().get_heaven_stem().get_mascot_direction().get_name());
  }

  #[test]
  fn test19() {
    assert_eq!("东", SolarDay::from_ymd(2023, 11, 6).unwrap().get_lunar_day().get_jupiter_direction().get_name());
  }

  #[test]
  fn test20() {
    let d: DogDay = SolarDay::from_ymd(2011, 7, 14).unwrap().get_dog_day().unwrap();
    assert_eq!("初伏", d.get_name());
    assert_eq!("初伏", d.get_dog().to_string());
    assert_eq!("初伏第1天", d.to_string());
  }

  #[test]
  fn test21() {
    let d: DogDay = SolarDay::from_ymd(2011, 7, 23).unwrap().get_dog_day().unwrap();
    assert_eq!("初伏", d.get_name());
    assert_eq!("初伏", d.get_dog().to_string());
    assert_eq!("初伏第10天", d.to_string());
  }

  #[test]
  fn test22() {
    let d: DogDay = SolarDay::from_ymd(2011, 7, 24).unwrap().get_dog_day().unwrap();
    assert_eq!("中伏", d.get_name());
    assert_eq!("中伏", d.get_dog().to_string());
    assert_eq!("中伏第1天", d.to_string());
  }

  #[test]
  fn test23() {
    let d: DogDay = SolarDay::from_ymd(2011, 8, 12).unwrap().get_dog_day().unwrap();
    assert_eq!("中伏", d.get_name());
    assert_eq!("中伏", d.get_dog().to_string());
    assert_eq!("中伏第20天", d.to_string());
  }

  #[test]
  fn test24() {
    let d: DogDay = SolarDay::from_ymd(2011, 8, 13).unwrap().get_dog_day().unwrap();
    assert_eq!("末伏", d.get_name());
    assert_eq!("末伏", d.get_dog().to_string());
    assert_eq!("末伏第1天", d.to_string());
  }

  #[test]
  fn test25() {
    let d: DogDay = SolarDay::from_ymd(2011, 8, 22).unwrap().get_dog_day().unwrap();
    assert_eq!("末伏", d.get_name());
    assert_eq!("末伏", d.get_dog().to_string());
    assert_eq!("末伏第10天", d.to_string());
  }

  #[test]
  fn test26() {
    let d: Option<DogDay> = SolarDay::from_ymd(2011, 7, 13).unwrap().get_dog_day();
    assert_eq!(true, d.is_none());
  }

  #[test]
  fn test27() {
    let d: Option<DogDay> = SolarDay::from_ymd(2011, 8, 23).unwrap().get_dog_day();
    assert_eq!(true, d.is_none());
  }

  #[test]
  fn test28() {
    let d: DogDay = SolarDay::from_ymd(2012, 7, 18).unwrap().get_dog_day().unwrap();
    assert_eq!("初伏", d.get_name());
    assert_eq!("初伏", d.get_dog().to_string());
    assert_eq!("初伏第1天", d.to_string());
  }

  #[test]
  fn test29() {
    let d: DogDay = SolarDay::from_ymd(2012, 8, 5).unwrap().get_dog_day().unwrap();
    assert_eq!("中伏", d.get_name());
    assert_eq!("中伏", d.get_dog().to_string());
    assert_eq!("中伏第9天", d.to_string());
  }

  #[test]
  fn test30() {
    let d: DogDay = SolarDay::from_ymd(2012, 8, 8).unwrap().get_dog_day().unwrap();
    assert_eq!("末伏", d.get_name());
    assert_eq!("末伏", d.get_dog().to_string());
    assert_eq!("末伏第2天", d.to_string());
  }

  #[test]
  fn test31() {
    assert_eq!("闭", SolarDay::from_ymd(2023, 10, 30).unwrap().get_lunar_day().get_duty().get_name());
  }

  #[test]
  fn test32() {
    assert_eq!("建", SolarDay::from_ymd(2023, 10, 19).unwrap().get_lunar_day().get_duty().get_name());
  }

  #[test]
  fn test33() {
    assert_eq!("除", SolarDay::from_ymd(2023, 10, 7).unwrap().get_lunar_day().get_duty().get_name());
  }

  #[test]
  fn test34() {
    assert_eq!("除", SolarDay::from_ymd(2023, 10, 8).unwrap().get_lunar_day().get_duty().get_name());
  }

  #[test]
  fn test35() {
    assert_eq!(Element::from_name("土").unwrap(), Element::from_name("火").unwrap().get_reinforce());
  }

  #[test]
  fn test36() {
    assert_eq!("火", HeavenStem::from_name("丙").unwrap().get_element().get_name());
  }

  #[test]
  fn test37() {
    assert_eq!("木", EarthBranch::from_name("寅").unwrap().get_element().get_name());
  }

  #[test]
  fn test38() {
    assert_eq!(Element::from_name("火").unwrap(), EarthBranch::from_name("寅").unwrap().get_element().get_reinforce());
  }
}
