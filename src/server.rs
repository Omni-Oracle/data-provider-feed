use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use crate::providers::{
    cocacola_eu::get_dynamic_json as cocacola,
    macdonald_us::get_dynamic_json as macdonald,
    nike_dunk::get_dynamic_json as nike_dunk,
    nrth_face_jacket::get_dynamic_json as nrth_face,
    playstation_eu::get_dynamic_json as playstation,
    redbull_eu::get_dynamic_json as redbull
};
use crate::client::helpers::initialize_asset_call;
use crate::providers::{
    cocacola_eu::{ AppState as CocaColaAppState},
    macdonald_us::{AppState as MacdonaldAppState},
    nike_dunk::{AppState as NikeDunkAppState},
    nrth_face_jacket::{AppState as NorthFaceState},
    playstation_eu::{AppState as PlaystationState},
    redbull_eu::{AppState as RedbullState}
};
use serde::Deserialize;

#[derive(Deserialize)]
struct InitializeAssetPayload {
    name: String,
    slug: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, This is Omni Data Feed Server!"
}

#[post("/initialize-asset")]
async fn initialize_asset(req: HttpRequest, payload: web::Json<InitializeAssetPayload>) -> impl Responder {
    let base_url = format!("{}://{}", req.connection_info().scheme(), req.connection_info().host());
    let name = &payload.name;
    let metadata = format!("{}/{}", base_url, &payload.slug);

    match initialize_asset_call(metadata, name.to_string()).await {
        Ok(signature) => {
            let signature_string = signature.to_string();
            HttpResponse::Ok().body(signature_string)
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn dynamic_route(
    path: web::Path<String>, 
    cocacola_state: web::Data<CocaColaAppState>,
    macdonald_state: web::Data<MacdonaldAppState>,
    nike_dunk_state: web::Data<NikeDunkAppState>,
    nrth_face_state: web::Data<NorthFaceState>,
    playstation_state: web::Data<PlaystationState>,
    redbull_state: web::Data<RedbullState>
) -> HttpResponse {
    match path.as_str() {
        "cocacola" => cocacola(cocacola_state).await,
        "redbull" => redbull(redbull_state).await,
        "macdonald" => macdonald(macdonald_state).await,
        "nike_dunk" => nike_dunk(nike_dunk_state).await,
        "nrth_face" => nrth_face(nrth_face_state).await,
        "playstation" => playstation(playstation_state).await,
        _ => HttpResponse::NotFound().body("Not Found"),
    }
}
pub async fn run_server(
    cocacola_shared_state: web::Data<CocaColaAppState>,
    macdonald_shared_state: web::Data<MacdonaldAppState>,
    nrth_face_state: web::Data<NorthFaceState>,
    nike_dunk_state: web::Data<NikeDunkAppState>,
    playstation_state: web::Data<PlaystationState>,
    redbull_state: web::Data<RedbullState>
) -> std::io::Result<()> {
    HttpServer::new(move|| {
        App::new()
            .app_data(cocacola_shared_state.clone())
            .app_data(macdonald_shared_state.clone())
            .app_data(nike_dunk_state.clone())
            .app_data(nrth_face_state.clone())
            .app_data(playstation_state.clone())
            .app_data(redbull_state.clone())
            .service(hello)
            .service(initialize_asset)
            //.route("/cocacola", web::get().to(cocacola))
            .route("/{name}", web::get().to(dynamic_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
