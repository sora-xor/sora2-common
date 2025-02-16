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

use bridge_types::{
    ton::{PayloadBuilder, TonAddress, TonBalance},
    MainnetAssetId,
};
use sp_std::prelude::*;

/// ## UnlockJetton
/// TLB: `unlock_jetton#f8bbc97d master:address recepient:address amount:uint128 = UnlockJetton`
/// Signature: `UnlockJetton{master:address,recepient:address,amount:uint128}`
pub struct MintPayload {
    pub token: TonAddress,
    pub recipient: TonAddress,
    pub amount: TonBalance,
}

pub const UNLOCK_JETTON_ID: u32 = 0xf8bbc97d;

impl MintPayload {
    pub fn encode(&self) -> Option<Vec<u8>> {
        PayloadBuilder::new()
            .write_id(UNLOCK_JETTON_ID)
            .write_address(self.token)
            .write_address(self.recipient)
            .write_balance(self.amount)
            .build()
    }
}

/// ## RegisterJetton
/// TLB: `register_jetton#73fce43f master:address wallet:address = RegisterJetton`
/// Signature: `RegisterJetton{master:address,wallet:address}`
pub struct RegisterJettonPayload {
    pub master: TonAddress,
    pub wallet: TonAddress,
}

pub const REGISTER_JETTON_PAYLOAD: u32 = 0x73fce43f;

impl RegisterJettonPayload {
    pub fn encode(&self) -> Option<Vec<u8>> {
        PayloadBuilder::new()
            .write_id(REGISTER_JETTON_PAYLOAD)
            .write_address(self.master)
            .write_address(self.wallet)
            .build()
    }
}

/// ## RegisterSoraJetton
/// TLB: `register_sora_jetton#f2fc9548 assetId:Bytes32{data:uint256} content:^cell = RegisterSoraJetton`
/// Signature: `RegisterSoraJetton{assetId:Bytes32{data:uint256},content:^cell}`
pub struct RegisterSoraJettonPayload;

pub const REGISTER_SORA_JETTON_ID: u32 = 0xf2fc9548;

impl RegisterSoraJettonPayload {
    pub fn encode(&self) -> Option<Vec<u8>> {
        None
        // Could not be implemented without better TON encode functions
        // PayloadBuilder::new()
        //     .write_id(REGISTER_SORA_JETTON_ID)
        //     .build()
    }
}

/// ## RegisterSoraJettonSimple
/// TLB: `register_sora_jetton_simple#7d195a20 assetId:Bytes32{data:uint256} content:^cell = RegisterSoraJettonSimple`
/// Signature: `RegisterSoraJettonSimple{assetId:Bytes32{data:uint256},content:^cell}`
/// Custom layout: `id: uint32, asset_id: bytes32, symbol_size: uint8, symbol: bytes[symbol_size], name_size: uint8, name: bytes[name_size]`
pub struct RegisterSoraJettonSimplePayload {
    pub asset_id: MainnetAssetId,
    pub name: Vec<u8>,
    pub symbol: Vec<u8>,
}

pub const REGISTER_SORA_JETTON_SIMPLE_ID: u32 = 0x7d195a20;

impl RegisterSoraJettonSimplePayload {
    pub fn encode(&self) -> Option<Vec<u8>> {
        if self.name.len() > u8::MAX as usize || self.symbol.len() > u8::MAX as usize {
            return None;
        }
        PayloadBuilder::new()
            .write_id(REGISTER_SORA_JETTON_SIMPLE_ID)
            .write_bytes(&self.asset_id.0)
            .write_u8(self.symbol.len() as u8)
            .write_bytes(&self.symbol)
            .write_u8(self.name.len() as u8)
            .write_bytes(&self.name)
            .build()
    }
}

/// ## RegisterTon
/// TLB: `register_ton#4ad75248  = RegisterTon`
/// Signature: `RegisterTon{}`
pub struct RegisterTonPayload;

pub const REGISTER_TON_ID: u32 = 0x4ad75248;

impl RegisterTonPayload {
    pub fn encode(&self) -> Option<Vec<u8>> {
        PayloadBuilder::new().write_id(REGISTER_TON_ID).build()
    }
}
