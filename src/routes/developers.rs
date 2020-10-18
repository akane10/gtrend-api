use gtrend::developers;
use gtrend::Since;
use rocket::http::RawStr;
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
    language: Option<&RawStr>,
    since: Option<String>,
) -> Result<Json<Value>, Box<dyn Error>> {
    let s = since.map(|x| Since::from_str(&x));
    let lang: Option<String> = language.and_then(|x| match x.as_str() {
        "" => None,
        _ => Some(x.to_lowercase()),
    });

    let data = match (lang, s) {
        (Some(l), None) => developers::builder().programming_language(&l).get_data(),
        (Some(l), Some(s)) => developers::builder()
            .programming_language(&l)
            .since(s)
            .get_data(),
        (None, Some(s)) => developers::builder().since(s).get_data(),
        _ => developers::builder().get_data(),
    };

    match data {
        Ok(val) => Ok(to_json(val)),
        Err(e) => Err(e),
    }
}
