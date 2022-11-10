use std::error::Error;
use rand::{distributions::Alphanumeric, Rng};
use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};
use tiny_keccak::keccak256;
use web3::types::{Address, U256};
use web3::{
    transports::{WebSocket, Http},
    Web3,
};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    return Ok(());
}

pub async fn create_provider(url: &str) -> Web3<Http>{
    let transport = web3::transports::Http::new(url).unwrap();
    let provider = web3::Web3::new(transport); 
    return provider;
}

pub async fn updated_with_amount(provider: &Web3::<Http>, addresses: &mut Vec<(Address,u64, SecretKey)>){
    for el in addresses{
        let address = el.0;
        let balance = provider.eth().balance(address,None).await.unwrap();
        el.1 = balance.low_u64();
    }
}

pub fn public_key_address(public_key: &PublicKey) -> Address {
    let public_key = public_key.serialize_uncompressed();

    debug_assert_eq!(public_key[0], 0x04);
    let hash = keccak256(&public_key[1..]);

    Address::from_slice(&hash[12..])
}

fn generate_key_pair(seed: u64) -> (SecretKey, PublicKey){
    let secp = secp256k1::Secp256k1::new();
    let mut random = rngs::StdRng::seed_from_u64(seed);
    let result = secp.generate_keypair(&mut random);
    return result;
}

fn load_wallet() -> Vec<(Address, u64, SecretKey)>{
    let mut result: Vec<(Address, u64, SecretKey)> = Vec::new();
    for i in 1..10{
        
        let key_pair = generate_key_pair(i);
        let address = public_key_address(&key_pair.1);
        result.push((address, 0, key_pair.0));
    }
    return result;
}