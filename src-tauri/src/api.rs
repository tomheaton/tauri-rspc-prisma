use std::path::PathBuf;

use prisma_client_rust::QueryError;
use rspc::{Config, Router};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{prisma, Context};

pub fn create_router() -> Router<Context> {
  return Router::<Context>::new()
    .config(
      Config::new()
        .set_ts_bindings_header("/* eslint-disable */")
        .export_ts_bindings(
          PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../src/types/bindings.ts"),
        ),
    )
    .query("version", |t| {
      return t(|_, _: ()| env!("CARGO_PKG_VERSION"));
    })
    .query("posts", |t| {
      return t(|ctx, _: ()| async move {
        let posts = ctx
          .db
          .post()
          .find_many(vec![])
          .exec()
          .await
          .map_err(|e: QueryError| {
            rspc::Error::with_cause(
              rspc::ErrorCode::InternalServerError,
              "Internal server error occurred while completing database operation!".into(),
              e,
            )
          })?;

        return Ok(posts);
      });
    })
    .mutation("createPost", |t| {
      #[derive(Debug, Clone, Deserialize, Serialize, Type)]
      struct CreatePostInput {
        title: String,
        content: String,
      }

      return t(|ctx, input: CreatePostInput| async move {
        let post = ctx
          .db
          .post()
          .create(input.title, input.content, vec![])
          .exec()
          .await
          .map_err(|e: QueryError| {
            rspc::Error::with_cause(
              rspc::ErrorCode::InternalServerError,
              "Internal server error occurred while completing database operation!".into(),
              e,
            )
          })?;

        return Ok(post);
      });
    })
    .mutation("deletePost", |t| {
      #[derive(Debug, Clone, Deserialize, Serialize, Type)]
      struct DeletePostInput {
        id: i32,
      }

      return t(|ctx, input: DeletePostInput| async move {
        ctx
          .db
          .post()
          .delete(prisma::post::id::equals(input.id))
          .exec()
          .await
          .map_err(|e: QueryError| {
            rspc::Error::with_cause(
              rspc::ErrorCode::InternalServerError,
              "Internal server error occurred while completing database operation!".into(),
              e,
            )
          })?;

        return Ok(());
      });
    })
    .build();
}
