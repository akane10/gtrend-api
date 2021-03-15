use crate::error::Error;
use crate::helpers::{read_json, write_json};
use gtrend::{repos, Since};
use rocket_contrib::json::Json;
use serde_json::{json, Value};

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
#[tokio::main]
pub async fn repo_index() -> Result<Json<Value>, Error> {
    let data_json = read_json(".cache/repo_index.json");

    match data_json {
        Ok(data) => Ok(Json(data)),
        _ => {
            let data = repos::builder().get_data().await?;

            let w = write_json(".cache/repo_index.json", &to_json(&data));
            match w {
                _ => {
                    let x = Json(to_json(&data));
                    Ok(x)
                }
            }
        }
    }
}

#[tokio::main]
#[get("/?<language>&<since>&<spoken_language_code>")]
pub async fn repo_repositories(
    language: Option<String>,
    since: Option<String>,
    spoken_language_code: Option<String>,
) -> Result<Json<Value>, Error> {
    let s = since.clone().and_then(|x| Since::from_str(x));
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
                (Some(l), Some(sl)) => builder.programming_language(l).spoken_language(sl),
                (Some(l), None) => builder.programming_language(l),
                (None, Some(sl)) => builder.spoken_language(sl),
                _ => builder,
            };

            let val = data.get_data().await?;
            let j = to_json(&val);
            let w = write_json(&filename, &j);
            match w {
                _ => Ok(Json(j)),
            }
        }
    }
}
