use std::{env, path::Path};

use reqwest::blocking::{multipart, Client};
use serde_json::Value;

use dotenvy::dotenv;

pub fn begin_upload(image_path: &Path) -> anyhow::Result<Value> {
    dotenv().ok();

    let uri_endpoint = env::var("URI_ENDPOINT").expect("URI_ENDPOINT not set");
    let files = multipart::Form::new().file("image", image_path)?;

    let client = Client::new();

    let response = client.post(uri_endpoint).multipart(files).send()?.json()?;

    Ok(response)
}
