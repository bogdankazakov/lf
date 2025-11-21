use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Record(String);
impl Record {
    pub fn new(v: String) -> Self {
        Self(v)
    }
}
impl Default for Record {
    fn default() -> Self {
        Self::new(String::from(""))
    }
}
impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<String> for Record {
    fn from(raw: String) -> Self {
        Self(raw)
    }
}
impl From<&str> for Record {
    fn from(raw: &str) -> Self {
        Self(raw.to_owned())
    }
}
impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}
impl AsRef<str> for Record {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
