use gtrend::spoken_languages;
use rocket_contrib::json::Json;
use serde_json::Value;
use std::error::Error;

#[get("/spoken_languages")]
pub fn spoken_languages() -> Result<Json<Value>, Box<dyn Error>> {
    let data_json: Value = spoken_languages::get_data_json();
    Ok(Json(data_json))
}
