extern crate build_support;

use std::{env, fs};

fn main() -> Result<(), std::io::Error> {
  let current_dir = env::current_dir().unwrap();
  let root_dir = current_dir.parent().unwrap();
  let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
  let mut build = build_support::create_cc();
  build
    .include(root_dir.join("WebKit/WebKitBuild/bmalloc/Headers"))
    .include(root_dir.join("WebKit/WebKitBuild/WTF/Headers"))
    .include(root_dir.join("WebKit/WebKitBuild/JavaScriptCore/Headers"))
    .include(root_dir.join("WebKit/WebKitBuild/JavaScriptCore/PrivateHeaders"))
    .include(root_dir.join("WebKit/WebKitBuild/JavaScriptCore/PrivateHeaders/JavaScriptCore"))
    .include(root_dir.join("WebKit/WebKitBuild/JavaScriptCore/DerivedSources"))
    .include(current_dir.join("c-api"));
  if target_os == "macos" {
    build.flag("-DJSC_API_AVAILABLE(...)=");
  }
  for file in fs::read_dir(current_dir.join("c-api")).expect("Read c-api dir failed") {
    let f = file?;
    if f.file_type()?.is_file() && f.path().extension().and_then(|e| e.to_str()) == Some("cpp") {
      build.file(f.path());
    }
  }
  build.compile("webcore");
  Ok(())
}
