use wasm_bindgen::prelude::*;
use web3::signing::{keccak256, recover};

#[wasm_bindgen]
pub fn eth_recover(sig: String, msg: String) -> String {
    let message = eth_message(msg);
    let signature = hex::decode(sig).unwrap();

    let pubkey = recover(&message, &signature[..64], (signature[64] - 27) as i32);
    assert!(pubkey.is_ok());

    let address = format!("{:02X?}", pubkey.unwrap());

    return address;
}

pub fn eth_message(message: String) -> [u8; 32] {
    keccak256(
        format!(
            "{}{}{}",
            "\x19Ethereum Signed Message:\n",
            message.len(),
            message
        )
        .as_bytes(),
    )
}
