#[cfg(not(feature = "std"))]
use crate::prelude::*;
use crate::ForkName;
use serde::Serialize;
#[cfg(feature = "std")]
use serde::{de::DeserializeOwned, Deserialize, Deserializer};
#[cfg(feature = "std")]
use serde_json::value::Value;

// Deserialize is only implemented for types that implement ForkVersionDeserialize
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct ExecutionOptimisticForkVersionedResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<ForkName>,
    pub execution_optimistic: Option<bool>,
    pub data: T,
}

#[cfg(feature = "std")]
impl<'de, F> serde::Deserialize<'de> for ExecutionOptimisticForkVersionedResponse<F>
where
    F: ForkVersionDeserialize,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            version: Option<ForkName>,
            execution_optimistic: Option<bool>,
            data: serde_json::Value,
        }

        let helper = Helper::deserialize(deserializer)?;
        let data = match helper.version {
            Some(fork_name) => F::deserialize_by_fork::<'de, D>(helper.data, fork_name)?,
            None => serde_json::from_value(helper.data).map_err(serde::de::Error::custom)?,
        };

        Ok(ExecutionOptimisticForkVersionedResponse {
            version: helper.version,
            execution_optimistic: helper.execution_optimistic,
            data,
        })
    }
}

#[cfg(feature = "std")]
pub trait ForkVersionDeserialize: Sized + DeserializeOwned {
    fn deserialize_by_fork<'de, D: Deserializer<'de>>(
        value: Value,
        fork_name: ForkName,
    ) -> Result<Self, D::Error>;
}

// Deserialize is only implemented for types that implement ForkVersionDeserialize
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct ForkVersionedResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<ForkName>,
    pub data: T,
}

#[cfg(feature = "std")]
impl<'de, F> serde::Deserialize<'de> for ForkVersionedResponse<F>
where
    F: ForkVersionDeserialize,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            version: Option<ForkName>,
            data: serde_json::Value,
        }

        let helper = Helper::deserialize(deserializer)?;
        let data = match helper.version {
            Some(fork_name) => F::deserialize_by_fork::<'de, D>(helper.data, fork_name)?,
            None => serde_json::from_value(helper.data).map_err(serde::de::Error::custom)?,
        };

        Ok(ForkVersionedResponse {
            version: helper.version,
            data,
        })
    }
}
