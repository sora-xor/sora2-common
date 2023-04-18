use core::marker::PhantomData;

use super::BeaconBlockHeader;
use crate::{
    light_client_update::ExecutionPayloadProofLen, prelude::*, EthSpec,
    ExecutionPayloadHeaderCapella, Hash256,
};
use serde::{Deserialize, Serialize};
use ssz_types::FixedVector;

#[superstruct(
    variants(Merge, Capella),
    variant_attributes(
        derive(
            Debug,
            Clone,
            Serialize,
            Deserialize,
            Derivative,
            ScaleEncode,
            ScaleDecode,
            TypeInfo,
            MaxEncodedLen,
        ),
        derivative(PartialEq, Hash(bound = "T: EthSpec")),
        serde(bound = "T: EthSpec", deny_unknown_fields),
        scale_info(skip_type_params(T))
    ),
    ref_attributes(derive(PartialEq))
)]
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Derivative,
    ScaleEncode,
    ScaleDecode,
    TypeInfo,
    MaxEncodedLen,
)]
#[derivative(PartialEq, Hash(bound = "T: EthSpec"))]
#[serde(bound = "T: EthSpec", untagged)]
#[scale_info(skip_type_params(T))]
pub struct LightClientHeader<T: EthSpec> {
    pub beacon: BeaconBlockHeader,
    #[superstruct(only(Capella))]
    pub execution: ExecutionPayloadHeaderCapella<T>,
    #[superstruct(only(Capella))]
    pub execution_branch: FixedVector<Hash256, ExecutionPayloadProofLen>,
    #[superstruct(only(Merge))]
    #[serde(skip)]
    _phantom: PhantomData<T>,
}

impl<'a, T: EthSpec> LightClientHeaderRef<'a, T> {
    pub fn owned(&self) -> LightClientHeader<T> {
        match *self {
            Self::Merge(update) => LightClientHeader::Merge(update.clone()),
            Self::Capella(update) => LightClientHeader::Capella(update.clone()),
        }
    }
}

#[cfg(feature = "std")]
impl<T: EthSpec> crate::ForkVersionDeserialize for LightClientHeader<T> {
    fn deserialize_by_fork<'de, D: serde::Deserializer<'de>>(
        value: serde_json::value::Value,
        fork_name: crate::ForkName,
    ) -> Result<Self, D::Error> {
        let convert_err = |e| {
            serde::de::Error::custom(format!(
                "ExecutionPayloadHeader failed to deserialize: {:?}",
                e
            ))
        };

        Ok(match fork_name {
            crate::ForkName::Merge => {
                Self::Merge(serde_json::from_value(value).map_err(convert_err)?)
            }
            crate::ForkName::Capella => {
                Self::Capella(serde_json::from_value(value).map_err(convert_err)?)
            }
            crate::ForkName::Base | crate::ForkName::Altair => {
                return Err(serde::de::Error::custom(format!(
                    "ExecutionPayloadHeader failed to deserialize: unsupported fork '{}'",
                    fork_name
                )));
            }
        })
    }
}
