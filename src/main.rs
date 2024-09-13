use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use reqwest::{get};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct APIResponse {
    url: String,
    name: String,
    response: serde_json::Value
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://themealdb.com/api/json/v1/1/lookup.php?i=53049";

    let resp = match get(url).await {
        Ok(resp) => resp,
        Err(err) => {
            return Err(Box::new(err) as Box<dyn Error>);
        }
    };

    let json: serde_json::Value = resp.json().await?;
    if let Err(e) = write_to_file("the_meal_db.json", &json) {
        eprintln!("Failed to write to file: {}", e);
        return Err(e);
    }
    Ok(())
}
 fn write_to_file<T: Serialize>(filename: &str, data: &T) -> Result<(), Box<dyn Error>> {
    let json_data = match serde_json::to_string(data) {
        Ok(json) => json,
        Err(e) => return Err(Box::new(e))
    };

    let dir = "tmp";
    let path = Path::new(dir).join(filename);

    create_dir_all(dir)?;

    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(e) => return Err(Box::new(e))
    };

    if let Err(e) = file.write_all(json_data.as_bytes()) {
        return Err(Box::new(e))
    }
    Ok(())
}
