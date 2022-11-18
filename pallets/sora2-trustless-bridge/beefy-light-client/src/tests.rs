use crate::{mock::*};
use bridge_common::beefy_types::ValidatorSet;
use frame_support::{assert_ok};
use hex_literal::hex;

#[test]
fn it_works_initialize_pallet() {
    new_test_ext().execute_with(|| {
        let root = hex!("36ee7c9903f810b22f7e6fca82c1c0cd6a151eca01f087683d92333094d94dc1");
        assert_ok!(
            BeefyLightClient::initialize(
                Origin::root(),
                1,
                ValidatorSet {
                    id: 0,
                    length: 3,
                    root,
                },
                ValidatorSet {
                    id: 1,
                    length: 3,
                    root,
                }
            ),
            ().into()
        )
    });
}
