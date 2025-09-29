use std::fmt::{Display, Formatter};

use crate::tyme::{AbstractCulture, AbstractCultureDay, Culture, LoopTyme, Tyme};
use crate::tyme::culture::{Direction, Duty, Element, God, Sound, Taboo, Ten, Terrain, Twenty, Zodiac};
use crate::tyme::eightchar::EightChar;
use crate::tyme::culture::fetus::FetusDay;
use crate::tyme::culture::star::nine::NineStar;
use crate::tyme::culture::star::ten::TenStar;
use crate::tyme::culture::star::twelve::TwelveStar;
use crate::tyme::culture::star::twenty_eight::TwentyEightStar;
use crate::tyme::enums::{HideHeavenStemType, YinYang};
use crate::tyme::lunar::{LunarDay, LunarHour, LunarMonth, LunarYear};
use crate::tyme::solar::{SolarDay, SolarTerm, SolarTime};

pub static HEAVEN_STEM_NAMES: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];

/// 天干（天元）
#[derive(Debug, Clone)]
pub struct HeavenStem {
  parent: LoopTyme,
}

impl Tyme for HeavenStem {
  fn next(&self, n: isize) -> Self {
    Self::from_index(self.parent.next_index(n) as isize)
  }
}

impl Culture for HeavenStem {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl HeavenStem {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(HEAVEN_STEM_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Self {
    Self {
      parent: LoopTyme::from_name(HEAVEN_STEM_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name)
    }
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  /// 五行
  pub fn get_element(&self) -> Element {
    Element::from_index((self.get_index() / 2) as isize)
  }

  /// 阴阳
  pub fn get_yin_yang(&self) -> YinYang {
    match self.get_index() % 2 {
      0 => YinYang::YANG,
      _ => YinYang::YIN
    }
  }

  /// 方位
  pub fn get_direction(&self) -> Direction {
    self.get_element().get_direction()
  }

  /// 喜神方位（《喜神方位歌》甲己在艮乙庚乾，丙辛坤位喜神安。丁壬只在离宫坐，戊癸原在在巽间。）
  pub fn get_joy_direction(&self) -> Direction {
    Direction::from_index([7, 5, 1, 8, 3][self.get_index() % 5])
  }

  /// 阳贵神方位（《阳贵神歌》甲戊坤艮位，乙己是坤坎，庚辛居离艮，丙丁兑与乾，震巽属何日，壬癸贵神安。）
  pub fn get_yang_direction(&self) -> Direction {
    Direction::from_index([1, 1, 6, 5, 7, 0, 8, 7, 2, 3][self.get_index()])
  }

  /// 阴贵神方位（《阴贵神歌》甲戊见牛羊，乙己鼠猴乡，丙丁猪鸡位，壬癸蛇兔藏，庚辛逢虎马，此是贵神方。）
  pub fn get_yin_direction(&self) -> Direction {
    Direction::from_index([7, 0, 5, 6, 1, 1, 7, 8, 3, 2][self.get_index()])
  }

  /// 财神方位（《财神方位歌》甲乙东北是财神，丙丁向在西南寻，戊己正北坐方位，庚辛正东去安身，壬癸原来正南坐，便是财神方位真。）
  pub fn get_wealth_direction(&self) -> Direction {
    Direction::from_index([7, 1, 0, 2, 8][self.get_index() / 2])
  }

  /// 福神方位（《福神方位歌》甲乙东南是福神，丙丁正东是堪宜，戊北己南庚辛坤，壬在乾方癸在西。）
  pub fn get_mascot_direction(&self) -> Direction {
    Direction::from_index([3, 3, 2, 2, 0, 8, 1, 1, 5, 6][self.get_index()])
  }

  /// 地势(长生十二神)
  pub fn get_terrain(&self, earth_branch: EarthBranch) -> Terrain {
    let mut earth_branch_index: isize = earth_branch.get_index() as isize;
    if self.get_yin_yang() == YinYang::YIN {
      earth_branch_index *= -1;
    }
    Terrain::from_index([1, 6, 10, 9, 10, 9, 7, 0, 4, 3][self.get_index()] + earth_branch_index)
  }

  /// 十神（生我者，正印偏印。我生者，伤官食神。克我者，正官七杀。我克者，正财偏财。同我者，劫财比肩。）
  pub fn get_ten_star(&self, target: Self) -> TenStar {
    let target_index: isize = target.get_index() as isize;
    let index: isize = self.get_index() as isize;
    let mut offset: isize = target_index - index;
    if index % 2 != 0 && target_index % 2 == 0 {
      offset += 2;
    }
    TenStar::from_index(offset)
  }

  /// 五合（甲己合，乙庚合，丙辛合，丁壬合，戊癸合）
  pub fn get_combine(&self) -> Self {
    self.next(5)
  }

  /// 合化（甲己合化土，乙庚合化金，丙辛合化水，丁壬合化木，戊癸合化火）
  ///
  /// 如果无法合化，返回None
  pub fn combine(&self, target: Self) -> Option<Element> {
    if self.get_combine() == target {
      Some(Element::from_index(self.get_index() as isize + 2))
    } else {
      None
    }
  }
}

impl Display for HeavenStem {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for HeavenStem {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for HeavenStem {}

impl Into<LoopTyme> for HeavenStem {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static EARTH_BRANCH_NAMES: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

/// 地支（地元）
#[derive(Debug, Clone)]
pub struct EarthBranch {
  parent: LoopTyme,
}

impl Tyme for EarthBranch {
  fn next(&self, n: isize) -> Self {
    Self::from_index(self.parent.next_index(n) as isize)
  }
}

impl Culture for EarthBranch {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl EarthBranch {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(EARTH_BRANCH_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Self {
    Self {
      parent: LoopTyme::from_name(EARTH_BRANCH_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name)
    }
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  /// 五行
  pub fn get_element(&self) -> Element {
    Element::from_index([4, 2, 0, 0, 2, 1, 1, 2, 3, 3, 2, 4][self.get_index()])
  }

  /// 阴阳
  pub fn get_yin_yang(&self) -> YinYang {
    match self.get_index() % 2 {
      0 => YinYang::YANG,
      _ => YinYang::YIN
    }
  }

  /// 藏干之本气
  pub fn get_hide_heaven_stem_main(&self) -> HeavenStem {
    HeavenStem::from_index([9, 5, 0, 1, 4, 2, 3, 5, 6, 7, 4, 8][self.get_index()])
  }

  /// 藏干之中气，无中气返回None
  pub fn get_hide_heaven_stem_middle(&self) -> Option<HeavenStem> {
    let n: isize = [-1, 9, 2, -1, 1, 6, 5, 3, 8, -1, 7, 0][self.get_index()];
    if n == -1 {
      None
    } else {
      Some(HeavenStem::from_index(n))
    }
  }

  /// 藏干之余气，无余气返回None
  pub fn get_hide_heaven_stem_residual(&self) -> Option<HeavenStem> {
    let n: isize = [-1, 7, 4, -1, 9, 4, -1, 1, 4, -1, 3, -1][self.get_index()];
    if n == -1 {
      None
    } else {
      Some(HeavenStem::from_index(n))
    }
  }

  /// 藏干列表
  pub fn get_hide_heaven_stems(&self) -> Vec<HideHeavenStem> {
    let mut l: Vec<HideHeavenStem> = Vec::new();
    l.push(HideHeavenStem::new(self.get_hide_heaven_stem_main(), HideHeavenStemType::MAIN));
    match self.get_hide_heaven_stem_middle() {
      Some(x) => l.push(HideHeavenStem::new(x, HideHeavenStemType::MIDDLE)),
      None => {}
    }
    match self.get_hide_heaven_stem_residual() {
      Some(x) => l.push(HideHeavenStem::new(x, HideHeavenStemType::RESIDUAL)),
      None => {}
    }
    l
  }

  /// 生肖
  pub fn get_zodiac(&self) -> Zodiac {
    Zodiac::from_index(self.get_index() as isize)
  }

  /// 方位
  pub fn get_direction(&self) -> Direction {
    Direction::from_index([0, 4, 2, 2, 4, 8, 8, 4, 6, 6, 4, 0][self.get_index()])
  }

  /// 六冲（子午冲，丑未冲，寅申冲，辰戌冲，卯酉冲，巳亥冲）
  pub fn get_opposite(&self) -> Self {
    self.next(6)
  }

  /// 煞（逢巳日、酉日、丑日必煞东；亥日、卯日、未日必煞西；申日、子日、辰日必煞南；寅日、午日、戌日必煞北。）
  pub fn get_ominous(&self) -> Direction {
    Direction::from_index([8, 2, 0, 6][self.get_index() % 4])
  }

  /// 六合（子丑合，寅亥合，卯戌合，辰酉合，巳申合，午未合）
  pub fn get_combine(&self) -> Self {
    Self::from_index(1 - self.get_index() as isize)
  }

  /// 合化（子丑合化土，寅亥合化木，卯戌合化火，辰酉合化金，巳申合化水，午未合化土）
  ///
  /// 如果无法合化，返回None
  pub fn combine(&self, target: Self) -> Option<Element> {
    if self.get_combine() == target {
      Some(Element::from_index([2, 2, 0, 1, 3, 4, 2, 2, 4, 3, 1, 0][self.get_index()]))
    } else {
      None
    }
  }

  /// 六害（子未害、丑午害、寅巳害、卯辰害、申亥害、酉戌害）
  pub fn get_harm(&self) -> Self {
    Self::from_index(19 - self.get_index() as isize)
  }
}

impl Display for EarthBranch {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for EarthBranch {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for EarthBranch {}

impl Into<LoopTyme> for EarthBranch {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

pub static SIXTY_CYCLE_NAMES: [&str; 60] = ["甲子", "乙丑", "丙寅", "丁卯", "戊辰", "己巳", "庚午", "辛未", "壬申", "癸酉", "甲戌", "乙亥", "丙子", "丁丑", "戊寅", "己卯", "庚辰", "辛巳", "壬午", "癸未", "甲申", "乙酉", "丙戌", "丁亥", "戊子", "己丑", "庚寅", "辛卯", "壬辰", "癸巳", "甲午", "乙未", "丙申", "丁酉", "戊戌", "己亥", "庚子", "辛丑", "壬寅", "癸卯", "甲辰", "乙巳", "丙午", "丁未", "戊申", "己酉", "庚戌", "辛亥", "壬子", "癸丑", "甲寅", "乙卯", "丙辰", "丁巳", "戊午", "己未", "庚申", "辛酉", "壬戌", "癸亥"];

/// 六十甲子(六十干支周)
#[derive(Debug, Clone)]
pub struct SixtyCycle {
  parent: LoopTyme,
}

impl Tyme for SixtyCycle {
  fn next(&self, n: isize) -> Self {
    Self::from_index(self.parent.next_index(n) as isize)
  }
}

impl Culture for SixtyCycle {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl SixtyCycle {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(SIXTY_CYCLE_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Self {
    Self {
      parent: LoopTyme::from_name(SIXTY_CYCLE_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name)
    }
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }

  pub fn get_heaven_stem(&self) -> HeavenStem {
    HeavenStem::from_index((self.get_index() % HEAVEN_STEM_NAMES.len()) as isize)
  }

  pub fn get_earth_branch(&self) -> EarthBranch {
    EarthBranch::from_index((self.get_index() % EARTH_BRANCH_NAMES.len()) as isize)
  }

  pub fn get_sound(&self) -> Sound {
    Sound::from_index((self.get_index() / 2) as isize)
  }

  pub fn get_ten(&self) -> Ten {
    Ten::from_index((self.get_heaven_stem().get_index() as isize - self.get_earth_branch().get_index() as isize) / 2)
  }

  pub fn get_extra_earth_branches(&self) -> Vec<EarthBranch> {
    let mut l: Vec<EarthBranch> = Vec::new();
    let earth_branch: EarthBranch = EarthBranch::from_index(10 + (self.get_earth_branch().get_index() as isize) - (self.get_heaven_stem().get_index() as isize));
    let next_earth_branch: EarthBranch = earth_branch.next(1);
    l.push(earth_branch);
    l.push(next_earth_branch);
    l
  }
}

impl Display for SixtyCycle {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for SixtyCycle {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for SixtyCycle {}

impl Into<LoopTyme> for SixtyCycle {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

/// 藏干（即人元，司令取天干，分野取天干的五行）
#[derive(Debug, Clone)]
pub struct HideHeavenStem {
  parent: AbstractCulture,
  heaven_stem: HeavenStem,
  hide_heaven_stem_type: HideHeavenStemType,
}

impl Culture for HideHeavenStem {
  fn get_name(&self) -> String {
    self.heaven_stem.get_name()
  }
}

impl HideHeavenStem {
  pub fn new(heaven_stem: HeavenStem, hide_heaven_stem_type: HideHeavenStemType) -> Self {
    Self {
      parent: AbstractCulture::new(),
      heaven_stem: heaven_stem,
      hide_heaven_stem_type: hide_heaven_stem_type,
    }
  }

  pub fn from_index(heaven_stem_index: isize, hide_heaven_stem_type: HideHeavenStemType) -> Self {
    Self {
      parent: AbstractCulture::new(),
      heaven_stem: HeavenStem::from_index(heaven_stem_index),
      hide_heaven_stem_type: hide_heaven_stem_type,
    }
  }

  pub fn from_name(heaven_stem_name: &str, hide_heaven_stem_type: HideHeavenStemType) -> Self {
    Self {
      parent: AbstractCulture::new(),
      heaven_stem: HeavenStem::from_name(heaven_stem_name),
      hide_heaven_stem_type: hide_heaven_stem_type,
    }
  }

  pub fn get_heaven_stem(&self) -> HeavenStem {
    self.heaven_stem.clone()
  }

  pub fn get_type(&self) -> HideHeavenStemType {
    self.hide_heaven_stem_type
  }
}

impl Display for HideHeavenStem {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for HideHeavenStem {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for HideHeavenStem {}

impl Into<AbstractCulture> for HideHeavenStem {
  fn into(self) -> AbstractCulture {
    self.parent
  }
}

/// 人元司令分野（地支藏干+天索引）
#[derive(Debug, Clone)]
pub struct HideHeavenStemDay {
  parent: AbstractCultureDay,
  hide_heaven_stem: HideHeavenStem
}

impl Culture for HideHeavenStemDay {
  fn get_name(&self) -> String {
    let heaven_stem: HeavenStem = self.get_hide_heaven_stem().get_heaven_stem();
    format!("{}{}", heaven_stem.get_name(), heaven_stem.get_element().get_name())
  }
}

impl HideHeavenStemDay {
  pub fn new(hide_heaven_stem: HideHeavenStem, day_index: usize) -> Self {
    let h1: HideHeavenStem = hide_heaven_stem.clone();
    let h2: HideHeavenStem = hide_heaven_stem.clone();
    let culture: AbstractCulture = h1.into();
    Self {
      parent: AbstractCultureDay::new(culture, day_index),
      hide_heaven_stem: h2
    }
  }

  pub fn get_hide_heaven_stem(&self) -> HideHeavenStem {
    self.hide_heaven_stem.clone()
  }

  pub fn get_day_index(&self) -> usize {
    self.parent.get_day_index()
  }
}

impl Display for HideHeavenStemDay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}第{}天", self.get_name(), self.parent.get_day_index() + 1)
  }
}

impl PartialEq for HideHeavenStemDay {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for HideHeavenStemDay {}

impl Into<AbstractCultureDay> for HideHeavenStemDay {
  fn into(self) -> AbstractCultureDay {
    self.parent
  }
}

/// 干支年
#[derive(Debug, Copy, Clone)]
pub struct SixtyCycleYear {
  /// 年
  year: isize,
}

impl Tyme for SixtyCycleYear {
  fn next(&self, n: isize) -> Self {
    Self::from_year(self.year + n)
  }
}

impl Culture for SixtyCycleYear {
  fn get_name(&self) -> String {
    format!("{}年", self.get_sixty_cycle())
  }
}

impl SixtyCycleYear {
  pub fn new(year: isize) -> Result<Self, String> {
    if year < -1 || year > 9999 {
      Err(format!("illegal sixty cycle year: {}", year))
    } else {
      Ok(Self {
        year
      })
    }
  }

  pub fn from_year(year: isize) -> Self {
    Self::new(year).unwrap()
  }

  pub fn get_year(&self) -> isize {
    self.year
  }

  pub fn get_first_month(&self) -> SixtyCycleMonth {
    let h: HeavenStem = HeavenStem::from_index((self.get_sixty_cycle().get_heaven_stem().get_index() as isize + 1) * 2);
    SixtyCycleMonth {
      year: *self,
      month: SixtyCycle::from_name(format!("{}寅", h.get_name()).as_str())
    }
  }

  pub fn get_months(&self) -> Vec<SixtyCycleMonth> {
    let mut l: Vec<SixtyCycleMonth> = Vec::new();
    let m: SixtyCycleMonth = self.get_first_month();
    l.push(m.clone());
    for i in 1..12 {
      l.push(m.next(i));
    }
    l
  }

  pub fn get_sixty_cycle(&self) -> SixtyCycle {
    SixtyCycle::from_index(self.year - 4)
  }

  pub fn get_twenty(&self) -> Twenty {
    Twenty::from_index(((self.year as f64 - 1864.0) / 20.0).floor() as isize)
  }

  pub fn get_jupiter_direction(&self) -> Direction {
    Direction::from_index([0, 7, 7, 2, 3, 3, 8, 1, 1, 6, 0, 0][self.get_sixty_cycle().get_earth_branch().get_index()])
  }

  pub fn get_nine_star(&self) -> NineStar {
    NineStar::from_index(63 + self.get_twenty().get_sixty().get_index() as isize * 3 - self.get_sixty_cycle().get_index() as isize)
  }
}

impl Display for SixtyCycleYear {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for SixtyCycleYear {
  fn eq(&self, other: &Self) -> bool {
    self.get_year() == other.get_year()
  }
}

impl Eq for SixtyCycleYear {}

/// 干支月
#[derive(Debug, Clone)]
pub struct SixtyCycleMonth {
  /// 干支年
  year: SixtyCycleYear,
  /// 月柱
  month: SixtyCycle,
}

impl Tyme for SixtyCycleMonth {
  fn next(&self, n: isize) -> Self {
    SixtyCycleMonth {
      year: SixtyCycleYear::from_year((self.year.get_year() * 12 + self.get_index_in_year() as isize + n) / 12),
      month: self.month.next(n),
    }
  }
}

impl Culture for SixtyCycleMonth {
  fn get_name(&self) -> String {
    format!("{}月", self.month)
  }
}

impl SixtyCycleMonth {
  pub fn from_index(year: isize, index: isize) -> Self {
    SixtyCycleYear::from_year(year).get_first_month().next(index)
  }

  pub fn get_sixty_cycle_year(&self) -> SixtyCycleYear {
    self.year
  }

  pub fn get_year(&self) -> SixtyCycle {
    self.year.get_sixty_cycle()
  }

  pub fn get_sixty_cycle(&self) -> SixtyCycle {
    self.month.clone()
  }

  pub fn get_index_in_year(&self) -> usize {
    self.month.get_earth_branch().next(-2).get_index()
  }

  pub fn get_first_day(&self) -> SixtyCycleDay {
    SixtyCycleDay::from_solar_day(SolarTerm::from_index(self.year.get_year(), 3 + self.get_index_in_year() as isize * 2).get_julian_day().get_solar_day())
  }

  pub fn get_days(&self) -> Vec<SixtyCycleDay> {
    let mut l: Vec<SixtyCycleDay> = Vec::new();
    let mut d: SixtyCycleDay = self.get_first_day();
    while d.get_sixty_cycle_month() == *self {
      l.push(d.clone());
      d = d.next(1);
    }
    l
  }

  pub fn get_jupiter_direction(&self) -> Direction {
    let n: isize = [7, -1, 1, 3][self.month.get_earth_branch().next(-2).get_index() % 4];
    match n {
      -1 => self.month.get_heaven_stem().get_direction(),
      _ => Direction::from_index(n)
    }
  }

  /// 九星
  pub fn get_nine_star(&self) -> NineStar {
    let mut index: isize = self.month.get_earth_branch().get_index() as isize;
    if index < 2 {
      index += 3;
    }
    NineStar::from_index(27 - self.get_year().get_earth_branch().get_index() as isize % 3 * 3 - index)
  }
}

impl Display for SixtyCycleMonth {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.year.to_string(), self.get_name())
  }
}

impl PartialEq for SixtyCycleMonth {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for SixtyCycleMonth {}

/// 干支日
#[derive(Debug, Clone)]
pub struct SixtyCycleDay {
  /// 公历日
  solar_day: SolarDay,
  /// 公历月
  month: SixtyCycleMonth,
  /// 日柱
  day: SixtyCycle
}

impl Tyme for SixtyCycleDay {
  fn next(&self, n: isize) -> Self {
    SixtyCycleDay::from_solar_day(self.solar_day.next(n))
  }
}

impl Culture for SixtyCycleDay {
  fn get_name(&self) -> String {
    format!("{}日", self.day)
  }
}

impl SixtyCycleDay {
  pub fn from_solar_day(solar_day: SolarDay) -> Self {
    let solar_year: isize = solar_day.get_year();
    let spring_solar_day: SolarDay = SolarTerm::from_index(solar_year, 3).get_julian_day().get_solar_day();
    let lunar_day: LunarDay = solar_day.get_lunar_day();
    let mut lunar_year: LunarYear = lunar_day.get_lunar_month().get_lunar_year();
    if lunar_year.get_year() == solar_year {
      if solar_day.is_before(spring_solar_day) {
        lunar_year = lunar_year.next(-1)
      }
    } else if lunar_year.get_year() < solar_year {
      if !solar_day.is_before(spring_solar_day) {
        lunar_year = lunar_year.next(1);
      }
    }
    let term: SolarTerm = solar_day.get_term();
    let mut index: isize = term.get_index() as isize - 3;
    if index < 0 && term.get_julian_day().get_solar_day().is_after(spring_solar_day) {
      index += 24;
    }
    Self {
      solar_day,
      month: SixtyCycleMonth {
        year: SixtyCycleYear::from_year(lunar_year.get_year()),
        month: LunarMonth::from_ym(solar_year, 1).get_sixty_cycle().next((index as f64 * 0.5).floor() as isize),
      },
      day: lunar_day.get_sixty_cycle(),
    }
  }

  /// 公历日
  pub fn get_solar_day(&self) -> SolarDay {
    self.solar_day
  }

  /// 干支月
  pub fn get_sixty_cycle_month(&self) -> SixtyCycleMonth {
    self.month.clone()
  }

  /// 年柱
  pub fn get_year(&self) -> SixtyCycle {
    self.month.get_year()
  }

  /// 月柱
  pub fn get_month(&self) -> SixtyCycle {
    self.month.get_sixty_cycle()
  }

  /// 干支
  pub fn get_sixty_cycle(&self) -> SixtyCycle {
    self.day.clone()
  }

  /// 建除十二值神
  pub fn get_duty(&self) -> Duty {
    Duty::from_index(self.day.get_earth_branch().get_index() as isize - self.get_month().get_earth_branch().get_index() as isize)
  }

  /// 太岁方位
  pub fn get_jupiter_direction(&self) -> Direction {
    let index: isize = self.day.get_index() as isize;
    if index % 12 < 6 {
      return Element::from_index(index / 12).get_direction();
    }
    self.month.get_sixty_cycle_year().get_jupiter_direction()
  }

  /// 黄道黑道十二神
  pub fn get_twelve_star(&self) -> TwelveStar {
    TwelveStar::from_index(self.day.get_earth_branch().get_index() as isize + (8 - self.get_month().get_earth_branch().get_index() as isize % 6) * 2)
  }

  /// 二十八宿
  pub fn get_twenty_eight_star(&self) -> TwentyEightStar {
    TwentyEightStar::from_index([10, 18, 26, 6, 14, 22, 2][self.solar_day.get_week().get_index()]).next(-7 * self.day.get_earth_branch().get_index() as isize)
  }

  /// 逐日胎神
  pub fn get_fetus_day(&self) -> FetusDay {
    FetusDay::from_sixty_cycle_day(self.clone())
  }

  pub fn get_nine_star(&self) -> NineStar {
    let d: SolarDay = self.solar_day;
    let dong_zhi: SolarTerm = SolarTerm::from_index(d.get_year(), 0);
    let dong_zhi_solar: SolarDay = dong_zhi.get_julian_day().get_solar_day();
    let xia_zhi_solar: SolarDay = dong_zhi.next(12).get_julian_day().get_solar_day();
    let dong_zhi_solar2: SolarDay = dong_zhi.next(24).get_julian_day().get_solar_day();
    let dong_zhi_index: isize = dong_zhi_solar.get_lunar_day().get_sixty_cycle().get_index() as isize;
    let xia_zhi_index: isize = xia_zhi_solar.get_lunar_day().get_sixty_cycle().get_index() as isize;
    let dong_zhi_index2: isize = dong_zhi_solar2.get_lunar_day().get_sixty_cycle().get_index() as isize;
    let solar_shun_bai: SolarDay = dong_zhi_solar.next(if dong_zhi_index > 29 { 60 - dong_zhi_index } else { -dong_zhi_index });
    let solar_shun_bai2: SolarDay = dong_zhi_solar2.next(if dong_zhi_index2 > 29 { 60 - dong_zhi_index2 } else { -dong_zhi_index2 });
    let solar_ni_zi: SolarDay = xia_zhi_solar.next(if xia_zhi_index > 29 { 60 - xia_zhi_index } else { -xia_zhi_index });
    let mut offset: isize = 0;
    if !d.is_before(solar_shun_bai) && d.is_before(solar_ni_zi) {
      offset = d.subtract(solar_shun_bai);
    } else if !d.is_before(solar_ni_zi) && d.is_before(solar_shun_bai2) {
      offset = 8 - d.subtract(solar_ni_zi);
    } else if !d.is_before(solar_shun_bai2) {
      offset = d.subtract(solar_shun_bai2);
    } else if d.is_before(solar_shun_bai) {
      offset = 8 + solar_shun_bai.subtract(d);
    }
    NineStar::from_index(offset)
  }

  pub fn get_hours(&self) -> Vec<SixtyCycleHour> {
    let mut l: Vec<SixtyCycleHour> = Vec::new();
    let d: SolarDay = self.solar_day.next(-1);
    let t: SolarTime = SolarTime::from_ymd_hms(d.get_year(), d.get_month(), d.get_day(), 23, 0, 0);
    let mut h: SixtyCycleHour = SixtyCycleHour::from_solar_time(t);
    l.push(h.clone());
    for _ in 0..11 {
      h = h.next(7200);
      l.push(h.clone());
    }
    l
  }

  pub fn get_gods(&self) -> Vec<God> {
    God::get_day_gods(self.get_month(), self.get_sixty_cycle())
  }

  pub fn get_recommends(&self) -> Vec<Taboo> {
    Taboo::get_day_recommends(self.get_month(), self.get_sixty_cycle())
  }

  pub fn get_avoids(&self) -> Vec<Taboo> {
    Taboo::get_day_avoids(self.get_month(), self.get_sixty_cycle())
  }
}

impl Display for SixtyCycleDay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.month, self.get_name())
  }
}

impl PartialEq for SixtyCycleDay {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for SixtyCycleDay {}

/// 干支时辰
#[derive(Debug, Clone)]
pub struct SixtyCycleHour {
  /// 公历时刻
  solar_time: SolarTime,
  /// 干支日
  day: SixtyCycleDay,
  /// 时柱
  hour: SixtyCycle,
}

impl Tyme for SixtyCycleHour {
  fn next(&self, n: isize) -> Self {
    SixtyCycleHour::from_solar_time(self.solar_time.next(n))
  }
}

impl Culture for SixtyCycleHour {
  fn get_name(&self) -> String {
    format!("{}时", self.hour)
  }
}

impl SixtyCycleHour {
  pub fn from_solar_time(solar_time: SolarTime) -> Self {
    let solar_year: isize = solar_time.get_year();
    let spring_solar_time: SolarTime = SolarTerm::from_index(solar_year, 3).get_julian_day().get_solar_time();
    let lunar_hour: LunarHour = solar_time.get_lunar_hour();
    let lunar_day: LunarDay = lunar_hour.get_lunar_day();
    let mut lunar_year: LunarYear = lunar_day.get_lunar_month().get_lunar_year();
    if lunar_year.get_year() == solar_year {
      if solar_time.is_before(spring_solar_time) {
        lunar_year = lunar_year.next(-1);
      }
    } else if lunar_year.get_year() < solar_year {
      if !solar_time.is_before(spring_solar_time) {
        lunar_year = lunar_year.next(1);
      }
    }
    let term: SolarTerm = solar_time.get_term();
    let mut index: isize = term.get_index() as isize - 3;
    if index < 0 && term.get_julian_day().get_solar_time().is_after(SolarTerm::from_index(solar_year, 3).get_julian_day().get_solar_time()) {
      index += 24;
    }
    let mut d: SixtyCycle = lunar_day.get_sixty_cycle();
    if solar_time.get_hour() == 23 {
      d = d.next(1);
    }
    let y: SixtyCycleYear = SixtyCycleYear::from_year(lunar_year.get_year());
    let m: LunarMonth = LunarMonth::from_ym(solar_year, 1);
    Self {
      solar_time,
      day: SixtyCycleDay {
        solar_day: solar_time.get_solar_day(),
        month: SixtyCycleMonth {
          year: y,
          month: m.get_sixty_cycle().next((index as f64 * 0.5).floor() as isize),
        },
        day: d,
      },
      hour: lunar_hour.get_sixty_cycle(),
    }
  }

  pub fn get_year(&self) -> SixtyCycle {
    self.day.get_year()
  }

  pub fn get_month(&self) -> SixtyCycle {
    self.day.get_month()
  }

  pub fn get_day(&self) -> SixtyCycle {
    self.day.get_sixty_cycle()
  }

  pub fn get_sixty_cycle(&self) -> SixtyCycle {
    self.hour.clone()
  }

  pub fn get_sixty_cycle_day(&self) -> SixtyCycleDay {
    self.day.clone()
  }

  pub fn get_solar_time(&self) -> SolarTime {
    self.solar_time
  }

  pub fn get_index_in_day(&self) -> usize {
    let h: usize = self.solar_time.get_hour();
    if h == 23 {
      return 0;
    }
    (h + 1) / 2
  }

  pub fn get_eight_char(&self) -> EightChar {
    EightChar::from_sixty_cycle(self.get_year(), self.get_month(), self.get_day(), self.get_sixty_cycle())
  }

  pub fn get_nine_star(&self) -> NineStar {
    let solar: SolarDay = self.solar_time.get_solar_day();
    let dong_zhi: SolarTerm = SolarTerm::from_index(solar.get_year(), 0);
    let earth_branch_index: isize = self.get_index_in_day() as isize % 12;
    let mut index: isize = [8, 5, 2][self.get_day().get_earth_branch().get_index() % 3];
    if !solar.is_before(dong_zhi.get_julian_day().get_solar_day()) && solar.is_before(dong_zhi.next(12).get_julian_day().get_solar_day()) {
      index = 8 + earth_branch_index - index
    } else {
      index -= earth_branch_index;
    }
    NineStar::from_index(index)
  }

  pub fn get_twelve_star(&self) -> TwelveStar {
    TwelveStar::from_index(self.hour.get_earth_branch().get_index() as isize + (8 - self.get_day().get_earth_branch().get_index() as isize % 6) * 2)
  }

  pub fn get_recommends(&self) -> Vec<Taboo> {
    Taboo::get_hour_recommends(self.get_day(), self.get_sixty_cycle())
  }

  pub fn get_avoids(&self) -> Vec<Taboo> {
    Taboo::get_hour_avoids(self.get_day(), self.get_sixty_cycle())
  }
}

impl Display for SixtyCycleHour {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.day, self.get_name())
  }
}

impl PartialEq for SixtyCycleHour {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for SixtyCycleHour {}

#[cfg(test)]
mod tests {
  use crate::tyme::Culture;
  use crate::tyme::sixtycycle::{EarthBranch, HeavenStem, SixtyCycle};

  #[test]
  fn test1() {
    assert_eq!("子", EarthBranch::from_index(0).get_name());
  }

  #[test]
  fn test2() {
    assert_eq!(0, EarthBranch::from_index(0).get_index());
  }

  #[test]
  fn test3() {
    assert_eq!("甲", HeavenStem::from_index(0).get_name());
  }

  #[test]
  fn test4() {
    assert_eq!(0, HeavenStem::from_index(0).get_index());
  }

  #[test]
  fn test5() {
    assert_eq!(HeavenStem::from_name("丙").get_element(), HeavenStem::from_name("甲").get_element().get_reinforce());
  }

  #[test]
  fn test6() {
    assert_eq!("比肩", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("甲")).get_name());
    assert_eq!("劫财", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("乙")).get_name());
    assert_eq!("食神", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("丙")).get_name());
    assert_eq!("伤官", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("丁")).get_name());
    assert_eq!("偏财", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("戊")).get_name());
    assert_eq!("正财", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("己")).get_name());
    assert_eq!("七杀", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("庚")).get_name());
    assert_eq!("正官", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("辛")).get_name());
    assert_eq!("偏印", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("壬")).get_name());
    assert_eq!("正印", HeavenStem::from_name("甲").get_ten_star(HeavenStem::from_name("癸")).get_name());
  }

  #[test]
  fn test7() {
    assert_eq!("丁丑", SixtyCycle::from_index(13).get_name());
  }

  #[test]
  fn test8() {
    assert_eq!(13, SixtyCycle::from_name("丁丑").get_index());
  }

  #[test]
  fn test9() {
    assert_eq!("石榴木", SixtyCycle::from_name("辛酉").get_sound().get_name());
    assert_eq!("剑锋金", SixtyCycle::from_name("癸酉").get_sound().get_name());
    assert_eq!("平地木", SixtyCycle::from_name("己亥").get_sound().get_name());
  }

  #[test]
  fn test10() {
    assert_eq!("甲子", SixtyCycle::from_name("甲子").get_ten().get_name());
    assert_eq!("甲寅", SixtyCycle::from_name("乙卯").get_ten().get_name());
    assert_eq!("甲申", SixtyCycle::from_name("癸巳").get_ten().get_name());
  }

  #[test]
  fn test11() {
    assert_eq!("长生", HeavenStem::from_name("丙").get_terrain(EarthBranch::from_name("寅")).get_name());
    assert_eq!("沐浴", HeavenStem::from_name("辛").get_terrain(EarthBranch::from_name("亥")).get_name());
  }
}