use crate::beacon_block_body::{
    BeaconBlockBodyAltair, BeaconBlockBodyBase, BeaconBlockBodyMerge, BeaconBlockBodyRef,
    BeaconBlockBodyRefMut,
};
use crate::prelude::*;
use crate::*;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use ssz::Decode;
use ssz_derive::{Decode, Encode};
use superstruct::superstruct;
use tree_hash::TreeHash;
use tree_hash_derive::TreeHash;

/// A block of the `BeaconChain`.
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
        derivative(PartialEq, Hash(bound = "T: EthSpec, Payload: AbstractExecPayload<T>")),
        serde(
            bound = "T: EthSpec, Payload: AbstractExecPayload<T>",
            deny_unknown_fields
        ),
        scale_info(skip_type_params(T))
    ),
    ref_attributes(
        derive(Debug, PartialEq, TreeHash),
        tree_hash(enum_behaviour = "transparent")
    ),
    map_ref_into(BeaconBlockBodyRef, BeaconBlock),
    map_ref_mut_into(BeaconBlockBodyRefMut)
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
#[derivative(PartialEq, Hash(bound = "T: EthSpec"))]
#[serde(untagged)]
#[serde(bound = "T: EthSpec, Payload: AbstractExecPayload<T>")]
#[tree_hash(enum_behaviour = "transparent")]
#[ssz(enum_behaviour = "transparent")]
#[scale_info(skip_type_params(T))]
pub struct BeaconBlock<T: EthSpec, Payload: AbstractExecPayload<T> = FullPayload<T>> {
    #[superstruct(getter(copy))]
    pub slot: Slot,
    #[superstruct(getter(copy))]
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub proposer_index: u64,
    #[superstruct(getter(copy))]
    pub parent_root: Hash256,
    #[superstruct(getter(copy))]
    pub state_root: Hash256,
    #[superstruct(only(Base), partial_getter(rename = "body_base"))]
    pub body: BeaconBlockBodyBase<T, Payload>,
    #[superstruct(only(Altair), partial_getter(rename = "body_altair"))]
    pub body: BeaconBlockBodyAltair<T, Payload>,
    #[superstruct(only(Merge), partial_getter(rename = "body_merge"))]
    pub body: BeaconBlockBodyMerge<T, Payload>,
    #[superstruct(only(Capella), partial_getter(rename = "body_capella"))]
    pub body: BeaconBlockBodyCapella<T, Payload>,
}

pub type BlindedBeaconBlock<E> = BeaconBlock<E, BlindedPayload<E>>;

impl<T: EthSpec, Payload: AbstractExecPayload<T>> SignedRoot for BeaconBlock<T, Payload> {}
impl<'a, T: EthSpec, Payload: AbstractExecPayload<T>> SignedRoot
    for BeaconBlockRef<'a, T, Payload>
{
}

impl<T: EthSpec, Payload: AbstractExecPayload<T>> BeaconBlock<T, Payload> {
    /// Try decoding each beacon block variant in sequence.
    ///
    /// This is *not* recommended unless you really have no idea what variant the block should be.
    /// Usually it's better to prefer `from_ssz_bytes` which will decode the correct variant based
    /// on the fork slot.
    pub fn any_from_ssz_bytes(bytes: &[u8]) -> Result<Self, ssz::DecodeError> {
        BeaconBlockCapella::from_ssz_bytes(bytes)
            .map(BeaconBlock::Capella)
            .or_else(|_| BeaconBlockMerge::from_ssz_bytes(bytes).map(BeaconBlock::Merge))
            .or_else(|_| BeaconBlockAltair::from_ssz_bytes(bytes).map(BeaconBlock::Altair))
            .or_else(|_| BeaconBlockBase::from_ssz_bytes(bytes).map(BeaconBlock::Base))
    }

