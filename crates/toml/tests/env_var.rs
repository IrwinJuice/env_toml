#![cfg(all(feature = "parse", feature = "display", feature = "serde"))]
#![allow(dead_code)]

use serde::Deserialize;

#[test]
fn test_env_var() {
    unsafe {
        std::env::set_var("DB_URL", "postgres://localhost:5432");
        std::env::set_var("DB_PORT", "9090");
        std::env::set_var("EMPTY_VAL", "");
    }

    let toml_str = r#"
db_url = ${DB_URL}
db_port = ${DB_PORT:8080}
default_port = ${MISSING_PORT:8080}
empty_string_default = ${EMPTY_VAL:}
empty_option_default = ${EMPTY_VAL:}
"#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        db_url: String,
        db_port: u16,
        default_port: u16,
        empty_string_default: String,
        empty_option_default: Option<u16>,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");

    assert_eq!(config.db_url, "postgres://localhost:5432");
    assert_eq!(config.db_port, 9090);
    assert_eq!(config.default_port, 8080);
    assert_eq!(config.empty_string_default, "");
    assert_eq!(config.empty_option_default, None);
}

#[test]
fn test_env_var_missing() {
    unsafe {
        std::env::remove_var("MISSING_VAL");
    }
    let toml_str = "db_url = ${MISSING_VAL}";

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        db_url: String,
    }

    let result: Result<Config, env_toml::de::Error> = env_toml::from_str(toml_str);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("environment variable `MISSING_VAL` not set")
    );
}

#[test]
fn test_env_var_in_array() {
    unsafe {
        std::env::set_var("VAL1", "a");
        std::env::set_var("VAL2", "b");
    }

    let toml_str = r#"
list = [ ${VAL1}, ${VAL2:c}, ${VAL3:d} ]
"#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        list: Vec<String>,
    }

    let config: Config = env_toml::from_str(toml_str).unwrap();
    assert_eq!(config.list, vec!["a", "b", "d"]);
}

#[test]
fn test_env_var_in_table() {
    unsafe {
        std::env::set_var("KEY1", "value1");
    }

    let toml_str = r#"
[table]
key = ${KEY1}
"#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        table: Table,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct Table {
        key: String,
    }

    let config: Config = env_toml::from_str(toml_str).unwrap();
    assert_eq!(config.table.key, "value1");
}

#[test]
fn test_env_var_numeric_types() {
    unsafe {
        std::env::set_var("PORT", "8080");
        std::env::set_var("TIMEOUT", "3.01");
        std::env::set_var("DEBUG", "true");
        std::env::set_var("COUNT", "42");
    }

    let toml_str = r#"
port    = ${PORT}
timeout = ${TIMEOUT}
debug   = ${DEBUG}
count   = ${COUNT}
port_s  = ${PORT}
"#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        port: u16,
        timeout: f64,
        debug: bool,
        count: i64,
        // Same env-var, but the field is String → raw string is preserved
        port_s: String,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");

    assert_eq!(config.port, 8080);
    assert!((config.timeout - 3.01).abs() < f64::EPSILON);
    assert!(config.debug);
    assert_eq!(config.count, 42);
    assert_eq!(config.port_s, "8080");
}

#[test]
fn test_visit_env_var_value_fallbacks() {
    unsafe {
        std::env::set_var("VAR_ARRAY", "[1, 2]");
        std::env::set_var("VAR_TABLE", "{ a = 1 }");
    }

    let toml_str = r#"
arr_s = ${VAR_ARRAY}
tbl_s = ${VAR_TABLE}
"#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        arr_s: String,
        tbl_s: String,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");

    // visit_env_var_value should fall back to the raw string for arrays/tables
    // and for non-TOML bare words.
    assert_eq!(config.arr_s, "[1, 2]");
    assert_eq!(config.tbl_s, "{ a = 1 }");

    // Attempting to deserialize the arr_s env-var as an actual sequence should fail,
    // because env-vars that parse as arrays are intentionally treated as strings.
    #[derive(Deserialize, Debug)]
    struct SeqConfig {
        arr_s: Vec<i64>,
    }

    let res: Result<SeqConfig, env_toml::de::Error> = env_toml::from_str(r#"arr_s = ${VAR_ARRAY}"#);
    assert!(res.is_err());
}

#[test]
fn test_env_var_optional_empty_default() {
    unsafe {
        std::env::remove_var("OPTIONAL_VAL");
    }
    let toml_str = "opt = ${OPTIONAL_VAL:}";

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        opt: Option<String>,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");
    assert_eq!(config.opt, None);
}

#[test]
fn test_env_var_optional_u16_empty_default() {
    unsafe {
        std::env::remove_var("OPTIONAL_PORT");
    }
    let toml_str = "port = ${OPTIONAL_PORT:}";

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        port: Option<u16>,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");
    assert_eq!(config.port, None);
}

#[test]
fn test_env_var_optional_no_default() {
    unsafe {
        std::env::remove_var("OPTIONAL_VAL_2");
    }
    let toml_str = "opt = ${OPTIONAL_VAL_2}";

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        opt: Option<String>,
    }

    let result: Result<Config, env_toml::de::Error> = env_toml::from_str(toml_str);
    // Should still be an error if no default is provided and env var is missing
    assert!(result.is_err());
}

