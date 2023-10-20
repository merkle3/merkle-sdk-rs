//! # merkle-sdk-rs
//!
//! The merkle SDK is a great way to access our products.
//!
//! ## Quickstart: `prelude`
//!
//! A prelude is provided which imports all the important data types and traits for you. Use this
//! when you want to quickly bootstrap a new project.
//!
//! ```rust
//! use merkle_sdk::prelude::*;
//! ```
//!
//! Examples on how you can use the types imported by the prelude can be found in the
//! [`examples` directory of the repository](https://github.com/merkle3/merkle-sdk-rs/tree/main/examples)
//! and in the `tests` modules of each crate.
//!
//! ## Modules
//!
//! The following paragraphs are a quick explanation of each module in ascending order of
//! abstraction.
//!
//! ### `transactions`
//!
//! Contains all the necessary data structures for connecting a transactions stream.
//! To simplify your imports, consider using the re-exported modules described in the next
//! subsection.
//!

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![deny(rustdoc::broken_intra_doc_links)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(test(
    no_crate_inject,
    attr(deny(rust_2018_idioms), allow(dead_code, unused_variables))
))]

#[doc(inline)]
pub use merkle_sdk_transactions as transactions;

/// Easy imports of frequently used type definitions and traits.
#[doc(hidden)]
#[allow(unknown_lints, ambiguous_glob_reexports)]
pub mod prelude {
    pub use super::transactions::*;
}
