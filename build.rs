use gl_generator::{Api, Fallbacks, Profile, Registry};
use std::env;
use std::fs::File;
use std::path::PathBuf;

/// Uses the gl_generator crate to general all of the constants and functions of
fn main() {
  let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());

  println!("cargo:rerun-if-changed=build.rs");

  let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();
  Registry::new(Api::Gles2, (4, 6), Profile::Core, Fallbacks::All, [])
    .write_bindings(gl_generator::StructGenerator, &mut file)
    .unwrap();
}
