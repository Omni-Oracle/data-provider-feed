use std::sync::Arc;
use actix_web::web;
use anchor_client::{solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair}, Client, Cluster, DynSigner};
use tokio::task;
use tracing::info;

use crate::providers::{
    cocacola_eu::{update_price_loop as cocacola_feed, AppState as CocaColaAppState},
    macdonald_us::{update_price_loop as macdonald_feed, AppState as MacdonaldAppState}
};

pub async fn omni_feed(cocacola_state: web::Data<CocaColaAppState>, macdonald_state: web::Data<MacdonaldAppState>) {
    let payer = Arc::new(DynSigner(Arc::new(Keypair::new())));
    // let client = crate::client::constants::get_client(payer);
    let client = Client::new_with_options(Cluster::Localnet, payer.clone(), CommitmentConfig::processed());
    
    let cocacola_state_clone = cocacola_state.clone();
    tokio::spawn(async move {
        cocacola_feed(client, cocacola_state_clone).await;
    });

    let macdonald_state_clone = macdonald_state.clone();
    tokio::spawn(async move {
        macdonald_feed(macdonald_state_clone).await;
    });

    // Add any additional concurrent tasks here if needed.
}

