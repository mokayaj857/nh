use ::bitcoin::Network;
use candid::{CandidType, Deserialize};
use ic_cdk::api::management_canister::bitcoin::{
    BitcoinNetwork, GetUtxosResponse, MillisatoshiPerByte,
};
use ic_cdk::{init, update};

use std::cell::{Cell, RefCell};

mod bitcoin;
mod ecdsa_api;

thread_local! {

    static NETWORK: Cell<BitcoinNetwork> = Cell::new(BitcoinNetwork::Testnet);


    static DERIVATION_PATH: Vec<Vec<u8>> = vec![];


    static KEY_NAME: RefCell<String> = RefCell::new(String::from(""));
}
#[init]
pub fn init(network: BitcoinNetwork) {
    NETWORK.with(|n| n.set(network));

    KEY_NAME.with(|key_name| {
        key_name.replace(String::from(match network {
            BitcoinNetwork::Regtest => "dfx_test_key",
            BitcoinNetwork::Testnet => "dfx_test_key",
            BitcoinNetwork::Mainnet => todo!(),

            _ => todo!(),
        }))
    });
}
#[update]
pub async fn get_p2pkh_address() -> String {
    let derivation_path = DERIVATION_PATH.with(|d| d.clone());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let network = NETWORK.with(|n| n.get());
    bitcoin::get_address(network, key_name, derivation_path).await
}
ic_cdk::export_candid!();
