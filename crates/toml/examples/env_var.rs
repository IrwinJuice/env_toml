//! An example showing env-var placeholders in TOML.

#![deny(warnings)]
#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    db_url: String,
    db_port: u16,
    default_port: u16,
    empty_default: String,
    empty_option_default: Option<u16>,
    port_s: String,
    arr_s: String,
    tbl_s: String,
}

fn main() {
    unsafe {
        std::env::set_var("DB_URL", "postgres://localhost:5432");
        std::env::set_var("DB_PORT", "9090");
        std::env::set_var("EMPTY_VAL", "");
        std::env::set_var("PORT", "8080");
        std::env::set_var("VAR_ARRAY", "[1, 2]");
        std::env::set_var("VAR_TABLE", "{ a = 1 }");
    }

    let toml_str = r#"
        db_url = ${DB_URL}
        db_port = ${DB_PORT:8080}
        default_port = ${MISSING_PORT:8080}
        empty_default = ${EMPTY_VAL:}
        empty_option_default = ${EMPTY_VAL:}
        port_s = ${PORT}
        arr_s = ${VAR_ARRAY}
        tbl_s = ${VAR_TABLE}
    "#;

    let config: Config = env_toml::from_str(toml_str).unwrap();

    println!("{config:#?}");
    assert_eq!(config.db_url, "postgres://localhost:5432");
    assert_eq!(config.db_port, 9090);
    assert_eq!(config.default_port, 8080);
    assert_eq!(config.empty_default, "");
    assert_eq!(config.empty_option_default, None);
    assert_eq!(config.port_s, "8080");
    assert_eq!(config.arr_s, "[1, 2]");
    assert_eq!(config.tbl_s, "{ a = 1 }");
}

