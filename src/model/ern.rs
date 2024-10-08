use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::ops::Add;

use crate::{Account, Category, Domain, EntityRoot, ErnComponent, Part, Parts};
use crate::errors::ErnError;

/// Represents an ERN (Entity Resource Name), which uniquely identifies resources within the Acton framework.
#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd)]
pub struct Ern {
    pub domain: Domain,
    pub category: Category,
    pub account: Account,
    pub root: EntityRoot,
    pub parts: Parts,
}

impl Ord for Ern {
    fn cmp(&self, other: &Self) -> Ordering {
        self.root.name().cmp(&other.root.name())
    }
}

impl Display for Ern {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut display = format!(
            "{}{}:{}:{}:{}",
            Domain::prefix(),
            self.domain,
            self.category,
            self.account,
            self.root
        );
        if !self.parts.0.is_empty() {
            display = format!("{}/{}", display, self.parts);
        }
        write!(f, "{}", display)
    }
}

impl Add for Ern {
    type Output = Ern;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_parts = self.parts.0;
        new_parts.extend(rhs.parts.0);
        Ern {
            domain: self.domain,
            category: self.category,
            account: self.account,
            root: self.root,
            parts: Parts(new_parts),
        }
    }
}

impl Ern {
    /// Creates a new ERN (Entity Resource Name) with the given components.
    pub fn new(
        domain: Domain,
        category: Category,
        account: Account,
        root: EntityRoot,
        parts: Parts,
    ) -> Self {
        Ern {
            domain,
            category,
            account,
            root,
            parts,
        }
    }

        /// Creates a new ERN (Entity Resource Name) with the given root and default values for other fields
        pub fn with_root(root: impl Into<String>) -> Result<Self, ErnError> {
            let root = EntityRoot::new(root.into())?;
            Ok(Ern {
                root,
                ..Default::default()
            })
        }

        /// Creates a new ERN (Entity Resource Name) based on an existing ERN (Entity Resource Name) but with a new root
        pub fn with_new_root(&self, new_root: impl Into<String>) -> Result<Self, ErnError> {
            let new_root = EntityRoot::new(new_root.into())?;
            Ok(Ern {
                domain: self.domain.clone(),
                category: self.category.clone(),
                account: self.account.clone(),
                root: new_root,
                parts: self.parts.clone(),
            })
        }

        pub fn with_domain(domain: impl Into<String>) -> Result<Self, ErnError> {
            let domain = Domain::new(domain)?;
            Ok(Ern {
                domain,
                category: Category::default(),
                account: Account::default(),
                root: EntityRoot::default(),
                parts: Parts::default(),
            })
        }

        pub fn with_category(category: impl Into<String>) -> Result<Self, ErnError> {
            let category = Category::new(category);
            Ok(Ern {
                domain: Domain::default(),
                category,
                account: Account::default(),
                root: EntityRoot::default(),
                parts: Parts::default(),
            })
        }

        pub fn with_account(account: impl Into<String>) -> Result<Self, ErnError> {
            let account = Account::new(account);
            Ok(Ern {
                domain: Domain::default(),
                category: Category::default(),
                account,
                root: EntityRoot::default(),
                parts: Parts::default(),
            })
        }

        pub fn add_part(&self, part: impl Into<String>) -> Result<Self, ErnError> {
            let mut new_parts = self.parts.clone();
            new_parts.0.push(Part::new(part)?);
            Ok(Ern {
                domain: self.domain.clone(),
                category: self.category.clone(),
                account: self.account.clone(),
                root: self.root.clone(),
                parts: new_parts,
            })
        }

        pub fn with_parts(
            &self,
            parts: impl IntoIterator<Item = impl Into<String>>,
        ) -> Result<Self, ErnError> {
            let new_parts: Result<Vec<Part>, _> = parts.into_iter().map(Part::new).collect();
            Ok(Ern {
                domain: self.domain.clone(),
                category: self.category.clone(),
                account: self.account.clone(),
                root: self.root.clone(),
                parts: Parts(new_parts?),
            })
        }

        pub fn is_child_of(&self, other: &Ern) -> bool {
            self.domain == other.domain
                && self.category == other.category
                && self.account == other.account
                && self.root == other.root
                && other.parts.0.len() < self.parts.0.len()
                && self.parts.0.starts_with(&other.parts.0)
        }

        pub fn parent(&self) -> Option<Self> {
            if self.parts.0.is_empty() {
                None
            } else {
                Some(Ern {
                    domain: self.domain.clone(),
                    category: self.category.clone(),
                    account: self.account.clone(),
                    root: self.root.clone(),
                    parts: Parts(self.parts.0[..self.parts.0.len() - 1].to_vec()),
                })
            }
        }
}

