// use rocket_contrib::json::Json;
use serde_json::Value;
// use std::error::Error;
use crate::error::Error;
use std::fs;
use std::fs::{metadata, File};
use std::io::Read;
use std::time::Duration;
use std::time::SystemTime;

pub fn write_json(path: &str, data: &Value) -> Result<(), Error> {
    // https://programming-idioms.org/idiom/92/save-object-into-json-file/2347/rust
    let is_exist = fs::metadata(".cache").is_ok();

    if !is_exist {
        fs::create_dir(".cache")?;
    }

    let val = &File::create(path)?;
    serde_json::to_writer(val, &data)?;
    Ok(())
}

pub fn read_json(path: &str) -> Result<Value, Error> {
    let meta = metadata(path)?;

    if let Ok(time) = meta.created() {
        let sys_time = SystemTime::now();
        let difference = sys_time.duration_since(time).unwrap();

        if difference > Duration::new(7200, 0) {
            fs::remove_file(path)?;
        }
    }

    let mut file = File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let data_json: Value = serde_json::from_str(&data)?;

    Ok(data_json)
}
