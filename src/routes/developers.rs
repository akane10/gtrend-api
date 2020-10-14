use gtrend::developers;
use gtrend::gtrend::Since;
use rocket_contrib::json::Json;
use serde_json::{json, Value};
use std::error::Error;

fn to_json(repos: Vec<developers::Developer>) -> Json<Value> {
    let x: Vec<_> = repos
        .into_iter()
        .map(|x| {
            json!({
                "username": x.username,
                "name": x.name,
                "url": x.url,
                "sponsorUrl": x.sponsor_url,
                "avatar": x.avatar,
                "repo": x.repo
            })
        })
        .collect();

    Json(Value::Array(x))
}

#[get("/developers?<language>&<since>")]
pub fn developers(
    language: Option<String>,
    since: Option<String>,
) -> Result<Json<Value>, Box<dyn Error>> {
    let lang: Option<String> = language.map(|x| x.to_lowercase());
    let s = since.map(|x| Since::from_str(&x));

    let data = developers::get_data(lang, s.unwrap_or(Since::Daily));

    match data {
        Ok(val) => Ok(to_json(val)),
        Err(e) => Err(e),
    }
}
