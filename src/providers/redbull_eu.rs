use std::{str::FromStr, sync::{Arc, Mutex}};
use anchor_client::{
    solana_client::rpc_client::RpcClient, solana_sdk::
    {signature::Signature, native_token::LAMPORTS_PER_SOL},
    ClientError};
use anchor_client::solana_sdk::pubkey::Pubkey;
use serde::Serialize;
use actix_web::{web, HttpResponse};
use tokio::time::{sleep, Duration};
use tracing::{info, error};
use omni_server as omni_oracle;
use crate::client::{helpers::Currency, math::RoundUp};
use rand::Rng;

#[derive(Serialize)]
pub struct AssetData {
    name: String,
    region: String,
    description: Option<String>,
    image: String,
    source: String,
    currency: Currency,
    value: f64,
}

pub struct AppState {
    pub value: Arc<Mutex<f64>>,
}

pub fn generate_random_number(lower_bound: f64, upper_bound: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(lower_bound..=upper_bound)
}

pub fn build_json(value: f64) -> AssetData {
    AssetData {
         name: "Red-Bull".to_string(),
         region: "US".to_string(),
         image: "https://i5.walmartimages.com/seo/Red-Bull-Energy-Drink-12-fl-oz-Can_552d2548-7ee5-4af8-af81-8cb854c38324.f905bd155feec34bc421645d9465265a.jpeg?odnHeight=2000&odnWidth=2000&odnBg=FFFFFF".to_string(),
         description: None,
         source: "https://www.walmart.com/ip/Red-Bull-Energy-Drink-12-fl-oz-Can/12018772".to_string(),
         value,
         currency: Currency::Usd
        }
}

pub async fn get_dynamic_json(data: web::Data<AppState>) -> HttpResponse {
    let value_guard = data.value.lock().unwrap();
    let response_data = build_json(*value_guard);
    HttpResponse::Ok().json(response_data)
}


pub async fn update_price(value: f64) -> Result<Signature, ClientError> {
    let payer = crate::client::constants::get_payer();
    let client = crate::client::constants::get_client(payer);
    let program = match client.program(omni_oracle::ID) {
        Ok(prog) => prog,
        Err(e) => return Err(e),
    };


    let asset_id = Pubkey::from_str("9ZnJX8rVkiEKpHgRAwNL2NTXvxoEpefoAphbtH3X137d").expect("Invalid bs58 string");

    match crate::client::helpers::update_price(&program, asset_id, value).await {
        Ok(tx) => {
            info!(name = "Real time feed to Red Bull", "TX: {:?}", tx);
            Ok(tx)
        },
        Err(e) => {
            error!("Failed to update price: {:?}", e);
            Err(e)
        }
    }
}


pub async fn update_price_loop(data: web::Data<AppState>) -> Result<Signature, ClientError> {
        let lower_bound = 1.29;
        let upper_bound = 1.32;
        let value = generate_random_number(lower_bound, upper_bound).round_up(4);

        let rpc = RpcClient::new("https://api.devnet.solana.com");

        {
            let mut value_guard = data.value.lock().unwrap();
            *value_guard = value;
        }

        let balance = rpc
          .get_balance(&crate::client::constants::get_payer_pubkey())
           .unwrap();

        let balance_in_sol: f64 = balance as f64 / LAMPORTS_PER_SOL as f64;
    
        match update_price(value).await {
            Ok(tx) => {
                // Fetch balance after update_price within the success block
                let balance_after = rpc
                    .get_balance(&crate::client::constants::get_payer_pubkey())
                    .unwrap();
                let balance_in_sol_after: f64 = balance_after as f64 / LAMPORTS_PER_SOL as f64;
    
                let cost = balance_in_sol - balance_in_sol_after;
    
                println!("COST: {:.10}", cost);
    
                // Return the transaction signature
                return Ok(tx);
            },
            Err(e) => {
                error!("Failed to update price: {:?}", e);
    
                // Return the error
                return Err(e);
            },
        }
        
    }