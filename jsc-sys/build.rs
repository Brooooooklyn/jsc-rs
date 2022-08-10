extern crate build_support;

use std::{env, process};

static LOW_LEVEL_INTERPRETER_LIB: &str = "libLowLevelInterpreterLib.a";

fn main() {
  println!("cargo:rerun-if-changed=c-api/binding.cpp");
  println!("cargo:rerun-if-changed=c-api/binding.hpp");
  let mut build = build_support::create_cc();
  let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
  let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
  let is_windows = target_os == "windows";
  let out_dir = env::var("OUT_DIR").unwrap();
  println!("cargo:rustc-link-search={}", &out_dir);
  let current_dir = env::current_dir().expect("get current_dir failed");
  let webkit_output_dir = current_dir.parent().unwrap().join("WebKit/WebKitBuild");
  let jsc_lib_dir = env::var("JSC_LIB_DIR").unwrap_or_else(|_| {
    {
      if is_windows {
        "lib64"
      } else {
        "lib"
      }
    }
    .to_owned()
  });
  let jsc_lib_dir_path = webkit_output_dir.join(&jsc_lib_dir);
  // WebKit/WebKitBuild/lib/libJavaScriptCore.a
  println!("cargo:rustc-link-search={}", jsc_lib_dir_path.display());
  build
    .file("c-api/binding.cpp")
    .include("c-api")
    .include(
      webkit_output_dir
        .join("JavaScriptCore")
        .to_str()
        .unwrap()
        .replace(r#"\"#, "/"),
    )
    .include(
      webkit_output_dir
        .join("JavaScriptCore/Headers")
        .to_str()
        .unwrap()
        .replace(r#"\"#, "/"),
    )
    .include(
      webkit_output_dir
        .join("WTF/Headers")
        .to_str()
        .unwrap()
        .replace(r#"\"#, "/"),
    )
    .include(
      webkit_output_dir
        .join("bmalloc/Headers")
        .to_str()
        .unwrap()
        .replace(r#"\"#, "/"),
    );
  let icu_header_dir = current_dir
    .parent()
    .unwrap()
    .join("icu/icu4c/include")
    .to_str()
    .unwrap()
    .to_owned();
  if is_windows {
    build.flag(&format!("-I{}", &icu_header_dir));
    println!(
      "cargo:rustc-link-search={}",
      current_dir
        .parent()
        .unwrap()
        .join("icu/icu4c/lib")
        .to_str()
        .unwrap()
    );
    println!("cargo:rustc-link-lib=static=sicudt");
    println!("cargo:rustc-link-lib=static=sicuuc");
    println!("cargo:rustc-link-lib=static=sicuin");
    println!("cargo:rustc-link-lib=winmm");
    println!("cargo:rustc-link-lib=shell32");
    println!("cargo:rustc-link-lib=static=JavaScriptCore");
    println!("cargo:rustc-link-lib=static=WTF");
    println!("cargo:rustc-link-lib=static=jscc");
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

      build.flag("-DJSC_API_AVAILABLE(...)=");

      if jsc_lib_dir_path.join(LOW_LEVEL_INTERPRETER_LIB).exists() {
        println!("cargo:rustc-link-lib=LowLevelInterpreterLib");
      }
      println!("cargo:rustc-link-search={}", xcode_sdk_path);
      println!("cargo:rustc-link-lib=icucore");
    } else if target_os == "linux" {
      build.flag(&format!("-I{}", &icu_header_dir));
      match target_arch.as_str() {
        "x86_64" => {
          println!(
            "cargo:rustc-link-search={}",
            current_dir
              .parent()
              .unwrap()
              .join("icu/icu4c/lib")
              .to_str()
              .unwrap()
          );
        }
        "aarch64" => {
          println!(
            "cargo:rustc-link-search={}",
            current_dir
              .parent()
              .unwrap()
              .join("icu-linux-aarch64/lib")
              .to_str()
              .unwrap()
          );
          build.flag("-DUSE_SYSTEM_MALLOC=1");
        }
        _ => {
          panic!("Unsupported arch {target_arch}");
        }
      }
      println!("cargo:rustc-link-lib=static=icudata");
      println!("cargo:rustc-link-lib=static=icuuc");
      println!("cargo:rustc-link-lib=static=icui18n");
    }
    if jsc_lib_dir_path.join("libbmalloc.a").exists() {
      println!("cargo:rustc-link-lib=static=bmalloc");
    }
    println!("cargo:rustc-link-lib=static=WTF");
    println!("cargo:rustc-link-lib=static=JavaScriptCore");
    println!("cargo:rustc-link-lib=static=jscc");
  }
  build.compile("jscc");
}
