use std::fmt;
use std::hash::Hash;

use derive_more::{AsRef, From, Into};
use mti::prelude::*;

use crate::errors::ErnError;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Interprets `value` as a content-addressable identifier.
///
/// A value that is already a fully-formed `MagicTypeId` is preserved verbatim; anything
/// else is treated as content and hashed into a deterministic v5 identifier. Preserving
/// formed identifiers keeps parsing and deserialization idempotent, the same way it does
/// for `EntityRoot`.
fn preserve_or_hash(value: &str) -> MagicTypeId {
    value
        .parse::<MagicTypeId>()
        .unwrap_or_else(|_| value.create_type_id::<V5>())
}

/// Represents a content-addressable identifier in an Entity Resource Name (ERN).
///
/// `SHA1Name` uses the UUID v5 algorithm (based on SHA1 hash) to generate
/// deterministic, content-addressable identifiers. Unlike `EntityRoot` which
/// generates different IDs for the same input (incorporating timestamps),
/// `SHA1Name` will always generate the same ID for the same input content.
///
/// This makes `SHA1Name` ideal for:
/// - Content-addressable resources where the same content should have the same identifier
/// - Deterministic resource naming where reproducibility is important
/// - Scenarios where you want to avoid duplicate resources with the same content
#[derive(AsRef, From, Into, Eq, Debug, PartialEq, Clone, Hash, Default, PartialOrd)]
pub struct SHA1Name {
    /// The unique identifier for this entity, generated using the `mti` crate's
    /// `MagicTypeId` type with UUID v5 algorithm.
    name: MagicTypeId,
}

impl SHA1Name {
    /// Returns a reference to the underlying `MagicTypeId`.
    ///
    /// This is useful when you need to access the raw identifier for
    /// comparison or other operations.
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let name1 = SHA1Name::new("document-content".to_string())?;
    /// let name2 = SHA1Name::new("document-content".to_string())?;
    ///
    /// // Same content produces the same ID
    /// assert_eq!(name1.name(), name2.name());
    /// # Ok(())
    /// # }
    /// ```
    pub fn name(&self) -> &MagicTypeId {
        &self.name
    }

    /// Returns the string representation of this identifier.
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let name = SHA1Name::new("document-content".to_string())?;
    /// let id_str = name.as_str();
    ///
    /// // The string will be a deterministic ID based on the content
    /// println!("SHA1 ID: {}", id_str);
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_str(&self) -> &str {
        &self.name
    }

    /// Creates a new `SHA1Name` with the given value.
    ///
    /// This method generates a deterministic, content-addressable identifier using
    /// the UUID v5 algorithm based on SHA1 hash. Unlike `EntityRoot`, the same input
    /// value will always produce the same ID, making it suitable for content-addressable
    /// resources.
    ///
    /// # Arguments
    ///
    /// * `value` - The string value to use for generating the SHA1 hash
    ///
    /// # Validation Rules
    ///
    /// * Value cannot be empty
    /// * Value must be between 1 and 1024 characters
    ///
    /// # Returns
    ///
    /// * `Ok(SHA1Name)` - If validation passes
    /// * `Err(ErnError)` - If validation fails
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let name1 = SHA1Name::new("document-content".to_string())?;
    /// let name2 = SHA1Name::new("document-content".to_string())?;
    ///
    /// // Same content produces the same ID
    /// assert_eq!(name1.to_string(), name2.to_string());
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(value: String) -> Result<Self, ErnError> {
        // Check if empty
        if value.is_empty() {
            return Err(ErnError::ParseFailure(
                "SHA1Name",
                "cannot be empty".to_string(),
            ));
        }

        // Check length
        if value.len() > 1024 {
            return Err(ErnError::ParseFailure(
                "SHA1Name",
                format!(
                    "length exceeds maximum of 1024 characters (got {})",
                    value.len()
                ),
            ));
        }

        Ok(SHA1Name {
            name: preserve_or_hash(&value),
        })
    }
}

impl fmt::Display for SHA1Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = &self.name;
        write!(f, "{id}")
    }
}

/// Implementation of `FromStr` for `SHA1Name` to create an entity from a string.
impl std::str::FromStr for SHA1Name {
    type Err = ErnError;

