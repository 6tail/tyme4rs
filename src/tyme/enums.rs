use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum FestivalType {
  DAY,
  TERM,
  EVE,
}

impl FestivalType {
  pub fn from_code(code: usize) -> Result<Self, String> {
    match code {
      0 => {
        Ok(Self::DAY)
      }
      1 => {
        Ok(Self::TERM)
      }
      2 => {
        Ok(Self::EVE)
      }
      _ => {
        Err(format!("illegal FestivalType code: {}", code))
      }
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    match name {
      "日期" => {
        Ok(Self::DAY)
      }
      "节气" => {
        Ok(Self::TERM)
      }
      "除夕" => {
        Ok(Self::EVE)
      }
      _ => {
        Err(format!("illegal FestivalType name: {}", name))
      }
    }
  }

  pub fn get_name(&self) -> String {
    self.to_string()
  }
}

impl Display for FestivalType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::DAY => write!(f, "{}", "日期"),
      Self::TERM => write!(f, "{}", "节气"),
      Self::EVE => write!(f, "{}", "除夕")
    }
  }
}

impl PartialEq for FestivalType {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for FestivalType {}

#[derive(Debug, Copy, Clone)]
pub enum HideHeavenStemType {
  RESIDUAL,
  MIDDLE,
  MAIN,
}

impl HideHeavenStemType {
  pub fn from_code(code: usize) -> Result<Self, String> {
    match code {
      0 => {
        Ok(Self::RESIDUAL)
      }
      1 => {
        Ok(Self::MIDDLE)
      }
      2 => {
        Ok(Self::MAIN)
      }
      _ => {
        Err(format!("illegal HideHeavenStemType code: {}", code))
      }
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    match name {
      "余气" => {
        Ok(Self::RESIDUAL)
      }
      "中气" => {
        Ok(Self::MIDDLE)
      }
      "本气" => {
        Ok(Self::MAIN)
      }
      _ => {
        Err(format!("illegal HideHeavenStemType name: {}", name))
      }
    }
  }

  pub fn get_name(&self) -> String {
    self.to_string()
  }
}

impl Display for HideHeavenStemType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::RESIDUAL => write!(f, "{}", "余气"),
      Self::MIDDLE => write!(f, "{}", "中气"),
      Self::MAIN => write!(f, "{}", "本气")
    }
  }
}

impl PartialEq for HideHeavenStemType {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for HideHeavenStemType {}

#[derive(Debug, Copy, Clone)]
pub enum Gender {
  WOMAN,
  MAN,
}

impl Gender {
  pub fn from_code(code: usize) -> Result<Self, String> {
    match code {
      0 => {
        Ok(Self::WOMAN)
      }
      1 => {
        Ok(Self::MAN)
      }
      _ => {
        Err(format!("illegal Gender code: {}", code))
      }
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    match name {
      "女" => {
        Ok(Self::WOMAN)
      }
      "男" => {
        Ok(Self::MAN)
      }
      _ => {
        Err(format!("illegal Gender name: {}", name))
      }
    }
  }

  pub fn get_name(&self) -> String {
    self.to_string()
  }
}

impl Display for Gender {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::WOMAN => write!(f, "{}", "女"),
      Self::MAN => write!(f, "{}", "男")
    }
  }
}

impl PartialEq for Gender {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Gender {}

#[derive(Debug, Copy, Clone)]
pub enum Side {
  IN,
  OUT,
}

impl Side {
  pub fn from_code(code: usize) -> Result<Self, String> {
    match code {
      0 => {
        Ok(Self::IN)
      }
      1 => {
        Ok(Self::OUT)
      }
      _ => {
        Err(format!("illegal Side code: {}", code))
      }
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    match name {
      "内" => {
        Ok(Self::IN)
      }
      "外" => {
        Ok(Self::OUT)
      }
      _ => {
        Err(format!("illegal Side name: {}", name))
      }
    }
  }

  pub fn get_name(&self) -> String {
    self.to_string()
  }
}

impl Display for Side {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::IN => write!(f, "{}", "内"),
      Self::OUT => write!(f, "{}", "外")
    }
  }
}

impl PartialEq for Side {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for Side {}

#[derive(Debug, Copy, Clone)]
pub enum YinYang {
  YIN,
  YANG,
}


impl YinYang {
  pub fn from_code(code: usize) -> Result<Self, String> {
    match code {
      0 => {
        Ok(Self::YIN)
      }
      1 => {
        Ok(Self::YANG)
      }
      _ => {
        Err(format!("illegal YinYang code: {}", code))
      }
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    match name {
      "阴" => {
        Ok(Self::YIN)
      }
      "阳" => {
        Ok(Self::YANG)
      }
      _ => {
        Err(format!("illegal YinYang name: {}", name))
      }
    }
  }

  pub fn get_name(&self) -> String {
    self.to_string()
  }
}

impl Display for YinYang {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::YIN => write!(f, "{}", "阴"),
      Self::YANG => write!(f, "{}", "阳")
    }
  }
}

impl PartialEq for YinYang {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for YinYang {}

#[cfg(test)]
mod tests {
  use crate::tyme::enums::Gender;

  #[test]
  fn test1() {
    assert_eq!(Gender::from_code(1).unwrap(), Gender::MAN);
  }

  #[test]
  fn test2() {
    assert_eq!(Gender::from_name("男").unwrap(), Gender::MAN);
  }

  #[test]
  fn test3() {
    assert!(Gender::from_name("未知").is_err());
  }
}
