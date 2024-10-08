use std::borrow::Cow;
use std::fmt;

use derive_more::{AsRef, Into};

use crate::errors::ErnError;

#[derive(AsRef, Into, Eq, Debug, PartialEq, Clone, Hash, PartialOrd)]
pub struct Part(pub(crate) String);

impl Part {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_owned(self) -> Part {
        Part(self.0.to_string())
    }

    pub fn new(value: impl Into<String>) -> Result<Part, ErnError> {
        let value = value.into();
        if value.contains(':') || value.contains('/') {
            return Err(ErnError::InvalidPartFormat);
        }
        if value.is_empty() {
            return Err(ErnError::ParseFailure(
                "Part",
                "cannot be empty".to_string(),
            ));
        }
        Ok(Part(value))
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Part {
    type Err = ErnError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Part::new(Cow::Owned(s.to_owned()))
    }
}

// impl From<Part> for String {
//     fn from(part: Part) -> Self {
//         part.0
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_creation() -> anyhow::Result<()> {
        let part = Part::new("segment")?;
        assert_eq!(part.as_str(), "segment");
        Ok(())
    }

    #[test]
    fn test_part_display() -> anyhow::Result<()> {
        let part = Part::new("example")?;
        assert_eq!(format!("{}", part), "example");
        Ok(())
    }

    #[test]
    fn test_part_from_str() {
        let part: Part = "test".parse().unwrap();
        assert_eq!(part.as_str(), "test");
    }

    #[test]
    fn test_part_equality() -> anyhow::Result<()> {
        let part1 = Part::new("segment1")?;
        let part2 = Part::new("segment1")?;
        let part3 = Part::new("segment2")?;
        assert_eq!(part1, part2);
        assert_ne!(part1, part3);
        Ok(())
    }

    #[test]
    fn test_part_into_string() -> anyhow::Result<()> {
        let part = Part::new("segment")?;
        let string: String = part.into();
        assert_eq!(string, "segment");
        Ok(())
    }
}
