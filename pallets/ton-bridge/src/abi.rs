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

use bridge_types::ton::{PayloadBuilder, TonAddress};
use sp_core::ed25519;
use sp_std::prelude::*;

/// ## RegisterApp
/// TLB: `register_app#267f9407 app:address = RegisterApp`
/// Signature: `RegisterApp{app:address}`
pub struct RegisterAppPayload {
    pub app: TonAddress,
}

pub const REGISTER_APP_ID: u32 = 0x267f9407;

impl RegisterAppPayload {
    pub fn encode(&self) -> Option<Vec<u8>> {
        PayloadBuilder::new()
            .write_id(REGISTER_APP_ID)
            .write_address(self.app)
            .build()
    }
}

/// ## RemoveApp
/// TLB: `remove_app#d53214f8 app:address = RemoveApp`
/// Signature: `RemoveApp{app:address}`
pub struct RemoveAppPayload {
    pub app: TonAddress,
}

pub const REMOVE_APP_ID: u32 = 0xd53214f8;

impl RemoveAppPayload {
    pub fn encode(&self) -> Option<Vec<u8>> {
        PayloadBuilder::new()
            .write_id(REMOVE_APP_ID)
            .write_address(self.app)
            .build()
    }
}

/// ## Migrate
/// TLB: `migrate#0485b71d receiver:address = Migrate`
/// Signature: `Migrate{receiver:address}`
pub struct MigratePayload {
    pub receiver: TonAddress,
}

pub const MIGRATE_ID: u32 = 0x0485b71d;

impl MigratePayload {
    pub fn encode(&self) -> Option<Vec<u8>> {
        PayloadBuilder::new()
            .write_id(MIGRATE_ID)
            .write_address(self.receiver)
            .build()
    }
}

/// ## AddPeer
/// TLB: `add_peer#8239be27 peer:uint256 = AddPeer`
/// Signature: `AddPeer{peer:uint256}`
pub struct AddPeerPayload {
    pub peer: ed25519::Public,
}

pub const ADD_PEER_ID: u32 = 0x8239be27;

impl AddPeerPayload {
    pub fn encode(&self) -> Option<Vec<u8>> {
        PayloadBuilder::new()
            .write_id(ADD_PEER_ID)
            .write_bytes(&self.peer)
            .build()
    }
}

/// ## RemovePeer
/// TLB: `remove_peer#1be81a7f peer:uint256 = RemovePeer`
/// Signature: `RemovePeer{peer:uint256}`
pub struct RemovePeerPayload {
    pub peer: ed25519::Public,
}

pub const REMOVE_PEER_ID: u32 = 0x1be81a7f;

impl RemovePeerPayload {
    pub fn encode(&self) -> Option<Vec<u8>> {
        PayloadBuilder::new()
            .write_id(REMOVE_PEER_ID)
            .write_bytes(&self.peer)
            .build()
    }
}