    /// Convenience accessor for the `body` as a `BeaconBlockBodyRef`.
    pub fn body(&self) -> BeaconBlockBodyRef<'_, T, Payload> {
        self.to_ref().body()
    }

    /// Convenience accessor for the `body` as a `BeaconBlockBodyRefMut`.
    pub fn body_mut(&mut self) -> BeaconBlockBodyRefMut<'_, T, Payload> {
        self.to_mut().body_mut()
    }

    /// Returns the epoch corresponding to `self.slot()`.
    pub fn epoch(&self) -> Epoch {
        self.slot().epoch(T::slots_per_epoch())
    }

    /// Returns the `tree_hash_root` of the block.
    pub fn canonical_root(&self) -> Hash256 {
        self.tree_hash_root()
    }

    /// Returns a full `BeaconBlockHeader` of this block.
    ///
    /// Note: This method is used instead of an `Into` impl to avoid a `Clone` of an entire block
    /// when you want to have the block _and_ the header.
    ///
    /// Note: performs a full tree-hash of `self.body`.
    pub fn block_header(&self) -> BeaconBlockHeader {
        self.to_ref().block_header()
    }

    /// Returns a "temporary" header, where the `state_root` is `Hash256::zero()`.
    pub fn temporary_block_header(&self) -> BeaconBlockHeader {
        self.to_ref().temporary_block_header()
    }

    /// Return the tree hash root of the block's body.
    pub fn body_root(&self) -> Hash256 {
        self.to_ref().body_root()
    }
}

impl<'a, T: EthSpec, Payload: AbstractExecPayload<T>> BeaconBlockRef<'a, T, Payload> {
    /// Convenience accessor for the `body` as a `BeaconBlockBodyRef`.
    pub fn body(&self) -> BeaconBlockBodyRef<'a, T, Payload> {
        map_beacon_block_ref_into_beacon_block_body_ref!(&'a _, *self, |block, cons| cons(
            &block.body
        ))
    }

    /// Return the tree hash root of the block's body.
    pub fn body_root(&self) -> Hash256 {
        map_beacon_block_ref!(&'a _, *self, |block, cons| {
            let _: Self = cons(block);
            block.body.tree_hash_root()
        })
    }

    /// Returns the epoch corresponding to `self.slot()`.
    pub fn epoch(&self) -> Epoch {
        self.slot().epoch(T::slots_per_epoch())
    }

    /// Returns a full `BeaconBlockHeader` of this block.
    pub fn block_header(&self) -> BeaconBlockHeader {
        BeaconBlockHeader {
            slot: self.slot(),
            proposer_index: self.proposer_index(),
            parent_root: self.parent_root(),
            state_root: self.state_root(),
            body_root: self.body_root(),
        }
    }

    /// Returns a "temporary" header, where the `state_root` is `Hash256::zero()`.
    pub fn temporary_block_header(self) -> BeaconBlockHeader {
        BeaconBlockHeader {
            state_root: Hash256::zero(),
            ..self.block_header()
        }
    }

    /// Extracts a reference to an execution payload from a block, returning an error if the block
    /// is pre-merge.
    pub fn execution_payload(&self) -> Result<Payload::Ref<'a>, Error> {
        self.body().execution_payload()
    }
}

impl<'a, T: EthSpec, Payload: AbstractExecPayload<T>> BeaconBlockRefMut<'a, T, Payload> {
    /// Convert a mutable reference to a beacon block to a mutable ref to its body.
    pub fn body_mut(self) -> BeaconBlockBodyRefMut<'a, T, Payload> {
        map_beacon_block_ref_mut_into_beacon_block_body_ref_mut!(&'a _, self, |block, cons| cons(
            &mut block.body
        ))
    }
}

// We can convert pre-Bellatrix blocks without payloads into blocks "with" payloads.
impl<E: EthSpec> From<BeaconBlockBase<E, BlindedPayload<E>>>
    for BeaconBlockBase<E, FullPayload<E>>
{
    fn from(block: BeaconBlockBase<E, BlindedPayload<E>>) -> Self {
        let BeaconBlockBase {
            slot,
            proposer_index,
            parent_root,
            state_root,
            body,
        } = block;

        BeaconBlockBase {
            slot,
            proposer_index,
            parent_root,
            state_root,
            body: body.into(),
        }
    }
}

impl<E: EthSpec> From<BeaconBlockAltair<E, BlindedPayload<E>>>
    for BeaconBlockAltair<E, FullPayload<E>>
{
    fn from(block: BeaconBlockAltair<E, BlindedPayload<E>>) -> Self {
        let BeaconBlockAltair {
            slot,
            proposer_index,
            parent_root,
            state_root,
            body,
        } = block;

        BeaconBlockAltair {
            slot,
            proposer_index,
            parent_root,
            state_root,
            body: body.into(),
        }
    }
}

