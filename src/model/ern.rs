use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Add;
use std::sync::Arc;

use crate::errors::ErnError;
use crate::{Account, Category, Domain, EntityRoot, ErnComponent, Part, Parts};

#[derive(Debug, PartialEq, Eq, Hash)]
struct ErnInner {
    domain: Domain,
    category: Category,
    account: Account,
    root: EntityRoot,
    parts: Parts,
}

/// Represents an Entity Resource Name (ERN), which uniquely identifies resources in distributed systems.
///
/// An ERN follows the URN format and has the structure:
/// `ern:domain:category:account:root/path/to/resource`
///
/// Each component serves a specific purpose:
/// - `domain`: Classifies the resource (e.g., internal, external, custom domains)
/// - `category`: Specifies the service or category within the system
/// - `account`: Identifies the owner or account responsible for the resource
/// - `root`: A unique identifier for the root of the resource hierarchy
/// - `parts`: Optional path-like structure showing the resource's position within the hierarchy
///
/// ERNs can be k-sortable when using `UnixTime` or `Timestamp` ID types, enabling
/// efficient ordering and range queries.
///
/// Internally wraps its fields in an `Arc`, making `clone()` a cheap atomic
/// reference-count increment instead of a deep copy.
#[derive(Debug, Clone)]
pub struct Ern {
    inner: Arc<ErnInner>,
}

impl Ern {
    /// Creates a new ERN with the specified components.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain component
    /// * `category` - The category component
    /// * `account` - The account component
    /// * `root` - The root component
    /// * `parts` - The path parts component
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let ern = Ern::new(
    ///     Domain::new("my-app")?,
    ///     Category::new("users")?,
    ///     Account::new("tenant123")?,
    ///     EntityRoot::new("profile".to_string())?,
    ///     Parts::new(vec![Part::new("settings")?]),
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        domain: Domain,
        category: Category,
        account: Account,
        root: EntityRoot,
        parts: Parts,
    ) -> Self {
        Ern {
            inner: Arc::new(ErnInner {
                domain,
                category,
                account,
                root,
                parts,
            }),
        }
    }

    fn from_inner(inner: ErnInner) -> Self {
        Ern {
            inner: Arc::new(inner),
        }
    }

    /// Returns a reference to the domain component.
    pub fn domain(&self) -> &Domain {
        &self.inner.domain
    }

    /// Returns a reference to the category component.
    pub fn category(&self) -> &Category {
        &self.inner.category
    }

    /// Returns a reference to the account component.
    pub fn account(&self) -> &Account {
        &self.inner.account
    }

    /// Returns a reference to the root component.
    pub fn root(&self) -> &EntityRoot {
        &self.inner.root
    }

    /// Returns a reference to the parts component.
    pub fn parts(&self) -> &Parts {
        &self.inner.parts
    }

    /// Creates a new ERN with the given root and default values for other components.
    ///
    /// This is a convenient way to create an ERN when you only care about the root component.
    ///
    /// # Arguments
    ///
    /// * `root` - The string value for the root component
    ///
    /// # Returns
    ///
    /// * `Ok(Ern)` - The created ERN with default values for domain, category, account, and parts
    /// * `Err(ErnError)` - If the root value is invalid
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let ern = Ern::with_root("profile")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_root(root: impl Into<String>) -> Result<Self, ErnError> {
        let root = EntityRoot::new(root.into())?;
        Ok(Ern::from_inner(ErnInner {
            root,
            domain: Domain::default(),
            category: Category::default(),
            account: Account::default(),
            parts: Parts::new(Vec::default()),
        }))
    }

