use std::fmt;
use std::hash::{Hash, Hasher};

use derive_new::new;

use crate::Part;
use crate::errors::ErnError;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The default maximum number of parts in an ERN path.
///
/// This bounds incremental construction only. [`Parts::new`] and [`ErnParser`] accept a path
/// of any length, so raising the bound with [`Parts::add_part_with_limit`] never produces an
/// ERN that fails to parse back.
///
/// Callers that nest deeper than this - a supervision tree, for instance - should use
/// [`Parts::add_part_with_limit`] or [`Ern::add_part_with_limit`] and choose their own bound.
///
/// [`ErnParser`]: crate::ErnParser
/// [`Ern::add_part_with_limit`]: crate::Ern::add_part_with_limit
pub const DEFAULT_MAX_PARTS: usize = 10;

/// Represents a collection of parts in the ERN (Entity Resource Name), handling multiple segments.
#[derive(new, Debug, PartialEq, Clone, Eq, Default, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Parts(pub(crate) Vec<Part>);

impl Parts {
    /// Adds a part to the collection, bounded by [`DEFAULT_MAX_PARTS`].
    ///
    /// # Arguments
    ///
    /// * `part` - The `Part` to be added to the collection.
    ///
    /// # Returns
    ///
    /// * `Result<Parts, ErnError>` - The updated Parts collection, or an error if the
    ///   collection already holds [`DEFAULT_MAX_PARTS`] parts.
    pub fn add_part<T>(self, part: T) -> Result<Self, ErnError>
    where
        T: Into<Part>,
    {
        self.add_part_with_limit(part, DEFAULT_MAX_PARTS)
    }

    /// Adds a part to the collection, bounded by a caller-chosen maximum.
    ///
    /// Use this when [`DEFAULT_MAX_PARTS`] is the wrong bound for your domain - deep
    /// supervision hierarchies being the motivating case.
    ///
    /// # Arguments
    ///
    /// * `part` - The `Part` to be added to the collection.
    /// * `max_parts` - The maximum number of parts this collection may hold.
    ///
    /// # Returns
    ///
    /// * `Result<Parts, ErnError>` - The updated Parts collection, or an error if the
    ///   collection already holds `max_parts` parts.
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let mut parts = Parts::default();
    /// for i in 0..32 {
    ///     parts = parts.add_part_with_limit(Part::new(format!("level{i}"))?, 64)?;
    /// }
    /// assert_eq!(parts.len(), 32);
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_part_with_limit<T>(mut self, part: T, max_parts: usize) -> Result<Self, ErnError>
    where
        T: Into<Part>,
    {
        // Check if adding another part would exceed the maximum
        if self.0.len() >= max_parts {
            return Err(ErnError::ParseFailure(
                "Parts",
                format!("cannot exceed maximum of {max_parts} parts"),
            ));
        }

        self.0.push(part.into());
        Ok(self)
    }

    /// Converts the Parts into an owned version with 'static lifetime
    pub fn into_owned(self) -> Parts {
        Parts(self.0.into_iter().collect())
    }

    /// Returns the number of parts in the collection.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Hash for Parts {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.len().hash(state);
        for part in &self.0 {
            part.hash(state);
        }
    }
}

impl FromIterator<Part> for Parts {
    fn from_iter<T: IntoIterator<Item = Part>>(iter: T) -> Self {
        Parts(iter.into_iter().collect())
    }
}

impl fmt::Display for Parts {
    /// Formats the collection of parts as a string, joining them with '/'.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|p| p.as_str())
                .collect::<Vec<_>>()
                .join("/")
        )
    }
}

impl IntoIterator for Parts {
    type Item = Part;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Parts {
    type Item = &'a Part;
    type IntoIter = std::slice::Iter<'a, Part>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts_creation() -> anyhow::Result<()> {
        let parts = Parts::new(vec![Part::new("segment1")?, Part::new("segment2")?]);
        assert_eq!(parts.to_string(), "segment1/segment2");
        Ok(())
    }

    #[test]
    fn test_parts_add_part() -> anyhow::Result<()> {
        let mut parts = Parts::new(vec![Part::new("segment1")?]);
        parts = parts.add_part(Part::new("segment2")?)?;
        parts = parts.add_part(Part::new("segment3")?)?;

        assert_eq!(parts.to_string(), "segment1/segment2/segment3");
        Ok(())
    }

    #[test]
    fn test_parts_from_iterator() -> anyhow::Result<()> {
        let parts: Result<Parts, _> = vec!["segment1", "segment2", "segment3"]
            .into_iter()
            .map(Part::new)
            .collect();
        match parts {
            Ok(parts) => {
                assert_eq!(parts.to_string(), "segment1/segment2/segment3");
                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }

    #[test]
    fn test_parts_into_owned() -> anyhow::Result<()> {
        let parts = Parts::new(vec![Part::new("segment1")?, Part::new("segment2")?]);
        let owned_parts: Parts = parts;
        assert_eq!(owned_parts.to_string(), "segment1/segment2");
        Ok(())
    }

    #[test]
    fn test_parts_iterator() -> anyhow::Result<()> {
        let parts = Parts::new(vec![Part::new("segment1")?, Part::new("segment2")?]);
        let collected: Vec<_> = parts.into_iter().collect();
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0].as_str(), "segment1");
        assert_eq!(collected[1].as_str(), "segment2");
        Ok(())
    }

    #[test]
    fn test_parts_ref_iterator() -> anyhow::Result<()> {
        let parts = Parts::new(vec![Part::new("segment1")?, Part::new("segment2")?]);
        let collected: Vec<_> = (&parts).into_iter().map(|p| p.as_str()).collect();
        assert_eq!(collected, vec!["segment1", "segment2"]);
        Ok(())
    }

    #[test]
    fn test_parts_for_loop() -> anyhow::Result<()> {
        let parts = Parts::new(vec![Part::new("segment1")?, Part::new("segment2")?]);
        let mut collected = Vec::new();
        for part in parts {
            collected.push(part.as_str().to_string());
        }
        assert_eq!(
            collected,
            vec!["segment1".to_string(), "segment2".to_string()]
        );
        Ok(())
    }
    #[test]
    fn test_parts_validation_max_parts() -> anyhow::Result<()> {
        // Create a Parts with 10 parts (maximum allowed)
        let mut parts = Parts::new(vec![]);
        for i in 0..10 {
            parts = parts.add_part(Part::new(format!("part{}", i))?)?;
        }

        // Adding an 11th part should fail
        let result = parts.add_part(Part::new("one_too_many")?);
        assert!(result.is_err());

        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "Parts");
                assert!(msg.contains("cannot exceed maximum"));
            }
            _ => panic!("Expected ParseFailure error for too many parts"),
        }

        Ok(())
    }
}
