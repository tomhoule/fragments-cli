extern crate actix_web;
extern crate failure;
extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

pub mod model;
mod traits;

pub struct Context;

pub use traits::{Aggregate, Command, CommandResponse};

pub mod fragmentary {
    pub mod common {
        include!(concat!(env!("OUT_DIR"), "/fragmentary.common.rs"));
    }

    pub mod text {
        include!(concat!(env!("OUT_DIR"), "/fragmentary.text.rs"));
    }
}

use actix_web::Json;

fn texts_index(_: actix_web::Path<()>) -> Json<Vec<model::text::Text>> {
    Json(Vec::new())
}

fn texts_command(req: Json<model::text::CreateText>) -> Json<model::text::Text> {
    Json(req.into_inner().text.unwrap())
}

pub fn app_factory() -> actix_web::App {
    let cors_middleware = actix_web::middleware::cors::Cors::build()
        .send_wildcard()
        .finish();
    actix_web::App::new()
        .middleware(cors_middleware)
        .middleware(<actix_web::middleware::Logger as ::std::default::Default>::default())
        .resource("/texts", |texts| {
            texts.get().with(texts_index);
            texts.post().with(texts_command);
        })
}
