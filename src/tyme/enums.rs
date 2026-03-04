use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FestivalType {
  DAY,
  TERM,
  EVE,
}

impl FestivalType {
  pub fn from_code(code: usize) -> Result<Self, String> {
    match code {
      0 => Ok(Self::DAY),
      1 => Ok(Self::TERM),
      2 => Ok(Self::EVE),
      _ => Err(format!("illegal FestivalType code: {}", code)),
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    match name {
      "日期" => Ok(Self::DAY),
      "节气" => Ok(Self::TERM),
      "除夕" => Ok(Self::EVE),
      _ => Err(format!("illegal FestivalType name: {}", name)),
    }
  }

  pub fn get_name(&self) -> String {
    self.to_string()
  }

  pub fn get_code(&self) -> usize {
    match self {
      Self::DAY => 0,
      Self::TERM => 1,
      Self::EVE => 2,
    }
  }
}

impl Display for FestivalType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::DAY => f.write_str("日期"),
      Self::TERM => f.write_str("节气"),
      Self::EVE => f.write_str("除夕"),
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HideHeavenStemType {
  RESIDUAL,
  MIDDLE,
  MAIN,
}

impl HideHeavenStemType {
  pub fn from_code(code: usize) -> Result<Self, String> {
    match code {
      0 => Ok(Self::RESIDUAL),
      1 => Ok(Self::MIDDLE),
      2 => Ok(Self::MAIN),
      _ => Err(format!("illegal HideHeavenStemType code: {}", code)),
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    match name {
      "余气" => Ok(Self::RESIDUAL),
      "中气" => Ok(Self::MIDDLE),
      "本气" => Ok(Self::MAIN),
      _ => Err(format!("illegal HideHeavenStemType name: {}", name)),
    }
  }

  pub fn get_name(&self) -> String {
    self.to_string()
  }
}

impl Display for HideHeavenStemType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::RESIDUAL => f.write_str("余气"),
      Self::MIDDLE => f.write_str("中气"),
      Self::MAIN => f.write_str("本气"),
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Gender {
  WOMAN,
  MAN,
}

impl Gender {
  pub fn from_code(code: usize) -> Result<Self, String> {
    match code {
      0 => Ok(Self::WOMAN),
      1 => Ok(Self::MAN),
      _ => Err(format!("illegal Gender code: {}", code)),
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    match name {
      "女" => Ok(Self::WOMAN),
      "男" => Ok(Self::MAN),
      _ => Err(format!("illegal Gender name: {}", name)),
    }
  }

  pub fn get_name(&self) -> String {
    self.to_string()
  }
}

impl Display for Gender {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::WOMAN => f.write_str("女"),
      Self::MAN => f.write_str("男"),
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Side {
  IN,
  OUT,
}

impl Side {
  pub fn from_code(code: usize) -> Result<Self, String> {
    match code {
      0 => Ok(Self::IN),
      1 => Ok(Self::OUT),
      _ => Err(format!("illegal Side code: {}", code)),
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    match name {
      "内" => Ok(Self::IN),
      "外" => Ok(Self::OUT),
      _ => Err(format!("illegal Side name: {}", name)),
    }
  }

  pub fn get_name(&self) -> String {
    self.to_string()
  }
}

impl Display for Side {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::IN => f.write_str("内"),
      Self::OUT => f.write_str("外"),
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum YinYang {
  YIN,
  YANG,
}


impl YinYang {
  pub fn from_code(code: usize) -> Result<Self, String> {
    match code {
      0 => Ok(Self::YIN),
      1 => Ok(Self::YANG),
      _ => Err(format!("illegal YinYang code: {}", code)),
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    match name {
      "阴" => Ok(Self::YIN),
      "阳" => Ok(Self::YANG),
      _ => Err(format!("illegal YinYang name: {}", name)),
    }
  }

  pub fn get_name(&self) -> String {
    self.to_string()
  }
}

impl Display for YinYang {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::YIN => f.write_str("阴"),
      Self::YANG => f.write_str("阳"),
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EventType {
  SolarDay,
  SolarWeek,
  LunarDay,
  TermDay,
  TermHs,
  TermEb
}


impl EventType {
  pub fn from_code(code: usize) -> Result<Self, String> {
    match code {
      0 => Ok(Self::SolarDay),
      1 => Ok(Self::SolarWeek),
      2 => Ok(Self::LunarDay),
      3 => Ok(Self::TermDay),
      4 => Ok(Self::TermHs),
      5 => Ok(Self::TermEb),
      _ => Err(format!("illegal EventType code: {}", code)),
    }
  }

  pub fn from_name(name: &str) -> Result<Self, String> {
    match name {
      "公历日期" => Ok(Self::SolarDay),
      "几月第几个星期几" => Ok(Self::SolarWeek),
      "农历日期" => Ok(Self::LunarDay),
      "节气日期" => Ok(Self::TermDay),
      "节气天干" => Ok(Self::TermHs),
      "节气地支" => Ok(Self::TermEb),
      _ => Err(format!("illegal EventType name: {}", name)),
    }
  }

  pub fn get_name(&self) -> String {
    self.to_string()
  }

  pub fn get_code(&self) -> usize {
    match self {
      Self::SolarDay => 0,
      Self::SolarWeek => 1,
      Self::LunarDay => 2,
      Self::TermDay => 3,
      Self::TermHs => 4,
      Self::TermEb => 5,
    }
  }
}

impl Display for EventType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::SolarDay => f.write_str("公历日期"),
      Self::SolarWeek => f.write_str("几月第几个星期几"),
      Self::LunarDay => f.write_str("农历日期"),
      Self::TermDay => f.write_str("节气日期"),
      Self::TermHs => f.write_str("节气天干"),
      Self::TermEb => f.write_str("节气地支"),
    }
  }
}

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
