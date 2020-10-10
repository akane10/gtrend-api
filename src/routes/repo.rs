use gtrend::gtrend::Since;
use gtrend::repo;
use rocket_contrib::json::Json;
// use serde::{Deserialize, Serialize};
use rocket::http::RawStr;
use serde_json::{json, Value};
use std::error::Error;
// #[get("/gempa/<data>")]
// pub fn gempa_data(data: String) -> Result<Json<Value>, Box<dyn Error>> {
// let url = match data.as_str() {
// "autogempa" => Autogempa,
// "gempaterkini" => GempaTerkini,
// "gempadirasakan" => GempaDirasakan,
// "lasttsunami" => LastTsunami,
// "en_autogempa" => EnAutogempa,
// "en_gempaterkini" => EnGempaTerkini,
// _ => Autogempa,
// };
// let data = gempa::get_data(url).and_then(gempa::to_json);

// match data {
// Ok(val) => Ok(Json(json!({ "data": val }))),
// Err(e) => Err(e),
// }
// }

#[get("/")]
pub fn repo_index() -> Result<Json<Value>, Box<dyn Error>> {
    let data = repo::get_data(None, Since::Daily, None);

    match data {
        Ok(val) => {
            let x: Vec<_> = val
                .clone()
                .into_iter()
                .map(|i| {
                    // let serialized = serde_json::to_string(&i).unwrap();
                    // println!("serialized = {}", serialized);
                    json!(i)
                })
                .collect();

            Ok(Json(Value::Array(x)))
        }
        Err(e) => Err(e),
    }
}

// #[get("/<lang>")]
// pub fn repo_with_lang(lang: String) -> Result<Json<Value>, Box<dyn Error>> {
// let data = repo::get_data(Some(&lang), Since::Daily, None);

// match data {
// Ok(val) => {
// let x: Vec<_> = val
// .clone()
// .into_iter()
// .map(|i| {
// // let serialized = serde_json::to_string(&i).unwrap();
// // println!("serialized = {}", serialized);
// json!(i)
// })
// .collect();

// Ok(Json(Value::Array(x)))
// }
// Err(e) => Err(e),
// }
// }

#[get("/repositories?<language>&<since>")]
pub fn repo_repositories(
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
    let data = repo::get_data(lang, s.unwrap_or(Since::Daily), None);

    match data {
        Ok(val) => {
            let x: Vec<_> = val
                .clone()
                .into_iter()
                .map(|i| {
                    // let serialized = serde_json::to_string(&i).unwrap();
                    // println!("serialized = {}", serialized);
                    json!(i)
                })
                .collect();

            Ok(Json(Value::Array(x)))
        }
        Err(e) => Err(e),
    }
}