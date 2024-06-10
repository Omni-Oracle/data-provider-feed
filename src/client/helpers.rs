use anchor_client::{DynSigner, Program};
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::{solana_sdk::{
    signature::{Keypair, Signer, Signature},
    system_program,
},
Client, Cluster, ClientError};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

use omni_server as omni_oracle;

#[derive(Serialize, Deserialize, Default)]
pub enum Currency {
    #[serde(rename = "USD")]
    #[default]
    Usd,
    #[serde(rename = "EUR")]
    Eur,
    #[serde(rename = "BONK")]
    BONK,
}

pub async fn initialize_asset(
    program: &Program<Rc<Keypair>>,
    assetId: Pubkey,
    metadata_url: String,
    name: String
) -> std::result::Result<Signature, ClientError> {
    
    let authority = crate::client::constants::get_payer();
    
    let (asset_pda, _bump) = Pubkey::find_program_address(&[b"OMNI".as_ref(), assetId.as_ref()], &program.id());

    let tx = program
        .request()
        .accounts(omni_oracle::accounts::InitializeAsset {
            asset: asset_pda,
            authority: authority.pubkey(),
            system_program: system_program::ID,
        })
        .args(omni_oracle::instruction::InitializeAsset {
            assetId,
            metadata_url,
            name,
        })
        .signer(&authority)
        .send()
        .await;

        match tx {
            Ok(signature) => Ok(signature),
            Err(err) => Err(err.into()), // Convert the error to a compatible type
        }
}

pub async fn update_price(program: &Program<Rc<Keypair>>, asset_pda: Pubkey, new_price: f64,) -> std::result::Result<Signature, ClientError> {

 let authority = crate::client::constants::get_payer();
  let tx =  program
    .request()
    .accounts(omni_oracle::accounts::UpdatePrice {
        asset: asset_pda,
        authority: authority.pubkey()
    }).args(omni_oracle::instruction::UpdatePrice {
        price: new_price,
    }).signer(&authority)
    .send().await;

    match tx {
        Ok(signature) => Ok(signature),
        Err(err) => Err(err.into()), // Convert the error to a compatible type
    }
}

pub async fn get_asset_price() -> std::result::Result<(), ClientError> {
  Ok(())
}


pub async fn initialize_asset_call(metadata: String, name: String) -> Result<Signature, ClientError> {
    let payer = crate::client::constants::get_payer();
    let client = crate::client::constants::get_client(payer);
    let program = client.program(omni_oracle::ID).unwrap();

    let asset_id = crate::client::constants::get_asset_id();

    let tx = initialize_asset(&program, asset_id.pubkey(), metadata, name).await;

    tx
}

pub fn get_assets_map() -> HashMap<String, Pubkey> {
    let mut assets_map = HashMap::new();
    
    // Add your assets and their corresponding public keys
    assets_map.insert("Coca-Cola".to_string(), "9jcPQz32ZnzH3x861wXVnRPKv4wWqBJTo7XYPzFf8FUt".parse().unwrap());
    assets_map.insert("Asset2".to_string(), "Asset2PublicKey".parse().unwrap());
    assets_map.insert("Asset3".to_string(), "Asset3PublicKey".parse().unwrap());

    assets_map
}