#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{
  generate_handler,
  generate_context,
  Builder,
};
mod api;
mod routers;
mod service;
mod http;
use crate::service::ipc::{hello,is_app};

fn main() {
  let context = generate_context!();
  Builder::default()
    .setup(|_| {
        // #[cfg(debug_assertions)]
        // app.get_window("main").unwrap().open_devtools();
        std::thread::spawn(|| {
          http::http_server::start();
        });
        Ok(())
    })
    .invoke_handler(generate_handler![
        hello,
        is_app
      ])
    .run(context)
    .expect("error while running tauri application");
}
