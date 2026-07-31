# Acton ERN (Entity Resource Name)

[![Crates.io](https://img.shields.io/crates/v/acton-ern.svg)](https://crates.io/crates/acton-ern)
[![Documentation](https://docs.rs/acton-ern/badge.svg)](https://docs.rs/acton-ern)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](README.md#license)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

## Standardized Resource Management in Distributed Systems

Acton ERN provides a standardized approach for uniquely identifying and managing resources across services, partitions, and hierarchies in distributed systems.

**Implement a consistent, type-safe resource naming scheme that scales with your system.**

## Why Acton ERN?

### 🔍 Problem: Resource Identification Inconsistency

In distributed systems, resources are scattered across multiple services, databases, and storage systems. Without a consistent naming scheme:

- Resources lack consistent location and reference methods
- Relationships between resources require additional tracking mechanisms
- Sorting and querying resources efficiently requires custom solutions
- Type safety is compromised, leading to runtime errors

### ✅ Solution: Structured, Type-Safe Resource Names

Acton ERN addresses these issues by providing:

- **Consistent Structure**: Every resource follows the same naming pattern, making them predictable and systematic
- **Hierarchical Organization**: Resources can be organized in parent-child relationships, reflecting their logical structure
- **Time-Ordered Sorting**: When using time-based ID types, resources can be efficiently sorted and queried by creation time
- **Content-Based Addressing**: When using hash-based ID types, resources with identical content can be deterministically identified
- **Type Safety**: The builder pattern ensures ERNs are constructed correctly at compile time

## Real-World Benefits

### For Microservice Architectures

- **Service Discovery**: Locate resources across different services using a consistent addressing scheme
- **Cross-Service References**: Maintain references between resources in different services without ambiguity
- **Versioning Support**: Track resource versions and changes over time with time-ordered IDs

### For Data-Intensive Applications

- **Efficient Querying**: K-sortable IDs enable range queries and time-based filtering
- **Data Lineage**: Track relationships between derived data and source data
- **Deduplication**: Content-addressable IDs help identify duplicate resources

### For Cloud-Native Applications

- **Multi-Tenant Support**: The account component clearly separates resources by tenant
- **Resource Categorization**: Organize resources by domain and category for structured management
- **Hierarchical Structure**: Model complex resource relationships with defined patterns

## Quick Start

Add Acton ERN to your project:

```toml
[dependencies]
acton-ern = "3.0.0"
```

### Creating an ERN

```rust
use acton_ern::prelude::*;

// Create a time-ordered, sortable ERN
let ern = ErnBuilder::new()
    .with::<Domain>("my-app")?
    .with::<Category>("users")?
    .with::<Account>("tenant123")?
    .with::<EntityRoot>("profile")?
    .with::<Part>("settings")?
    .build()?;

// The resulting ERN will look like:
// ern:my-app:users:tenant123:profile_01h9xz7n2e5p6q8r3t1u2v3w4x/settings
```

### Parsing an ERN

```rust
use acton_ern::prelude::*;

// Parse an ERN from a string
let ern_str = "ern:my-app:users:tenant123:profile_01h9xz7n2e5p6q8r3t1u2v3w4x/settings";
let parsed_ern = ErnParser::new(ern_str.to_string()).parse()?;

// Access components
println!("Domain: {}", parsed_ern.domain);
println!("Category: {}", parsed_ern.category);
println!("Account: {}", parsed_ern.account);
println!("Root: {}", parsed_ern.root);
println!("Parts: {}", parsed_ern.parts);
```

## Choose the Right ID Type for Your Needs

Acton ERN supports different ID types for different use cases:

- **UnixTime (Default)**: Time-ordered IDs with millisecond precision for chronological sorting
- **Timestamp**: Time-ordered IDs with microsecond precision for higher resolution timing needs
- **SHA1Name**: Content-addressable IDs that are deterministic based on input, suitable for content-based resources

```rust
// Time-ordered ID (sortable by creation time)
let time_ern: Ern = ErnBuilder::new()
    .with::<Domain>("my-app")?
    .with::<Category>("events")?
    .with::<Account>("tenant123")?
    .with::<EntityRoot>("log")?
    .build()?;

// Content-addressable ID (same content = same ID)
let content_ern: Ern = ErnBuilder::new()
    .with::<Domain>("my-app")?
    .with::<Category>("documents")?
    .with::<Account>("tenant123")?
    .with::<SHA1Name>("report-2023-q4")?
    .build()?;
```

## Optional Features

Acton ERN includes optional features:

```toml
[dependencies]
acton-ern = { version = "3.0.0", features = ["serde"] }
```

- **serde**: Add serialization/deserialization support for JSON, YAML, and more
- **std**: Enabled by default, can be disabled for no_std environments

## Working with ERNs

### Hierarchical Relationships

```rust
// Check if one ERN is a child of another
if child_ern.is_child_of(&parent_ern) {
    println!("Child resource found.");
}

// Get the parent of an ERN
if let Some(parent) = child_ern.parent() {
    println!("Parent: {}", parent);
}
```

### Combining ERNs

```rust
// Combine two ERNs (appends the parts)
let combined_ern = base_ern + extension_ern;
```

## Development Status

**Acton ERN 1.0.0 Release**

The 1.0.0 release includes all core functionality, comprehensive testing, and production-ready features. See the [CHANGELOG.md](CHANGELOG.md) for details on what's included in this release.

## License

This project is licensed under either of:

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- The Acton Framework team for their contributions
- All contributors who have provided input to this project
## Sponsor

Govcraft is a one-person shop—no corporate backing, no investors, just me building useful tools. If this project helps you, [sponsoring](https://github.com/sponsors/Govcraft) keeps the work going.

[![Sponsor on GitHub](https://img.shields.io/badge/Sponsor-%E2%9D%A4-%23db61a2?logo=GitHub)](https://github.com/sponsors/Govcraft)