    /// Creates a new ERN based on an existing ERN but with a different root.
    ///
    /// This method preserves all other components (domain, category, account, parts)
    /// but replaces the root with a new value.
    ///
    /// # Arguments
    ///
    /// * `new_root` - The string value for the new root component
    ///
    /// # Returns
    ///
    /// * `Ok(Ern)` - A new ERN with the updated root
    /// * `Err(ErnError)` - If the new root value is invalid
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let ern1 = Ern::with_root("profile")?;
    /// let ern2 = ern1.with_new_root("settings")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_new_root(&self, new_root: impl Into<String>) -> Result<Self, ErnError> {
        let new_root = EntityRoot::new(new_root.into())?;
        Ok(Ern::from_inner(ErnInner {
            domain: self.inner.domain.clone(),
            category: self.inner.category.clone(),
            account: self.inner.account.clone(),
            root: new_root,
            parts: self.inner.parts.clone(),
        }))
    }

    pub fn with_domain(domain: impl Into<String>) -> Result<Self, ErnError> {
        let domain = Domain::new(domain)?;
        Ok(Ern::from_inner(ErnInner {
            domain,
            category: Category::default(),
            account: Account::default(),
            root: EntityRoot::default(),
            parts: Parts::default(),
        }))
    }

    pub fn with_category(category: impl Into<String>) -> Result<Self, ErnError> {
        let category = Category::new(category)?;
        Ok(Ern::from_inner(ErnInner {
            domain: Domain::default(),
            category,
            account: Account::default(),
            root: EntityRoot::default(),
            parts: Parts::default(),
        }))
    }

    pub fn with_account(account: impl Into<String>) -> Result<Self, ErnError> {
        let account = Account::new(account)?;
        Ok(Ern::from_inner(ErnInner {
            domain: Domain::default(),
            category: Category::default(),
            account,
            root: EntityRoot::default(),
            parts: Parts::default(),
        }))
    }

    /// Adds a new part to the ERN's path.
    ///
    /// This method creates a new ERN with the same domain, category, account, and root,
    /// but with an additional part appended to the path.
    ///
    /// # Arguments
    ///
    /// * `part` - The string value for the new part
    ///
    /// # Returns
    ///
    /// * `Ok(Ern)` - A new ERN with the added part
    /// * `Err(ErnError)` - If the part value is invalid or adding it would exceed the maximum of 10 parts
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let ern1 = Ern::with_root("profile")?;
    /// let ern2 = ern1.add_part("settings")?;
    /// let ern3 = ern2.add_part("appearance")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_part(&self, part: impl Into<String>) -> Result<Self, ErnError> {
        let new_part = Part::new(part)?;
        let mut new_parts = self.inner.parts.clone();

        // Check if adding another part would exceed the maximum
        if new_parts.0.len() >= 10 {
            return Err(ErnError::ParseFailure(
                "Parts",
                "cannot exceed maximum of 10 parts".to_string(),
            ));
        }

        new_parts.0.push(new_part);
        Ok(Ern::from_inner(ErnInner {
            domain: self.inner.domain.clone(),
            category: self.inner.category.clone(),
            account: self.inner.account.clone(),
            root: self.inner.root.clone(),
            parts: new_parts,
        }))
    }

    pub fn with_parts(
        &self,
        parts: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, ErnError> {
        let new_parts: Result<Vec<Part>, _> = parts.into_iter().map(Part::new).collect();
        Ok(Ern::from_inner(ErnInner {
            domain: self.inner.domain.clone(),
            category: self.inner.category.clone(),
            account: self.inner.account.clone(),
            root: self.inner.root.clone(),
            parts: Parts(new_parts?),
        }))
    }

    /// Checks if this ERN is a child of another ERN.
    ///
    /// An ERN is considered a child of another ERN if:
    /// 1. They have the same domain, category, account, and root
    /// 2. The child's parts start with all of the parent's parts
    /// 3. The child has more parts than the parent
    ///
    /// # Arguments
    ///
    /// * `other` - The potential parent ERN
    ///
    /// # Returns
    ///
    /// * `true` - If this ERN is a child of the other ERN
    /// * `false` - Otherwise
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let parent = Ern::with_root("profile")?.add_part("settings")?;
    /// let child = parent.add_part("appearance")?;
    ///
    /// assert!(child.is_child_of(&parent));
    /// assert!(!parent.is_child_of(&child));
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_child_of(&self, other: &Ern) -> bool {
        self.inner.domain == other.inner.domain
            && self.inner.category == other.inner.category
            && self.inner.account == other.inner.account
            && self.inner.root == other.inner.root
            && other.inner.parts.0.len() < self.inner.parts.0.len()
            && self.inner.parts.0.starts_with(&other.inner.parts.0)
    }

    /// Returns the parent ERN of this ERN, if it exists.
    ///
    /// The parent ERN has the same domain, category, account, and root,
    /// but with one fewer part in the path. If this ERN has no parts,
    /// it has no parent and this method returns `None`.
    ///
    /// # Returns
    ///
    /// * `Some(Ern)` - The parent ERN
    /// * `None` - If this ERN has no parts (and thus no parent)
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let ern1 = Ern::with_root("profile")?;
    /// let ern2 = ern1.add_part("settings")?;
    /// let ern3 = ern2.add_part("appearance")?;
    ///
    /// assert_eq!(ern3.parent().unwrap().to_string(), ern2.to_string());
    /// assert_eq!(ern2.parent().unwrap().to_string(), ern1.to_string());
    /// assert!(ern1.parent().is_none());
    /// # Ok(())
    /// # }
    /// ```
    pub fn parent(&self) -> Option<Self> {
        if self.inner.parts.0.is_empty() {
            None
        } else {
            Some(Ern::from_inner(ErnInner {
                domain: self.inner.domain.clone(),
                category: self.inner.category.clone(),
                account: self.inner.account.clone(),
                root: self.inner.root.clone(),
                parts: Parts(self.inner.parts.0[..self.inner.parts.0.len() - 1].to_vec()),
            }))
        }
    }
}

