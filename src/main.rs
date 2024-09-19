use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use reqwest::{get};
use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};
use serde_json::Value;

#[derive(Deserialize)]
struct APIResponse {
    url: String,
    name: String,
    response: serde_json::Value
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    url: Option<String>,
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Save,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    if let Some(url) = args.url {
        let resp = match get(&url).await {
            Ok(resp) => resp,
            Err(err) => {
                std::process::exit(1);
                // return Err(Box::new(err) as Box<dyn Error>);
            }
        };

        let json: Value = match resp.json().await {
            Ok(json) => json,
            Err(err) => {
                eprintln!("Failed to parse JSON: {}", err);
                std::process::exit(1);
            }
        };

        if let Err(e) = write_to_file("the_meal_db.json", &json) {
            eprintln!("Failed to write to file: {}", e);
            std::process::exit(1);
        }
    }
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
