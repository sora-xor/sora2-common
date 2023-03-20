use crate::prelude::*;
use crate::*;
use bls::Signature;
use core::fmt;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use superstruct::superstruct;
use tree_hash::TreeHash;
use tree_hash_derive::TreeHash;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct SignedBeaconBlockHash(Hash256);

impl fmt::Debug for SignedBeaconBlockHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SignedBeaconBlockHash({:?})", self.0)
    }
}

impl fmt::Display for SignedBeaconBlockHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Hash256> for SignedBeaconBlockHash {
    fn from(hash: Hash256) -> SignedBeaconBlockHash {
        SignedBeaconBlockHash(hash)
    }
}

impl From<SignedBeaconBlockHash> for Hash256 {
    fn from(signed_beacon_block_hash: SignedBeaconBlockHash) -> Hash256 {
        signed_beacon_block_hash.0
    }
}

/// A `BeaconBlock` and a signature from its proposer.
#[superstruct(
    variants(Base, Altair, Merge, Capella),
    variant_attributes(
        derive(
            Debug,
            Clone,
            Serialize,
            Deserialize,
            Encode,
            Decode,
            TreeHash,
            Derivative,
            ScaleEncode,
            ScaleDecode,
            TypeInfo,
            MaxEncodedLen,
        ),
        derivative(PartialEq, Hash(bound = "E: EthSpec")),
        serde(bound = "E: EthSpec, Payload: AbstractExecPayload<E>"),
    ),
    map_into(BeaconBlock),
    map_ref_into(BeaconBlockRef),
    map_ref_mut_into(BeaconBlockRefMut)
)]
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Encode,
    TreeHash,
    Derivative,
    ScaleEncode,
    ScaleDecode,
    TypeInfo,
    MaxEncodedLen,
)]
#[derivative(PartialEq, Hash(bound = "E: EthSpec"))]
#[serde(untagged)]
#[serde(bound = "E: EthSpec, Payload: AbstractExecPayload<E>")]
#[tree_hash(enum_behaviour = "transparent")]
#[ssz(enum_behaviour = "transparent")]
pub struct SignedBeaconBlock<E: EthSpec, Payload: AbstractExecPayload<E> = FullPayload<E>> {
    #[superstruct(only(Base), partial_getter(rename = "message_base"))]
    pub message: BeaconBlockBase<E, Payload>,
    #[superstruct(only(Altair), partial_getter(rename = "message_altair"))]
    pub message: BeaconBlockAltair<E, Payload>,
    #[superstruct(only(Merge), partial_getter(rename = "message_merge"))]
    pub message: BeaconBlockMerge<E, Payload>,
    #[superstruct(only(Capella), partial_getter(rename = "message_capella"))]
    pub message: BeaconBlockCapella<E, Payload>,
    pub signature: Signature,
}

pub type SignedBlindedBeaconBlock<E> = SignedBeaconBlock<E, BlindedPayload<E>>;

impl<E: EthSpec, Payload: AbstractExecPayload<E>> SignedBeaconBlock<E, Payload> {
    /// Returns the name of the fork pertaining to `self`.
    ///
    /// Will return an `Err` if `self` has been instantiated to a variant conflicting with the fork
    /// dictated by `self.slot()`.
    pub fn fork_name(&self, spec: &ChainSpec) -> Result<ForkName, InconsistentFork> {
        self.message().fork_name(spec)
    }

    /// SSZ decode with fork variant determined by slot.
    pub fn from_ssz_bytes(bytes: &[u8], spec: &ChainSpec) -> Result<Self, ssz::DecodeError> {
        Self::from_ssz_bytes_with(bytes, |bytes| BeaconBlock::from_ssz_bytes(bytes, spec))
    }

    /// SSZ decode which attempts to decode all variants (slow).
    pub fn any_from_ssz_bytes(bytes: &[u8]) -> Result<Self, ssz::DecodeError> {
        Self::from_ssz_bytes_with(bytes, BeaconBlock::any_from_ssz_bytes)
    }

