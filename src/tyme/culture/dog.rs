use std::fmt::{Display, Formatter};
use crate::tyme::{AbstractCulture, AbstractCultureDay, AbstractTyme, Culture, LoopTyme, Tyme};

pub static DOG_NAMES: [&str; 3] = ["初伏", "中伏", "末伏"];

/// 三伏
#[derive(Debug, Clone)]
pub struct Dog {
  parent: LoopTyme,
}

impl Tyme for Dog {
  fn next(&self, n: isize) -> Result<Self, String> {
    Ok(Self::from_index(self.parent.next_index(n) as isize))
  }
}

impl Culture for Dog {
  fn get_name(&self) -> String {
    self.parent.get_name()
  }
}

impl Dog {
  pub fn from_index(index: isize) -> Self {
    Self {
      parent: LoopTyme::from_index(DOG_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), index)
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    Ok(Self {
      parent: LoopTyme::from_name(DOG_NAMES.to_vec().iter().map(|x| x.to_string()).collect(), name).unwrap()
    })
  }

  pub fn get_index(&self) -> usize {
    self.parent.get_index()
  }

  pub fn get_size(&self) -> usize {
    self.parent.get_size()
  }
}

impl Display for Dog {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl PartialEq for Dog {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Dog {}

impl Into<LoopTyme> for Dog {
  fn into(self) -> LoopTyme {
    self.parent
  }
}

/// 三伏天
#[derive(Debug, Clone)]
pub struct DogDay {
  parent: AbstractCultureDay,
  dog: Dog
}

impl Culture for DogDay {
  fn get_name(&self) -> String {
    self.dog.get_name()
  }
}

impl DogDay {
  pub fn new(dog: Dog, day_index: usize) -> Self {
    let loop_tyme: LoopTyme = dog.clone().into();
    let abstract_tyme: AbstractTyme = loop_tyme.into();
    let culture: AbstractCulture = abstract_tyme.into();
    Self {
      parent: AbstractCultureDay::new(culture, day_index),
      dog
    }
  }

  pub fn get_dog(&self) -> Dog {
    self.dog.clone()
  }

  pub fn get_day_index(&self) -> usize {
    self.parent.get_day_index()
  }
}

impl Display for DogDay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}第{}天", self.get_name(), self.parent.get_day_index() + 1)
  }
}

impl PartialEq for DogDay {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for DogDay {}

impl Into<AbstractCultureDay> for DogDay {
  fn into(self) -> AbstractCultureDay {
    self.parent
  }
}

#[cfg(test)]
mod tests {
  use crate::tyme::culture::dog::{Dog, DogDay};

  #[test]
  fn test1() {
    assert_eq!("初伏第3天", DogDay::new(Dog::from_index(0), 2).to_string());
  }

  #[test]
  fn test2() {
    assert_eq!("初伏", DogDay::new(Dog::from_index(0), 2).get_dog().to_string());
  }

}
