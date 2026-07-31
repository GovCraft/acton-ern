use std::fmt;
use std::hash::Hash;

use derive_more::{AsRef, From, Into};
use mti::prelude::*;

use crate::errors::ErnError;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Interprets `value` as a root identifier.
///
/// A value that is already a fully-formed `MagicTypeId` (such as the
/// `pool_01kytrwjv4eb1rt080sp4txr5t` found in an ERN's `Display` output) is preserved
/// verbatim. Anything else is treated as a bare name and receives a freshly minted,
/// time-ordered v7 suffix.
///
/// Preserving formed identifiers is what makes parsing idempotent: without it, every
/// parse folds the previous suffix into the prefix and mints another one, so repeated
/// parse cycles accumulate garbage and an ERN never round-trips through its own
/// `Display` output.
fn preserve_or_mint(value: &str) -> MagicTypeId {
    value
        .parse::<MagicTypeId>()
        .unwrap_or_else(|_| value.create_type_id::<V7>())
}

/// Represents the root component in an Entity Resource Name (ERN).
///
/// The root component is a unique identifier for the base resource in the ERN hierarchy.
/// It uses the `mti` crate's `MagicTypeId` with UUID v7 algorithm to generate
/// time-ordered, unique identifiers that enable k-sortability.
///
/// When using `EntityRoot`, each call to create a new root from a bare *name* will
/// generate a different ID, as it incorporates the current timestamp. This makes
/// `EntityRoot` suitable for resources that should be ordered by creation time.
/// A value that is already a fully-formed identifier (for example one taken from an
/// existing ERN) is preserved as-is rather than reissued, so ERNs round-trip through
/// parsing and serialization unchanged.
///
/// For content-addressable, deterministic IDs, use `SHA1Name` instead.
#[derive(AsRef, From, Into, Eq, Debug, PartialEq, Clone, Hash, Default, PartialOrd)]
pub struct EntityRoot {
    /// The unique identifier for this root entity, generated using the `mti` crate's
    /// `MagicTypeId` type.
    name: MagicTypeId,
}

impl EntityRoot {
    /// Returns a reference to the underlying `MagicTypeId`.
    ///
    /// This is useful when you need to access the raw identifier for
    /// comparison or sorting operations.
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let root1 = EntityRoot::new("resource1".to_string())?;
    /// let root2 = EntityRoot::new("resource2".to_string())?;
    ///
    /// // Compare roots by their MagicTypeId
    /// let comparison = root1.name().cmp(root2.name());
    /// # Ok(())
    /// # }
    /// ```
    pub fn name(&self) -> &MagicTypeId {
        &self.name
    }

    /// Returns the string representation of this root's identifier.
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let root = EntityRoot::new("profile".to_string())?;
    /// let id_str = root.as_str();
    ///
    /// // The string will contain the original name followed by a timestamp-based suffix
    /// assert!(id_str.starts_with("profile_"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_str(&self) -> &str {
        &self.name
    }

    /// Creates a new `EntityRoot` with the given value.
    ///
    /// When `value` is a bare name, this method generates a time-ordered, unique identifier
    /// using the UUID v7 algorithm. Each call with the same name will generate a different ID,
    /// as it incorporates the current timestamp. This makes `EntityRoot` suitable for
    /// resources that should be ordered by creation time.
    ///
    /// When `value` is already a fully-formed identifier, it is preserved verbatim so that
    /// existing roots survive a parse or deserialization round trip.
    ///
    /// # Arguments
    ///
    /// * `value` - The string value to use as the base for the entity root ID
    ///
    /// # Validation Rules
    ///
    /// * Value cannot be empty
    /// * Value must be between 1 and 255 characters
    ///
    /// # Returns
    ///
    /// * `Ok(EntityRoot)` - If validation passes
    /// * `Err(ErnError)` - If validation fails
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let root = EntityRoot::new("profile".to_string())?;
    ///
    /// // The ID will contain the original name followed by a timestamp-based suffix
    /// assert!(root.to_string().starts_with("profile_"));
    ///
    /// // Re-creating from a formed identifier preserves it
    /// let same = EntityRoot::new(root.to_string())?;
    /// assert_eq!(root, same);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(value: String) -> Result<Self, ErnError> {
        // Check if empty
        if value.is_empty() {
            return Err(ErnError::ParseFailure(
                "EntityRoot",
                "cannot be empty".to_string(),
            ));
        }

        // Check length
        if value.len() > 255 {
            return Err(ErnError::ParseFailure(
                "EntityRoot",
                format!(
                    "length exceeds maximum of 255 characters (got {})",
                    value.len()
                ),
            ));
        }

        Ok(EntityRoot {
            name: preserve_or_mint(&value),
        })
    }
}