impl PartialEq for Ern {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner) || self.inner == other.inner
    }
}

impl Eq for Ern {}

impl Hash for Ern {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl Ord for Ern {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.root.name().cmp(other.inner.root.name())
    }
}

impl PartialOrd for Ern {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Ern {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut display = format!(
            "{}{}:{}:{}:{}",
            Domain::prefix(),
            self.inner.domain,
            self.inner.category,
            self.inner.account,
            self.inner.root
        );
        if !self.inner.parts.0.is_empty() {
            display = format!("{}/{}", display, self.inner.parts);
        }
        write!(f, "{}", display)
    }
}

/// Composes two ERNs by nesting the right-hand side beneath the left.
///
/// The result keeps the left-hand side's domain, category, account, and root, then extends
/// its path with the right-hand side's root followed by the right-hand side's own parts.
/// Folding the right-hand root into the path is what makes `parent + child` carry the
/// child's name; dropping it would discard half the input.
///
/// This operation is fallible, so `Output` is a `Result`:
///
/// * The right-hand root must form a valid `Part`. A root is at most 63 characters plus a
///   26-character suffix, which can exceed the 63-character limit on a part.
/// * The combined path must stay within the 10-part maximum.
///
/// # Example
///
/// ```
/// # use acton_ern::prelude::*;
/// # fn example() -> Result<(), ErnError> {
/// let parent = Ern::with_root("pool")?;
/// let child = (parent.clone() + Ern::with_root("worker")?)?;
///
/// assert!(child.to_string().contains("worker"));
/// assert!(child.is_child_of(&parent));
/// # Ok(())
/// # }
/// ```
impl Add for Ern {
    type Output = Result<Ern, ErnError>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_parts = self.inner.parts.clone();
        new_parts = new_parts.add_part(Part::new(rhs.inner.root.as_str())?)?;
        for part in &rhs.inner.parts.0 {
            new_parts = new_parts.add_part(part.clone())?;
        }

        Ok(Ern::from_inner(ErnInner {
            domain: self.inner.domain.clone(),
            category: self.inner.category.clone(),
            account: self.inner.account.clone(),
            root: self.inner.root.clone(),
            parts: new_parts,
        }))
    }
}