    /// Creates a `SHA1Name` from a string.
    ///
    /// This method generates a deterministic, content-addressable identifier using
    /// the UUID v5 algorithm based on SHA1 hash. The same input string will always
    /// produce the same ID.
    ///
    /// # Arguments
    ///
    /// * `s` - The string value to use for generating the SHA1 hash
    ///
    /// # Returns
    ///
    /// * `Ok(SHA1Name)` - If validation passes
    /// * `Err(ErnError)` - If validation fails
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # use std::str::FromStr;
    /// # fn example() -> Result<(), ErnError> {
    /// let name1 = SHA1Name::from_str("document-content")?;
    /// let name2 = SHA1Name::new("document-content".to_string())?;
    ///
    /// // FromStr and new() produce the same result for the same input
    /// assert_eq!(name1.to_string(), name2.to_string());
    /// # Ok(())
    /// # }
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check if empty
        if s.is_empty() {
            return Err(ErnError::ParseFailure(
                "SHA1Name",
                "cannot be empty".to_string(),
            ));
        }

        // Check length
        if s.len() > 1024 {
            return Err(ErnError::ParseFailure(
                "SHA1Name",
                format!(
                    "length exceeds maximum of 1024 characters (got {})",
                    s.len()
                ),
            ));
        }

        Ok(SHA1Name {
            name: preserve_or_hash(s),
        })
    }
}

#[cfg(feature = "serde")]
impl Serialize for SHA1Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the MagicTypeId as a string
        serializer.serialize_str(self.name.as_ref())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for SHA1Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize as a string, then create a new SHA1Name
        let s = String::deserialize(deserializer)?;
        SHA1Name::new(s).map_err(serde::de::Error::custom)
    }
}

use crate::traits::ErnComponent;
use crate::{EntityRoot, Part};

impl ErnComponent for SHA1Name {
    fn prefix() -> &'static str {
        ""
    }
    type NextState = Part;

    /// Contributes a deterministic, content-addressable root.
    ///
    /// The v5 identifier is handed to `EntityRoot` already formed, so the builder stores it
    /// verbatim rather than reissuing it as a time-ordered v7 identifier.
    fn build_root(value: &str) -> Option<Result<EntityRoot, ErnError>> {
        Some(
            value
                .parse::<SHA1Name>()
                .map(|name| EntityRoot::from(name.name)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_sha1name_deterministic() {
        // Same input should produce the same SHA1Name
        let name1 = SHA1Name::new("test_content".to_string()).unwrap();
        let name2 = SHA1Name::new("test_content".to_string()).unwrap();

        assert_eq!(name1, name2);
        assert_eq!(name1.to_string(), name2.to_string());
    }

    #[test]
    fn test_sha1name_different_inputs() {
        // Different inputs with more distinct content should produce different SHA1Names
        let name1 = SHA1Name::new("completely_different_content_1".to_string()).unwrap();
        let name2 = SHA1Name::new("entirely_unique_content_2".to_string()).unwrap();

        assert_ne!(name1, name2);
        assert_ne!(name1.to_string(), name2.to_string());
    }

    #[test]
    fn test_sha1name_from_str() {
        // FromStr should produce the same result as new()
        let name1 = SHA1Name::new("test_content".to_string()).unwrap();
        let name2 = SHA1Name::from_str("test_content").unwrap();

        assert_eq!(name1, name2);
        assert_eq!(name1.to_string(), name2.to_string());
    }

    #[test]
    fn test_sha1name_display() {
        let name = SHA1Name::new("test_content".to_string()).unwrap();

        // The string representation should contain the input value
        let display = name.to_string();
        assert!(!display.is_empty());
    }
    #[test]
    fn test_sha1name_validation_empty() {
        let result = SHA1Name::new("".to_string());
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "SHA1Name");
                assert!(msg.contains("empty"));
            }
            _ => panic!("Expected ParseFailure error for empty SHA1Name"),
        }
    }

    #[test]
    fn test_sha1name_validation_too_long() {
        let long_value = "a".repeat(1025);
        let result = SHA1Name::new(long_value);
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "SHA1Name");
                assert!(msg.contains("length exceeds maximum"));
            }
            _ => panic!("Expected ParseFailure error for too long SHA1Name"),
        }
    }

    #[test]
    fn test_sha1name_from_str_validation() {
        let result = SHA1Name::from_str("");
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "SHA1Name");
                assert!(msg.contains("empty"));
            }
            _ => panic!("Expected ParseFailure error for empty SHA1Name from_str"),
        }
    }
}
