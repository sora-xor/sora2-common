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

use crate::{traits::BridgeAssetLocker, GenericNetworkId};
use core::marker::PhantomData;
use sp_runtime::{traits::Hash, AccountId32};

pub struct BridgeAssetLockerImpl<T>(PhantomData<T>);

impl<T> BridgeAssetLockerImpl<T> {
    pub fn bridge_account(network_id: GenericNetworkId) -> AccountId32 {
        let hash = sp_runtime::traits::BlakeTwo256::hash_of(&(b"bridge-lock-account", &network_id));
        AccountId32::new(hash.0)
    }
    pub fn bridge_fee_account(network_id: GenericNetworkId) -> AccountId32 {
        let hash = sp_runtime::traits::BlakeTwo256::hash_of(&(b"bridge-fee-account", &network_id));
        AccountId32::new(hash.0)
    }
}

impl<T: traits::MultiCurrency<AccountId32>> BridgeAssetLocker<AccountId32>
    for BridgeAssetLockerImpl<T>
{
    type AssetId = T::CurrencyId;
    type Balance = T::Balance;

    fn lock_asset(
        network_id: crate::GenericNetworkId,
        asset_kind: crate::types::AssetKind,
        who: &AccountId32,
        asset_id: &T::CurrencyId,
        amount: &T::Balance,
    ) -> frame_support::dispatch::DispatchResult {
        match asset_kind {
            crate::types::AssetKind::Thischain => {
                let bridge_acc = Self::bridge_account(network_id);
                T::transfer(*asset_id, who, &bridge_acc, *amount)?;
            }
            crate::types::AssetKind::Sidechain => {
                T::withdraw(*asset_id, who, *amount)?;
            }
        }
        Ok(())
    }

    fn unlock_asset(
        network_id: crate::GenericNetworkId,
        asset_kind: crate::types::AssetKind,
        who: &AccountId32,
        asset_id: &T::CurrencyId,
        amount: &T::Balance,
    ) -> frame_support::dispatch::DispatchResult {
        match asset_kind {
            crate::types::AssetKind::Thischain => {
                let bridge_acc = Self::bridge_account(network_id);
                T::transfer(*asset_id, &bridge_acc, who, *amount)?;
            }
            crate::types::AssetKind::Sidechain => {
                T::deposit(*asset_id, who, *amount)?;
            }
        }
        Ok(())
    }

    fn refund_fee(
        network_id: GenericNetworkId,
        who: &AccountId32,
        asset_id: &Self::AssetId,
        amount: &Self::Balance,
    ) -> frame_support::dispatch::DispatchResult {
        let bridge_acc = Self::bridge_fee_account(network_id);
        T::transfer(*asset_id, &bridge_acc, who, *amount)?;
        Ok(())
    }

    fn withdraw_fee(
        network_id: GenericNetworkId,
        who: &AccountId32,
        asset_id: &Self::AssetId,
        amount: &Self::Balance,
    ) -> frame_support::dispatch::DispatchResult {
        let bridge_acc = Self::bridge_fee_account(network_id);
        T::transfer(*asset_id, who, &bridge_acc, *amount)?;
        Ok(())
    }
}
