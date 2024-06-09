use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use crate::providers::{
    cocacola_eu::get_dynamic_json as cocacola,
    macdonald_us::get_dynamic_json as macdonald
};
use crate::client::helpers::initialize_asset_call;
use crate::providers::cocacola_eu::AppState;
use serde::Deserialize;

#[derive(Deserialize)]
struct InitializeAssetPayload {
    name: String,
}

#[get("/")]
async fn hello(req: HttpRequest) -> impl Responder {
    let base_url = format!("{}://{}", req.connection_info().scheme(), req.connection_info().host());
    let full_url = format!("{}/test", base_url);
    println!("Base URL: {}, Full URL: {}", base_url, full_url);
    "Hello, This is Omni Data Feed Server!"
}

#[post("/initialize-asset")]
async fn initialize_asset(req: HttpRequest, payload: web::Json<InitializeAssetPayload>) -> impl Responder {
    let base_url = format!("{}://{}", req.connection_info().scheme(), req.connection_info().host());
    let name = &payload.name;
    let metadata = format!("{}/{}", base_url, name);

    match initialize_asset_call(metadata, name.to_string()).await {
        Ok(signature) => {
            let signature_string = signature.to_string();
            HttpResponse::Ok().body(signature_string)
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn run_server(shared_state: web::Data<AppState>) -> std::io::Result<()> {
    HttpServer::new(move|| {
        App::new()
            .app_data(shared_state.clone())
            .service(hello)
            .service(initialize_asset)
            .route("/cocacola", web::get().to(cocacola))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
