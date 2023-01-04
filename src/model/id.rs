use serde::{de, Deserialize, Serialize};
use std::{fmt::Write, num::ParseIntError};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Id(Vec<u8>);

impl Id {
    pub fn from_slice(value: &[u8]) -> Self {
        let inner = value.to_vec();

        Id(inner)
    }

    pub fn from_hex_string(value: &str) -> Result<Self, ParseIntError> {
        let result: Result<Vec<u8>, ParseIntError> = (0..value.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&value[i..i + 2], 16))
            .collect();

        result.map(|inner| Id(inner))
    }

    pub fn to_hex_string(&self) -> String {
        let mut result = String::with_capacity(self.0.len() * 2);

        for byte in &self.0 {
            write!(&mut result, "{:02x}", byte).unwrap();
        }

        result
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_hex_string())
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct IdVisitor;

        impl<'de> de::Visitor<'de> for IdVisitor {
            type Value = Id;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Id")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Id::from_hex_string(v).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_string(IdVisitor)
    }
}
