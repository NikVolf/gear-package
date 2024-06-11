#![no_std]

use gstd::{msg, prelude::*};
use gprimitives::H256;
use const_random::const_random;

const ID: u128 = const_random!(u128);

#[no_mangle]
pub extern "C" fn handle() {
    let mut d = [0u8; 32];
    d[0..16].copy_from_slice(&ID.to_le_bytes());
    msg::reply(H256::from(d), 0).expect("Unable to send response message");
}

#[no_mangle]
pub extern "C" fn init() {
}
