extern crate prost;
extern crate rocksdb;
#[macro_use]
extern crate prost_derive;

pub mod model;
pub mod proto {
    pub mod text {
        include!(concat!(env!("OUT_DIR"), "/fragments.text.rs"));
    }
}
