use gtrend::languages;
use rocket_contrib::json::Json;
use serde_json::Value;
use std::error::Error;

#[get("/")]
pub fn languages() -> Result<Json<Value>, Box<dyn Error>> {
    let data: Value = languages::get_data_json();
    Ok(Json(data))
}
