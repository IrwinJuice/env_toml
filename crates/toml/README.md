# env_toml

[![Documentation](https://img.shields.io/badge/docs-main-blue.svg)](https://docs.rs/env_toml)
![License](https://img.shields.io/crates/l/toml.svg)
[![Crates Status](https://img.shields.io/crates/v/toml.svg)](https://crates.io/crates/env_toml)

A [serde]-compatible [TOML][toml] decoder and encoder for Rust extended with Environment Variable syntax.

This project is entirely based on and follows the upstream [toml][toml_crate] crate and will continue to incorporate all related changes.

Environment variables are not part of the official TOML specification.
If you do not require environment variable support, we highly recommend using the original [toml][toml_crate] crate instead.

> `std` is required for environment variables in Rust.


## Environment variable parsing
This crate extends TOML syntax with environment variable placeholders:
```toml
db_url = ${DB_URL} # Required: Fails if not set
db_port = ${DB_PORT:8080} # Optional: Defaults to 8080
default_port = ${MISSING_PORT:8080}  # Optional: Defaults to 8080
empty_default = ${ENV_VALUE:} # Optional: Defaults to empty string
list = [ ${VAL1}, ${VAL2:c}, ${VAL3:d} ]
```

### Resolution rules
- `${NAME}` reads the environment variable `NAME`
- `${NAME:default}` uses `default` when `NAME` is not set
- `${NAME:}` uses an __empty string__ as the fallback

### Type behavior
After resolution, env-var values are reinterpreted as TOML scalars when appropriate:

- `"8080"` → integer if the destination type is numeric
- `"3.14"` → float
- `"true"` → boolean
- `"hello"` → string

Structured TOML values are not treated as structured env-var payloads.
If an env-var contains something that looks like an array or table, the parser falls back to the raw string instead of deserializing it as TOML structure.

For example:

```rust
std::env::set_var("VAR_ARRAY", "[1, 2]");
std::env::set_var("VAR_TABLE", "{ a = 1 }");

#[derive(Deserialize)]
struct Config {
    arr_s: String,
    tbl_s: String,
}

let config: Config = env_toml::from_str(r#"
arr_s = ${VAR_ARRAY}
tbl_s = ${VAR_TABLE}
"#).unwrap();

assert_eq!(config.arr_s, "[1, 2]");
assert_eq!(config.tbl_s, "{ a = 1 }");
```
If you deserialize the same env-var into a structured type like `Vec<i64>`, it will fail.

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
