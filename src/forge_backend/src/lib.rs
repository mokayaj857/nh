use ::bitcoin::Network;
use candid::{CandidType, Deserialize};
use ic_cdk::api::management_canister::bitcoin::{
    bitcoin_get_balance, bitcoin_get_utxos, BitcoinNetwork, GetBalanceRequest, GetUtxosRequest,
    GetUtxosResponse, MillisatoshiPerByte,
};

use ic_cdk::{init, update};

use std::cell::{Cell, RefCell};

mod bitcoin;
mod ecdsa_api;
mod wallet;

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
#[update]
pub async fn get_balance(address: String) -> u64 {
    let network = NETWORK.with(|n| n.get());
    _get_balance(network, address).await
}
pub async fn _get_balance(network: BitcoinNetwork, address: String) -> u64 {
    let min_confirmations = None;
    let balance_res = bitcoin_get_balance(GetBalanceRequest {
        address,
        network,
        min_confirmations,
    })
    .await;

    balance_res.unwrap().0
}
#[update]
pub async fn get_utxos(address: String) -> GetUtxosResponse {
    let network = NETWORK.with(|n| n.get());
    _get_utxos(network, address).await
}
pub async fn _get_utxos(network: BitcoinNetwork, address: String) -> GetUtxosResponse {
    let filter = None;
    let utxos_res = bitcoin_get_utxos(GetUtxosRequest {
        address,
        network,
        filter,
    })
    .await;

    utxos_res.unwrap().0
}

ic_cdk::export_candid!();
