use std::str::FromStr;

use crate::EntityRoot;
use crate::errors::ErnError;
use crate::model::{Account, Category, Domain, Ern, Part, Parts};

/// A parser for decoding ERN (Entity Resource Name) strings into their constituent components.
pub struct ErnParser {
    /// The ERN (Entity Resource Name) string to be parsed.
    ern: String,
}

impl ErnParser {
    /// Constructs a new `ErnParser` for a given ERN (Entity Resource Name) string.
    ///
    /// # Arguments
    ///
    /// * `ern` - A string slice or owned String representing the ERN (Entity Resource Name) to be parsed.
    ///
    /// # Returns
    ///
    /// Returns an `ErnParser` instance initialized with the given ERN (Entity Resource Name) string.
    pub fn new(ern: String) -> Self {
        Self {
            ern,
        }
    }

    /// Parses the ERN (Entity Resource Name) into its component parts and returns them as a structured result.
    /// Verifies correct ERN (Entity Resource Name) format and validates each part.
    ///
    /// # Returns
    ///
    /// Returns an `ERN (Entity Resource Name)` instance containing the parsed components.
    /// If parsing fails, returns an error message as a `String`.
    pub fn parse(&self) -> Result<Ern, ErnError> {
        let parts: Vec<String> = self.ern.splitn(5, ':').map(|s| s.to_string()).collect();

        if parts.len() != 5 || parts[0] != "ern" {
            return Err(ErnError::InvalidFormat);
        }

        let domain = Domain::from_str(&parts[1])?;
        let category = Category::from_str(&parts[2])?;
        let account = Account::from_str(&parts[3])?;

        // Split the root and the path part
        let root_path: Vec<String> = parts[4].splitn(2, '/').map(|s| s.to_string()).collect();
        let root_str = root_path[0].clone();
        let root: EntityRoot = EntityRoot::from_str(root_str.as_str())?;

        // Continue with the path parts
        let mut ern_parts = Vec::new();
        if root_path.len() > 1 {
            let path_parts: Vec<String> = root_path[1].split('/').map(|s| s.to_string()).collect();
            for part in path_parts.iter() {
                ern_parts.push(Part::from_str(part)?);
            }
        }

        let parts = Parts::new(ern_parts);
        Ok(Ern::new(domain, category, account, root, parts))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::UnixTime;
//     use super::*;
//
//     #[test]
//     fn test_valid_ern_parsing() {
//         let ern_str = "ern:custom:service:account123:root/resource/subresource".to_string();
//         let parser: ErnParser = ErnParser::new(ern_str);
//         let result = parser.parse();
//
//         assert!(result.is_ok());
//         let ern = result.unwrap();
//         assert_eq!(ern.domain.as_str(), "custom");
//     }
//
//     #[test]
//     fn test_invalid_ern_format() {
//         let ern_str = "invalid:ern:format";
//         let parser: ErnParser = ErnParser::new(ern_str.to_string());
//         let result = parser.parse();
//         assert!(result.is_err());
//         assert_eq!(result.err().unwrap(), ErnError::InvalidFormat);
//         // assert_eq!(result.unwrap_err().to_string(), "Invalid Ern format");
//     }
//
//     #[test]
//     fn test_ern_with_invalid_part() -> anyhow::Result<()> {
//         let ern_str = "ern:domain:category:account:root/invalid:part";
//         let parser: ErnParser = ErnParser::new(ern_str.to_string());
//         let result = parser.parse();
//         assert!(result.is_err());
//         // assert!(result.unwrap_err().to_string().starts_with("Failed to parse Part"));
//         Ok(())
//     }
//
//     #[test]
//     fn test_ern_parsing_with_owned_string() {
//         let ern_str = String::from("ern:custom:service:account123:root/resource");
//         let parser: ErnParser = ErnParser::new(ern_str);
//         let result = parser.parse();
//         assert!(result.is_ok());
//     }
// }
