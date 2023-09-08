use std::sync::Arc;

use prisma::PrismaClient;

// Stops the client from outputting a huge number of warnings during compilation.
#[allow(warnings, unused)]
pub mod prisma;

pub mod api;
pub mod db;
pub mod utils;

#[derive(Debug, Clone)]
pub struct Context {
  pub db: Arc<PrismaClient>,
}
