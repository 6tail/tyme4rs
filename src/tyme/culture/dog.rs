use crate::tyme::{AbstractCulture, AbstractCultureDay, AbstractTyme, Culture, LoopTyme, Tyme};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

pub static DOG_NAMES: [&str; 3] = ["初伏", "中伏", "末伏"];

/// 三伏
#[derive(Debug, Clone)]
pub struct Dog {
    parent: LoopTyme,
}

impl Deref for Dog {
    type Target = LoopTyme;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for Dog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Tyme for Dog {
    fn next(&self, n: isize) -> Self {
        Self::from_index(self.parent.next_index(n) as isize)
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
            parent: LoopTyme::from_index(
                DOG_NAMES.to_vec().iter().map(|x| x.to_string()).collect(),
                index,
            ),
        }
    }

    pub fn from_name(name: &str) -> Self {
        Self {
            parent: LoopTyme::from_name(
                DOG_NAMES.to_vec().iter().map(|x| x.to_string()).collect(),
                name,
            ),
        }
    }
}

impl Display for Dog {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_name())
    }
}

impl PartialEq for Dog {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for Dog {}

impl From<Dog> for LoopTyme {
    fn from(val: Dog) -> Self {
        val.parent
    }
}

/// 三伏天
#[derive(Debug, Clone)]
pub struct DogDay {
    parent: AbstractCultureDay,
    dog: Dog,
}

impl Deref for DogDay {
    type Target = AbstractCultureDay;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for DogDay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
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
            dog,
        }
    }

    pub fn get_dog(&self) -> Dog {
        self.dog.clone()
    }
}

impl Display for DogDay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}第{}天", self.get_name(), self.get_day_index() + 1)
    }
}

impl PartialEq for DogDay {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for DogDay {}

impl From<DogDay> for AbstractCultureDay {
    fn from(val: DogDay) -> Self {
        val.parent
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
        assert_eq!(
            "初伏",
            DogDay::new(Dog::from_index(0), 2).get_dog().to_string()
        );
    }
}
