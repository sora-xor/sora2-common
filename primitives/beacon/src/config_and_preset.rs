#[cfg(feature = "std")]
use crate::consts::altair;
use crate::prelude::*;
use crate::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use serde_json::Value;
use superstruct::superstruct;

/// Fusion of a runtime-config with the compile-time preset values.
///
/// Mostly useful for the API.
#[superstruct(
    variants(Bellatrix, Capella),
    variant_attributes(derive(Serialize, Deserialize, Debug, PartialEq, Clone))
)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub struct ConfigAndPreset {
    #[serde(flatten)]
    pub config: Config,

    #[serde(flatten)]
    pub base_preset: BasePreset,
    #[serde(flatten)]
    pub altair_preset: AltairPreset,
    #[serde(flatten)]
    pub bellatrix_preset: BellatrixPreset,
    #[superstruct(only(Capella))]
    #[serde(flatten)]
    pub capella_preset: CapellaPreset,
    /// The `extra_fields` map allows us to gracefully decode fields intended for future hard forks.
    #[serde(flatten)]
    #[cfg(feature = "std")]
    pub extra_fields: BTreeMap<String, Value>,
}

impl ConfigAndPreset {
    #[cfg(feature = "std")]
    pub fn from_chain_spec<T: EthSpec>(spec: &ChainSpec, fork_name: Option<ForkName>) -> Self {
        let config = Config::from_chain_spec::<T>(spec);
        let base_preset = BasePreset::from_chain_spec::<T>(spec);
        let altair_preset = AltairPreset::from_chain_spec::<T>(spec);
        let bellatrix_preset = BellatrixPreset::from_chain_spec::<T>(spec);
        let extra_fields = get_extra_fields(spec);

        if spec.capella_fork_epoch.is_some()
            || fork_name.is_none()
            || fork_name == Some(ForkName::Capella)
        {
            let capella_preset = CapellaPreset::from_chain_spec::<T>(spec);

            ConfigAndPreset::Capella(ConfigAndPresetCapella {
                config,
                base_preset,
                altair_preset,
                bellatrix_preset,
                capella_preset,
                extra_fields,
            })
        } else {
            ConfigAndPreset::Bellatrix(ConfigAndPresetBellatrix {
                config,
                base_preset,
                altair_preset,
                bellatrix_preset,
                extra_fields,
            })
        }
    }
}

/// Get a hashmap of constants to add to the `PresetAndConfig`
#[cfg(feature = "std")]
pub fn get_extra_fields(spec: &ChainSpec) -> BTreeMap<String, Value> {
    let hex_string = |value: &[u8]| format!("0x{}", hex::encode(value)).into();
    let u32_hex = |v: u32| hex_string(&v.to_le_bytes());
    let u8_hex = |v: u8| hex_string(&v.to_le_bytes());
    maplit::btreemap! {
        "bls_withdrawal_prefix".to_uppercase() => u8_hex(spec.bls_withdrawal_prefix_byte),
        "domain_beacon_proposer".to_uppercase() => u32_hex(spec.domain_beacon_proposer),
        "domain_beacon_attester".to_uppercase() => u32_hex(spec.domain_beacon_attester),
        "domain_blobs_sidecar".to_uppercase() => u32_hex(spec.domain_blobs_sidecar),
        "domain_randao".to_uppercase()=> u32_hex(spec.domain_randao),
        "domain_deposit".to_uppercase()=> u32_hex(spec.domain_deposit),
        "domain_voluntary_exit".to_uppercase() => u32_hex(spec.domain_voluntary_exit),
        "domain_selection_proof".to_uppercase() => u32_hex(spec.domain_selection_proof),
        "domain_aggregate_and_proof".to_uppercase() => u32_hex(spec.domain_aggregate_and_proof),
        "domain_application_mask".to_uppercase()=> u32_hex(spec.domain_application_mask),
        "target_aggregators_per_committee".to_uppercase() =>
            spec.target_aggregators_per_committee.to_string().into(),
        "random_subnets_per_validator".to_uppercase() =>
            spec.random_subnets_per_validator.to_string().into(),
        "epochs_per_random_subnet_subscription".to_uppercase() =>
            spec.epochs_per_random_subnet_subscription.to_string().into(),
        "domain_contribution_and_proof".to_uppercase() =>
            u32_hex(spec.domain_contribution_and_proof),
        "domain_sync_committee".to_uppercase() => u32_hex(spec.domain_sync_committee),
        "domain_sync_committee_selection_proof".to_uppercase() =>
            u32_hex(spec.domain_sync_committee_selection_proof),
        "sync_committee_subnet_count".to_uppercase() =>
            altair::SYNC_COMMITTEE_SUBNET_COUNT.to_string().into(),
        "target_aggregators_per_sync_subcommittee".to_uppercase() =>
            altair::TARGET_AGGREGATORS_PER_SYNC_SUBCOMMITTEE.to_string().into(),
    }
}