// We can convert blocks with payloads to blocks without payloads, and an optional payload.
macro_rules! impl_from {
    ($ty_name:ident, <$($from_params:ty),*>, <$($to_params:ty),*>, $body_expr:expr) => {
        impl<E: EthSpec> From<$ty_name<$($from_params),*>>
            for ($ty_name<$($to_params),*>, Option<ExecutionPayload<E>>)
        {
            #[allow(clippy::redundant_closure_call)]
            fn from(block: $ty_name<$($from_params),*>) -> Self {
                let $ty_name {
                    slot,
                    proposer_index,
                    parent_root,
                    state_root,
                    body,
                } = block;

                let (body, payload) = ($body_expr)(body);

                ($ty_name {
                    slot,
                    proposer_index,
                    parent_root,
                    state_root,
                    body,
                }, payload.map(Into::into))
            }
        }
    }
}

impl_from!(BeaconBlockBase, <E, FullPayload<E>>, <E, BlindedPayload<E>>, |body: BeaconBlockBodyBase<_, _>| body.into());
impl_from!(BeaconBlockAltair, <E, FullPayload<E>>, <E, BlindedPayload<E>>, |body: BeaconBlockBodyAltair<_, _>| body.into());
impl_from!(BeaconBlockMerge, <E, FullPayload<E>>, <E, BlindedPayload<E>>, |body: BeaconBlockBodyMerge<_, _>| body.into());
impl_from!(BeaconBlockCapella, <E, FullPayload<E>>, <E, BlindedPayload<E>>, |body: BeaconBlockBodyCapella<_, _>| body.into());

// We can clone blocks with payloads to blocks without payloads, without cloning the payload.
macro_rules! impl_clone_as_blinded {
    ($ty_name:ident, <$($from_params:ty),*>, <$($to_params:ty),*>) => {
        impl<E: EthSpec> $ty_name<$($from_params),*>
        {
            pub fn clone_as_blinded(&self) -> $ty_name<$($to_params),*> {
                let $ty_name {
                    slot,
                    proposer_index,
                    parent_root,
                    state_root,
                    body,
                } = self;

                $ty_name {
                    slot: *slot,
                    proposer_index: *proposer_index,
                    parent_root: *parent_root,
                    state_root: *state_root,
                    body: body.clone_as_blinded(),
                }
            }
        }
    }
}

impl_clone_as_blinded!(BeaconBlockBase, <E, FullPayload<E>>, <E, BlindedPayload<E>>);
impl_clone_as_blinded!(BeaconBlockAltair, <E, FullPayload<E>>, <E, BlindedPayload<E>>);
impl_clone_as_blinded!(BeaconBlockMerge, <E, FullPayload<E>>, <E, BlindedPayload<E>>);
impl_clone_as_blinded!(BeaconBlockCapella, <E, FullPayload<E>>, <E, BlindedPayload<E>>);

// A reference to a full beacon block can be cloned into a blinded beacon block, without cloning the
// execution payload.
impl<'a, E: EthSpec> From<BeaconBlockRef<'a, E, FullPayload<E>>>
    for BeaconBlock<E, BlindedPayload<E>>
{
    fn from(
        full_block: BeaconBlockRef<'a, E, FullPayload<E>>,
    ) -> BeaconBlock<E, BlindedPayload<E>> {
        map_beacon_block_ref_into_beacon_block!(&'a _, full_block, |inner, cons| {
            cons(inner.clone_as_blinded())
        })
    }
}

impl<E: EthSpec> From<BeaconBlock<E, FullPayload<E>>>
    for (
        BeaconBlock<E, BlindedPayload<E>>,
        Option<ExecutionPayload<E>>,
    )
{
    fn from(block: BeaconBlock<E, FullPayload<E>>) -> Self {
        map_beacon_block!(block, |inner, cons| {
            let (block, payload) = inner.into();
            (cons(block), payload)
        })
    }
}

#[cfg(feature = "std")]
impl<T: EthSpec, Payload: AbstractExecPayload<T>> ForkVersionDeserialize
    for BeaconBlock<T, Payload>
{
    fn deserialize_by_fork<'de, D: serde::Deserializer<'de>>(
        value: serde_json::value::Value,
        fork_name: ForkName,
    ) -> Result<Self, D::Error> {
        Ok(map_fork_name!(
            fork_name,
            Self,
            serde_json::from_value(value).map_err(|e| serde::de::Error::custom(format!(
                "BeaconBlock failed to deserialize: {:?}",
                e
            )))?
        ))
    }
}
