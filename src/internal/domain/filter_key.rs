use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct FilterKey(String);
impl FilterKey {
    pub fn new(v: String) -> Self {
        Self(v)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl Default for FilterKey {
    fn default() -> Self {
        Self::new(String::from(""))
    }
}
impl fmt::Display for FilterKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<String> for FilterKey {
    fn from(raw: String) -> Self {
        Self(raw)
    }
}
impl From<&str> for FilterKey {
    fn from(raw: &str) -> Self {
        Self(raw.to_owned())
    }
}
impl FromStr for FilterKey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}
impl AsRef<str> for FilterKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
