use std::str::FromStr;

use crate::EntityRoot;
use crate::errors::ErnError;
use crate::model::{Account, Category, Domain, Ern, Part, Parts};

/// A parser for converting ERN strings into structured `Ern` objects.
///
/// The `ErnParser` takes an ERN string in the format `ern:domain:category:account:root/part1/part2/...`
/// and parses it into its constituent components, performing validation on each part.
pub struct ErnParser {
    /// The ERN string to be parsed.
    ern: String,
}

impl ErnParser {
    /// Creates a new `ErnParser` for the given ERN string.
    ///
    /// # Arguments
    ///
    /// * `ern` - The ERN string to parse, in the format `ern:domain:category:account:root/part1/part2/...`
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// let parser = ErnParser::new("ern:my-app:users:tenant123:profile/settings".to_string());
    /// ```
    pub fn new(ern: String) -> Self {
        Self { ern }
    }

    /// Parses the ERN string into a structured `Ern` object.
    ///
    /// This method validates the ERN format, extracts each component, and ensures
    /// all parts meet the validation requirements.
    ///
    /// # Returns
    ///
    /// * `Ok(Ern)` - A structured Ern object containing all the parsed components
    /// * `Err(ErnError)` - If the ERN string is invalid or any component fails validation
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let parser = ErnParser::new("ern:my-app:users:tenant123:profile/settings".to_string());
    /// let ern = parser.parse()?;
    ///
    /// assert_eq!(ern.domain().as_str(), "my-app");
    /// assert_eq!(ern.category().as_str(), "users");
    /// assert_eq!(ern.account().as_str(), "tenant123");
    /// assert_eq!(ern.parts().to_string(), "settings");
    /// # Ok(())
    /// # }
    /// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ern_parsing() {
        let ern_str = "ern:custom:service:account123:root/resource/subresource".to_string();
        let parser: ErnParser = ErnParser::new(ern_str);
        let result = parser.parse();

        assert!(result.is_ok());
        let ern = result.unwrap();
        assert_eq!(ern.domain().as_str(), "custom");
    }

    #[test]
    fn test_invalid_ern_format() {
        let ern_str = "invalid:ern:format";
        let parser: ErnParser = ErnParser::new(ern_str.to_string());
        let result = parser.parse();
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), ErnError::InvalidFormat);
        // assert_eq!(result.unwrap_err().to_string(), "Invalid Ern format");
    }

    #[test]
    fn test_ern_with_invalid_part() -> anyhow::Result<()> {
        let ern_str = "ern:domain:category:account:root/invalid:part";
        let parser: ErnParser = ErnParser::new(ern_str.to_string());
        let result = parser.parse();
        assert!(result.is_err());
        // assert!(result.unwrap_err().to_string().starts_with("Failed to parse Part"));
        Ok(())
    }

    #[test]
    fn test_parse_round_trips_display_output() -> anyhow::Result<()> {
        let ern = Ern::with_root("pool")?;
        let reparsed = ErnParser::new(ern.to_string()).parse()?;

        assert_eq!(ern, reparsed);
        assert_eq!(ern.to_string(), reparsed.to_string());
        Ok(())
    }

    #[test]
    fn test_parse_round_trips_with_parts() -> anyhow::Result<()> {
        let ern = Ern::with_root("pool")?
            .add_part("worker")?
            .add_part("inbox")?;
        let reparsed = ErnParser::new(ern.to_string()).parse()?;

        assert_eq!(ern, reparsed);
        Ok(())
    }

    #[test]
    fn test_parse_is_idempotent() -> anyhow::Result<()> {
        // Repeated parses previously grew the prefix and minted a new suffix each time.
        let expected = Ern::with_root("pool")?.to_string();
        let mut current = expected.clone();

        for _ in 0..5 {
            current = ErnParser::new(current).parse()?.to_string();
            assert_eq!(current, expected);
        }
        Ok(())
    }

    #[test]
    fn test_parse_mints_root_for_bare_name() -> anyhow::Result<()> {
        // A bare name is not a formed identifier, so it still receives a fresh suffix.
        let ern =
            ErnParser::new("ern:custom:service:account123:root/resource".to_string()).parse()?;

        assert!(ern.root().as_str().starts_with("root_"));
        assert_ne!(ern.root().as_str(), "root");
        Ok(())
    }

    #[test]
    fn test_ern_parsing_with_owned_string() {
        let ern_str = String::from("ern:custom:service:account123:root/resource");
        let parser: ErnParser = ErnParser::new(ern_str);
        let result = parser.parse();
        assert!(result.is_ok());
    }
}
