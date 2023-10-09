mod file_operation;
mod upload_file;

use anyhow::Context;
use file_operation::read_folder;
use rand::Rng;
use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    thread::sleep,
    time::{Duration, SystemTime},
};

use crate::upload_file::begin_upload;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let folder_path = args
        .get(1)
        .expect("Please provide folder path as first argument");

    println!("folder path : {}", folder_path);
    let savedfile_name = args
        .get(2)
        .expect("Please provide folder path as first argument");

    println!("saved file name : {}", savedfile_name);

    let folder_path = Path::new(&folder_path);
    let images = read_folder(&folder_path).expect("No image file in folder");

    match savedfile_name.to_lowercase().trim() {
        "watermark" => start(images, "watermark")?,
        "thumbnail" => start(images, "thumbnail")?,
        "main" => start(images, "main")?,
        _ => println!("Please pick watermark/thumbnail/main in 2nd argument"),
    }

    Ok(())
}

fn start(images: Vec<PathBuf>, savedfile_name: &str) -> anyhow::Result<()> {
    let mut metadata = Vec::new();
    let mut rng = rand::thread_rng();

    for image_path in images {
        let wait_time = rng.gen_range(1.5..3.0);
        sleep(Duration::from_secs_f64(wait_time));
        let response_upload = begin_upload(&image_path)?;
        println!("success uploading : {:?}", &image_path);

        let data = response_upload.get("data").context("data json not found")?;

        metadata.push(data.clone());
    }

    let now = SystemTime::now();
    let time_now = now.duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
    let jsonfile_name = format!("imgbb-{}-{}.json", savedfile_name, time_now);

    let json_str = serde_json::to_string_pretty(&metadata)?;

    let mut json_file = File::create(jsonfile_name)?;
    json_file.write_all(json_str.as_bytes())?;
    Ok(())
}
