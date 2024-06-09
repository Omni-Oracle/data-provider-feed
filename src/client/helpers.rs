use anchor_client::{DynSigner, Program};
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::{solana_sdk::{
    signature::{Keypair, Signer, Signature},
    system_program,
},
Client, Cluster, ClientError};
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
}

pub async fn initialize_asset(
    program: &Program<Rc<Keypair>>,
    assetId: Pubkey,
    metadata_url: String,
    name: String
) -> std::result::Result<Signature, ClientError> {
    
    let authority = Keypair::from_base58_string("43xQCVeSAFPtDjEwyTQCirhDfybneT9p4A8HBzX3VBUonkqwi9VPUkgiS1ViZLDbhYBRRrmS6byt1EtoBMcnBESu");
    
    let (asset_pda, _bump) = Pubkey::find_program_address(&[b"OMNI".as_ref(), assetId.as_ref()], &program.id());
    println!("assetpda {}", asset_pda);

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

 let authority = Keypair::from_base58_string("43xQCVeSAFPtDjEwyTQCirhDfybneT9p4A8HBzX3VBUonkqwi9VPUkgiS1ViZLDbhYBRRrmS6byt1EtoBMcnBESu");
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