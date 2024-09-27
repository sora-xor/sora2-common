use bridge_types::ton::{PayloadBuilder, TonAddress, TonBalance};
use sp_std::prelude::*;

pub const MINT_ID: u32 = 0x0485b71d;

pub struct MintPayload {
    pub token: TonAddress,
    pub recipient: TonAddress,
    pub amount: TonBalance,
}

impl MintPayload {
    pub fn encode(&self) -> Option<Vec<u8>> {
        PayloadBuilder::new()
            .write_id(MINT_ID)
            .write_address(self.token)
            .write_address(self.recipient)
            .write_balance(self.amount)
            .build()
    }
}
