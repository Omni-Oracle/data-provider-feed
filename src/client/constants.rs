use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::{Keypair, Signer}
    },
    Client, Cluster,
};
use std::{rc::Rc, sync::Arc};

// Define the functions to initialize and return the values
pub fn get_payer() -> Keypair {
    Keypair::from_base58_string("")
}

pub fn get_payer_pubkey() -> Pubkey {
    Arc::new(Keypair::from_base58_string("")).pubkey()
}

pub fn get_client(payer: Keypair) -> Client<Rc<Keypair>> {
    Client::new(Cluster::Devnet, Rc::new(payer))
}

pub fn get_asset_id() -> Keypair {
    Keypair::new()
}
