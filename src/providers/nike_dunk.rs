use std::{borrow::{Borrow, BorrowMut}, str::FromStr, sync::{Arc, Mutex}};
use anchor_client::{
    solana_client::rpc_client::RpcClient, solana_sdk::
    {signature::Signature, signer::Signer, native_token::LAMPORTS_PER_SOL},
    ClientError, Cluster};
use anchor_client::solana_sdk::pubkey::Pubkey;
use serde::Serialize;
use actix_web::{web, HttpResponse, Responder};
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};
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
         name: "Nike Dunk Low".to_string(),
         region: "US".to_string(),
         image: "https://static.nike.com/a/images/t_PDP_1728_v1/f_auto,q_auto:eco/52084f93-98d3-4d52-9b57-3f9a7061bb7f/dunk-low-mens-shoes-l12Bc1.png".to_string(),
         description: None,
         source: "https://www.nike.com/t/dunk-low-mens-shoes-l12Bc1/HF4292-100".to_string(),
         currency: Currency::Usd,
         value }
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


    let asset_id = Pubkey::from_str("9HLn9YNycj7sRwA5BWsYSFaECF84MZfv4SUtzugFkee6").expect("Invalid bs58 string");

    match crate::client::helpers::update_price(&program, asset_id, value).await {
        Ok(tx) => {
            info!(name = "Real time feed to Nike Dunk's", "TX: {:?}", tx);
            Ok(tx)
        },
        Err(e) => {
            error!("Failed to update price: {:?}", e);
            Err(e)
        }
    }
}


pub async fn update_price_loop(data: web::Data<AppState>) -> Result<Signature, ClientError> {
        let lower_bound = 125.02;
        let upper_bound = 125.32;
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