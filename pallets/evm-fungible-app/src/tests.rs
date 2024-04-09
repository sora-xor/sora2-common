use crate::mock::{
    new_tester, AccountId, FungibleApp, RuntimeEvent, RuntimeOrigin, System, Test, Tokens,
    BASE_NETWORK_ID, ETH, XOR,
};
use crate::Error;
use crate::{AppAddresses, AssetKinds, AssetsByAddresses, TokenAddresses};
use bridge_types::evm::AdditionalEVMInboundData;
use bridge_types::types::{AssetKind, CallOriginOutput};
use bridge_types::{EVMChainId, H160};
use frame_support::assert_noop;
use frame_support::assert_ok;
use sp_keyring::AccountKeyring as Keyring;
use traits::MultiCurrency;

fn last_event() -> RuntimeEvent {
    System::events().pop().expect("Event expected").event
}

#[test]
fn mints_after_handling_ethereum_event() {
    new_tester().execute_with(|| {
        let peer_contract = H160::repeat_byte(2);
        let asset_id = XOR;
        let token = TokenAddresses::<Test>::get(BASE_NETWORK_ID, asset_id).unwrap();
        let sender = H160::repeat_byte(3);
        let recipient: AccountId = Keyring::Charlie.into();
        let bob: AccountId = Keyring::Bob.into();
        let amount = 10;

        Tokens::deposit(asset_id, &bob, 500).unwrap();
        assert_ok!(FungibleApp::burn(
            RuntimeOrigin::signed(bob.clone()),
            BASE_NETWORK_ID,
            asset_id,
            H160::repeat_byte(9),
            amount
        ));

        assert_ok!(FungibleApp::mint(
            dispatch::RawOrigin::new(CallOriginOutput {
                network_id: BASE_NETWORK_ID,
                additional: AdditionalEVMInboundData {
                    source: peer_contract,
                },
                ..Default::default()
            })
            .into(),
            token,
            sender,
            recipient.clone(),
            amount.into(),
        ));
        assert_eq!(Tokens::total_balance(asset_id, &recipient), amount.into());

        assert_eq!(
            RuntimeEvent::FungibleApp(crate::Event::<Test>::Minted(
                BASE_NETWORK_ID,
                asset_id,
                sender,
                recipient,
                amount.into()
            )),
            last_event()
        );
    });
}

#[test]
fn mint_zero_amount_must_fail() {
    new_tester().execute_with(|| {
        let peer_contract = H160::repeat_byte(2);
        let asset_id = XOR;
        let token = TokenAddresses::<Test>::get(BASE_NETWORK_ID, asset_id).unwrap();
        let sender = H160::repeat_byte(3);
        let recipient: AccountId = Keyring::Charlie.into();
        let amount = 0;

        assert_noop!(
            FungibleApp::mint(
                dispatch::RawOrigin::new(CallOriginOutput {
                    network_id: BASE_NETWORK_ID,
                    additional: AdditionalEVMInboundData {
                        source: peer_contract,
                    },
                    ..Default::default()
                })
                .into(),
                token,
                sender,
                recipient.clone(),
                amount.into(),
            ),
            Error::<Test>::WrongAmount
        );
    });
}

#[test]
fn burn_should_emit_bridge_event() {
    new_tester().execute_with(|| {
        let asset_id = XOR;
        let recipient = H160::repeat_byte(2);
        let bob: AccountId = Keyring::Bob.into();
        let amount = 20;
        Tokens::deposit(asset_id, &bob, 500).unwrap();

        assert_ok!(FungibleApp::burn(
            RuntimeOrigin::signed(bob.clone()),
            BASE_NETWORK_ID,
            asset_id,
            recipient.clone(),
            amount
        ));

        assert_eq!(
            RuntimeEvent::FungibleApp(crate::Event::<Test>::Burned(
                BASE_NETWORK_ID,
                asset_id,
                bob,
                recipient,
                amount
            )),
            last_event()
        );
    });
}

#[test]
fn burn_zero_amount_must_fail() {
    new_tester().execute_with(|| {
        let asset_id = XOR;
        let recipient = H160::repeat_byte(2);
        let bob: AccountId = Keyring::Bob.into();
        let amount = 0;
        Tokens::deposit(asset_id, &bob, 500).unwrap();

        assert_noop!(
            FungibleApp::burn(
                RuntimeOrigin::signed(bob.clone()),
                BASE_NETWORK_ID,
                asset_id,
                recipient.clone(),
                amount
            ),
            Error::<Test>::WrongAmount
        );
    });
}

#[test]
fn test_register_asset_internal() {
    new_tester().execute_with(|| {
        let asset_id = ETH;
        let who = AppAddresses::<Test>::get(BASE_NETWORK_ID, AssetKind::Thischain).unwrap();
        let origin = dispatch::RawOrigin::new(CallOriginOutput {
            network_id: BASE_NETWORK_ID,
            additional: AdditionalEVMInboundData { source: who },
            ..Default::default()
        });
        let address = H160::repeat_byte(98);
        assert!(!TokenAddresses::<Test>::contains_key(
            BASE_NETWORK_ID,
            asset_id
        ));
        FungibleApp::register_asset_internal(origin.into(), asset_id, address).unwrap();
        assert_eq!(
            AssetKinds::<Test>::get(BASE_NETWORK_ID, asset_id),
            Some(AssetKind::Thischain)
        );
        assert!(TokenAddresses::<Test>::contains_key(
            BASE_NETWORK_ID,
            asset_id
        ));
    })
}

#[test]
fn test_register_erc20_asset() {
    new_tester().execute_with(|| {
        let address = H160::repeat_byte(98);
        let network_id = BASE_NETWORK_ID;
        assert!(!AssetsByAddresses::<Test>::contains_key(
            network_id, address
        ));
        FungibleApp::register_sidechain_asset(
            RuntimeOrigin::root(),
            network_id,
            address,
            "ETH".to_string().into(),
            "ETH".to_string().into(),
            18,
        )
        .unwrap();
        assert!(AssetsByAddresses::<Test>::contains_key(network_id, address));
    })
}

#[test]
fn test_register_native_asset() {
    new_tester().execute_with(|| {
        let asset_id = ETH;
        let network_id = BASE_NETWORK_ID;
        assert!(!TokenAddresses::<Test>::contains_key(network_id, asset_id));
        FungibleApp::register_thischain_asset(RuntimeOrigin::root(), network_id, asset_id).unwrap();
        assert!(!TokenAddresses::<Test>::contains_key(network_id, asset_id));
    })
}

#[test]
fn test_register_erc20_app() {
    new_tester().execute_with(|| {
        let address = H160::repeat_byte(98);
        let network_id = EVMChainId::from_low_u64_be(2);
        assert!(!AppAddresses::<Test>::contains_key(
            network_id,
            AssetKind::Sidechain
        ));
        FungibleApp::register_fungible_app(RuntimeOrigin::root(), network_id, address).unwrap();
        assert!(AppAddresses::<Test>::contains_key(
            network_id,
            AssetKind::Sidechain
        ));
    })
}

#[test]
fn test_register_native_app() {
    new_tester().execute_with(|| {
        let address = H160::repeat_byte(98);
        let network_id = EVMChainId::from_low_u64_be(2);
        assert!(!AppAddresses::<Test>::contains_key(
            network_id,
            AssetKind::Thischain
        ));
        FungibleApp::register_fungible_app(RuntimeOrigin::root(), network_id, address).unwrap();
        assert!(AppAddresses::<Test>::contains_key(
            network_id,
            AssetKind::Thischain
        ));
    })
}
