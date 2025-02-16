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

#![cfg(test)]

use super::mock::*;
use super::*;
use crate::Error;
use bridge_types::ton::{AdditionalTONInboundData, TonAddress, TonBalance, TonTransactionId};
use bridge_types::types::{CallOriginOutput, GenericAdditionalInboundData};
use bridge_types::{GenericNetworkId, GenericTimepoint};
use frame_support::{assert_noop, assert_ok};
use sp_core::H256;

fn ton_call_origin(
    network_id: TonNetworkId,
    channel: TonAddress,
) -> dispatch::RawOrigin<CallOriginOutput<GenericNetworkId, H256, GenericAdditionalInboundData>> {
    dispatch::RawOrigin::new(CallOriginOutput {
        additional: GenericAdditionalInboundData::TON(AdditionalTONInboundData { source: channel }),
        network_id: network_id.into(),
        message_id: H256::random(),
        timepoint: GenericTimepoint::TON(TonTransactionId {
            lt: 11,
            hash: H256::random(),
        }),
    })
}

#[test]
fn claim_relayer_fees_works() {
    new_test_ext().execute_with(|| {
        let relayer = TonAddress::new(0, [1; 32].into());
        let signer = bob();
        let asset_id = TON;
        let initial_spent_by_relayer = 50;
        let initial_collected = 100;

        let config = NetworkConfig {
            network_id: TonNetworkId::Testnet,
            channel: TonAddress::new(0, [0xAA; 32].into()),
            fee_info: Some(FeeInfo {
                asset: asset_id,
                precision: 9,
            }),
        };
        NetworkConfigs::<Test>::put(config);

        RelayerOwners::<Test>::insert(relayer, signer.clone());

        SpentFees::<Test>::insert(relayer, initial_spent_by_relayer);
        CollectedFees::<Test>::put(initial_collected);

        assert_ok!(TonBridge::claim_relayer_fees(
            RawOrigin::Signed(signer.clone()).into(),
            relayer
        ));

        let new_spent = SpentFees::<Test>::get(relayer);
        assert_eq!(new_spent, 0);

        let new_collected = CollectedFees::<Test>::get();
        assert_eq!(new_collected, 50);

        let events = System::events();
        assert!(events.iter().any(|r| {
            matches!(
                r.event.clone(),
                RuntimeEvent::TonBridge(pallet::Event::FeesClaimed { recipient, asset_id, amount })
                if recipient == signer && asset_id == asset_id && amount == 50
            )
        }));
    });
}

#[test]
fn claim_relayer_fees_fails_if_not_owner() {
    new_test_ext().execute_with(|| {
        let relayer = TonAddress::new(0, [2; 32].into());
        let signer = bob();

        let config = NetworkConfig {
            network_id: TonNetworkId::Testnet,
            channel: TonAddress::new(0, [0xAB; 32].into()),
            fee_info: Some(FeeInfo {
                asset: TON,
                precision: 9,
            }),
        };
        NetworkConfigs::<Test>::put(config);

        RelayerOwners::<Test>::insert(relayer, alice());
        SpentFees::<Test>::insert(relayer, 50);
        CollectedFees::<Test>::put(100);

        assert_noop!(
            TonBridge::claim_relayer_fees(RawOrigin::Signed(signer).into(), relayer),
            Error::<Test>::AccountIsNotOwner
        );
    });
}

#[test]
fn register_network_works_with_root() {
    new_test_ext().execute_with(|| {
        let channel = TonAddress::new(0, [0x11; 32].into());
        let asset_id = TON;
        let fee_info = Some(FeeInfo {
            asset: asset_id,
            precision: 9,
        });

        assert_ok!(TonBridge::register_network(
            RawOrigin::Root.into(),
            TonNetworkId::Testnet,
            channel,
            fee_info.clone()
        ));

        let cfg = NetworkConfigs::<Test>::get().expect("Network config not stored");
        assert_eq!(cfg.network_id, TonNetworkId::Testnet);
        assert_eq!(cfg.channel, channel);
        assert_eq!(cfg.fee_info, fee_info);
    });
}

#[test]
fn register_network_fails_if_not_root() {
    new_test_ext().execute_with(|| {
        let channel = TonAddress::new(0, [0x99; 32].into());

        assert_noop!(
            TonBridge::register_network(
                RawOrigin::Signed(bob()).into(),
                TonNetworkId::Testnet,
                channel,
                None
            ),
            sp_runtime::DispatchError::BadOrigin
        );
    });
}

