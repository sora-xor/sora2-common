use codec::{Decode, Encode, MaxEncodedLen};
use derivative::Derivative;
use scale_info::TypeInfo;
use sp_core::{ecdsa, ed25519, Get, RuntimeDebug, H256};
use sp_runtime::{BoundedBTreeMap, BoundedBTreeSet};

#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Derivative)]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = "")
)]
#[scale_info(skip_type_params(MaxSigs))]
pub enum MultiSignatures<MaxSigs: Get<u32>> {
    /// Substrate and EVM Bridge
    Ecdsa(BoundedBTreeMap<ecdsa::Public, ecdsa::Signature, MaxSigs>),
    /// TON Bridge
    Ed25519(BoundedBTreeMap<ed25519::Public, ed25519::Signature, MaxSigs>),
}

#[derive(RuntimeDebug, Clone, Decode, Encode, MaxEncodedLen, TypeInfo, PartialEq, Eq)]
pub enum MultiSignature {
    /// Substrate and EVM Bridge
    Ecdsa(ecdsa::Public, ecdsa::Signature),
    /// TON Bridge
    Ed25519(ed25519::Public, ed25519::Signature),
}

#[derive(RuntimeDebug, Clone, Decode, Encode, MaxEncodedLen, TypeInfo, PartialEq, Eq)]
pub enum MultiSigner {
    /// Substrate and EVM Bridge
    Ecdsa(ecdsa::Public),
    /// TON Bridge
    Ed25519(ed25519::Public),
}

#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Derivative)]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = "")
)]
#[scale_info(skip_type_params(MaxPeers))]
pub enum MultiSigners<MaxPeers: Get<u32>> {
    /// Substrate and EVM Bridge
    Ecdsa(BoundedBTreeSet<ecdsa::Public, MaxPeers>),
    /// TON Bridge
    Ed25519(BoundedBTreeSet<ed25519::Public, MaxPeers>),
}

impl MultiSignature {
    pub fn verify(&self, msg: H256) -> bool {
        match self {
            Self::Ecdsa(pub_key, sig) => {
                sp_io::crypto::ecdsa_verify_prehashed(sig, &msg.0, pub_key)
            }
            Self::Ed25519(pub_key, sig) => sp_io::crypto::ed25519_verify(sig, &msg.0, pub_key),
        }
    }

    pub fn public(&self) -> MultiSigner {
        match self {
            Self::Ecdsa(pub_key, _) => MultiSigner::Ecdsa(pub_key.clone()),
            Self::Ed25519(pub_key, _) => MultiSigner::Ed25519(pub_key.clone()),
        }
    }
}

impl<MaxSigs: Get<u32>> MultiSignatures<MaxSigs> {
    pub fn verify(&self, signers: &MultiSigners<MaxSigs>, msg: H256) -> bool {
        if self.len() < crate::utils::threshold(signers.len() as u32) as usize {
            return false;
        }
        for sig in self.signatures() {
            if !signers.contains(&sig.public()) {
                return false;
            }
            if !sig.verify(msg) {
                return false;
            }
        }
        true
    }

    pub fn add_signature(&mut self, sig: MultiSignature) -> bool {
        match (self, sig) {
            (Self::Ecdsa(sigs), MultiSignature::Ecdsa(pub_key, sig)) => {
                if !matches!(sigs.try_insert(pub_key, sig), Ok(None)) {
                    return false;
                }
            }
            (Self::Ed25519(sigs), MultiSignature::Ed25519(pub_key, sig)) => {
                if !matches!(sigs.try_insert(pub_key, sig), Ok(None)) {
                    return false;
                }
            }
            _ => return false,
        }
        true
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Ecdsa(sigs) => sigs.len(),
            Self::Ed25519(sigs) => sigs.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contains(&self, signer: &MultiSigner) -> bool {
        match (self, signer) {
            (Self::Ecdsa(sigs), MultiSigner::Ecdsa(pub_key)) => sigs.contains_key(pub_key),
            (Self::Ed25519(sigs), MultiSigner::Ed25519(pub_key)) => sigs.contains_key(pub_key),
            _ => false,
        }
    }
    pub fn signatures(&self) -> Vec<MultiSignature> {
        match self {
            MultiSignatures::Ecdsa(sigs) => sigs
                .iter()
                .map(|(pk, sig)| MultiSignature::Ecdsa(pk.clone(), sig.clone()))
                .collect(),
            MultiSignatures::Ed25519(sigs) => sigs
                .iter()
                .map(|(pk, sig)| MultiSignature::Ed25519(pk.clone(), sig.clone()))
                .collect(),
        }
    }
}

impl<MaxPeers: Get<u32>> MultiSigners<MaxPeers> {
    pub fn contains(&self, signer: &MultiSigner) -> bool {
        match (self, signer) {
            (Self::Ecdsa(pub_keys), MultiSigner::Ecdsa(pub_key)) => pub_keys.contains(pub_key),
            (Self::Ed25519(pub_keys), MultiSigner::Ed25519(pub_key)) => pub_keys.contains(pub_key),
            _ => false,
        }
    }

    pub fn empty_signatures(&self) -> MultiSignatures<MaxPeers> {
        match self {
            Self::Ecdsa(_) => MultiSignatures::Ecdsa(BoundedBTreeMap::new()),
            Self::Ed25519(_) => MultiSignatures::Ecdsa(BoundedBTreeMap::new()),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Ecdsa(pub_keys) => pub_keys.len(),
            Self::Ed25519(pub_keys) => pub_keys.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add_peer(&mut self, pub_key: MultiSigner) -> bool {
        match (self, pub_key) {
            (Self::Ecdsa(pub_keys), MultiSigner::Ecdsa(pub_key)) => {
                return pub_keys.try_insert(pub_key).unwrap_or(false);
            }
            (Self::Ed25519(pub_keys), MultiSigner::Ed25519(pub_key)) => {
                return pub_keys.try_insert(pub_key).unwrap_or(false);
            }
            _ => return false,
        }
    }

    pub fn remove_peer(&mut self, pub_key: &MultiSigner) -> bool {
        match (self, pub_key) {
            (Self::Ecdsa(pub_keys), MultiSigner::Ecdsa(pub_key)) => pub_keys.remove(pub_key),
            (Self::Ed25519(pub_keys), MultiSigner::Ed25519(pub_key)) => pub_keys.remove(pub_key),
            _ => false,
        }
    }

    pub fn signers(&self) -> Vec<MultiSigner> {
        match self {
            MultiSigners::Ecdsa(peers) => peers.iter().cloned().map(Into::into).collect(),
            MultiSigners::Ed25519(peers) => peers.iter().cloned().map(Into::into).collect(),
        }
    }
}

impl From<ecdsa::Public> for MultiSigner {
    fn from(value: ecdsa::Public) -> Self {
        MultiSigner::Ecdsa(value)
    }
}

impl From<ed25519::Public> for MultiSigner {
    fn from(value: ed25519::Public) -> Self {
        MultiSigner::Ed25519(value)
    }
}
