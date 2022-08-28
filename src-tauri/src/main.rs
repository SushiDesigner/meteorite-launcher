#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)
]
#[tauri::command]
async fn download_file(url: String, path: std::path::PathBuf) -> Result<(), String> {
  use std::io::Write;
  use std::fs::File;
  let file_contents = reqwest::get(&url).await.map_err(|_| "Can't download file".to_string())?.bytes().await.map_err(|_| "Can't extract bytes from download".to_string())?;
  let mut file = File::create(&path).map_err(|_| "Can't create file".to_string())?;
  file.write_all(&file_contents).map_err(|_| "Can't write file contents to disk".to_string())?;
  Ok(())
}
#[tauri::command]
async fn extract(path: std::path::PathBuf, extracto: std::path::PathBuf) -> Result<(), String> {
  use zip_extensions::*;
  zip_extract(&path, &extracto).unwrap();
Ok(())
}

#[tauri::command]
async fn exists(path: std::path::PathBuf) -> bool {
  path.exists()
}

#[tauri::command]
async fn hash(path: std::path::PathBuf) -> String {
  let bytes = std::fs::read(path).unwrap();  // Vec<u8>
  sha256::digest_bytes(&bytes).into()
}

#[tauri::command]
async fn exec(exe: String) -> Result<(), String> {
  use std::process::{Command, Stdio};

  use execute::{Execute, command};
  
  let mut command = command(exe);
  
  command.stdout(Stdio::piped());

  command.spawn();
  
Ok(())
}
#[tauri::command]
async fn removefile(file: std::path::PathBuf) -> Result<(), String> {
  use std::fs;
  fs::remove_file(file);
Ok(())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![download_file, extract, exists, hash, exec,removefile])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
