# Acton ERN (Entity Resource Name)

## Overview

The `acton-ern` crate provides a robust, type-safe implementation for handling Entity Resource Names (ERNs). ERNs are structured identifiers used to uniquely identify and manage hierarchical resources across different services and partitions in distributed systems. While ERNs adhere to the Uniform Resource Name (URN) format as defined in [RFC 8141](https://tools.ietf.org/html/rfc8141), they extend beyond standard URNs by offering additional features:

1. **K-Sortability**: Unlike standard URNs, ERNs can be k-sortable when using `UnixTime` or `Timestamp` ID types. This allows for efficient ordering and range queries on ERNs, which is particularly valuable in distributed systems and databases.

2. **Type-Safe Construction**: The builder pattern ensures that ERNs are constructed correctly, with all required components in the right order.

3. **Flexible ID Types**: ERNs support various ID types for the root component, allowing for different use cases such as content-based hashing or time-based ordering.

These features make ERNs particularly suitable for use in distributed systems, providing a standardized, hierarchical, and sortable naming scheme that is both human-readable and machine-parseable.

## Table of Contents

- [Installation](#installation)
- [ERN Structure](#ern-structure)
- [Basic Usage](#basic-usage)
- [Advanced Usage](#advanced-usage)
  - [Building ERNs](#building-erns)
  - [Parsing ERNs](#parsing-erNS)
  - [Manipulating ERNs](#manipulating-erns)
- [ERN Components](#ern-components)
- [ID Types](#id-types)
- [Error Handling](#error-handling)
- [Best Practices](#best-practices)
- [Contributing](#contributing)
- [License](#license)

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
acton-ern = "2.1.0-alpha"
```

## ERN Structure

An ERN follows the URN format and has the following structure:

```
ern:domain:category:account:root/path/to/resource
```

This structure can be mapped to the URN format as follows:

- `ern`: NID (Namespace Identifier)
- `domain:category:account:root`: NSS (Namespace Specific String)
- `/path/to/resource`: Optional path components

The components of an ERN are:

- `ern`: Prefix indicating an Entity Resource Name (serves as the URN namespace)
- `domain`: Classifies the resource (e.g., internal, external, custom domains)
- `category`: Specifies the service or category within the system
- `account`: Identifies the owner or account responsible for the resource
- `root`: A unique identifier for the root of the resource hierarchy. When using `UnixTime` or `Timestamp` ID types, this component enables k-sortability.
- `path`: Optional path-like structure showing the resource's position within the hierarchy

By extending the URN format with k-sortability and type-safe construction, ERNs provide a powerful naming scheme for distributed systems that goes beyond the capabilities of standard URNs.

## Basic Usage

Here's a simple example of creating and using an ERN:

```rust
use acton_ern::prelude::*;

fn main() -> Result<(), ErnError> {
    // Create an ERN
    let ern: Ern<UnixTime> = ErnBuilder::new()
        .with::<Domain>("custom-domain")?
        .with::<Category>("service")?
        .with::<Account>("user123")?
        .with::<Root<UnixTime>>("resource")?
        .with::<Part>("subresource")?
        .build()?;

    // Convert ERN to string
    println!("ERN: {}", ern);

    // Parse an ERN string
    let ern_str = "ern:custom-domain:service:user123:resource/subresource";
    let parsed_ern: Ern<UnixTime> = ErnParser::new(ern_str).parse()?;

    assert_eq!(ern, parsed_ern);

    Ok(())
}
```

## Advanced Usage

### Building ERNs

The `ErnBuilder` provides a fluent interface for constructing ERNs, ensuring that required parts are added in the correct order:

```rust
use acton_ern::prelude::*;

fn create_ern() -> Result<Ern, ErnError> {
    ErnBuilder::new()
        .with::<Domain>("custom-domain")?
        .with::<Category>("iot")?
        .with::<Account>("device_manufacturer")?
        .with::<Root<UnixTime>>("sensors")?
        .with::<Part>("region1")?
        .with::<Part>("device42")?
        .build()
}
```

### Parsing ERNs

Use `ErnParser` to parse ERN strings:

```rust
use acton_ern::prelude::*;

fn parse_ern(ern_str: &str) -> Result<Ern, ErnError> {
    ErnParser::new(ern_str).parse()
}
```

### Manipulating ERNs

ERNs can be manipulated after creation:

```rust
use acton_ern::prelude::*;

fn manipulate_ern(ern: &Ern) -> Result<Ern, ErnError> {
    // Add a new part
    let new_ern = ern.add_part("new_subsystem")?;

    // Change the root
    let new_root_ern = ern.with_new_root("new_root")?;

    // Combine ERNs
    let combined_ern = ern.clone() + new_ern;

    Ok(combined_ern)
}
```

## ERN Components

The `acton-ern` crate provides separate types for each ERN component:

- `Domain`: Represents the domain of the resource
- `Category`: Specifies the service category
- `Account`: Identifies the account or owner
- `Root`: Represents the root of the resource hierarchy
- `Part`: Represents a single part in the resource path
- `Parts`: A collection of `Part`s

Each component is created with validation, returning a `Result`:

```rust
use acton_ern::prelude::*;

fn work_with_components() -> Result<(), ErnError> {
    let domain = Domain::new("custom-domain")?;
    let category = Category::new("finance");
    let account = Account::new("acme_corp");
    let root = Root::new("transactions")?;
    let part = Part::new("2023")?;
    let parts = Parts::new(vec![part]);

    Ok(())
}
```

## ID Types

The `acton-ern` crate supports different ID types for the `Root` component:

- `SHA1Name`: Uses UUID v5 (SHA1 hash)
  - Use case: When you need a deterministic, content-based identifier
- `Timestamp`: Uses UUID v6 (timestamp-based)
  - Use case: For time-ordered identifiers with microsecond precision
- `UnixTime`: Uses UUID v7 (Unix timestamp-based)
  - Use case: For time-ordered identifiers with millisecond precision, following the TypeId specification

Note: `UnixTime` and `Timestamp` implement `Ord` and are K-sortable based on their root value, allowing ERNs of these types to be used as sortable keys.

When using `UnixTime`, the root follows the TypeId specification found at: [typeid/spec](https://github.com/jetify-com/typeid).

Example of creating ERNs with different ID types:

```rust
use acton_ern::prelude::*;

fn create_erns_with_different_id_types() -> Result<(), ErnError> {
    let sha1_ern: Ern<SHA1Name> = Ern::with_root("sha1_root")?;
    let timestamp_ern: Ern<Timestamp> = Ern::with_root("timestamp_root")?;
    let unix_time_ern: Ern<UnixTime> = Ern::with_root("unix_time_root")?;

    Ok(())
}
```

## Error Handling

The crate uses a custom `ErnError` type for error handling. Always check for and handle potential errors when working with ERNs:

```rust
use acton_ern::prelude::*;

fn handle_ern_errors() {
    match ErnBuilder::new().with::<Domain>("").build() {
        Ok(ern) => println!("Created ERN: {}", ern),
        Err(ErnError::ParseFailure(component, msg)) => {
            eprintln!("Failed to parse {}: {}", component, msg);
        }
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}
```

## Best Practices

1. Use the builder pattern (`ErnBuilder`) for creating new ERNs, ensuring required parts are added in the correct order.
2. Parse ERN strings using `ErnParser` to ensure validity.
3. Choose appropriate ID types based on your use case (e.g., `UnixTime` for timestamp-based, sortable IDs).
4. Handle all potential errors using the `ErnError` type.
5. Use the provided component types (`Domain`, `Category`, etc.) for type safety.
6. Leverage the `is_child_of` and `parent` methods for working with hierarchical ERNs.
7. When using `UnixTime` or `Timestamp` ID types, take advantage of their `Ord` implementation for sorting and ordering ERNs.

## Contributing

Contributions to `acton-ern` are welcome! Please refer to the project's GitHub repository for contribution guidelines.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.