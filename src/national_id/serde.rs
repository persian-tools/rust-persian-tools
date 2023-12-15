//! [serde] helpers to deserialize Iranian National Number. Enabled if `serde` Cargo feature is enabled.

use crate::national_id::verify_iranian_national_id;
use serde::Deserializer;

struct NationalId;
struct NationalIdOption;

impl<'de> serde::de::Visitor<'de> for NationalId {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expecting Iranian national-id, e.g. 0076229645")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        verify_iranian_national_id(s)
            .map(|_| s.to_string())
            .map_err(serde::de::Error::custom)
    }
}

impl<'de> serde::de::Visitor<'de> for NationalIdOption {
    type Value = Option<String>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expecting Iranian national-id, e.g. 0076229645")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::Deserialize;
        let s: Option<String> = Option::deserialize(d)?;
        if let Some(s) = s {
            verify_iranian_national_id(&s)
                .map(|_| Some(s))
                .map_err(serde::de::Error::custom)
        } else {
            Ok(None)
        }
    }
}

/// Deserializes Iranian National Number in [serde].
///
/// For more info see [crate::national_id] module example.
pub fn national_id_de<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(NationalId)
}

/// Deserializes Iranian National Number (if exists) in [serde].
///
/// For more info see [crate::national_id] module example.
pub fn national_id_option_de<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(NationalIdOption)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::national_id::NationalIdError;
    use serde::Deserialize;

    #[derive(Debug, PartialEq, Deserialize)]
    struct FooStr {
        id: String,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct FooNationalId {
        #[serde(deserialize_with = "national_id_de")]
        id: String,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct FooOptionStr {
        #[serde(default)]
        id: Option<String>,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct FooOptionNationalId {
        #[serde(default, deserialize_with = "national_id_option_de")]
        id: Option<String>,
    }

    #[test]
    fn de() {
        // Valid:
        let json_str = "{\"id\": \"0076229645\"}";
        assert!(serde_json::from_str::<FooStr>(json_str).is_ok());
        assert!(serde_json::from_str::<FooOptionStr>(json_str).is_ok());
        assert!(serde_json::from_str::<FooNationalId>(json_str).is_ok());
        assert_eq!(
            serde_json::from_str::<FooNationalId>(json_str).unwrap(),
            FooNationalId {
                id: "0076229645".to_string()
            }
        );
        assert!(serde_json::from_str::<FooOptionNationalId>(json_str).is_ok());
        assert_eq!(
            serde_json::from_str::<FooOptionNationalId>(json_str).unwrap(),
            FooOptionNationalId {
                id: Some("0076229645".to_string())
            }
        );

        // Invalid:
        let json_str = "{\"id\": \"12345\"}";
        assert!(serde_json::from_str::<FooStr>(json_str).is_ok());
        assert!(serde_json::from_str::<FooNationalId>(json_str).is_err());
        assert!(serde_json::from_str::<FooOptionNationalId>(json_str)
            .err()
            .unwrap()
            .to_string()
            .find(&NationalIdError::Length(5).to_string())
            .is_some());
        assert!(serde_json::from_str::<FooOptionStr>(json_str).is_ok());
        assert!(serde_json::from_str::<FooOptionNationalId>(json_str).is_err());
        assert!(serde_json::from_str::<FooOptionNationalId>(json_str)
            .err()
            .unwrap()
            .to_string()
            .find(&NationalIdError::Length(5).to_string())
            .is_some());

        // Test Option::None
        let json_str = "{}";
        assert!(serde_json::from_str::<FooOptionNationalId>(json_str).is_ok());
        assert!(serde_json::from_str::<FooOptionNationalId>(json_str)
            .unwrap()
            .id
            .is_none());
    }
}
