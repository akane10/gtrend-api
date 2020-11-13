use crate::helpers::{read_json, write_json};
use gtrend::{developers, Since};
use rocket_contrib::json::Json;
use serde_json::{json, Value};
use std::error::Error;

fn to_json(repos: &Vec<developers::Developer>) -> Value {
    let x: Vec<_> = repos
        .iter()
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

    Value::Array(x)
}

#[get("/?<language>&<since>")]
#[tokio::main]
pub async fn developers(
    language: Option<String>,
    since: Option<String>,
) -> Result<Json<Value>, Box<dyn Error>> {
    let s = since.and_then(|x| Since::from_str(&x));
    let lang: Option<String> = language.and_then(|x| match x.as_str() {
        "" => None,
        _ => Some(x.to_lowercase()),
    });
    // println!("{:?}", lang);
    let filename = format!(
        ".cache/developers_{}_{}.json",
        lang.clone().unwrap_or("".to_string()),
        s.clone().map(|x| x.to_string()).unwrap_or("".to_string()),
    );

    let data_json = read_json(&filename);

    match data_json {
        Ok(data) => Ok(Json(data)),
        _ => {
            let data = match (lang, s) {
                (Some(l), None) => {
                    developers::builder()
                        .programming_language(&l)
                        .get_data()
                        .await
                }
                (Some(l), Some(s)) => {
                    developers::builder()
                        .programming_language(&l)
                        .since(s)
                        .get_data()
                        .await
                }
                (None, Some(s)) => developers::builder().since(s).get_data().await,
                _ => developers::builder().get_data().await,
            };

            match data {
                Ok(val) => {
                    let j = to_json(&val);

                    let w = write_json(&filename, &j);
                    match w {
                        _ => Ok(Json(j)),
                    }
                }
                Err(e) => Err(e),
            }
        }
    }
}
