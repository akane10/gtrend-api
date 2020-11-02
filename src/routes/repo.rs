use crate::helpers::{read_json, write_json};
use gtrend::{repos, Since};
use rocket_contrib::json::Json;
use serde_json::{json, Value};
use std::error::Error;

fn to_json(repos: &Vec<repos::Repository>) -> Value {
    let x: Vec<_> = repos
        .into_iter()
        .map(|x| {
            json!({
                "author": x.author,
                "name": x.name,
                "avatar": x.avatar,
                "description": x.description,
                "url": x.url,
                "language": x.programming_language,
                "languageColor": x.lang_color,
                "stars": x.stars,
                "forks": x.forks,
                "currentPeriodStars": x.current_star,
                "builtBy": x.built_by
            })
        })
        .collect();

    Value::Array(x)
}

#[get("/")]
pub fn repo_index() -> Result<Json<Value>, Box<dyn Error>> {
    let data_json = read_json(".cache/repo_index.json");

    match data_json {
        Ok(data) => Ok(Json(data)),
        _ => {
            let data = repos::builder().get_data();

            match data {
                Ok(val) => {
                    let w = write_json(".cache/repo_index.json", &to_json(&val));
                    match w {
                        _ => {
                            let x = Json(to_json(&val));
                            Ok(x)
                        }
                    }
                }
                Err(e) => Err(e),
            }
        }
    }
}

#[get("/repositories?<language>&<since>&<spoken_language_code>")]
pub fn repo_repositories(
    language: Option<String>,
    since: Option<String>,
    spoken_language_code: Option<String>,
) -> Result<Json<Value>, Box<dyn Error>> {
    let s = since.clone().map(|x| Since::from_str(&x));
    let lang: Option<String> = language.and_then(|x| match x.as_str() {
        "" => None,
        _ => Some(x.to_lowercase()),
    });
    let s_lang: Option<String> = spoken_language_code.and_then(|x| match x.as_str() {
        "" => None,
        _ => Some(x.to_lowercase()),
    });

    // println!("{:?}", lang);
    let filename = format!(
        ".cache/{}_{}_{}.json",
        lang.clone().unwrap_or("".to_string()),
        s.clone().map(|x| x.to_string()).unwrap_or("".to_string()),
        s_lang.clone().unwrap_or("".to_string())
    );

    let data_json = read_json(&filename);

    match data_json {
        Ok(data) => Ok(Json(data)),
        _ => {
            let builder = match s {
                Some(val) => repos::builder().since(val),
                _ => repos::builder().since(Since::Daily),
            };

            let data = match (lang, s_lang) {
                (Some(l), Some(sl)) => builder
                    .programming_language(&l)
                    .spoken_language(&sl)
                    .get_data(),
                (Some(l), None) => builder.programming_language(&l).get_data(),
                (None, Some(sl)) => builder.spoken_language(&sl).get_data(),
                _ => builder.get_data(),
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
