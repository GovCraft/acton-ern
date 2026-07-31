//! # Acton Entity Resource Name (ERN) Library
//!
//! `acton-ern` is a Rust library for working with Entity Resource Names (ERNs), which are structured identifiers
//! used to uniquely identify and manage hierarchical resources across different services and partitions in distributed systems.
//! While ERNs follow the Uniform Resource Name (URN) format defined in [RFC 8141](https://tools.ietf.org/html/rfc8141),
//! they extend beyond standard URNs by offering additional features like k-sortability and type-safe construction.
//!
//! ## Key Features
//!
//! - **Structured Resource Naming**: Create standardized, hierarchical resource identifiers that are both human-readable and machine-parseable
//! - **K-Sortable Identifiers**: When using `UnixTime` or `Timestamp` ID types, ERNs can be efficiently sorted and queried by creation time
//! - **Content-Addressable IDs**: When using `SHA1Name` ID type, generate deterministic IDs based on content
//! - **Type-Safe Construction**: The builder pattern ensures ERNs are constructed correctly at compile time
//! - **Flexible ID Types**: Choose the right ID type for your use case (time-based ordering or content-based addressing)
//! - **Hierarchical Relationships**: Model parent-child relationships between resources naturally
//! - **Serialization Support**: Serialize and deserialize ERNs to/from JSON and YAML (with the `serde` feature)
//!
//! ## Crate Structure
//!
//! - `builder`: Type-safe builder pattern for constructing ERNs
//! - `parser`: Tools for parsing ERN strings into structured components
//! - `model`: Component models (Domain, Category, Account, Root, Part)
//! - `traits`: Common traits used across the crate
//!
//! ## Basic Usage
//!
//! ```rust,ignore
//! use acton_ern::prelude::*;
//!
//! // Create a time-ordered, sortable ERN
//! let ern = ErnBuilder::new()
//!     .with::<Domain>("my-app")?
//!     .with::<Category>("users")?
//!     .with::<Account>("tenant123")?
//!     .with::<EntityRoot>("profile")?
//!     .with::<Part>("settings")?
//!     .build()?;
//!
//! // Parse an ERN from a string
//! let ern_str = "ern:my-app:users:tenant123:profile_01h9xz7n2e5p6q8r3t1u2v3w4x/settings";
//! let parsed_ern = ErnParser::new(ern_str.to_string()).parse()?;
//! ```
//!
//! ## Serialization/Deserialization
//!
//! With the `serde` feature enabled, ERNs can be serialized to and deserialized from formats like JSON and YAML:
//!
//! ```rust,ignore
//! // Enable the serde feature in Cargo.toml:
//! // acton-ern = { version = "3.0.0", features = ["serde"] }
//!
//! use acton_ern::prelude::*;
//! use serde_json;
//!
//! // Create an ERN
//! let ern = ErnBuilder::new()
//!     .with::<Domain>("my-app")?
//!     .with::<Category>("users")?
//!     .with::<Account>("tenant123")?
//!     .with::<EntityRoot>("profile")?
//!     .build()?;
//!
//! // Serialize to JSON
//! let json = serde_json::to_string(&ern)?;
//!
//! // Deserialize from JSON
//! let deserialized: Ern = serde_json::from_str(&json)?;
//! ```
//!

#![allow(missing_docs)]

extern crate core;

// Re-exporting the public API under the root of the crate for direct access
pub use builder::*;
pub use model::*;
pub use parser::*;
pub use traits::*;

mod builder;
mod errors;
mod model;
mod parser;
mod traits;

pub mod prelude {
    //! The prelude module for `acton-ern`.
    //!
    //! This module re-exports essential traits and structures for easy use in your application.
    //! Import this module with `use acton_ern::prelude::*;` to get access to all the commonly used
    //! types and traits without having to import them individually.

    pub use super::builder::ErnBuilder;
    pub use super::errors::ErnError;
    pub use super::model::{Account, Category, Domain, EntityRoot, Ern, Part, Parts, SHA1Name};
    pub use super::parser::ErnParser;
    pub use super::traits::*;
}

#[cfg(test)]
mod tests {
    use std::sync::Once;

    use tracing::Level;
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::{EnvFilter, FmtSubscriber};

    #[allow(dead_code)]
    static INIT: Once = Once::new();

    #[allow(dead_code)]
    pub fn init_tracing() {
        INIT.call_once(|| {
            // Define an environment filter to suppress logs from the specific function

            // let filter = EnvFilter::new("")
            //     // .add_directive("acton_core::common::context::emit_pool=trace".parse().unwrap())
            //     // .add_directive("acton_core::common::context::my_func=trace".parse().unwrap())
            //     .add_directive("acton_core::common::context[my_func]=trace".parse().unwrap())
            //     .add_directive(Level::INFO.into()); // Set global log level to INFO

            let filter = EnvFilter::new("")
                .add_directive("acton-ern::parser::tests=trace".parse().unwrap())
                .add_directive("broker_tests=trace".parse().unwrap())
                .add_directive("launchpad_tests=trace".parse().unwrap())
                .add_directive("lifecycle_tests=info".parse().unwrap())
                .add_directive("actor_tests=info".parse().unwrap())
                .add_directive("load_balancer_tests=info".parse().unwrap())
                .add_directive(
                    "acton::tests::setup::actors::pool_item=info"
                        .parse()
                        .unwrap(),
                )
                .add_directive("messaging_tests=info".parse().unwrap());
            // .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into()); // Set global log level to TRACE

            let subscriber = FmtSubscriber::builder()
                // .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
                .with_span_events(FmtSpan::NONE)
                .with_max_level(Level::TRACE)
                .compact()
                .with_line_number(true)
                .without_time()
                .with_env_filter(filter)
                .finish();

            tracing::subscriber::set_global_default(subscriber)
                .expect("setting default subscriber failed");
        });
    }
}
