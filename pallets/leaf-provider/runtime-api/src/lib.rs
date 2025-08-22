// This file is part of the SORA network and Polkaswap app.

// Copyright (c) 2020, 2021, Polka Biome Ltd. All rights reserved.
// SPDX-License-Identifier: BSD-4-Clause

// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:

// Redistributions of source code must retain the above copyright notice, this list
// of conditions and the following disclaimer.
// Redistributions in binary form must reproduce the above copyright notice, this
// list of conditions and the following disclaimer in the documentation and/or other
// materials provided with the distribution.
//
// All advertising materials mentioning features or use of this software must display
// the following acknowledgement: This product includes software developed by Polka Biome
// Ltd., SORA, and Polkaswap.
//
// Neither the name of the Polka Biome Ltd. nor the names of its contributors may be used
// to endorse or promote products derived from this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY Polka Biome Ltd. AS IS AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL Polka Biome Ltd. BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
// BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
// OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

pub use bridge_types::types::AuxiliaryDigest;
pub use bridge_types::GenericNetworkId;

sp_api::decl_runtime_apis! {
    pub trait LeafProviderAPI
    {
        fn latest_digest() -> Option<AuxiliaryDigest>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bridge_types::{types::AuxiliaryDigestItem, GenericNetworkId, H256};
    use codec::{Decode, Encode};
    use sp_runtime::{Digest, DigestItem};

    #[test]
    fn auxiliary_digest_codec_roundtrip() {
        let item = AuxiliaryDigestItem::Commitment(
            GenericNetworkId::Sub(Default::default()),
            H256::repeat_byte(9),
        );
        let aux = AuxiliaryDigest { logs: vec![item] };
        let bytes = aux.encode();
        let decoded = AuxiliaryDigest::decode(&mut &bytes[..]).unwrap();
        assert_eq!(aux, decoded);
    }

    #[test]
    fn auxiliary_digest_from_runtime_digest_filters_non_other() {
        let other = AuxiliaryDigestItem::Commitment(
            GenericNetworkId::Sub(Default::default()),
            H256::repeat_byte(7),
        );
        let digest = Digest {
            logs: vec![
                DigestItem::Other(other.encode()),
                // This one should be ignored by conversion
                DigestItem::PreRuntime([1u8, 2, 3, 4], vec![0]),
            ],
        };
        let aux: AuxiliaryDigest = digest.into();
        assert_eq!(aux.logs.len(), 1);
        assert_eq!(aux.logs[0], other);
    }
}
