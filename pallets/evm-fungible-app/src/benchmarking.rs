//! ERC20App pallet benchmarking

use crate::*;
use bridge_types::evm::AdditionalEVMInboundData;
use bridge_types::traits::BridgeAssetRegistry;
use bridge_types::types::AssetKind;
use bridge_types::types::CallOriginOutput;
use bridge_types::EVMChainId;
use bridge_types::H256;
use currencies::Pallet as Currencies;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::traits::UnfilteredDispatchable;
use frame_system::RawOrigin;
use sp_std::prelude::*;
use traits::MultiCurrency;

pub const BASE_NETWORK_ID: EVMChainId = EVMChainId::zero();

benchmarks! {
    where_clause {where
        <T as frame_system::Config>::RuntimeOrigin: From<dispatch::RawOrigin<CallOriginOutput<EVMChainId, H256, AdditionalEVMInboundData>>>,
        AssetNameOf<T>: From<Vec<u8>>,
        AssetSymbolOf<T>: From<Vec<u8>>,
        BalanceOf<T>: From<u128>,
        T: currencies::Config,
        Currencies<T>: MultiCurrency<T::AccountId, CurrencyId = AssetIdOf<T>, Balance = BalanceOf<T>>
    }

    burn {
        let caller: T::AccountId = whitelisted_caller();
        let asset_id = <T as Config>::AssetRegistry::register_asset(BASE_NETWORK_ID.into(), b"ETH".to_vec().into(), b"ETH".to_vec().into())?;
        let recipient = H160::repeat_byte(2);
        let amount = 1000u128;

        Currencies::<T>::deposit(asset_id.clone(), &caller, amount.into())?;
    }: burn(RawOrigin::Signed(caller.clone()), BASE_NETWORK_ID, asset_id.clone(), recipient, amount.into())
    verify {
        assert_eq!(Currencies::<T>::free_balance(asset_id.into(), &caller), 0u128.into());
    }

    // Benchmark `mint` extrinsic under worst case conditions:
    // * `mint` successfully adds amount to recipient account
    mint {
        let asset_id = <T as Config>::AssetRegistry::register_asset(BASE_NETWORK_ID.into(), b"ETH".to_vec().into(), b"ETH".to_vec().into())?;
        let token = H160::repeat_byte(2);
        crate::Pallet::<T>::register_fungible_app(RawOrigin::Root.into(), BASE_NETWORK_ID, H160::repeat_byte(1)).unwrap();
        crate::Pallet::<T>::register_existing_sidechain_asset(RawOrigin::Root.into(), BASE_NETWORK_ID, token, asset_id.clone(), 18).unwrap();
        let asset_kind = AssetKinds::<T>::get(BASE_NETWORK_ID, &asset_id).unwrap();
        let caller = AppAddresses::<T>::get(BASE_NETWORK_ID, asset_kind).unwrap();
        let origin = dispatch::RawOrigin::new(CallOriginOutput {network_id: BASE_NETWORK_ID, additional: AdditionalEVMInboundData{source: caller}, ..Default::default()});

        let recipient: T::AccountId = account("recipient", 0, 0);
        let sender = H160::zero();
        let amount = 500u128;

        let call = Call::<T>::mint { token, sender, recipient: recipient.clone(), amount: amount.into()};

    }: { call.dispatch_bypass_filter(origin.into())? }
    verify {
        assert_eq!(Currencies::<T>::free_balance(asset_id.into(), &recipient), amount.into());
    }

    register_fungible_app {
        let address = H160::repeat_byte(98);
        let network_id = BASE_NETWORK_ID;
        assert!(!AppAddresses::<T>::contains_key(network_id, AssetKind::Sidechain));
    }: _(RawOrigin::Root, network_id, address)
    verify {
        assert!(AppAddresses::<T>::contains_key(network_id, AssetKind::Sidechain));
    }

    register_native_app {
        let address = H160::repeat_byte(98);
        let network_id = BASE_NETWORK_ID;
        let asset_id = <T as Config>::AssetRegistry::register_asset(BASE_NETWORK_ID.into(), b"ETH".to_vec().into(), b"ETH".to_vec().into())?;
        assert!(!AppAddresses::<T>::contains_key(network_id, AssetKind::Native));
    }: _(RawOrigin::Root, network_id, address, asset_id, 18)
    verify {
        assert!(AppAddresses::<T>::contains_key(network_id, AssetKind::Native));
    }

    register_existing_sidechain_asset {
        let asset_id = <T as Config>::AssetRegistry::register_asset(BASE_NETWORK_ID.into(), b"ETH".to_vec().into(), b"ETH".to_vec().into())?;
        let token = H160::repeat_byte(2);
        crate::Pallet::<T>::register_fungible_app(RawOrigin::Root.into(), BASE_NETWORK_ID, H160::repeat_byte(1)).unwrap();
        assert!(!AssetsByAddresses::<T>::contains_key(BASE_NETWORK_ID, token));
    }: _(RawOrigin::Root, BASE_NETWORK_ID, token, asset_id, 18)
    verify {
        assert!(AssetsByAddresses::<T>::contains_key(BASE_NETWORK_ID, token));
    }

    register_sidechain_asset {
        let asset_id = <T as Config>::AssetRegistry::register_asset(BASE_NETWORK_ID.into(), b"ETH".to_vec().into(), b"ETH".to_vec().into())?;
        let token = H160::repeat_byte(2);
        let asset_name = b"ETH".to_vec();
        let asset_symbol = b"ETH".to_vec();
        crate::Pallet::<T>::register_fungible_app(RawOrigin::Root.into(), BASE_NETWORK_ID, H160::repeat_byte(1)).unwrap();
        assert!(!AssetsByAddresses::<T>::contains_key(BASE_NETWORK_ID, token));
    }: _(RawOrigin::Root, BASE_NETWORK_ID, token, asset_symbol.into(), asset_name.into(), 18)
    verify {
        assert!(AssetsByAddresses::<T>::contains_key(BASE_NETWORK_ID, token));
    }

    register_thischain_asset {
        let asset_id = <T as Config>::AssetRegistry::register_asset(BASE_NETWORK_ID.into(), b"ETH".to_vec().into(), b"ETH".to_vec().into())?;
        crate::Pallet::<T>::register_fungible_app(RawOrigin::Root.into(), BASE_NETWORK_ID, H160::repeat_byte(1)).unwrap();
    }: _(RawOrigin::Root, BASE_NETWORK_ID, asset_id)
    verify {
    }

    register_asset_internal {
        let asset_id = <T as Config>::AssetRegistry::register_asset(BASE_NETWORK_ID.into(), b"ETH".to_vec().into(), b"ETH".to_vec().into())?;
        crate::Pallet::<T>::register_fungible_app(RawOrigin::Root.into(), BASE_NETWORK_ID, H160::repeat_byte(1)).unwrap();
        let who = AppAddresses::<T>::get(BASE_NETWORK_ID, AssetKind::Thischain).unwrap();
        let origin = dispatch::RawOrigin::new(CallOriginOutput {network_id: BASE_NETWORK_ID, additional: AdditionalEVMInboundData{source: who}, ..Default::default()});
        let address = H160::repeat_byte(98);
        assert!(!TokenAddresses::<T>::contains_key(BASE_NETWORK_ID, &asset_id));
    }: _(origin, asset_id.clone(), address)
    verify {
        assert_eq!(AssetKinds::<T>::get(BASE_NETWORK_ID, &asset_id), Some(AssetKind::Thischain));
        assert!(TokenAddresses::<T>::contains_key(BASE_NETWORK_ID, &asset_id));
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_tester(), crate::mock::Test,);
}
