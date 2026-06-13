#![recursion_limit = "256"]
#![allow(clippy::dbg_macro)]
#![cfg(all(feature = "parse", feature = "display", feature = "serde"))]

macro_rules! parse_value {
    ($s:expr) => {{
        let v = $s.parse::<env_toml::Value>();
        assert!(
            v.is_ok(),
            "Failed with `{}` when parsing:
```
{}
```
",
            v.unwrap_err(),
            $s
        );
        v.unwrap()
    }};
}

mod invalid;
mod parse;

use env_toml::Table as RustDocument;
use env_toml::Value as RustValue;
