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
         name: "Playstation: Star Wars Outlaws - Standard Edition".to_string(),
         region: "EU".to_string(),
         image: "https://image.api.playstation.com/vulcan/ap/rnd/202306/3020/1916ec71f4af6c3e583a816988c41d54bafea762e02a39b2.jpg?w=940&thumb=false".to_string(),
         description: None,
         source: "https://store.playstation.com/en-us/product/UP0001-PPSA08261_00-GAME000000000000".to_string(),
         currency: Currency::Usd,
         value 
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


    let asset_id = Pubkey::from_str("8h7vkrE6hjYMLoaZLKSSkEHebrTVMoDqEUy4oXiNRWbd").expect("Invalid bs58 string");

    match crate::client::helpers::update_price(&program, asset_id, value).await {
        Ok(tx) => {
            info!(name = "Real time feed to PlayStation", "TX: {:?}", tx);
            Ok(tx)
        },
        Err(e) => {
            error!("Failed to update price: {:?}", e);
            Err(e)
        }
    }
}


pub async fn update_price_loop(data: web::Data<AppState>) -> Result<Signature, ClientError> {

        let lower_bound = 69.82;
        let upper_bound = 70.03;
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