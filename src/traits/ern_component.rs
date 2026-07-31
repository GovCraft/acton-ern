use std::str::FromStr;

use crate::errors::ErnError;
use crate::{Account, Category, Domain, EntityRoot, Part, Parts};

/// Represents a component of an Entity Resource Name (ERN).
///
/// This trait is used to ensure type safety and proper ordering when building ERNs.
/// Each component in an ERN implements this trait, defining its prefix and the
/// type of the next component that should follow it in the ERN structure.
///
/// The trait is primarily used by the `ErnBuilder` to enforce the correct order
/// of components during ERN construction.
pub trait ErnComponent {
    /// Returns the prefix string that should appear before this component in an ERN.
    ///
    /// For example, the `Domain` component has the prefix "ern:" to indicate the
    /// start of an ERN string.
    fn prefix() -> &'static str;

    /// The type of the next component that should follow this one in the ERN structure.
    ///
    /// This associated type is used by the builder pattern to enforce the correct
    /// sequence of components. For example, `Domain::NextState` is `Category`,
    /// indicating that a `Category` should follow a `Domain` in an ERN.
    type NextState;

    /// Builds the ERN root component this type contributes, if it can occupy the root slot.
    ///
    /// `ErnBuilder` dispatches on component *position*, which is not enough to tell the
    /// root-capable components apart: `EntityRoot` and `SHA1Name` share the same prefix
    /// and the same `NextState`. This method carries the identifier algorithm down to the
    /// builder, so `with::<SHA1Name>(..)` really does produce a deterministic v5 root
    /// instead of silently falling back to a time-ordered v7 one.
    ///
    /// Components that never occupy the root slot use the default implementation and
    /// return `None`.
    fn build_root(_value: &str) -> Option<Result<EntityRoot, ErnError>> {
        None
    }
}

macro_rules! impl_ern_component {
    ($type:ty, $prefix:expr, $next:ty) => {
        impl ErnComponent for $type {
            fn prefix() -> &'static str {
                $prefix
            }
            type NextState = $next;
        }
    };
}
impl ErnComponent for EntityRoot {
    fn prefix() -> &'static str {
        ""
    }
    type NextState = Part;

    fn build_root(value: &str) -> Option<Result<EntityRoot, ErnError>> {
        Some(EntityRoot::from_str(value))
    }
}

impl ErnComponent for Account {
    fn prefix() -> &'static str {
        ""
    }
    type NextState = EntityRoot;
}

impl_ern_component!(Domain, "ern:", Category);
impl_ern_component!(Category, "", Account);
impl_ern_component!(Part, "", Parts);

/// Implementation for the `Parts` component of an ERN.
///
/// The `Parts` component represents a collection of path parts in the ERN.
/// Its `NextState` is itself, allowing for multiple parts to be added.
impl ErnComponent for Parts {
    fn prefix() -> &'static str {
        ":"
    }
    type NextState = Parts;
}
