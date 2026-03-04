use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

/// 传统文化(民俗)
pub trait Culture {
    /// Return 名称
    fn get_name(&self) -> String;
}

pub trait Tyme: Culture {
    fn next(&self, n: isize) -> Self
    where
        Self: Sized;
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
        f.write_str(&self.get_name())
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
        Self { culture, day_index }
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

impl Deref for AbstractTyme {
    type Target = AbstractCulture;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for AbstractTyme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl AbstractTyme {
    pub fn new() -> Self {
        Self {
            parent: AbstractCulture::new(),
        }
    }
}

impl Display for AbstractTyme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.parent.get_name())
    }
}

impl PartialEq for AbstractTyme {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for AbstractTyme {}

impl From<AbstractTyme> for AbstractCulture {
    fn from(val: AbstractTyme) -> Self {
        val.parent
    }
}

#[derive(Debug, Clone)]
pub struct LoopTyme {
    parent: AbstractTyme,
    names: Vec<String>,
    index: usize,
}

impl Tyme for LoopTyme {
    fn next(&self, _n: isize) -> Self {
        unimplemented!()
    }
}

impl Culture for LoopTyme {
    fn get_name(&self) -> String {
        self.names[self.index].to_string()
    }
}

impl Deref for LoopTyme {
    type Target = AbstractTyme;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for LoopTyme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl LoopTyme {
    pub fn new(names: Vec<String>, name: &str) -> Result<Self, String> {
        match names.iter().position(|x| x == name) {
            None => Err(format!("illegal name: {}", name)),
            Some(n) => Ok(Self {
                parent: AbstractTyme::new(),
                names,
                index: n,
            }),
        }
    }

    pub fn from_index(names: Vec<String>, index: isize) -> Self {
        let size: usize = names.len();
        let parent: AbstractTyme = AbstractTyme::new();
        let i: usize = parent.index_of(index, size);
        Self {
            parent,
            names,
            index: i,
        }
    }

    pub fn from_name(names: Vec<String>, name: &str) -> Self {
        Self::new(names, name).unwrap()
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_size(&self) -> usize {
        self.names.len()
    }

    fn index_of_index(&self, index: isize) -> usize {
        self.index_of(index, self.get_size())
    }

    pub fn next_index(&self, n: isize) -> usize {
        self.index_of_index(self.index as isize + n)
    }

    pub fn steps_to(&self, target_index: isize) -> usize {
        self.index_of_index(target_index - self.index as isize)
    }

    pub fn steps_back_to(&self, target_index: isize) -> isize {
        let n: isize = self.get_size() as isize;
        -((self.index as isize - target_index + n) % n)
    }
}

impl Display for LoopTyme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_name())
    }
}

impl PartialEq for LoopTyme {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for LoopTyme {}

impl From<LoopTyme> for AbstractTyme {
    fn from(val: LoopTyme) -> Self {
        val.parent
    }
}

pub mod culture;
pub mod eightchar;
pub mod enums;
pub mod event;
pub mod festival;
pub mod holiday;
pub mod jd;
pub mod lunar;
pub mod rabbyung;
pub mod sixtycycle;
pub mod solar;
pub mod unit;
pub mod util;
