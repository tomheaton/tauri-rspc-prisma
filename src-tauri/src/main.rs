// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

pub mod api;
#[allow(warnings, unused)]
pub mod prisma;

#[derive(Debug, Clone)]
pub struct Context {
  pub db: Arc<prisma::PrismaClient>,
}

#[tokio::main]
async fn main() {
  let router = api::create_router();
  let db = prisma::new_client().await.unwrap();

  #[cfg(debug_assertions)]
  db._db_push().await.unwrap();

  tauri::Builder::default()
    .plugin(rspc_tauri::plugin(router.arced(), |_| Context {
      db: Arc::new(db),
    }))
    .run(tauri::generate_context!())
    .expect("Error while running Tauri application!");
}
