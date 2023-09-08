// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use tauri_rspc_prisma_core::{prisma::PrismaClient, Context};

async fn migrate_and_populate(
  client: &Arc<PrismaClient>,
) -> Result<(), Box<dyn std::error::Error>> {
  #[cfg(debug_assertions)]
  client._db_push().await?;

  #[cfg(not(debug_assertions))]
  client._migrate_deploy().await.unwrap();

  return Ok(());
}

#[tokio::main]
async fn main() {
  let router = tauri_rspc_prisma_core::api::create_router();
  let db = Arc::new(tauri_rspc_prisma_core::db::create_db().await.unwrap());

  migrate_and_populate(&db).await.unwrap();

  tauri::Builder::default()
    .plugin(rspc::integrations::tauri::plugin(
      router.arced(),
      move || Context {
        db: Arc::clone(&db),
      },
    ))
    .run(tauri::generate_context!())
    .expect("Error while running Tauri application!");
}
