//! A [serde]-compatible [TOML]-parsing library extended with Environment Variable syntax
//!
//! TOML itself is a simple, ergonomic, and readable configuration format:
//!
//! ```toml
//! [package]
//! name = "toml"
//!
//! [dependencies]
//! serde = "1.0"
//! ```
//!
//! The TOML format tends to be relatively common throughout the Rust community
//! for configuration, notably being used by [Cargo], Rust's package manager.
//!
//! ### Why this crate?
//! While standard TOML is great, configuration files often need to dynamically adapt to
//! different environments (e.g., development, staging, production). The official TOML
//! specification and upstream implementation do not natively support environment variables.
//!
//! This crate bridges that gap by allowing you to inject environment variables directly into
//! your TOML files using a clean syntax (e.g., `${MY_ENV_VAR}`), while remaining fully
//! compatible with [serde].
//!
//! If you do not require environment variable support, we highly recommend using the original [toml][toml_crate] crate instead.
//!
//! ### Serialization vs. Deserialization
//! It is important to note how environment variables interact with Serde's traits:
//! * **Deserialization (Parsing):** Environment variable syntax (`${...}`) is resolved when reading a TOML string into your Rust structures.
//! * **Serialization (Writing):** Serializing Rust structures back into TOML works exactly like the upstream crate. However, **the environment variable syntax is not injected during serialization**.
//! For example, if a struct field contains the value `"production"`, it will be written as a plain string `"production"`, not as `${ENV_TYPE}`.
//!
//! ### Platform & Standard Library Support
//! Please note that resolving environment variables requires access to the operating system's
//! environment via the Rust standard library (`std`). Consequently, **this crate requires `std`**
//! and cannot be used in strict `#![no_std]` environments where standard library access is unavailable.
//!
//! ### Environment variable parsing
//! This crate extends TOML syntax with environment variable placeholders.
//! Placeholders can be written either as bare values or inside quoted strings:
//!
//! #### Quoted string syntax — valid TOML, friendlier to standard tooling
//! ```toml
//! db_url = "${DB_URL}"           # Required: Fails if not set
//! db_port = "${DB_PORT:8080}"    # Optional: Defaults to 8080
//! empty_default = "${ENV_VALUE:}" # Optional: Defaults to `None` for optional fields, or an empty string for other types
//! list = [ "${VAL1}", "${VAL2:c}", "${VAL3:d}" ]
//! ```
//!
//! #### Bare syntax
//! ```toml
//! db_url = ${DB_URL}              # Required: Fails if not set
//! db_port = ${DB_PORT:8080}       # Optional: Defaults to 8080
//! empty_default = ${ENV_VALUE:}   # Optional: Defaults to `None` for optional fields, or an empty string for other types
//! list = [ ${VAL1}, ${VAL2:c}, ${VAL3:d} ]
//! ```
//!
//! Both forms resolve identically at deserialization time.
//!
//! ### Resolution rules
//! - `${NAME}` reads the environment variable `NAME`
//! - `${NAME:default}` uses `default` when `NAME` is not set
//! - `${NAME:}` uses `None` for optional fields, or an __empty string__ for other types when `NAME` is not set
//!
//! ### Quoted string syntax
//! Wrapping a placeholder in double quotes (`"${VAR}"`) is standard TOML and is accepted by
//! any spec-compliant TOML parser. The resolution behaviour is identical to the bare form.
//! Only a string that consists **entirely** of a single placeholder is resolved as an env var —
//! a string like `"prefix-${VAR}"` is treated as a plain literal string and is **not** interpolated.
//!
//! ### Type behavior
//! After resolution, env-var values are reinterpreted as TOML scalars when appropriate:
//!
//! - `"8080"` → integer if the destination type is numeric
//! - `"3.14"` → float
//! - `"true"` → boolean
//! - `"hello"` → string
//!
//! Structured TOML values are not treated as structured env-var payloads.
//! If an env-var contains something that looks like an array or table, the parser falls back to the raw string instead of deserializing it as TOML structure.
//!
#![cfg_attr(not(feature = "default"), doc = " ```ignore")]
#![cfg_attr(feature = "default", doc = " ```")]
//!  use serde::Deserialize;
//!  unsafe {
//!      std::env::set_var("VAR_ARRAY", "[1, 2]");
//!      std::env::set_var("VAR_TABLE", "{ a = 1 }");
//!  }
//!
//!  #[derive(Deserialize)]
//!  struct Config {
//!      arr_s: String,
//!      tbl_s: String,
//!  }
//!
//!  let config: Config = env_toml::from_str(r#"
//!  arr_s = ${VAR_ARRAY}
//!  tbl_s = ${VAR_TABLE}
//!  "#).unwrap();
//!
//!  assert_eq!(config.arr_s, "[1, 2]");
//!  assert_eq!(config.tbl_s, "{ a = 1 }");
//! ```
//! If you deserialize the same env-var into a structured type like `Vec<i64>`, it will fail.
//!
//! ## TOML values
//!
//! A TOML document is represented with the [`Table`] type which maps `String` to the [`Value`] enum:
//!
#![cfg_attr(not(feature = "default"), doc = " ```ignore")]
#![cfg_attr(feature = "default", doc = " ```")]
//! # use env_toml::value::{Datetime, Array, Table};
//! pub enum Value {
//!     String(String),
//!     Integer(i64),
//!     Float(f64),
//!     Boolean(bool),
//!     Datetime(Datetime),
//!     Array(Array),
//!     Table(Table),
//! }
//! ```
//!
//! ## Parsing TOML
//!
//! The easiest way to parse a TOML document is via the [`Table`] type:
//!
#![cfg_attr(not(feature = "default"), doc = " ```ignore")]
#![cfg_attr(feature = "default", doc = " ```")]
//! use env_toml::Table;
//!
//! let value = "foo = 'bar'".parse::<Table>().unwrap();
//!
//! assert_eq!(value["foo"].as_str(), Some("bar"));
//! ```
//!
//! The [`Table`] type implements a number of convenience methods and
//! traits; the example above uses [`FromStr`] to parse a [`str`] into a
//! [`Table`].
//!
//! ## Deserialization and Serialization
//!
//! This crate supports [`serde`] 1.0 with a number of
//! implementations of the `Deserialize`, `Serialize`, `Deserializer`, and
//! `Serializer` traits. Namely, you'll find:
//!
//! * `Deserialize for Table`
//! * `Serialize for Table`
//! * `Deserialize for Value`
//! * `Serialize for Value`
//! * `Deserialize for Datetime`
//! * `Serialize for Datetime`
//! * `Deserializer for de::Deserializer`
//! * `Serializer for ser::Serializer`
//! * `Deserializer for Table`
//! * `Deserializer for Value`
//!
//! This means that you can use Serde to deserialize/serialize the
//! [`Table`] type as well as [`Value`] and [`Datetime`] type in this crate. You can also
//! use the [`Deserializer`], [`Serializer`], or [`Table`] type itself to act as
//! a deserializer/serializer for arbitrary types.
//!
//! An example of deserializing with TOML is:
//!
#![cfg_attr(not(feature = "default"), doc = " ```ignore")]
#![cfg_attr(feature = "default", doc = " ```")]
//! use serde::Deserialize;
//!
//!  unsafe {
//!      std::env::set_var("DB_URL", "postgres://localhost:5432");
//!      std::env::set_var("DB_PORT", "9090");
//!      std::env::set_var("EMPTY_VAL", "");
//!  }
//!   let toml_str = r#"
//!   db_url = ${DB_URL}
//!   db_port = ${DB_PORT:8080}
//!   default_port = ${MISSING_PORT:8080}
//!   empty_default_string = ${EMPTY_VAL:}
//!   empty_option_default = ${EMPTY_VAL:}
//!   "#;
//!
//!   #[derive(Deserialize)]
//!   struct Config {
//!       db_url: String,
//!       db_port: u16,
//!       default_port: u16,
//!       empty_default_string: String,
//!       empty_option_default: Option<u16>,
//!   }
//!
//!   let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");
//!
//!   assert_eq!(config.db_url, "postgres://localhost:5432");
//!   assert_eq!(config.db_port, 9090);
//!   assert_eq!(config.default_port, 8080);
//!   assert_eq!(config.empty_default_string, "");
//!   assert_eq!(config.empty_option_default, None);
//! ```
//! You can serialize types in a similar fashion:
//!
#![cfg_attr(not(feature = "default"), doc = " ```ignore")]
#![cfg_attr(feature = "default", doc = " ```")]
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct Config {
//!     ip: String,
//!     port: Option<u16>,
//!     keys: Keys,
//! }
//!
//! #[derive(Serialize)]
//! struct Keys {
//!     github: String,
//!     travis: Option<String>,
//! }
//!
//! let config = Config {
//!     ip: "127.0.0.1".to_string(),
//!     port: None,
//!     keys: Keys {
//!         github: "xxxxxxxxxxxxxxxxx".to_string(),
//!         travis: Some("yyyyyyyyyyyyyyyyy".to_string()),
//!     },
//! };
//!
//! let toml = env_toml::to_string(&config).unwrap();
//! ```
//!
//! [TOML]: https://github.com/toml-lang/toml
//! [Cargo]: https://crates.io/
//! [`serde`]: https://serde.rs/
//! [serde]: https://serde.rs/
//! [toml_crate]: https://crates.io/crates/toml

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
// Makes rustc abort compilation if there are any unsafe blocks in the crate.
// Presence of this annotation is picked up by tools such as cargo-geiger
// and lets them ensure that there is indeed no unsafe code as opposed to
// something they couldn't detect (e.g. unsafe added via macro expansion, etc).
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

#[allow(unused_extern_crates)]
extern crate alloc;

pub(crate) mod alloc_prelude {
    pub(crate) use alloc::borrow::ToOwned as _;
    pub(crate) use alloc::format;
    pub(crate) use alloc::string::String;
    pub(crate) use alloc::string::ToString as _;
    pub(crate) use alloc::vec::Vec;
}

pub mod map;
#[cfg(feature = "serde")]
pub mod value;

pub mod de;
#[cfg(feature = "serde")]
pub mod ser;

#[doc(hidden)]
#[cfg(feature = "serde")]
pub mod macros;

#[cfg(feature = "serde")]
mod table;

#[doc(inline)]
#[cfg(feature = "parse")]
#[cfg(feature = "serde")]
pub use crate::de::{Deserializer, from_slice, from_str};
#[doc(inline)]
#[cfg(feature = "display")]
#[cfg(feature = "serde")]
pub use crate::ser::{Serializer, to_string, to_string_pretty};
#[doc(inline)]
#[cfg(feature = "serde")]
pub use crate::value::Value;
pub use serde_spanned::Spanned;
#[cfg(feature = "serde")]
pub use table::Table;

// Shortcuts for the module doc-comment
#[allow(unused_imports)]
use core::str::FromStr;
#[allow(unused_imports)]
use toml_datetime::Datetime;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
