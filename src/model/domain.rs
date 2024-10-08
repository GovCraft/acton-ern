use std::fmt;

use derive_more::{AsRef, From, Into};

use crate::errors::ErnError;

#[derive(AsRef, From, Into, Eq, Debug, PartialEq, Clone, Hash, PartialOrd)]
pub struct Domain(pub(crate) String);

impl Domain {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_owned(self) -> Domain {
        Domain(self.0)
    }
    pub fn new(value: impl Into<String>) -> Result<Self, ErnError> {
        let val = value.into();
        if val.is_empty() {
            Err(ErnError::ParseFailure("Domain", "cannot be empty".to_string()))
        } else {
            Ok(Domain(val))
        }
    }
}

impl Default for Domain {
    fn default() -> Self {
        Domain("acton".to_string())
    }
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Domain {
    type Err = ErnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Domain::new(s.to_string())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_domain_creation() {
//         let domain = Domain::new("test").unwrap();
//         assert_eq!(domain.as_str(), "test");
//     }
//
//     #[test]
//     fn test_domain_default() {
//         let domain = Domain::default();
//         assert_eq!(domain.as_str(), "acton");
//     }
//
//     #[test]
//     fn test_domain_display() {
//         let domain = Domain::new("example");
//         assert_eq!(format!("{}", domain.unwrap()), "example");
//     }
//
//     #[test]
//     fn test_domain_from_str() {
//         let domain: Domain = "test".parse().unwrap();
//         assert_eq!(domain.as_str(), "test");
//     }
//
//     #[test]
//     fn test_domain_equality() -> anyhow::Result<()> {
//         let domain1 = Domain::new("test")?;
//         let domain2 = Domain::new("test")?;
//         let domain3 = Domain::new("other")?;
//         assert_eq!(domain1, domain2);
//         assert_ne!(domain1, domain3);
//         Ok(())
//     }
//
//     #[test]
//     fn test_domain_into_string() {
//         let domain = Domain::new("test").unwrap();
//         let string: String = domain.into();
//         assert_eq!(string, "test");
//     }
// }
