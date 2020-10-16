use rocket_contrib::json::Json;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[get("/spoken_languages")]
pub fn spoken_languages() -> Result<Json<Value>, Box<dyn Error>> {
    let mut file = File::open("spoken-languages.json")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let json_: Value = serde_json::from_str(&data).unwrap();
    Ok(Json(json_))
}
