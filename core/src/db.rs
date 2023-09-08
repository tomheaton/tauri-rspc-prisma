use crate::{
  prisma::{new_client_with_url, PrismaClient},
  utils::get_trp_dir,
};

pub async fn create_db() -> Result<PrismaClient, Box<dyn std::error::Error>> {
  let library_url = get_trp_dir().join("dev.db");

  println!(
    "Connecting to library database at {}",
    library_url.display()
  );

  tokio::fs::create_dir_all(library_url.parent().unwrap()).await?;

  if !library_url.exists() {
    tokio::fs::File::create(library_url.clone()).await?;
  }

  let client = new_client_with_url(&("file:".to_string() + library_url.to_str().unwrap())).await?;

  return Ok(client);
}
