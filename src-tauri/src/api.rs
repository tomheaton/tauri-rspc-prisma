use std::path::PathBuf;

use rspc::{Config, Router};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::Context;

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
        let posts = ctx.db.post().find_many(vec![]).exec().await?;

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
          .await?;

        return Ok(post);
      });
    })
    .build();
}
