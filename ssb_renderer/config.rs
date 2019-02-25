use std::env;

fn main() {
    // Multisampling not supported on CI machine, else 8 samples are absolutely enough
    println!("cargo:rustc-env=SAMPLES={}", if env::var("TRAVIS_RUST_VERSION").is_ok() {1} else {8});
}