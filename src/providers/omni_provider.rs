use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use actix_web::web;
use anchor_client::{
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair},
    Client, Cluster, DynSigner,
};
use tracing::info;

use crate::providers::{
    cocacola_eu::{update_price_loop as cocacola_feed, AppState as CocaColaAppState},
    macdonald_us::{update_price_loop as macdonald_feed, AppState as MacdonaldAppState},
    nike_dunk::{update_price_loop as nike_dunk_feed, AppState as NikeDunkAppState},
    nrth_face_jacket::{update_price_loop as nrth_face_feed, AppState as NorthFaceState},
    playstation_eu::{update_price_loop as playstation_feed, AppState as PlaystationState},
    redbull_eu::{update_price_loop as redbull_feed, AppState as RedbullState}
};

pub async fn omni_feed(
    cocacola_state: web::Data<CocaColaAppState>, 
    macdonald_state: web::Data<MacdonaldAppState>,
    nike_dunk_state: web::Data<NikeDunkAppState>,
    nrth_face_state: web::Data<NorthFaceState>,
    playstation_state: web::Data<PlaystationState>,
    redbull_state: web::Data<RedbullState>
) {
    // let payer = Arc::new(DynSigner(Arc::new(Keypair::new())));
    // let client = Client::new_with_options(Cluster::Localnet, payer.clone(), CommitmentConfig::processed());

    let task_queue = Arc::new(Mutex::new(VecDeque::new()));

    // Add tasks to the queue
    {
        let mut queue = task_queue.lock().unwrap();
        queue.push_back(Task::CocaCola);
        queue.push_back(Task::MacDonald);
        queue.push_back(Task::NikeDunk);
        queue.push_back(Task::NorthFace);
        queue.push_back(Task::Playstation);
        queue.push_back(Task::RedBull);
    }

    // Process tasks sequentially
    process_tasks( cocacola_state, macdonald_state, nike_dunk_state, nrth_face_state, playstation_state, redbull_state, task_queue).await;
}

// Define the task types
enum Task {
    CocaCola,
    MacDonald,
    NikeDunk,
    NorthFace,
    Playstation,
    RedBull
}

async fn process_tasks(
    cocacola_state: web::Data<CocaColaAppState>,
    macdonald_state: web::Data<MacdonaldAppState>,
    nike_dunk_state: web::Data<NikeDunkAppState>,
    nrth_face_state: web::Data<NorthFaceState>,
    playstation_state: web::Data<PlaystationState>,
    redbull_state: web::Data<RedbullState>,
    task_queue: Arc<Mutex<VecDeque<Task>>>,
) {
    loop {
        let mut cocacola_processed = false;
        let mut macdonald_processed = false;
        let mut nikedunk_processed = false;
        let mut playstation_processed = false;
        let mut redbull_processed = false;

        while !cocacola_processed || !macdonald_processed || !nikedunk_processed || !playstation_processed  {
            let task_option = {
                let mut queue = task_queue.lock().unwrap();
                queue.pop_front()
            };

            if let Some(task) = task_option {
                match task {
                    Task::CocaCola => {
                        if let Err(e) = cocacola_feed(cocacola_state.clone()).await {
                            info!("Failed to update CocaCola price: {:?}", e);
                        }
                        cocacola_processed = true;
                    }
                    Task::MacDonald => {
                        if let Err(e) = macdonald_feed(macdonald_state.clone()).await {
                            info!("Failed to update MacDonald price: {:?}", e);
                        }
                        macdonald_processed = true;
                    }
                    Task::NikeDunk => {
                        if let Err(e) = nike_dunk_feed(nike_dunk_state.clone()).await {
                            info!(" Failed to update Nike Dunk's price: {:?}", e);
                        }
                        nikedunk_processed = true;
                    }
                    Task::NorthFace => {
                        if let Err(e) = nrth_face_feed(nrth_face_state.clone()).await {
                            info!("Failed to update North Face Jacket price: {:?}", e);
                        }
                    }
                    Task::Playstation => {
                        if let Err(e) = playstation_feed(playstation_state.clone()).await {
                            info!("Failed to update PlayStation price: {:?}", e);
                        }
                        playstation_processed = true
                    }
                    Task::RedBull => {
                        if let Err(e) = redbull_feed(redbull_state.clone()).await {
                            info!("Failed to update Red Bull price: {:?}", e);
                        }
                        redbull_processed = true;
                    }
                }

                // Re-add the task to the queue for the next iteration
                {
                    let mut queue = task_queue.lock().unwrap();
                    queue.push_back(task);
                }
            } else {
                // If no tasks are available, sleep for a while before checking again
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }
}
