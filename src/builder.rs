use std::hash::Hash;
use std::str::FromStr;

use crate::EntityRoot;
use crate::errors::ErnError;
use crate::model::{Account, Category, Domain, Ern, Part, Parts};
use crate::traits::ErnComponent;

/// A type-safe builder for constructing ERN instances.
///
/// `ErnBuilder` uses a state-driven approach to ensure that ERN components are added
/// in the correct order and with proper validation. The generic `State` parameter
/// tracks which component should be added next, providing compile-time guarantees
/// that ERNs are constructed correctly.
///
/// # Example
///
/// ```
/// # use acton_ern::prelude::*;
/// # fn example() -> Result<(), ErnError> {
/// let ern = ErnBuilder::new()
///     .with::<Domain>("my-app")?
///     .with::<Category>("users")?
///     .with::<Account>("tenant123")?
///     .with::<EntityRoot>("profile")?
///     .with::<Part>("settings")?
///     .build()?;
/// # Ok(())
/// # }
/// ```
pub struct ErnBuilder<State> {
    builder: PrivateErnBuilder,
    _marker: std::marker::PhantomData<State>,
}

/// Implementation of `ErnBuilder` for the initial state.
impl ErnBuilder<()> {
    /// Creates a new ERN builder to start the construction process.
    ///
    /// This is always the first step when creating an ERN.
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// let builder = ErnBuilder::new();
    /// ```
    pub fn new() -> ErnBuilder<Domain> {
        ErnBuilder {
            builder: PrivateErnBuilder::new(),
            _marker: std::marker::PhantomData,
        }
    }
}

/// Implementation for the `Part` state, allowing finalization of the ERN.
impl ErnBuilder<Part> {
    /// Finalizes the building process and constructs the ERN.
    ///
    /// This method is available after at least one `Part` has been added.
    ///
    /// # Returns
    ///
    /// * `Ok(Ern)` - The fully constructed ERN
    /// * `Err(ErnError)` - If any validation fails
    pub fn build(self) -> Result<Ern, ErnError> {
        self.builder.build()
    }
}

/// Implementation for the `Parts` state, allowing finalization of the ERN.
impl ErnBuilder<Parts> {
    /// Finalizes the building process and constructs the ERN.
    ///
    /// This method is available after multiple `Part`s have been added.
    ///
    /// # Returns
    ///
    /// * `Ok(Ern)` - The fully constructed ERN
    /// * `Err(ErnError)` - If any validation fails
    pub fn build(self) -> Result<Ern, ErnError> {
        self.builder.build()
    }
}

/// Generic implementation for all component states.
impl<Component: ErnComponent + Hash + Clone + PartialEq + Eq> ErnBuilder<Component> {
    /// Adds the next component to the ERN, transitioning to the appropriate state.
    ///
    /// The type parameter `N` determines which component is being added and ensures
    /// components are added in the correct order.
    ///
    /// # Arguments
    ///
    /// * `part` - The string value for this component
    ///
    /// # Returns
    ///
    /// * `Ok(ErnBuilder<NextState>)` - The builder in its next state
    /// * `Err(ErnError)` - If the component value is invalid
    pub fn with<N>(self, part: impl Into<String>) -> Result<ErnBuilder<N::NextState>, ErnError>
    where
        N: ErnComponent<NextState = Component::NextState> + Hash,
    {
        let part = part.into();
        // `N::build_root` carries the requested identifier algorithm down to the builder.
        // Position alone cannot distinguish `EntityRoot` from `SHA1Name`: both share the
        // empty prefix and both declare `NextState = Part`.
        let root = N::build_root(&part);
        Ok(ErnBuilder {
            builder: self.builder.add_part(N::prefix(), part, root)?,
            _marker: std::marker::PhantomData,
        })
    }
}

/// Internal implementation for building ERNs.
struct PrivateErnBuilder {
    domain: Option<Domain>,
    category: Option<Category>,
    account: Option<Account>,
    root: Option<EntityRoot>,
    parts: Parts,
}

impl PrivateErnBuilder {
    /// Constructs a new private ERN (Entity Resource Name) builder.
    fn new() -> Self {
        Self {
            domain: None,
            category: None,
            account: None,
            root: None,
            parts: Parts::new(Vec::new()),
        }
    }

    /// Adds a component to the builder.
    ///
    /// `root` carries the root the calling component would contribute, if it is one of the
    /// root-capable components. It is only consulted in the root slot; components that
    /// never occupy that slot pass `None`.
    fn add_part(
        mut self,
        prefix: &'static str,
        part: String,
        root: Option<Result<EntityRoot, ErnError>>,
    ) -> Result<Self, ErnError> {
        match prefix {
            p if p == Domain::prefix() => {
                self.domain = Some(Domain::new(part)?);
            }
            "" => {
                if self.domain.is_some() && self.category.is_none() {
                    self.category = Some(Category::new(part)?);
                } else if self.category.is_some() && self.account.is_none() {
                    self.account = Some(Account::new(part)?);
                } else if self.account.is_some() && self.root.is_none() {
                    self.root = Some(match root {
                        Some(root) => root?,
                        None => EntityRoot::from_str(part.as_str())?,
                    });
                } else {
                    // add the first part
                    self.parts = self.parts.add_part(Part::new(part)?)?;
                }
            }
            ":" => {
                self.parts = self.parts.add_part(Part::new(part)?)?;
            }
            _ => return Err(ErnError::InvalidPrefix(prefix.to_string())),
        }
        Ok(self)
    }

    /// Finalizes and builds the ERN (Entity Resource Name).
    fn build(self) -> Result<Ern, ErnError> {
        let domain = self
            .domain
            .ok_or(ErnError::MissingPart("domain".to_string()))?;
        let category = self
            .category
            .ok_or(ErnError::MissingPart("category".to_string()))?;
        let account = self
            .account
            .ok_or(ErnError::MissingPart("account".to_string()))?;
        let root = self.root.ok_or(ErnError::MissingPart("root".to_string()))?;

        Ok(Ern::new(domain, category, account, root, self.parts))
    }
}
