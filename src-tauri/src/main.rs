// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::{Error, Response};
use scraper::{Html, Selector};
use tauri::{command, State};
 
#[tauri::command]
async fn Crawler_WallHaven(page: String) -> Vec<std::string::String> {
    let url = format!("https://wallhaven.cc/toplist?page={}", page);
    println!("开始打开网址:{}", url);
    let response = reqwest::get(&url).await.unwrap();
    println!("接收:{}", url);
    // let status = response.status();

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

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![Crawler_WallHaven])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
