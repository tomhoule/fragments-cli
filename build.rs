extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/proto/text.proto"], &["src/proto"]).unwrap();
}