    /// SSZ decode with custom decode function.
    pub fn from_ssz_bytes_with(
        bytes: &[u8],
        block_decoder: impl FnOnce(&[u8]) -> Result<BeaconBlock<E, Payload>, ssz::DecodeError>,
    ) -> Result<Self, ssz::DecodeError> {
        // We need the customer decoder for `BeaconBlock`, which doesn't compose with the other
        // SSZ utils, so we duplicate some parts of `ssz_derive` here.
        let mut builder = ssz::SszDecoderBuilder::new(bytes);

        builder.register_anonymous_variable_length_item()?;
        builder.register_type::<Signature>()?;

        let mut decoder = builder.build()?;

        // Read the first item as a `BeaconBlock`.
        let message = decoder.decode_next_with(block_decoder)?;
        let signature = decoder.decode_next()?;

        Ok(Self::from_block(message, signature))
    }

    /// Create a new `SignedBeaconBlock` from a `BeaconBlock` and `Signature`.
    pub fn from_block(block: BeaconBlock<E, Payload>, signature: Signature) -> Self {
        match block {
            BeaconBlock::Base(message) => {
                SignedBeaconBlock::Base(SignedBeaconBlockBase { message, signature })
            }
            BeaconBlock::Altair(message) => {
                SignedBeaconBlock::Altair(SignedBeaconBlockAltair { message, signature })
            }
            BeaconBlock::Merge(message) => {
                SignedBeaconBlock::Merge(SignedBeaconBlockMerge { message, signature })
            }
            BeaconBlock::Capella(message) => {
                SignedBeaconBlock::Capella(SignedBeaconBlockCapella { message, signature })
            }
        }
    }

    /// Deconstruct the `SignedBeaconBlock` into a `BeaconBlock` and `Signature`.
    ///
    /// This is necessary to get a `&BeaconBlock` from a `SignedBeaconBlock` because
    /// `SignedBeaconBlock` only contains a `BeaconBlock` _variant_.
    pub fn deconstruct(self) -> (BeaconBlock<E, Payload>, Signature) {
        map_signed_beacon_block_into_beacon_block!(self, |block, beacon_block_cons| {
            (beacon_block_cons(block.message), block.signature)
        })
    }

    /// Accessor for the block's `message` field as a ref.
    pub fn message<'a>(&'a self) -> BeaconBlockRef<'a, E, Payload> {
        map_signed_beacon_block_ref_into_beacon_block_ref!(
            &'a _,
            self.to_ref(),
            |inner, cons| cons(&inner.message)
        )
    }

    /// Accessor for the block's `message` as a mutable reference (for testing only).
    pub fn message_mut<'a>(&'a mut self) -> BeaconBlockRefMut<'a, E, Payload> {
        map_signed_beacon_block_ref_mut_into_beacon_block_ref_mut!(
            &'a _,
            self.to_mut(),
            |inner, cons| cons(&mut inner.message)
        )
    }

    /// Verify `self.signature`.
    ///
    /// If the root of `block.message` is already known it can be passed in via `object_root_opt`.
    /// Otherwise, it will be computed locally.
    pub fn verify_signature(
        &self,
        object_root_opt: Option<Hash256>,
        pubkey: &PublicKey,
        fork: &Fork,
        genesis_validators_root: Hash256,
        spec: &ChainSpec,
    ) -> bool {
        // Refuse to verify the signature of a block if its structure does not match the fork at
        // `self.slot()`.
        if self.fork_name(spec).is_err() {
            return false;
        }

        let domain = spec.get_domain(
            self.slot().epoch(E::slots_per_epoch()),
            Domain::BeaconProposer,
            fork,
            genesis_validators_root,
        );

        let message = if let Some(object_root) = object_root_opt {
            SigningData {
                object_root,
                domain,
            }
            .tree_hash_root()
        } else {
            self.message().signing_root(domain)
        };

        self.signature().verify(pubkey, message)
    }

    /// Produce a signed beacon block header corresponding to this block.
    pub fn signed_block_header(&self) -> SignedBeaconBlockHeader {
        SignedBeaconBlockHeader {
            message: self.message().block_header(),
            signature: self.signature().clone(),
        }
    }

    /// Convenience accessor for the block's slot.
    pub fn slot(&self) -> Slot {
        self.message().slot()
    }

    /// Convenience accessor for the block's parent root.
    pub fn parent_root(&self) -> Hash256 {
        self.message().parent_root()
    }

    /// Convenience accessor for the block's state root.
    pub fn state_root(&self) -> Hash256 {
        self.message().state_root()
    }

    /// Returns the `tree_hash_root` of the block.
    pub fn canonical_root(&self) -> Hash256 {
        self.message().tree_hash_root()
    }
}

