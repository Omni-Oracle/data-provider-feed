use std::{borrow::{Borrow, BorrowMut}, str::FromStr};
use std::sync::{Arc, Mutex};
use anchor_client::{
    solana_client::rpc_client::RpcClient, solana_sdk::
    {signature::Signature, native_token::LAMPORTS_PER_SOL},
    ClientError, Cluster};
use anchor_client::solana_sdk::pubkey::Pubkey;
use serde::Serialize;
use actix_web::{web, HttpResponse, Responder};
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};
use omni_server as omni_oracle;
use crate::client::helpers::Currency;
use crate::client::math::RoundUp;
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
         name: "Double Quarter Pounder with Cheese Price & Calories".to_string(),
         region: "US".to_string(),
         image: "https://mcd-menu.com/wp-content/uploads/2024/01/Double-Quarter-Pounder%C2%AE-with-Cheese.webp".to_string(),
         source: "https://mcd-menu.com/double-quarter-pounder-with-cheese/".to_string(),
         currency: Currency::Usd,
         description: None,
         value }
}

pub async fn get_dynamic_json(data: web::Data<AppState>) -> impl Responder {
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


    let asset_id = Pubkey::from_str("9jcPQz32ZnzH3x861wXVnRPKv4wWqBJTo7XYPzFf8FUt").expect("Invalid bs58 string");

    match crate::client::helpers::update_price(&program, asset_id, value).await {
        Ok(tx) => {
            info!(name = "Real time feed to MacDonalds: Double Quarter", "TX: {:?}", tx);
            Ok(tx)
        },
        Err(e) => {
            error!("Failed to update price: {:?}", e);
            Err(e)
        }
    }
}


pub async fn update_price_loop(data: web::Data<AppState>) -> Result<Signature, ClientError> {
    loop {
        let lower_bound = 9.69;
        let upper_bound = 9.85; 
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
        Ok(tx) =>  {
                // Fetch balance after update_price within the success block
                let balance_after = rpc
                    .get_balance(&crate::client::constants::get_payer_pubkey())
                    .unwrap();
                let balance_in_sol_after: f64 = balance_after as f64 / LAMPORTS_PER_SOL as f64;

                let cost = balance_in_sol - balance_in_sol_after;
                
                println!("COST: {:.10}", cost);
            },
            Err(e) => error!("Failed to update price: {:?}", e),
        }
        sleep(Duration::from_secs(1)).await;
    }
}