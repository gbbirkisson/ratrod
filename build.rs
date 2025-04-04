extern crate cbindgen;

use std::env;

use cbindgen::Config;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(Config::from_file("cbindgen.toml").expect("Unable to find config"))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("target/release/libratrod.h");
}
