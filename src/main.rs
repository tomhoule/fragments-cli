#![feature(extern_prelude)]

extern crate actix_web;
extern crate dotenv;
extern crate env_logger;
extern crate failure;
extern crate serde;
#[macro_use]
extern crate structopt;
extern crate fragmentary;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Cli {
    #[structopt(name = "server")]
    Server,
}

fn main() -> Result<(), failure::Error> {
    dotenv::dotenv().ok();
    env_logger::init();
    match Cli::from_args() {
        Cli::Server => {
            actix_web::server::new(fragmentary::app_factory)
                .bind("127.0.0.1:8080")?
                .run();
        }
    };

    Ok(())
}
