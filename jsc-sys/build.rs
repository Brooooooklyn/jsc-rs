extern crate cc;

use std::env;
use std::process;

fn main() {
  println!("cargo:rerun-if-changed=c-api/binding.cpp");
  println!("cargo:rerun-if-changed=c-api/binding.hpp");
  cc::Build::new()
    .file("c-api/binding.cpp")
    .cpp(true)
    .flag("-std=c++20")
    .include("c-api")
    .include("../WebKit/Source")
    .include("../WebKit/WebKitBuild/JavaScriptCore")
    .include("../WebKit/WebKitBuild/WTF/Headers")
    .include("../WebKit/WebKitBuild/bmalloc/Headers")
    .cargo_metadata(false)
    .compile("jscc");
  let out_dir = env::var("OUT_DIR").unwrap();
  println!("cargo:rustc-link-search={}", &out_dir);
  let current_dir = env::current_dir().expect("get current_dir failed");
  let jsc_lib_dir = env::var("JSC_LIB_DIR").unwrap_or_else(|_| "WebKit/WebKitBuild/lib".to_owned());
  let jsc_lib_dir_path = current_dir.parent().unwrap().join(&jsc_lib_dir);
  // WebKit/WebKitBuild/lib/libJavaScriptCore.a
  println!("cargo:rustc-link-search={}", jsc_lib_dir_path.display());
  let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
  let is_windows = target_os == "windows";
  if is_windows {
    println!("cargo:rustc-link-lib=bmalloc");
    println!("cargo:rustc-link-lib=WTF");
    println!("cargo:rustc-link-lib=JavaScriptCore");
    println!("cargo:rustc-link-lib=jscc");
  } else {
    if target_os == "macos" {
      let xcrun_output = process::Command::new("xcrun")
        .args(&["-sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("failed to get macos sdk path")
        .stdout;
      let xcode_sdk_path = String::from_utf8_lossy(xcrun_output.as_slice())
        .trim()
        .to_owned();
      println!("cargo:rustc-link-search={}", xcode_sdk_path);
      println!("cargo:rustc-link-lib=c++");
      println!("cargo:rustc-link-lib=icucore");
    }
    println!("cargo:rustc-link-lib=static=bmalloc");
    println!("cargo:rustc-link-lib=static=WTF");
    println!("cargo:rustc-link-lib=static=JavaScriptCore");
    println!("cargo:rustc-link-lib=static=jscc");
  }
}
