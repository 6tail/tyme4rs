use std::fmt::{Display, Formatter};

/// 传统文化(民俗)
pub trait Culture {

  /// Return 名称
  fn get_name(&self) -> String;
}

pub trait Tyme: Culture {
  fn next(&self, n: isize) -> Result<Self, String> where Self: Sized;
}

#[derive(Debug, Copy, Clone)]
pub struct AbstractCulture {}

impl Culture for AbstractCulture {
  fn get_name(&self) -> String {
    unimplemented!()
  }
}

impl AbstractCulture {
  pub fn new() -> Self {
    Self {}
  }

  pub fn index_of(&self, index: isize, size: usize) -> usize {
    let n: isize = size as isize;
    let mut i: isize = index % n;
    if i < 0 {
      i += n;
    }
    i as usize
  }
}

impl Display for AbstractCulture {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for AbstractCulture {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for AbstractCulture {}

#[derive(Debug, Copy, Clone)]
pub struct AbstractCultureDay {
  culture: AbstractCulture,
  day_index: usize,
}

impl Culture for AbstractCultureDay {
  fn get_name(&self) -> String {
    self.culture.get_name()
  }
}

impl AbstractCultureDay {
  pub fn new(culture: AbstractCulture, day_index: usize) -> Self {
    Self {
      culture,
      day_index,
    }
  }

  pub fn get_day_index(&self) -> usize {
    self.day_index
  }

  pub fn get_culture(&self) -> AbstractCulture {
    self.culture
  }
}

impl Display for AbstractCultureDay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}第{}天", self.get_name(), self.day_index + 1)
  }
}

impl PartialEq for AbstractCultureDay {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for AbstractCultureDay {}

#[derive(Debug, Copy, Clone)]
pub struct AbstractTyme {
  parent: AbstractCulture,
}

impl AbstractTyme {
  pub fn new() -> Self {
    Self {
      parent: AbstractCulture::new()
    }
  }
}

impl Display for AbstractTyme {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.parent.get_name())
  }
}

impl PartialEq for AbstractTyme {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for AbstractTyme {}

impl Into<AbstractCulture> for AbstractTyme {
  fn into(self) -> AbstractCulture {
    self.parent
  }
}

#[derive(Debug, Clone)]
pub struct LoopTyme {
  parent: AbstractTyme,
  names: Vec<String>,
  index: usize,
}

impl Tyme for LoopTyme {
  fn next(&self, _n: isize) -> Result<Self, String> {
    unimplemented!()
  }
}

impl Culture for LoopTyme {
  fn get_name(&self) -> String {
    self.names[self.index].to_string()
  }
}

impl LoopTyme {
  pub fn from_index(names: Vec<String>, index: isize) -> Self {
    let size: usize = names.len();
    let parent: AbstractTyme = AbstractTyme::new();
    let culture: AbstractCulture = parent.into();
    Self {
      parent,
      names,
      index: culture.index_of(index, size),
    }
  }

  pub fn from_name(names: Vec<String>, name: &str) -> Result<Self, String> {
    let mut real_index: Option<usize> = None;
    for i in 0..names.len() {
      if names[i].to_string() == name {
        real_index = Some(i);
        break;
      }
    }
    match real_index {
      None => {
        Err(format!("illegal name: {}", name))
      }
      Some(n) => {
        Ok(Self {
          parent: AbstractTyme::new(),
          names,
          index: n,
        })
      }
    }
  }

  pub fn get_index(&self) -> usize {
    self.index
  }

  pub fn get_size(&self) -> usize {
    self.names.len()
  }

  fn index_of_index(&self, index: isize) -> usize {
    let culture: AbstractCulture = self.parent.into();
    culture.index_of(index, self.get_size())
  }

  pub fn next_index(&self, n: isize) -> usize {
    self.index_of_index(self.index as isize + n)
  }
}

impl Display for LoopTyme {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for LoopTyme {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for LoopTyme {}

impl Into<AbstractTyme> for LoopTyme {
  fn into(self) -> AbstractTyme {
    self.parent
  }
}

pub mod enums;
pub mod culture;
pub mod jd;
pub mod sixtycycle;
pub mod lunar;
pub mod solar;
pub mod util;
pub mod holiday;
pub mod festival;
