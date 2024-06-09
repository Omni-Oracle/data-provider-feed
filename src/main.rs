use tracing::{error, info};
use std::sync::{Arc, Mutex};
use actix_web::web;
use tokio::task;

mod providers;
mod client;
mod server;

use providers::cocacola_eu::AppState;

use crate::providers::{
    cocacola_eu::{update_price_loop as cocacola_feed, AppState as CocaColaAppState},
    macdonald_us::{update_price_loop as macdonald_feed, AppState as MacdonaldAppState}
};


// #[cfg(feature = "async")]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();


    let shared_state = web::Data::new(AppState {
        value: Arc::new(Mutex::new(0.0)),
    });

    let server_state = shared_state.clone();
    let server_thread = std::thread::spawn(move || {
        let sys = actix_rt::System::new();
        if let Err(e) = sys.block_on(server::run_server(server_state)) {
            error!("Failed to start server: {:?}", e);
        }
    });

    // Your existing async code here
    // match providers::cocacola_eu::update_price_loop(shared_state).await {
    //     Ok(s) => info!("{:?}", s),
    //     Err(e) => error!("{:?}", e),
    // }

    let cocacola_state = web::Data::new(CocaColaAppState {
        value: Arc::new(Mutex::new(0.0)),
    });

    let macdonald_state = web::Data::new(MacdonaldAppState {
        value: Arc::new(Mutex::new(0.0)),
    });

    crate::providers::omni_provider::omni_feed(cocacola_state, macdonald_state).await;

    // Wait for the server thread to finish (it won't, as HttpServer runs indefinitely)
    if let Err(e) = server_thread.join() {
        error!("Server thread encountered an error: {:?}", e);
    }

    Ok(())
}
