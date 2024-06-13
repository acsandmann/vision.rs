extern crate napi_build;

fn main() {
  println!("cargo:rustc-link-lib=framework=Vision");
  napi_build::setup();
}
