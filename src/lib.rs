#![no_std]

use soroban_sdk::{contract, contractimpl, Bytes, BytesN, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn v1(env: Env) -> BytesN<32> {
        let mut bytes = Bytes::from_array(&env, &[u8::MAX; 5000]);

        for (i, _v) in bytes.iter().enumerate() {
            bytes.set(i as u32, i as u8);
        }

        env.crypto().sha256(&bytes)
    }

    pub fn v2 (env: Env) -> BytesN<32> {
        let mut bytes = Bytes::new(&env);

        for i in 0..5000 {
            bytes.push_back(i as u8)
        }

        env.crypto().sha256(&bytes)
    }

    pub fn v3 (env: Env) -> BytesN<32> {
        let mut bytes = [u8::MAX; 5000];

        for (i, byte) in bytes.iter_mut().enumerate() {
            *byte = i as u8;
        }

        env.crypto().sha256(&Bytes::from_array(&env, &bytes))
    }
}

mod test;