// We can convert pre-Bellatrix blocks without payloads into blocks with payloads.
impl<E: EthSpec> From<SignedBeaconBlockBase<E, BlindedPayload<E>>>
    for SignedBeaconBlockBase<E, FullPayload<E>>
{
    fn from(signed_block: SignedBeaconBlockBase<E, BlindedPayload<E>>) -> Self {
        let SignedBeaconBlockBase { message, signature } = signed_block;
        SignedBeaconBlockBase {
            message: message.into(),
            signature,
        }
    }
}

impl<E: EthSpec> From<SignedBeaconBlockAltair<E, BlindedPayload<E>>>
    for SignedBeaconBlockAltair<E, FullPayload<E>>
{
    fn from(signed_block: SignedBeaconBlockAltair<E, BlindedPayload<E>>) -> Self {
        let SignedBeaconBlockAltair { message, signature } = signed_block;
        SignedBeaconBlockAltair {
            message: message.into(),
            signature,
        }
    }
}

// Post-Bellatrix blocks can be "unblinded" by adding the full payload.
// NOTE: It might be nice to come up with a `superstruct` pattern to abstract over this before
// the first fork after Bellatrix.
impl<E: EthSpec> SignedBeaconBlockMerge<E, BlindedPayload<E>> {
    pub fn into_full_block(
        self,
        execution_payload: ExecutionPayloadMerge<E>,
    ) -> SignedBeaconBlockMerge<E, FullPayload<E>> {
        let SignedBeaconBlockMerge {
            message:
                BeaconBlockMerge {
                    slot,
                    proposer_index,
                    parent_root,
                    state_root,
                    body:
                        BeaconBlockBodyMerge {
                            randao_reveal,
                            eth1_data,
                            graffiti,
                            proposer_slashings,
                            attester_slashings,
                            attestations,
                            deposits,
                            voluntary_exits,
                            sync_aggregate,
                            execution_payload: BlindedPayloadMerge { .. },
                        },
                },
            signature,
        } = self;
        SignedBeaconBlockMerge {
            message: BeaconBlockMerge {
                slot,
                proposer_index,
                parent_root,
                state_root,
                body: BeaconBlockBodyMerge {
                    randao_reveal,
                    eth1_data,
                    graffiti,
                    proposer_slashings,
                    attester_slashings,
                    attestations,
                    deposits,
                    voluntary_exits,
                    sync_aggregate,
                    execution_payload: FullPayloadMerge { execution_payload },
                },
            },
            signature,
        }
    }
}

