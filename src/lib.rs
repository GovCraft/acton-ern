//! # Acton Resource Name (ERN (Entity Resource Name)) Library
//!
//! `acton-ern` is a Rust library designed to handle Acton Resource Names (Erns), which are structured identifiers used within the [Acton distributed actor framework](https://github.com/GovCraft/acton-framework) to uniquely identify and manage hierarchical resources across different services and partitions.
//!
//! This crate provides tools for generating, parsing, and managing Erns, ensuring type safety and alignment with the hierarchical structure needed Acton-based cloud-native solutions.
//!
//! ## Features
//! - **ERN (Entity Resource Name) Parsing**: Parse ERN (Entity Resource Name) strings into structured components.
//! - **ERN (Entity Resource Name) Building**: Programmatically build Erns with validation.
//! - **Type Safety**: Strongly typed components prevent mixing parts of ERN (Entity Resource Name).
//! - **Easy Integration**: Designed to be integrated with other systems managing hierarchical resources.
//!
//! ## Usage
//! This crate is structured into several modules, each providing distinct functionalities:
//! - `builder`: Module for building Erns.
//! - `parser`: Module for parsing Erns.
//! - `model`: Contains the models representing different parts of an ERN (Entity Resource Name).
//! - `traits`: Traits used across the crate for common functionality.
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
    //! This module re-exports essential traits and structures for easy use by downstream consumers.

    pub use super::builder::ErnBuilder;
    pub use super::errors::ErnError;
    pub use super::model::{Account, Category, Domain, Ern, Part, Parts};
    pub use super::parser::ErnParser;
    pub use super::traits::*;
}

// #[cfg(test)]
// mod tests {
//     use std::sync::Once;
//
//     use tracing::Level;
//     use tracing_subscriber::{EnvFilter, FmtSubscriber};
//     use tracing_subscriber::fmt::format::FmtSpan;
//
//     static INIT: Once = Once::new();
//
//     pub fn init_tracing() {
//         INIT.call_once(|| {
//             // Define an environment filter to suppress logs from the specific function
//
//             // let filter = EnvFilter::new("")
//             //     // .add_directive("acton_core::common::context::emit_pool=trace".parse().unwrap())
//             //     // .add_directive("acton_core::common::context::my_func=trace".parse().unwrap())
//             //     .add_directive("acton_core::common::context[my_func]=trace".parse().unwrap())
//             //     .add_directive(Level::INFO.into()); // Set global log level to INFO
//
//             let filter = EnvFilter::new("")
//                 .add_directive("acton-ern::parser::tests=trace".parse().unwrap())
//                 .add_directive("broker_tests=trace".parse().unwrap())
//                 .add_directive("launchpad_tests=trace".parse().unwrap())
//                 .add_directive("lifecycle_tests=info".parse().unwrap())
//                 .add_directive("actor_tests=info".parse().unwrap())
//                 .add_directive("load_balancer_tests=info".parse().unwrap())
//                 .add_directive(
//                     "acton::tests::setup::actors::pool_item=info"
//                         .parse()
//                         .unwrap(),
//                 )
//                 .add_directive("messaging_tests=info".parse().unwrap());
//             // .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into()); // Set global log level to TRACE
//
//             let subscriber = FmtSubscriber::builder()
//                 // .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
//                 .with_span_events(FmtSpan::NONE)
//                 .with_max_level(Level::TRACE)
//                 .compact()
//                 .with_line_number(true)
//                 .without_time()
//                 .with_env_filter(filter)
//                 .finish();
//
//             tracing::subscriber::set_global_default(subscriber)
//                 .expect("setting default subscriber failed");
//         });
//     }
// }
