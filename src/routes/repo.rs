use gtrend::gtrend::Since;
use gtrend::repos;
use rocket_contrib::json::Json;
use serde_json::{json, Value};
use std::error::Error;

#[get("/")]
pub fn repo_index() -> Result<Json<Value>, Box<dyn Error>> {
    let data = repos::get_data(None, Since::Daily, None);

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

#[get("/repos?<language>&<since>&<spoken_lang>")]
pub fn repo_repositories(
    language: Option<String>,
    since: Option<String>,
    spoken_lang: Option<String>,
) -> Result<Json<Value>, Box<dyn Error>> {
    let s = since.map(|x| match x.to_lowercase().as_str() {
        "daily" => Since::Daily,
        "weekly" => Since::Weekly,
        "monthly" => Since::Monthly,
        _ => Since::Daily,
    });
    let lang: Option<String> = language.map(|x| x.to_lowercase());
    let s_lang: Option<String> = spoken_lang.map(|x| x.to_lowercase());

    let data = repos::get_data(lang, s.unwrap_or(Since::Daily), s_lang);

    match data {
        Ok(val) => {
            let x: Vec<_> = val.clone().into_iter().map(|i| json!(i)).collect();

            Ok(Json(Value::Array(x)))
        }
        Err(e) => Err(e),
    }
}