#[test]
fn register_fee_info_works() {
    new_test_ext().execute_with(|| {
        NetworkConfigs::<Test>::put(NetworkConfig {
            network_id: TonNetworkId::Testnet,
            channel: TonAddress::new(0, [0xEE; 32].into()),
            fee_info: None,
        });

        let new_fee_info = Some(FeeInfo {
            asset: TON,
            precision: 5,
        });
        assert_ok!(TonBridge::register_fee_info(
            RawOrigin::Root.into(),
            new_fee_info.clone()
        ));

        let cfg = NetworkConfigs::<Test>::get().expect("Should exist");
        assert_eq!(cfg.fee_info, new_fee_info);
    });
}

#[test]
fn register_fee_info_fails_if_not_root() {
    new_test_ext().execute_with(|| {
        NetworkConfigs::<Test>::put(NetworkConfig {
            network_id: TonNetworkId::Testnet,
            channel: TonAddress::new(0, [0xEE; 32].into()),
            fee_info: None,
        });

        assert_noop!(
            TonBridge::register_fee_info(RawOrigin::Signed(bob()).into(), None),
            sp_runtime::DispatchError::BadOrigin
        );
    });
}

#[test]
fn register_relayer_works() {
    new_test_ext().execute_with(|| {
        let config = NetworkConfig {
            network_id: TonNetworkId::Testnet,
            channel: TonAddress::new(0, [0x55; 32].into()),
            fee_info: None,
        };
        NetworkConfigs::<Test>::put(config.clone());

        let relayer = TonAddress::new(0, [0x03; 32].into());
        let owner = bob();
        let origin = ton_call_origin(TonNetworkId::Testnet, config.channel);

        assert_ok!(TonBridge::register_relayer(
            origin.into(),
            relayer.into(),
            owner.clone()
        ));

        let stored_owner = RelayerOwners::<Test>::get(relayer).unwrap();
        assert_eq!(stored_owner, owner);
    });
}

#[test]
fn register_relayer_fails_wrong_channel() {
    new_test_ext().execute_with(|| {
        let config = NetworkConfig {
            network_id: TonNetworkId::Testnet,
            channel: TonAddress::new(0, [0x99; 32].into()),
            fee_info: None,
        };
        NetworkConfigs::<Test>::put(config);

        let origin = ton_call_origin(TonNetworkId::Testnet, TonAddress::new(0, [0x00; 32].into()));
        let relayer = TonAddress::new(0, [0x03; 32].into());
        let owner = bob();

        assert_noop!(
            TonBridge::register_relayer(origin.into(), relayer.into(), owner),
            Error::<Test>::InvalidSourceChannel
        );
    });
}

#[test]
fn commitment_submitted_works() {
    new_test_ext().execute_with(|| {
        let config = NetworkConfig {
            network_id: TonNetworkId::Testnet,
            channel: TonAddress::new(0, [0x77; 32].into()),
            fee_info: Some(FeeInfo {
                asset: TON,
                precision: 9,
            }),
        };
        NetworkConfigs::<Test>::put(config.clone());

        let origin = ton_call_origin(TonNetworkId::Testnet, config.channel);

        let relayer = TonAddress::new(0, [0x03; 32].into());
        let fee = TonBalance::new(10);

        assert_eq!(SpentFees::<Test>::get(relayer), 0);

        assert_ok!(TonBridge::commitment_submitted(
            origin.into(),
            relayer.into(),
            fee
        ));

        let stored_spent = SpentFees::<Test>::get(relayer);
        assert!(stored_spent > 0);
    });
}

#[test]
fn commitment_submitted_fails_wrong_network() {
    new_test_ext().execute_with(|| {
        let config = NetworkConfig {
            network_id: TonNetworkId::Testnet,
            channel: TonAddress::new(0, [0x77; 32].into()),
            fee_info: None,
        };
        NetworkConfigs::<Test>::put(config.clone());

        let origin = ton_call_origin(TonNetworkId::Mainnet, config.channel);
        let relayer = TonAddress::new(0, [0x03; 32].into());
        let fee = TonBalance::new(100);

        assert_noop!(
            TonBridge::commitment_submitted(origin.into(), relayer.into(), fee),
            Error::<Test>::InvalidNetwork
        );
    });
}