impl fmt::Display for EntityRoot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = &self.name;
        write!(f, "{id}")
    }
}

/// Implementation of `FromStr` for `EntityRoot` to create an entity root from a string.
impl std::str::FromStr for EntityRoot {
    type Err = ErnError;

    /// Creates an `EntityRoot` from a string.
    ///
    /// A bare name receives a freshly minted, time-ordered v7 identifier, so each call with
    /// the same name yields a different ID. A string that is already a fully-formed
    /// identifier is preserved verbatim, which is what allows `ErnParser` to round-trip an
    /// ERN's own `Display` output.
    ///
    /// # Arguments
    ///
    /// * `s` - The string value to use as the base for the entity root ID
    ///
    /// # Returns
    ///
    /// * `Ok(EntityRoot)` - If validation passes
    /// * `Err(ErnError)` - If validation fails
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check if empty
        if s.is_empty() {
            return Err(ErnError::ParseFailure(
                "EntityRoot",
                "cannot be empty".to_string(),
            ));
        }

        // Check length
        if s.len() > 255 {
            return Err(ErnError::ParseFailure(
                "EntityRoot",
                format!("length exceeds maximum of 255 characters (got {})", s.len()),
            ));
        }

        Ok(EntityRoot {
            name: preserve_or_mint(s),
        })
    }
}

#[cfg(feature = "serde")]
impl Serialize for EntityRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the MagicTypeId as a string
        serializer.serialize_str(self.name.as_ref())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for EntityRoot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize as a string, then create a new EntityRoot
        let s = String::deserialize(deserializer)?;
        EntityRoot::new(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_entity_root_creation() -> anyhow::Result<()> {
        let root = EntityRoot::new("test-entity".to_string())?;
        assert!(!root.to_string().is_empty());
        Ok(())
    }

    #[test]
    fn test_entity_root_uniqueness() -> anyhow::Result<()> {
        // EntityRoot should generate different IDs for the same input (non-deterministic)
        let root1 = EntityRoot::new("same-content".to_string())?;
        let root2 = EntityRoot::new("same-content".to_string())?;

        // The string representations should be different
        assert_ne!(root1.to_string(), root2.to_string());
        Ok(())
    }

    #[test]
    fn test_entity_root_from_str() -> anyhow::Result<()> {
        let root = EntityRoot::from_str("test-entity")?;
        assert!(!root.to_string().is_empty());
        Ok(())
    }

    #[test]
    fn test_entity_root_validation_empty() {
        let result = EntityRoot::new("".to_string());
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "EntityRoot");
                assert!(msg.contains("empty"));
            }
            _ => panic!("Expected ParseFailure error for empty EntityRoot"),
        }
    }

    #[test]
    fn test_entity_root_validation_too_long() {
        let long_value = "a".repeat(256);
        let result = EntityRoot::new(long_value);
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "EntityRoot");
                assert!(msg.contains("length exceeds maximum"));
            }
            _ => panic!("Expected ParseFailure error for too long EntityRoot"),
        }
    }

    #[test]
    fn test_entity_root_preserves_formed_identifier() -> anyhow::Result<()> {
        // Re-reading a root's own string representation must reproduce it exactly,
        // rather than folding the suffix into the prefix and minting a new one.
        let root = EntityRoot::new("pool".to_string())?;
        let reread = EntityRoot::from_str(root.as_str())?;

        assert_eq!(root, reread);
        assert_eq!(root.to_string(), reread.to_string());
        Ok(())
    }

    #[test]
    fn test_entity_root_parsing_is_idempotent() -> anyhow::Result<()> {
        // Repeated round trips must converge, not accumulate prefix garbage.
        let mut root = EntityRoot::new("pool".to_string())?;
        let expected = root.to_string();

        for _ in 0..5 {
            root = EntityRoot::from_str(root.as_str())?;
            assert_eq!(root.to_string(), expected);
        }
        Ok(())
    }

    #[test]
    fn test_entity_root_bare_name_still_mints() -> anyhow::Result<()> {
        // A name that merely contains an underscore is not a formed identifier.
        let root1 = EntityRoot::from_str("root_a")?;
        let root2 = EntityRoot::from_str("root_a")?;

        assert_ne!(root1, root2);
        assert!(root1.as_str().starts_with("root_a_"));
        Ok(())
    }

    #[test]
    fn test_entity_root_from_str_validation() {
        let result = EntityRoot::from_str("");
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "EntityRoot");
                assert!(msg.contains("empty"));
            }
            _ => panic!("Expected ParseFailure error for empty EntityRoot from_str"),
        }
    }
}
