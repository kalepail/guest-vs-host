#![cfg(test)]

use std::println;
extern crate std;

use soroban_sdk::Env;
use crate::{Contract, ContractClient};

#[test]
fn test_v1() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    env.budget().reset_limits(100000000, 41943040);

    let hash = client.v1();

    env.budget().print();
    // Cpu limit: 18446744073709551615; used: 22932352
    // Mem limit: 18446744073709551615; used: 38303900

    println!("{:?}", hash);
}

#[test]
fn test_v2() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    env.budget().reset_limits(100000000, 41943040);

    let hash = client.v2();

    env.budget().print();
    // Cpu limit: 18446744073709551615; used: 9131770
    // Mem limit: 18446744073709551615; used: 12903884

    println!("{:?}", hash);
}

#[test]
fn test_v3() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    env.budget().reset_limits(100000000, 41943040);

    let hash = client.v3();

    env.budget().print();
    // Cpu limit: 18446744073709551615; used: 292230
    // Mem limit: 18446744073709551615; used: 6400

    println!("{:?}", hash);
}