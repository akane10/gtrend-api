use gtrend::developers;
use gtrend::gtrend::Since;
use rocket::http::RawStr;
use rocket_contrib::json::Json;
use serde_json::{json, Value};
use std::error::Error;

#[get("/developers?<language>&<since>")]
pub fn developers(
    language: Option<&RawStr>,
    since: Option<String>,
) -> Result<Json<Value>, Box<dyn Error>> {
    let s = since.map(|x| match x.as_str() {
        "daily" => Since::Daily,
        "weekly" => Since::Weekly,
        "monthly" => Since::Monthly,
        _ => Since::Daily,
    });
    let lang: Option<&str> = language.map(|x| x.as_str());

    let data = developers::get_data(lang, s.unwrap_or(Since::Daily));

    match data {
        Ok(val) => {
            let x: Vec<_> = val.clone().into_iter().map(|i| json!(i)).collect();

            Ok(Json(Value::Array(x)))
        }
        Err(e) => Err(e),
    }
}
