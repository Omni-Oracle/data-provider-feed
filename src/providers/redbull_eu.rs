use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, Responder};
use rand::Rng;
use crate::client::helpers::Currency;

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

pub async fn get_dynamic_json() -> impl Responder {
    let lower_bound = 2.8;
    let upper_bound = 2.89;
    let value = generate_random_number(lower_bound, upper_bound);
    // Example input value, you can make this dynamic
    let response_data = build_json(value);
    HttpResponse::Ok().json(response_data)
}
