
use candid::Encode;
use candid::Nat;
use candid::Principal;
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use ic_cdk::api::management_canister::main::create_canister;
use ic_cdk::api::management_canister::main::install_code;
use ic_cdk::api::management_canister::main::CreateCanisterArgument;
use ic_cdk::api::management_canister::main::InstallCodeArgument;
use ic_ledger_types::Tokens;
use mint_cycles::mint_cycles;
use ic_cdk::{ update};
mod errors;
use crate::errors::CustomError;
const FORGE_WASM: &[u8] = include_bytes!("./assets/forge_backend.wasm");

#[update]
async fn create_btc_holder(network: BitcoinNetwork) -> Result<Principal, CustomError> {
    let minted_cycles = mint_cycles(Tokens::from_e8s(70000000))
        .await
        .expect("Could not mint cycles");
    let args = CreateCanisterArgument {
        ..Default::default()
    };

    let wasm_module = FORGE_WASM.to_vec();

    let create_response = create_canister(args, nat_to_u128(minted_cycles))
        .await
        .map_err(|e| CustomError::custom(format!("Failed to create canister: {:?}", e)))?;

    let new_canister_id = create_response.0;
    let serialized_args = Encode!(&network).expect("  args serialization failed");
    let install_args = InstallCodeArgument {
        canister_id: new_canister_id.canister_id, //b77ix-eeaaa-aaaaa-qaada-cai or replace with the created canister id
        wasm_module,
        arg: serialized_args, // here is where we pass argurments for our token but serialized as bytes encoded in candid
        mode: ic_cdk::api::management_canister::main::CanisterInstallMode::Install,
    };
    install_code(install_args)
        .await
        .map_err(|e| CustomError::custom(format!("Failed to install code: {:?},{:?}", e.0, e.1)))?;

    Ok(new_canister_id.canister_id)
}
fn nat_to_u128(value: Nat) -> u128 {
    TryFrom::try_from(value.0).unwrap()
}
ic_cdk::export_candid!();