use std::path::PathBuf;

use rspc::{Config, Router};

use crate::Context;

pub fn create_router() -> Router<Context> {
  let r = Router::<Context>::new()
    .config(
      Config::new()
        .set_ts_bindings_header("/* eslint-disable */")
        .export_ts_bindings(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../src/types/bindings.ts")),
    )
    .query("version", |t| t(|_, _: ()| env!("CARGO_PKG_VERSION")))
    .query("posts", |t| {
      t(|ctx, _: ()| async move {
        let posts = ctx.db.post().find_many(vec![]).exec().await?;

        return Ok(posts);
      })
    })
    // .mutation("createPost", |t| {
    //   t(|ctx, _: ()| async move {
    //     // let
    //   })
    // })
    .build();

  return r;
}