impl Default for Ern {
    /// Provides a default value for ERN (Entity Resource Name) using the defaults of all its components.
    fn default() -> Self {
        Ern {
            domain: Domain::default(),
            category: Category::default(),
            account: Account::default(),
            root: EntityRoot::default(),
            parts: Parts::new(Vec::default()),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;
//     use std::thread::sleep;
//     use std::time::Duration;
//
//     use crate::{Part, UnixTime};
//
//     use super::*;
//
//     #[test]
//     fn test_ern_timestamp_ordering() {
//         let ern1: Ern = Ern::with_root("root_a").unwrap();
//         sleep(Duration::from_millis(10));
//         let ern2: Ern = Ern::with_root("root_b").unwrap();
//         sleep(Duration::from_millis(10));
//         let ern3: Ern = Ern::with_root("root_c").unwrap();
//
//         // Test ascending order
//         assert!(ern1 < ern2);
//         assert!(ern2 < ern3);
//         assert!(ern1 < ern3);
//
//         // Test descending order
//         assert!(ern3 > ern2);
//         assert!(ern2 > ern1);
//         assert!(ern3 > ern1);
//
//         // Test equality
//         let ern1_clone = ern1.clone();
//         assert_eq!(ern1, ern1_clone);
//
//         // Test sorting
//         let mut erns = vec![ern3.clone(), ern1.clone(), ern2.clone()];
//         erns.sort();
//         assert_eq!(erns, vec![ern1, ern2, ern3]);
//     }
//
//     #[test]
//     fn test_ern_unixtime_ordering() {
//         let ern1: Ern = Ern::with_root("root_a").unwrap();
//         sleep(Duration::from_millis(10));
//         let ern2: Ern = Ern::with_root("root_b").unwrap();
//         sleep(Duration::from_millis(10));
//         let ern3: Ern = Ern::with_root("root_c").unwrap();
//
//         // Test ascending order
//         assert!(ern1 < ern2);
//         assert!(ern2 < ern3);
//         assert!(ern1 < ern3);
//
//         // Test descending order
//         assert!(ern3 > ern2);
//         assert!(ern2 > ern1);
//         assert!(ern3 > ern1);
//
//         // Test equality
//         let ern1_clone = ern1.clone();
//         assert_eq!(ern1, ern1_clone);
//
//         // Test sorting
//         let mut erns = vec![ern3.clone(), ern1.clone(), ern2.clone()];
//         erns.sort();
//         assert_eq!(erns, vec![ern1, ern2, ern3]);
//     }
//
//     #[test]
//     fn test_ern_timestamp_unixtime_consistency() {
//         let ern_timestamp1: Ern = Ern::with_root("root_a").unwrap();
//         let ern_unixtime1: Ern = Ern::with_root("root_a").unwrap();
//
//         sleep(Duration::from_millis(10));
//
//         let ern_timestamp2: Ern = Ern::with_root("root_b").unwrap();
//         let ern_unixtime2: Ern = Ern::with_root("root_b").unwrap();
//
//         // Ensure that the ordering is consistent between Timestamp and UnixTime
//         assert_eq!(
//             ern_timestamp1 < ern_timestamp2,
//             ern_unixtime1 < ern_unixtime2
//         );
//     }
//     #[test]
//     fn test_ern_with_root() {
//         let ern: Ern = Ern::with_root("custom_root").unwrap();
//         assert!(ern.root.as_str().starts_with("custom_root"));
//         assert_eq!(ern.domain, Domain::default());
//         assert_eq!(ern.category, Category::default());
//         assert_eq!(ern.account, Account::default());
//         assert_eq!(ern.parts, Parts::default());
//     }
//
//     #[test]
//     fn test_ern_with_new_root() {
//         let original_ern: Ern = Ern::default();
//         let new_ern: Ern = original_ern.with_new_root("new_root").unwrap();
//         assert!(new_ern.root.as_str().starts_with("new_root"));
//         assert_eq!(new_ern.domain, original_ern.domain);
//         assert_eq!(new_ern.category, original_ern.category);
//         assert_eq!(new_ern.account, original_ern.account);
//         assert_eq!(new_ern.parts, original_ern.parts);
//     }
//
//     #[test]
//     fn test_add_erns() -> anyhow::Result<()> {
//         let parent_root = Root::from_str("root_a")?;
//         let parent: Ern = Ern::new(
//             Domain::from_str("acton-internal").unwrap(),
//             Category::from_str("hr").unwrap(),
//             Account::from_str("company123").unwrap(),
//             parent_root.clone(),
//             Parts(vec![
//                 Part::from_str("department_a").unwrap(),
//                 Part::from_str("team1").unwrap(),
//             ]),
//         );
//
//         let child: Ern = Ern::new(
//             Domain::from_str("acton-internal").unwrap(),
//             Category::from_str("hr").unwrap(),
//             Account::from_str("company123").unwrap(),
//             Root::from_str("root_b").unwrap(),
//             Parts(vec![Part::from_str("role_x").unwrap()]),
//         );
//
//         let combined: Ern = parent + child;
//
//         assert_eq!(combined.domain, Domain::from_str("acton-internal").unwrap());
//         assert_eq!(combined.category, Category::from_str("hr").unwrap());
//         assert_eq!(combined.account, Account::from_str("company123").unwrap());
//         assert_eq!(combined.root, parent_root);
//         assert_eq!(
//             combined.parts,
//             Parts(vec![
//                 Part::from_str("department_a").unwrap(),
//                 Part::from_str("team1").unwrap(),
//                 Part::from_str("role_x").unwrap(),
//             ])
//         );
//         Ok(())
//     }
//
//     #[test]
//     fn test_add_erns_empty_child() {
//         let parent: Ern = Ern::new(
//             Domain::from_str("acton-internal").unwrap(),
//             Category::from_str("hr").unwrap(),
//             Account::from_str("company123").unwrap(),
//             Root::from_str("rootp").unwrap(),
//             Parts(vec![Part::from_str("department_a").unwrap()]),
//         );
//
//         let child: Ern = Ern::new(
//             Domain::from_str("acton-internal").unwrap(),
//             Category::from_str("hr").unwrap(),
//             Account::from_str("company123").unwrap(),
//             Root::from_str("rootc").unwrap(),
//             Parts(vec![]),
//         );
//
//         let combined = parent + child;
//
//         assert_eq!(
//             combined.parts,
//             Parts(vec![Part::from_str("department_a").unwrap()])
//         );
//     }
//
//     #[test]
//     fn test_add_erns_empty_parent() {
//         let parent: Ern = Ern::new(
//             Domain::from_str("acton-internal").unwrap(),
//             Category::from_str("hr").unwrap(),
//             Account::from_str("company123").unwrap(),
//             Root::from_str("rootp").unwrap(),
//             Parts(vec![]),
//         );
//         let child: Ern = Ern::new(
//             Domain::from_str("acton-internal").unwrap(),
//             Category::from_str("hr").unwrap(),
//             Account::from_str("company123").unwrap(),
//             Root::from_str("rootc").unwrap(),
//             Parts(vec![Part::from_str("role_x").unwrap()]),
//         );
//         let combined = parent + child;
//         assert_eq!(
//             combined.parts,
//             Parts(vec![Part::from_str("role_x").unwrap()])
//         );
//     }
//
//     #[test]
//     fn test_add_erns_display() {
//         let parent: Ern = Ern::new(
//             Domain::from_str("acton-internal").unwrap(),
//             Category::from_str("hr").unwrap(),
//             Account::from_str("company123").unwrap(),
//             Root::from_str("rootp").unwrap(),
//             Parts(vec![Part::from_str("department_a").unwrap()]),
//         );
//
//         let child: Ern = Ern::new(
//             Domain::from_str("acton-internal").unwrap(),
//             Category::from_str("hr").unwrap(),
//             Account::from_str("company123").unwrap(),
//             Root::from_str("rootc").unwrap(),
//             Parts(vec![Part::from_str("team1").unwrap()]),
//         );
//
//         let combined = parent + child;
//
//         assert!(combined
//             .to_string()
//             .starts_with("ern:acton-internal:hr:company123:rootp"));
//     }
//     #[test]
//     fn test_ern_custom() -> anyhow::Result<()> {
//         let ern: Ern = Ern::new(
//             Domain::new("custom")?,
//             Category::new("service"),
//             Account::new("account123"),
//             Root::new("root")?,
//             Parts::new(vec![Part::new("resource")?]),
//         );
//         assert!(ern
//             .to_string()
//             .starts_with("ern:custom:service:account123:root"));
//         Ok(())
//     }
//
//     #[test]
//     fn test_ern_append_invalid_part() -> anyhow::Result<()> {
//         let invalid_part = Part::new(":invalid");
//
//         assert!(invalid_part.is_err());
//         Ok(())
//     }
// }
