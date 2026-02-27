use crate::tyme::{Culture, LoopTyme, Tyme};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

pub static TEN_STAR_NAMES: [&str; 10] = [
    "比肩", "劫财", "食神", "伤官", "偏财", "正财", "七杀", "正官", "偏印", "正印",
];

/// 十神
#[derive(Debug, Clone)]
pub struct TenStar {
    parent: LoopTyme,
}

impl Deref for TenStar {
    type Target = LoopTyme;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for TenStar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Tyme for TenStar {
    fn next(&self, n: isize) -> Self {
        Self::from_index(self.parent.next_index(n) as isize)
    }
}

impl Culture for TenStar {
    fn get_name(&self) -> String {
        self.parent.get_name()
    }
}

impl TenStar {
    pub fn from_index(index: isize) -> Self {
        Self {
            parent: LoopTyme::from_index(
                TEN_STAR_NAMES
                    .to_vec()
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
                index,
            ),
        }
    }

    pub fn from_name(name: &str) -> Self {
        Self {
            parent: LoopTyme::from_name(
                TEN_STAR_NAMES
                    .to_vec()
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
                name,
            ),
        }
    }
}

impl Display for TenStar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

impl PartialEq for TenStar {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for TenStar {}

impl From<TenStar> for LoopTyme {
    fn from(val: TenStar) -> Self {
        val.parent
    }
}
