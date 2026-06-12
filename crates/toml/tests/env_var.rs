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
empty_default = ${EMPTY_VAL:}
"#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        db_url: String,
        db_port: u16,
        default_port: u16,
        empty_default: String,
    }

    let config: Config = toml::from_str(toml_str).expect("failed to parse TOML");

    assert_eq!(config.db_url, "postgres://localhost:5432");
    assert_eq!(config.db_port, 9090);
    assert_eq!(config.default_port, 8080);
    assert_eq!(config.empty_default, "");
}

#[test]
fn test_env_var_missing() {
    unsafe { std::env::remove_var("MISSING_VAL"); }
    let toml_str = "db_url = ${MISSING_VAL}";

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        db_url: String,
    }

    let result: Result<Config, toml::de::Error> = toml::from_str(toml_str);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("environment variable `MISSING_VAL` not set"));
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

    let config: Config = toml::from_str(toml_str).unwrap();
    assert_eq!(config.list, vec!["a", "b", "d"]);
}

#[test]
fn test_env_var_in_table() {
    unsafe { std::env::set_var("KEY1", "value1"); }

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

    let config: Config = toml::from_str(toml_str).unwrap();
    assert_eq!(config.table.key, "value1");
}

#[test]
fn test_env_var_numeric_types() {
    unsafe {
        std::env::set_var("PORT", "8080");
        std::env::set_var("TIMEOUT", "3.14");
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

    let config: Config = toml::from_str(toml_str).expect("failed to parse TOML");

    assert_eq!(config.port, 8080);
    assert!((config.timeout - 3.14).abs() < f64::EPSILON);
    assert!(config.debug);
    assert_eq!(config.count, 42);
    assert_eq!(config.port_s, "8080");
}


#[test]
fn test_visit_env_var_value_fallbacks() {
    unsafe {
        std::env::set_var("VAR_ARRAY", "[1, 2]");
        std::env::set_var("VAR_TABLE", "{ a = 1 }");
        std::env::set_var("VAR_BARE", "unquoted_word");
    }

    let toml_str = r#"
arr_s = ${VAR_ARRAY}
tbl_s = ${VAR_TABLE}
bare_s = ${VAR_BARE}
"#;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        arr_s: String,
        tbl_s: String,
        bare_s: String,
    }

    let config: Config = toml::from_str(toml_str).expect("failed to parse TOML");

    // visit_env_var_value should fall back to the raw string for arrays/tables
    // and for non-TOML bare words.
    assert_eq!(config.arr_s, "[1, 2]");
    assert_eq!(config.tbl_s, "{ a = 1 }");
    assert_eq!(config.bare_s, "unquoted_word");

    // Attempting to deserialize the arr_s env-var as an actual sequence should fail,
    // because env-vars that parse as arrays are intentionally treated as strings.
    #[derive(Deserialize, Debug)]
    struct SeqConfig {
        arr_s: Vec<i64>,
    }

    let res: Result<SeqConfig, toml::de::Error> = toml::from_str(r#"arr_s = ${VAR_ARRAY}"#);
    assert!(res.is_err());
}


