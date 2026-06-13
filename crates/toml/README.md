# env_toml

[![Documentation](https://img.shields.io/badge/docs-main-blue.svg)](https://docs.rs/env_toml)
![License](https://img.shields.io/crates/l/toml.svg)
[![Crates Status](https://img.shields.io/crates/v/toml.svg)](https://crates.io/crates/env_toml)

A [serde]-compatible [TOML][toml] decoder and encoder for Rust extended with Environment Variable syntax.

```toml
db_url = ${DB_URL} # Required: Fails if not set
db_port = ${DB_PORT:8080} # Optional: Defaults to 8080
default_port = ${MISSING_PORT:8080}  # Optional: Defaults to 8080
empty_default = ${ENV_VALUE:} # Optional: Defaults to empty string
```

This project is entirely based on and follows the original [toml][toml_crate] crate and will continue to incorporate all related changes.

Environment Variables in a TOML file is not part of the specification.
If you don't need Environment Variables, it's better to use [toml][toml_crate] crate.

> `std` is required for environment variables in Rust.

[serde]: https://serde.rs/
[toml]: https://github.com/toml-lang/toml
[toml_edit]: https://docs.rs/toml_edit
[toml_crate]: https://crates.io/crates/toml

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/license/mit>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.
