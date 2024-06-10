use tracing::{error, info};
use std::sync::{Arc, Mutex};
use actix_web::web;
use tokio::task;

mod providers;
mod client;
mod server;


use crate::providers::{
    cocacola_eu::{update_price_loop as cocacola_feed, AppState as CocaColaAppState},
    macdonald_us::{update_price_loop as macdonald_feed, AppState as MacdonaldAppState},
    nike_dunk::{update_price_loop as nike_dunk_feed, AppState as NikeDunkAppState},
    nrth_face_jacket::{AppState as NorthFaceState},
    playstation_eu::{AppState as PlaystationState},
    redbull_eu::{AppState as RedbullState}
};


// #[cfg(feature = "async")]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();


    let cocacola_state = web::Data::new(CocaColaAppState {
        value: Arc::new(Mutex::new(0.0)),
    });

    let nike_dunk_state = web::Data::new(NikeDunkAppState {
        value: Arc::new(Mutex::new(0.0)),
    });

    let macdonald_state = web::Data::new(MacdonaldAppState {
        value: Arc::new(Mutex::new(0.0)),
    });

    let nrth_face_state = web::Data::new(NorthFaceState {
        value: Arc::new(Mutex::new(0.0)),
    });

    let playstation_state = web::Data::new(PlaystationState {
        value: Arc::new(Mutex::new(0.0)),
    });

    let redbull_state = web::Data::new(RedbullState {
        value: Arc::new(Mutex::new(0.0)),
    });


    let cocacola_state_clone = cocacola_state.clone();
    let nike_dunk_state_clone = nike_dunk_state.clone();
    let macdonald_state_clone = macdonald_state.clone();
    let nrth_face_state_clone = nrth_face_state.clone();
    let playstation_state_clone = playstation_state.clone();
    let redbull_state_clone = redbull_state.clone();



    // let server_state = shared_state.clone();
    let server_thread = std::thread::spawn(move || {
        let sys = actix_rt::System::new();
        if let Err(e) = sys.block_on(server::run_server(
            cocacola_state_clone, 
            macdonald_state_clone, 
            nrth_face_state_clone, 
            nike_dunk_state_clone, 
            playstation_state_clone,
            redbull_state_clone
        )) {
            error!("Failed to start server: {:?}", e);
        }
    });

     crate::providers::omni_provider::omni_feed(cocacola_state, macdonald_state, nike_dunk_state, nrth_face_state, playstation_state, redbull_state).await;

    // Wait for the server thread to finish // HttpServer runs indefinitely
    if let Err(e) = server_thread.join() {
        error!("Server thread encountered an error: {:?}", e);
    }

    Ok(())
}
