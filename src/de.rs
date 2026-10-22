//! Lenient deserializers for Shiprocket responses.
//!
//! The API is inconsistent about types: ids and amounts come back as numbers on one
//! endpoint and as strings on another, booleans as `0`/`1`, empty lists as `{}` or `null`.
//! Every helper here accepts all of those shapes and never fails on a type mismatch,
//! falling back to `None` / empty instead. Use them with `#[serde(default, deserialize_with = "...")]`.

use serde::{Deserialize, Deserializer, de::DeserializeOwned};
use serde_json::Value;

/// String, number or bool as `Option<String>`. Empty strings become `None`.
pub fn opt_string<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    Ok(match Value::deserialize(d)? {
        Value::String(s) if s.is_empty() => None,
        Value::String(s) => Some(s),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    })
}

/// Number or numeric string as `Option<i64>`.
pub fn opt_i64<'de, D: Deserializer<'de>>(d: D) -> Result<Option<i64>, D::Error> {
    Ok(match Value::deserialize(d)? {
        Value::Number(n) => n.as_i64().or_else(|| n.as_f64().map(|f| f as i64)),
        Value::String(s) => {
            let s = s.trim();
            s.parse::<i64>()
                .ok()
                .or_else(|| s.parse::<f64>().ok().map(|f| f as i64))
        }
        Value::Bool(b) => Some(b as i64),
        _ => None,
    })
}

/// Number or numeric string (commas allowed) as `Option<f64>`.
pub fn opt_f64<'de, D: Deserializer<'de>>(d: D) -> Result<Option<f64>, D::Error> {
    Ok(match Value::deserialize(d)? {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.trim().replace(',', "").parse::<f64>().ok(),
        _ => None,
    })
}

/// `true`/`false`, `0`/`1`, `"0"`/`"1"`, `"true"`/`"false"`, `"yes"`/`"no"` as `Option<bool>`.
pub fn opt_bool<'de, D: Deserializer<'de>>(d: D) -> Result<Option<bool>, D::Error> {
    Ok(match Value::deserialize(d)? {
        Value::Bool(b) => Some(b),
        Value::Number(n) => n.as_f64().map(|f| f != 0.0),
        Value::String(s) => match s.trim().to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "y" => Some(true),
            "0" | "false" | "no" | "n" => Some(false),
            _ => None,
        },
        _ => None,
    })
}

/// A list that may also arrive as `null`, `{}` or an object keyed by index.
/// Elements that fail to decode are skipped.
pub fn vec<'de, D: Deserializer<'de>, T: DeserializeOwned>(d: D) -> Result<Vec<T>, D::Error> {
    let items = match Value::deserialize(d)? {
        Value::Array(items) => items,
        Value::Object(map) => map.into_iter().map(|(_, v)| v).collect(),
        _ => Vec::new(),
    };
    Ok(items
        .into_iter()
        .filter_map(|v| serde_json::from_value(v).ok())
        .collect())
}

/// A nested object that may also arrive as `null`, `[]`, `""` or with the wrong shape.
pub fn opt_struct<'de, D: Deserializer<'de>, T: DeserializeOwned>(
    d: D,
) -> Result<Option<T>, D::Error> {
    Ok(match Value::deserialize(d)? {
        v @ Value::Object(_) => serde_json::from_value(v).ok(),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize)]
    struct T {
        #[serde(default, deserialize_with = "opt_i64")]
        i: Option<i64>,
        #[serde(default, deserialize_with = "opt_f64")]
        f: Option<f64>,
        #[serde(default, deserialize_with = "opt_string")]
        s: Option<String>,
        #[serde(default, deserialize_with = "opt_bool")]
        b: Option<bool>,
        #[serde(default, deserialize_with = "vec")]
        v: Vec<i64>,
    }

    #[test]
    fn accepts_mixed_shapes() {
        let t: T =
            serde_json::from_str(r#"{"i":"42","f":"1,234.5","s":17,"b":"1","v":{}}"#).unwrap();
        assert_eq!(t.i, Some(42));
        assert_eq!(t.f, Some(1234.5));
        assert_eq!(t.s.as_deref(), Some("17"));
        assert_eq!(t.b, Some(true));
        assert!(t.v.is_empty());

        let t: T = serde_json::from_str(r#"{"i":"","f":null,"s":"","b":0,"v":[1,"x",2]}"#).unwrap();
        assert_eq!(t.i, None);
        assert_eq!(t.f, None);
        assert_eq!(t.s, None);
        assert_eq!(t.b, Some(false));
        assert_eq!(t.v, vec![1, 2]);

        let t: T = serde_json::from_str("{}").unwrap();
        assert_eq!(t.i, None);
    }
}