impl<E: EthSpec> SignedBeaconBlockCapella<E, BlindedPayload<E>> {
    pub fn into_full_block(
        self,
        execution_payload: ExecutionPayloadCapella<E>,
    ) -> SignedBeaconBlockCapella<E, FullPayload<E>> {
        let SignedBeaconBlockCapella {
            message:
                BeaconBlockCapella {
                    slot,
                    proposer_index,
                    parent_root,
                    state_root,
                    body:
                        BeaconBlockBodyCapella {
                            randao_reveal,
                            eth1_data,
                            graffiti,
                            proposer_slashings,
                            attester_slashings,
                            attestations,
                            deposits,
                            voluntary_exits,
                            sync_aggregate,
                            execution_payload: BlindedPayloadCapella { .. },
                            bls_to_execution_changes,
                        },
                },
            signature,
        } = self;
        SignedBeaconBlockCapella {
            message: BeaconBlockCapella {
                slot,
                proposer_index,
                parent_root,
                state_root,
                body: BeaconBlockBodyCapella {
                    randao_reveal,
                    eth1_data,
                    graffiti,
                    proposer_slashings,
                    attester_slashings,
                    attestations,
                    deposits,
                    voluntary_exits,
                    sync_aggregate,
                    execution_payload: FullPayloadCapella { execution_payload },
                    bls_to_execution_changes,
                },
            },
            signature,
        }
    }
}

impl<E: EthSpec> SignedBeaconBlock<E, BlindedPayload<E>> {
    pub fn try_into_full_block(
        self,
        execution_payload: Option<ExecutionPayload<E>>,
    ) -> Option<SignedBeaconBlock<E, FullPayload<E>>> {
        let full_block = match (self, execution_payload) {
            (SignedBeaconBlock::Base(block), _) => SignedBeaconBlock::Base(block.into()),
            (SignedBeaconBlock::Altair(block), _) => SignedBeaconBlock::Altair(block.into()),
            (SignedBeaconBlock::Merge(block), Some(ExecutionPayload::Merge(payload))) => {
                SignedBeaconBlock::Merge(block.into_full_block(payload))
            }
            (SignedBeaconBlock::Capella(block), Some(ExecutionPayload::Capella(payload))) => {
                SignedBeaconBlock::Capella(block.into_full_block(payload))
            }
            // avoid wildcard matching forks so that compiler will
            // direct us here when a new fork has been added
            (SignedBeaconBlock::Merge(_), _) => return None,
            (SignedBeaconBlock::Capella(_), _) => return None,
        };
        Some(full_block)
    }
}

// We can blind blocks with payloads by converting the payload into a header.
//
// We can optionally keep the header, or discard it.
impl<E: EthSpec> From<SignedBeaconBlock<E>>
    for (SignedBlindedBeaconBlock<E>, Option<ExecutionPayload<E>>)
{
    fn from(signed_block: SignedBeaconBlock<E>) -> Self {
        let (block, signature) = signed_block.deconstruct();
        let (blinded_block, payload) = block.into();
        (
            SignedBeaconBlock::from_block(blinded_block, signature),
            payload,
        )
    }
}

impl<E: EthSpec> From<SignedBeaconBlock<E>> for SignedBlindedBeaconBlock<E> {
    fn from(signed_block: SignedBeaconBlock<E>) -> Self {
        let (blinded_block, _) = signed_block.into();
        blinded_block
    }
}

// We can blind borrowed blocks with payloads by converting the payload into a header (without
// cloning the payload contents).
impl<E: EthSpec> SignedBeaconBlock<E> {
    pub fn clone_as_blinded(&self) -> SignedBlindedBeaconBlock<E> {
        SignedBeaconBlock::from_block(self.message().into(), self.signature().clone())
    }
}

#[cfg(feature = "std")]
impl<E: EthSpec, Payload: AbstractExecPayload<E>> ForkVersionDeserialize
    for SignedBeaconBlock<E, Payload>
{
    fn deserialize_by_fork<'de, D: serde::Deserializer<'de>>(
        value: serde_json::value::Value,
        fork_name: ForkName,
    ) -> Result<Self, D::Error> {
        Ok(map_fork_name!(
            fork_name,
            Self,
            serde_json::from_value(value).map_err(|e| serde::de::Error::custom(format!(
                "SignedBeaconBlock failed to deserialize: {:?}",
                e
            )))?
        ))
    }
}