#[test]
fn test_env_var_optional_with_default() {
    unsafe {
        std::env::remove_var("OPTIONAL_VAL_3");
    }
    let toml_str = "opt = ${OPTIONAL_VAL_3:default}";

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        opt: Option<String>,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");
    assert_eq!(config.opt, Some("default".to_owned()));
}

#[test]
fn test_env_var_non_optional_empty_default() {
    unsafe {
        std::env::remove_var("NON_OPTIONAL_VAL");
    }
    let toml_str = "val = ${NON_OPTIONAL_VAL:}";

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        val: String,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");
    // Should still be ""
    assert_eq!(config.val, "");
}

#[test]
fn test_env_var_optional_u16_present() {
    unsafe {
        std::env::set_var("PRESENT_PORT", "9090");
    }
    let toml_str = "port = ${PRESENT_PORT:8080}";

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        port: Option<u16>,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");
    assert_eq!(config.port, Some(9090));
}

#[test]
fn test_env_var_set_to_empty_string() {
    unsafe {
        std::env::set_var("SET_EMPTY_VAL", "");
    }
    let toml_str = "opt = ${SET_EMPTY_VAL:}";

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        opt: Option<String>,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");
    // It should now be None
    assert_eq!(config.opt, None);
}

/////////// In String
#[test]
fn test_env_var_in_string() {
    unsafe {
        std::env::set_var("DB_URL", "postgres://localhost:5432");
        std::env::set_var("DB_PORT", "9090");
        std::env::set_var("EMPTY_VAL", "");
    }

    let toml_str = r#"
db_url = "${DB_URL}"
db_port = "${DB_PORT:8080}"
default_port = "${MISSING_PORT:8080}"
empty_string_default = "${EMPTY_VAL:}"
empty_option_default = "${EMPTY_VAL:}"
"#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        db_url: String,
        db_port: u16,
        default_port: u16,
        empty_string_default: String,
        empty_option_default: Option<u16>,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");

    assert_eq!(config.db_url, "postgres://localhost:5432");
    assert_eq!(config.db_port, 9090);
    assert_eq!(config.default_port, 8080);
    assert_eq!(config.empty_string_default, "");
    assert_eq!(config.empty_option_default, None);
}

#[test]
fn test_env_var_missing_in_string() {
    unsafe {
        std::env::remove_var("MISSING_VAL");
    }
    let toml_str = r#"db_url = "${MISSING_VAL}""#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        db_url: String,
    }

    let result: Result<Config, env_toml::de::Error> = env_toml::from_str(toml_str);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("environment variable `MISSING_VAL` not set")
    );
}

#[test]
fn test_env_var_in_string_in_array() {
    unsafe {
        std::env::set_var("VAL1", "a");
        std::env::set_var("VAL2", "b");
    }

    let toml_str = r#"
list = [ "${VAL1}", "${VAL2:c}", "${VAL3:d}" ]
"#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        list: Vec<String>,
    }

    let config: Config = env_toml::from_str(toml_str).unwrap();
    assert_eq!(config.list, vec!["a", "b", "d"]);
}

#[test]
fn test_env_var_in_string_in_table() {
    unsafe {
        std::env::set_var("KEY1", "value1");
    }

    let toml_str = r#"
[table]
key = "${KEY1}"
"#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        table: Table,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct Table {
        key: String,
    }

    let config: Config = env_toml::from_str(toml_str).unwrap();
    assert_eq!(config.table.key, "value1");
}

#[test]
fn test_env_var_in_string_numeric_types() {
    unsafe {
        std::env::set_var("PORT", "8080");
        std::env::set_var("TIMEOUT", "3.01");
        std::env::set_var("DEBUG", "true");
        std::env::set_var("COUNT", "42");
    }

    let toml_str = r#"
port    = "${PORT}"
timeout = "${TIMEOUT}"
debug   = "${DEBUG}"
count   = "${COUNT}"
port_s  = "${PORT}"
"#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        port: u16,
        timeout: f64,
        debug: bool,
        count: i64,
        // Same env-var, but the field is String → raw string is preserved
        port_s: String,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");

    assert_eq!(config.port, 8080);
    assert!((config.timeout - 3.01).abs() < f64::EPSILON);
    assert!(config.debug);
    assert_eq!(config.count, 42);
    assert_eq!(config.port_s, "8080");
}

