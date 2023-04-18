pub mod serde_str {
    use core::str::FromStr;
    use serde::Deserialize;

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: TryFrom<u128>,
        <T as TryFrom<u128>>::Error: std::fmt::Debug,
    {
        let value = String::deserialize(deserializer)?;
        let res = u128::from_str(&value).map_err(|err| {
            serde::de::Error::custom(format!("Failed to deserialize from string: {:?}", err))
        })?;
        let res = T::try_from(res).map_err(|err| {
            serde::de::Error::custom(format!("Failed to deserialize from string: {:?}", err))
        })?;
        Ok(res)
    }

    pub fn serialize<T: ToString, S: serde::Serializer>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.to_string())
    }
}

pub mod serde_hex {
    use serde::Deserialize;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let value = if value.len() > 1 && value[0..2] == *"0x" {
            &value[2..]
        } else {
            &value
        };
        let res = hex::decode(value).map_err(|err| {
            serde::de::Error::custom(format!("Failed to deserialize from hex: {:?}", err))
        })?;
        Ok(res)
    }

    pub fn serialize<S: serde::Serializer>(value: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("0x{}", hex::encode(value)))
    }
}
