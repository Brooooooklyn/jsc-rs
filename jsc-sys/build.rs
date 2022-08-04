extern crate cc;

use std::env;
use std::process;

static LOW_LEVEL_INTERPRETER_LIB: &str = "libLowLevelInterpreterLib.a";

fn main() {
  println!("cargo:rerun-if-changed=c-api/binding.cpp");
  println!("cargo:rerun-if-changed=c-api/binding.hpp");
  let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
  let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
  let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
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
  let mut build = cc::Build::new();
  build
    .file("c-api/binding.cpp")
    .cpp(true)
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
    )
    .cargo_metadata(false);
  let icu_header_dir = current_dir
    .parent()
    .unwrap()
    .join("icu/icu4c/include")
    .to_str()
    .unwrap()
    .to_owned();
  if is_windows {
    // WebKit/Source/cmake/OptionsMSVC.cmake
    static MSVC_FLAGS: &[&str] = &[
      "/wd4018", //'token' : signed/unsigned mismatch
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-3-c4018
      "/wd4060", //switch statement contains no 'case' or 'default' labels
      "/wd4068", //unknown pragma
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-1-c4068
      "/wd4100", //'identifier' : unreferenced formal parameter
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-4-c4100
      "/wd4127", //conditional expression is constant
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-4-c4127
      "/wd4146", //unary minus operator applied to unsigned type, result still unsigned
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-2-c4146
      "/wd4189", //'identifier' : local variable is initialized but not referenced
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-4-c4189
      "/wd4201", //nonstandard extension used : nameless struct/union
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-4-c4201
      "/wd4244", //'argument' : conversion from 'type1' to 'type2', possible loss of data
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-2-c4244
      "/wd4245", //'conversion' : conversion from 'type1' to 'type2', signed/unsigned mismatch
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-4-c4245
      "/wd4251", //'identifier' : class 'type' needs to have dll-interface to be used by clients of class 'type2'
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-1-c4251
      "/wd4275", //non - DLL-interface class 'class_1' used as base for DLL-interface class 'class_2'
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-2-c4275
      "/wd4267", //'var' : conversion from 'size_t' to 'type', possible loss of data
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-3-c4267
      "/wd4305", //'context' : truncation from 'type1' to 'type2'
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-1-c4305
      "/wd4309", //'conversion' : truncation of constant value
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-2-c4309
      "/wd4312", //'operation' : conversion from 'type1' to 'type2' of greater size
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-1-c4312
      "/wd4324", //'struct_name' : structure was padded due to __declspec(align())
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-4-c4324
      "/wd4389", //'operator' : signed/unsigned mismatch
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-4-c4389
      "/wd4456", //declaration of 'identifier' hides previous local declaration
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-4-c4456
      "/wd4457", //declaration of 'identifier' hides function parameter
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-4-c4457
      "/wd4458", //declaration of 'identifier' hides class member
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-4-c4458
      "/wd4459", //declaration of 'identifier' hides global declaration
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-4-c4459
      "/wd4505", //'function' : unreferenced local function has been removed
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-4-c4505
      "/wd4611", //interaction between 'function' and C++ object destruction is non-portable
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-4-c4611
      "/wd4646", //function declared with __declspec(noreturn) has non-void return type
      //https://docs.microsoft.com/mt-mt/cpp/error-messages/compiler-warnings/compiler-warning-level-3-c4646
      "/wd4701", //Potentially uninitialized local variable 'name' used
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-4-c4701
      "/wd4702", //unreachable code
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-4-c4702
      "/wd4706", //assignment within conditional expression
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-4-c4706
      //NOTE: Can't fix without changes to style guide
      "/wd4715", //'function' : not all control paths return a value
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-1-c4715
      "/wd4722", //'function' : destructor never returns, potential memory leak
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-1-c4722
      "/wd4838", //conversion from 'type_1' to 'type_2' requires a narrowing conversion
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-1-c4838
      "/wd4840", //non-portable use of class 'type' as an argument to a variadic function
      //https://docs.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warning-level-4-c4840
      "/wd4996", //Your code uses a function, class member, variable, or typedef that's marked deprecated
      "/wd5205", //delete of an abstract class 'type-name' that has a non-virtual destructor results in undefined behavior
      "/wd5054", //operator 'operator-name': deprecated between enumerations of different types
      "/wd5055", //operator 'operator-name': deprecated between enumerations and floating-point types
    ];
    build
      .flag("/std:c++20")
      .flag(&format!("-I{}", &icu_header_dir));
    for flag in MSVC_FLAGS.iter() {
      build.flag(*flag);
    }
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
    build.flag("-std=c++20");
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
      println!("cargo:rustc-link-lib=c++");
      println!("cargo:rustc-link-lib=icucore");
    } else if target_os == "linux" {
      build
        .compiler("clang++")
        .flag(&format!("-I{}", &icu_header_dir));
      match target_arch.as_str() {
        "x86_64" => {
          if target_env == "musl" {
            build
              .include("/usr/include/c++/11.2.1/")
              .include("/usr/include/c++/11.2.1/x86_64-alpine-linux-musl");
          } else {
            build.cpp_set_stdlib("c++");
            println!("cargo:rustc-link-search=/usr/lib/llvm-14/lib");
            println!("cargo:rustc-link-search=/usr/lib/gcc/x86_64-linux-gnu/9");
          }
          println!("cargo:rustc-link-lib=static=atomic");
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
          println!("cargo:rustc-link-search=/usr/aarch64-unknown-linux-gnu/lib/llvm-14/lib");
          println!("cargo:rustc-link-search=/usr/aarch64-unknown-linux-gnu/lib");
          println!("cargo:rustc-link-search=/usr/aarch64-unknown-linux-gnu/aarch64-unknown-linux-gnu/sysroot/lib");
          println!("cargo:rustc-link-search=/usr/aarch64-unknown-linux-gnu/lib/gcc/aarch64-unknown-linux-gnu/4.8.5");
          println!(
            "cargo:rustc-link-search={}",
            current_dir
              .parent()
              .unwrap()
              .join("icu-linux-aarch64/lib")
              .to_str()
              .unwrap()
          );
          build
            .include("/usr/aarch64-unknown-linux-gnu/lib/llvm-14/include/c++/v1")
            .include("/usr/aarch64-unknown-linux-gnu/aarch64-unknown-linux-gnu/sysroot/usr/include")
            .flag("-DUSE_SYSTEM_MALLOC=1")
            .flag("--sysroot=/usr/aarch64-unknown-linux-gnu/aarch64-unknown-linux-gnu/sysroot");
        }
        _ => {
          panic!("Unsupported arch {target_arch}");
        }
      }
      if target_env == "musl" {
        println!("cargo:rustc-link-search=/usr/lib");
        println!("cargo:rustc-link-lib=static=stdc++");
      } else {
        println!("cargo:rustc-link-lib=static=c++");
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