#[test]
fn test_visit_env_var_in_string_value_fallbacks() {
    unsafe {
        std::env::set_var("VAR_ARRAY", "[1, 2]");
        std::env::set_var("VAR_TABLE", "{ a = 1 }");
    }

    let toml_str = r#"
arr_s = "${VAR_ARRAY}"
tbl_s = "${VAR_TABLE}"
"#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        arr_s: String,
        tbl_s: String,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");

    // visit_env_var_value should fall back to the raw string for arrays/tables
    // and for non-TOML bare words.
    assert_eq!(config.arr_s, "[1, 2]");
    assert_eq!(config.tbl_s, "{ a = 1 }");

    // Attempting to deserialize the arr_s env-var as an actual sequence should fail,
    // because env-vars that parse as arrays are intentionally treated as strings.
    #[derive(Deserialize, Debug)]
    struct SeqConfig {
        arr_s: Vec<i64>,
    }

    let res: Result<SeqConfig, env_toml::de::Error> =
        env_toml::from_str(r#"arr_s = "${VAR_ARRAY}""#);
    assert!(res.is_err());
}

#[test]
fn test_env_var_in_string_optional_empty_default() {
    unsafe {
        std::env::remove_var("OPTIONAL_VAL");
    }
    let toml_str = r#"opt = "${OPTIONAL_VAL:}""#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        opt: Option<String>,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");
    assert_eq!(config.opt, None);
}

#[test]
fn test_env_var_in_string_optional_u16_empty_default() {
    unsafe {
        std::env::remove_var("OPTIONAL_PORT");
    }
    let toml_str = r#"port = "${OPTIONAL_PORT:}""#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        port: Option<u16>,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");
    assert_eq!(config.port, None);
}

#[test]
fn test_env_var_in_string_optional_no_default() {
    unsafe {
        std::env::remove_var("OPTIONAL_VAL_2");
    }
    let toml_str = r#"opt = "${OPTIONAL_VAL_2}""#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        opt: Option<String>,
    }

    let result: Result<Config, env_toml::de::Error> = env_toml::from_str(toml_str);
    // Should still be an error if no default is provided and env var is missing
    assert!(result.is_err());
}

#[test]
fn test_env_var_in_string_optional_with_default() {
    unsafe {
        std::env::remove_var("OPTIONAL_VAL_3");
    }
    let toml_str = r#"opt = "${OPTIONAL_VAL_3:default}""#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        opt: Option<String>,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");
    assert_eq!(config.opt, Some("default".to_owned()));
}

#[test]
fn test_env_var_in_string_non_optional_empty_default() {
    unsafe {
        std::env::remove_var("NON_OPTIONAL_VAL");
    }
    let toml_str = r#"val = "${NON_OPTIONAL_VAL:}""#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        val: String,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");
    // Should still be ""
    assert_eq!(config.val, "");
}

#[test]
fn test_env_var_in_string_optional_u16_present() {
    unsafe {
        std::env::set_var("PRESENT_PORT", "9090");
    }
    let toml_str = r#"port = "${PRESENT_PORT:8080}""#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        port: Option<u16>,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");
    assert_eq!(config.port, Some(9090));
}

#[test]
fn test_env_var_in_string_set_to_empty_string() {
    unsafe {
        std::env::set_var("SET_EMPTY_VAL", "");
    }
    let toml_str = r#"opt = "${SET_EMPTY_VAL:}""#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        opt: Option<String>,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");
    // It should now be None
    assert_eq!(config.opt, None);
}

#[test]
fn test_env_var_dollar_in_default_in_string() {
    unsafe {
        std::env::remove_var("MISSING_WITH_DOLLAR_DEFAULT");
    }
    // The default value itself contains a '$' — only possible via the quoted-string form.
    // e.g. a shell-style variable reference as a fallback: "${VAR:$OTHER}"
    let toml_str = r#"val = "${MISSING_WITH_DOLLAR_DEFAULT:$5.00}""#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        val: String,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");
    assert_eq!(config.val, "$5.00");
}

#[test]
fn test_env_var_dollar_in_default_set_from_env() {
    unsafe {
        std::env::set_var("PRESENT_WITH_DOLLAR_DEFAULT", "actual");
    }
    // When the env var IS set, the '$'-containing default is never used.
    let toml_str = r#"val = "${PRESENT_WITH_DOLLAR_DEFAULT:$fallback}""#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        val: String,
    }

    let config: Config = env_toml::from_str(toml_str).expect("failed to parse TOML");
    assert_eq!(config.val, "actual");
}
