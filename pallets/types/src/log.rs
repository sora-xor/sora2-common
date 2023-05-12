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

use crate::{H160, H256};
use codec::{Decode, Encode};
use ethabi::RawLog;
use sp_std::prelude::*;

#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, Default, scale_info::TypeInfo)]
pub struct Log {
    pub address: H160,
    pub topics: Vec<H256>,
    pub data: Vec<u8>,
}

impl rlp::Decodable for Log {
    /// We need to implement rlp::Decodable manually as the derive macro RlpDecodable
    /// didn't seem to generate the correct code for parsing our logs.
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let mut iter = rlp.iter();

        let address: H160 = match iter.next() {
            Some(data) => data.as_val()?,
            None => return Err(rlp::DecoderError::Custom("Expected log address")),
        };

        let topics: Vec<H256> = match iter.next() {
            Some(data) => data.as_list()?,
            None => return Err(rlp::DecoderError::Custom("Expected log topics")),
        };

        let data: Vec<u8> = match iter.next() {
            Some(data) => data.data()?.to_vec(),
            None => return Err(rlp::DecoderError::Custom("Expected log data")),
        };

        Ok(Self {
            address,
            topics,
            data,
        })
    }
}

impl From<Log> for RawLog {
    fn from(log: Log) -> Self {
        RawLog::from((log.topics, log.data))
    }
}

impl rlp::Encodable for Log {
    fn rlp_append(&self, s: &mut rlp::RlpStream) {
        s.begin_list(3);
        s.append(&self.address);
        s.append_list(&self.topics);
        s.append(&self.data);
    }
}

#[cfg(test)]
mod tests {

    use super::Log;
    use hex_literal::hex;

    const RAW_LOG: [u8; 605] = hex!(
        "
		f9025a941cfd66659d44cfe2e627c5742ba7477a3284cffae1a0266413be5700ce8dd5ac6b9a7dfb
		abe99b3e45cae9a68ac2757858710b401a38b9022000000000000000000000000000000000000000
		00000000000000000000000060000000000000000000000000000000000000000000000000000000
		00000000c00000000000000000000000000000000000000000000000000000000000000100000000
		00000000000000000000000000000000000000000000000000000000283163466436363635394434
		34636665324536323763353734324261373437376133323834634666410000000000000000000000
		00000000000000000000000000000000000000000000000000000000000000000000000000000000
		000000000773656e6445544800000000000000000000000000000000000000000000000000000000
		00000000000000000000000000000000000000000000000000000001000000000000000000000000
		00cffeaaf7681c89285d65cfbe808b80e50269657300000000000000000000000000000000000000
		000000000000000000000000a0000000000000000000000000000000000000000000000000000000
		0000000000000000000000000000000000000000000000000000000000000000000000000a000000
		00000000000000000000000000000000000000000000000000000000020000000000000000000000
		00000000000000000000000000000000000000002f3146524d4d3850456957585961783772705336
		5834585a5831614141785357783143724b5479725659685632346667000000000000000000000000
		0000000000
	"
    );

    #[test]
    fn decode_log() {
        let log: Log = rlp::decode(&RAW_LOG).unwrap();
        assert_eq!(
            log.address.as_bytes(),
            hex!["1cfd66659d44cfe2e627c5742ba7477a3284cffa"]
        );
        assert_eq!(
            log.topics[0].as_bytes(),
            hex!["266413be5700ce8dd5ac6b9a7dfbabe99b3e45cae9a68ac2757858710b401a38"]
        );
    }
}
