use bmkg_wrapper::gempa;
use gempa::Url::*;
use rocket_contrib::json::Json;
use serde_json::{json, Value};
use std::error::Error;

#[get("/gempa/<data>")]
pub fn gempa_data(data: String) -> Result<Json<Value>, Box<dyn Error>> {
  let url = match data.as_str() {
    "autogempa" => Autogempa,
    "gempaterkini" => GempaTerkini,
    "gempadirasakan" => GempaDirasakan,
    "lasttsunami" => LastTsunami,
    "en_autogempa" => EnAutogempa,
    "en_gempaterkini" => EnGempaTerkini,
    _ => Autogempa,
  };
  let data = gempa::get_data(url).and_then(gempa::to_json);

  match data {
    Ok(val) => Ok(Json(json!({ "data": val }))),
    Err(e) => Err(e),
  }
}

#[get("/gempa")]
pub fn gempa() -> Result<Json<Value>, Box<dyn Error>> {
  let data = gempa::get_data(Autogempa).and_then(gempa::to_json);

  match data {
    Ok(val) => Ok(Json(json!({ "data": val }))),
    Err(e) => Err(e),
  }
}

