#![recursion_limit = "256"]
#![cfg(all(feature = "parse", feature = "display", feature = "serde"))]

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(t) => t,
            Err(e) => panic!("{} failed with {}", stringify!($e), e),
        }
    };
}

mod de_enum;
mod de_errors;
mod de_key;
mod general;
mod ser_enum;
mod ser_key;
mod ser_tables_last;
mod ser_to_string;
mod ser_to_string_pretty;
mod spanned;

use env_toml::Spanned;
use env_toml::from_str;
use env_toml::to_string;
use env_toml::to_string_pretty;
use env_toml::value::Date;
use env_toml::value::Datetime;
use env_toml::value::Time;

use env_toml::Table as SerdeDocument;
use env_toml::Table as SerdeTable;
use env_toml::Value as SerdeValue;

fn value_from_str<T>(s: &'_ str) -> Result<T, env_toml::de::Error>
where
    T: serde::de::DeserializeOwned,
{
    T::deserialize(env_toml::de::ValueDeserializer::parse(s)?)
}

fn to_string_value<T>(value: &T) -> Result<String, env_toml::ser::Error>
where
    T: serde::ser::Serialize + ?Sized,
{
    let mut output = String::new();
    let serializer = env_toml::ser::ValueSerializer::new(&mut output);
    value.serialize(serializer)?;
    Ok(output)
}
