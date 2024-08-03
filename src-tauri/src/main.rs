// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use reqwest;
use scraper::{Html, Selector};
use std::fs::create_dir_all;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use tauri::{command, State};
#[tauri::command]
async fn greet(page: String) -> Vec<std::string::String> {
    let url = format!("https://wallhaven.cc/toplist?page={}", page);
    let response = reqwest::get(&url).await.unwrap(); 
    let body = response.text().await.unwrap();
    let document = Html::parse_document(&body);
    let selector = Selector::parse(".lazyload").unwrap();

    let mut data_srcs = Vec::new();
    for element in document.select(&selector) {
        if let Some(data_src) = element.value().attr("data-src") {
            data_srcs.push(data_src.to_string());
        }
    }
    data_srcs
}
#[tauri::command]
async fn download_img(url: String) {
    let folder_path = "download";

    if !Path::new(&folder_path).exists() {
        create_dir_all(folder_path).unwrap();
        println!("Folder created successfully.");
    }
    println!("下载图片地址: {}", _url);

    // let _url = "https://w.wallhaven.cc/full/d6/wallhaven-d6yrml.jpg";

    // let mut resp = reqwest::blocking::get(_url).await.unwrap();
    // let mut out = File::create("download/img.jpg").unwrap();
    // resp.copy_to(&mut out).unwrap();
    // Command::new("open download")
    //    .output()
    //    .expect("Failed to execute command");
}

fn main() {
    tauri::Builder::default()
        // .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, download_img])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
