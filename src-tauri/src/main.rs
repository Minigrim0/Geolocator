// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod utils;
mod coordinates;
mod image;
use glob::glob;


/// Opens a folder, reads all the images in it and returns a list of "image" structures
#[tauri::command]
fn open(path: &str) -> Result<Vec<image::Image>, String> {

    let mut images = Vec::new();
    for extension in &["jpg", "jpeg", "png"] {
        for entry in glob(format!("{path}/**/*.{extension}").as_str()).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    match image::Image::new(path.to_str().unwrap()) {
                        Ok(image) => images.push(image),
                        Err(e) => println!("{:?}", e),
                    }
                },
                Err(e) => println!("{:?}", e),
            }
        }
    }

    Ok(images)
}

/// Opens a folder, reads all the images in it and returns a list of "image" structures
#[tauri::command]
fn save_image(file: image::Image) -> Result<(), String> {
    file.save()
}


fn main() {
    env_logger::init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open, save_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
