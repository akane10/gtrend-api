use gtrend::repos;
use gtrend::Since;
use rocket_contrib::json::Json;
use serde_json::{json, Value};
use std::error::Error;
use std::fs;
use std::fs::{metadata, File};
use std::io::Read;
use std::time::Duration;
use std::time::SystemTime;

fn write_json(path: &str, data: &Value) -> Result<(), Box<dyn Error>> {
    // https://programming-idioms.org/idiom/92/save-object-into-json-file/2347/rust
    let is_exist = fs::metadata(".cache").is_ok();

    if !is_exist {
        fs::create_dir(".cache")?;
    }

    let val = &File::create(path)?;
    serde_json::to_writer(val, &data)?;
    Ok(())
}

fn read_json(path: &str) -> Result<Value, Box<dyn Error>> {
    let meta = metadata(path)?;

    if let Ok(time) = meta.created() {
        let sys_time = SystemTime::now();
        let difference = sys_time.duration_since(time).unwrap();

        println!("{:?}", difference);
        println!("{:?}", time);

        if difference > Duration::new(3600, 0) {
            fs::remove_file(path)?;
        }
    }
    // else {
    // println!("Not supported on this platform or filesystem");
    // };

    let mut file = File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let data_json: Value = serde_json::from_str(&data)?;

    Ok(data_json)
}

fn to_json(repos: Vec<repos::Repository>) -> Json<Value> {
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

    Json(Value::Array(x))
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
                    let w = write_json(".cache/repo_index.json", &to_json(val.clone()));
                    match w {
                        _ => Ok(to_json(val)),
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
                    let j = to_json(val.clone());
                    let x = json!(val);

                    let w = write_json(&filename, &x);
                    match w {
                        _ => Ok(j),
                    }
                }
                Err(e) => Err(e),
            }
        }
    }
}