impl Default for Ern {
    /// Provides a default ERN using the default values of all its components.
    ///
    /// This is primarily used internally and for testing. For creating ERNs in
    /// application code, prefer using `ErnBuilder` or the `with_root` method.
    fn default() -> Self {
        Ern::new(
            Domain::default(),
            Category::default(),
            Account::default(),
            EntityRoot::default(),
            Parts::new(Vec::default()),
        )
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Ern {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Ern", 5)?;
        state.serialize_field("domain", &self.inner.domain)?;
        state.serialize_field("category", &self.inner.category)?;
        state.serialize_field("account", &self.inner.account)?;
        state.serialize_field("root", &self.inner.root)?;
        state.serialize_field("parts", &self.inner.parts)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Ern {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct ErnFields {
            domain: Domain,
            category: Category,
            account: Account,
            root: EntityRoot,
            parts: Parts,
        }
        let fields = ErnFields::deserialize(deserializer)?;
        Ok(Ern::new(
            fields.domain,
            fields.category,
            fields.account,
            fields.root,
            fields.parts,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::thread::sleep;
    use std::time::Duration;

    use crate::Part;

    use super::*;

    #[test]
    fn test_ern_timestamp_ordering() {
        let ern1: Ern = Ern::with_root("root_a").unwrap();
        sleep(Duration::from_millis(10));
        let ern2: Ern = Ern::with_root("root_b").unwrap();
        sleep(Duration::from_millis(10));
        let ern3: Ern = Ern::with_root("root_c").unwrap();

        // Test ascending order
        assert!(ern1 < ern2);
        assert!(ern2 < ern3);
        assert!(ern1 < ern3);

        // Test descending order
        assert!(ern3 > ern2);
        assert!(ern2 > ern1);
        assert!(ern3 > ern1);

        // Test equality
        let ern1_clone = ern1.clone();
        assert_eq!(ern1, ern1_clone);

        // Test sorting
        let mut erns = vec![ern3.clone(), ern1.clone(), ern2.clone()];
        erns.sort();
        assert_eq!(erns, vec![ern1, ern2, ern3]);
    }

    #[test]
    fn test_ern_unixtime_ordering() {
        let ern1: Ern = Ern::with_root("root_a").unwrap();
        sleep(Duration::from_millis(10));
        let ern2: Ern = Ern::with_root("root_b").unwrap();
        sleep(Duration::from_millis(10));
        let ern3: Ern = Ern::with_root("root_c").unwrap();

        // Test ascending order
        assert!(ern1 < ern2);
        assert!(ern2 < ern3);
        assert!(ern1 < ern3);

        // Test descending order
        assert!(ern3 > ern2);
        assert!(ern2 > ern1);
        assert!(ern3 > ern1);

        // Test equality
        let ern1_clone = ern1.clone();
        assert_eq!(ern1, ern1_clone);

        // Test sorting
        let mut erns = vec![ern3.clone(), ern1.clone(), ern2.clone()];
        erns.sort();
        assert_eq!(erns, vec![ern1, ern2, ern3]);
    }

    #[test]
    fn test_ern_timestamp_unixtime_consistency() {
        let ern_timestamp1: Ern = Ern::with_root("root_a").unwrap();
        let ern_unixtime1: Ern = Ern::with_root("root_a").unwrap();

        sleep(Duration::from_millis(10));

        let ern_timestamp2: Ern = Ern::with_root("root_b").unwrap();
        let ern_unixtime2: Ern = Ern::with_root("root_b").unwrap();

        // Ensure that the ordering is consistent between Timestamp and UnixTime
        assert_eq!(
            ern_timestamp1 < ern_timestamp2,
            ern_unixtime1 < ern_unixtime2
        );
    }
    #[test]
    fn test_ern_with_root() {
        let ern: Ern = Ern::with_root("custom_root").unwrap();
        assert!(ern.root().as_str().starts_with("custom_root"));
        assert_eq!(*ern.domain(), Domain::default());
        assert_eq!(*ern.category(), Category::default());
        assert_eq!(*ern.account(), Account::default());
        assert_eq!(*ern.parts(), Parts::default());
    }

    #[test]
    fn test_ern_with_new_root() {
        let original_ern: Ern = Ern::default();
        let new_ern: Ern = original_ern.with_new_root("new_root").unwrap();
        assert!(new_ern.root().as_str().starts_with("new_root"));
        assert_eq!(new_ern.domain(), original_ern.domain());
        assert_eq!(new_ern.category(), original_ern.category());
        assert_eq!(new_ern.account(), original_ern.account());
        assert_eq!(new_ern.parts(), original_ern.parts());
    }

    /// Builds an ERN with fixed non-root components so `Add` tests differ only in root and parts.
    fn ern_with(root: &str, parts: &[&str]) -> anyhow::Result<Ern> {
        Ok(Ern::new(
            Domain::from_str("acton-internal")?,
            Category::from_str("hr")?,
            Account::from_str("company123")?,
            EntityRoot::from_str(root)?,
            parts
                .iter()
                .map(|p| Part::from_str(p))
                .collect::<Result<Parts, _>>()?,
        ))
    }

    #[test]
    fn test_add_erns() -> anyhow::Result<()> {
        let parent = ern_with("root_a", &["department_a", "team1"])?;
        let child = ern_with("root_b", &["role_x"])?;
        let child_root = child.root().as_str().to_string();

        let combined: Ern = (parent.clone() + child)?;

        assert_eq!(combined.domain(), parent.domain());
        assert_eq!(combined.category(), parent.category());
        assert_eq!(combined.account(), parent.account());
        assert_eq!(combined.root(), parent.root());
        assert_eq!(
            *combined.parts(),
            Parts(vec![
                Part::from_str("department_a")?,
                Part::from_str("team1")?,
                Part::new(child_root)?,
                Part::from_str("role_x")?,
            ])
        );
        Ok(())
    }

    #[test]
    fn test_add_erns_preserves_child_name() -> anyhow::Result<()> {
        // The right-hand root must survive the operation; dropping it discarded half the input.
        let parent = Ern::with_root("pool")?;
        let child = (parent.clone() + Ern::with_root("worker")?)?;

        assert!(child.to_string().contains("worker"));
        assert!(child.is_child_of(&parent));
        assert_ne!(child, parent);
        Ok(())
    }

    #[test]
    fn test_add_erns_empty_child() -> anyhow::Result<()> {
        let parent = ern_with("rootp", &["department_a"])?;
        let child = ern_with("rootc", &[])?;
        let child_root = child.root().as_str().to_string();

        let combined = (parent + child)?;

        assert_eq!(
            *combined.parts(),
            Parts(vec![
                Part::from_str("department_a")?,
                Part::new(child_root)?
            ])
        );
        Ok(())
    }

    #[test]
    fn test_add_erns_empty_parent() -> anyhow::Result<()> {
        let parent = ern_with("rootp", &[])?;
        let child = ern_with("rootc", &["role_x"])?;
        let child_root = child.root().as_str().to_string();

        let combined = (parent + child)?;

        assert_eq!(
            *combined.parts(),
            Parts(vec![Part::new(child_root)?, Part::from_str("role_x")?])
        );
        Ok(())
    }

    #[test]
    fn test_add_erns_display() -> anyhow::Result<()> {
        let parent = ern_with("rootp", &["department_a"])?;
        let child = ern_with("rootc", &["team1"])?;

        let combined = (parent + child)?;

        assert!(
            combined
                .to_string()
                .starts_with("ern:acton-internal:hr:company123:rootp")
        );
        assert!(combined.to_string().ends_with("/team1"));
        Ok(())
    }

    #[test]
    fn test_add_erns_rejects_empty_root() -> anyhow::Result<()> {
        // A default root carries no name, so there is nothing valid to fold into the path.
        let parent = ern_with("rootp", &[])?;
        let result = parent + Ern::default();

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_add_erns_rejects_overlong_path() -> anyhow::Result<()> {
        let parts: Vec<String> = (0..10).map(|i| format!("part{i}")).collect();
        let parent = ern_with(
            "rootp",
            &parts.iter().map(String::as_str).collect::<Vec<_>>(),
        )?;

        let result = parent + ern_with("rootc", &[])?;

        assert!(result.is_err());
        Ok(())
    }
    #[test]
    fn test_ern_custom() -> anyhow::Result<()> {
        let ern: Ern = Ern::new(
            Domain::new("custom")?,
            Category::new("service")?,
            Account::new("account123")?,
            EntityRoot::new("root".to_string())?,
            Parts::new(vec![Part::new("resource")?]),
        );
        assert!(
            ern.to_string()
                .starts_with("ern:custom:service:account123:root")
        );
        Ok(())
    }

    #[test]
    fn test_ern_append_invalid_part() -> anyhow::Result<()> {
        let invalid_part = Part::new(":invalid");

        assert!(invalid_part.is_err());
        Ok(())
    }
}
