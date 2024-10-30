use crate::ecdsa_api;
use bitcoin::{Address, Network, PublicKey};
use ic_cdk::api::management_canister::bitcoin::{
    BitcoinAddress, BitcoinNetwork, MillisatoshiPerByte, Satoshi, Utxo,
};

pub async fn get_address(
    network: BitcoinNetwork,
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
) -> String {
    let public_key = ecdsa_api::get_ecdsa_public_key(key_name, derivation_path).await;

    public_key_to_p2pkh_address(network, &public_key)
}

fn public_key_to_p2pkh_address(network: BitcoinNetwork, public_key: &[u8]) -> String {
    Address::p2pkh(
        &PublicKey::from_slice(public_key).expect("failed to parse public key"),
        Network::Testnet,
    )
    .to_string()
}
