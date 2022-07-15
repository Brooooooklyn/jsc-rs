use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn build() {
  let current_dir = env::current_dir().expect("get current_dir failed");
  let cmake_build_dir = current_dir.join("WebKit/WebKitBuild");
  fs::create_dir_all(&cmake_build_dir).expect("Create WebKitBuild dir failed");
  let icu4c_dir = current_dir.join("icu/icu4c/source");
  #[cfg(target_os = "macos")]
  {
    build_osx(cmake_build_dir);
  }
  #[cfg(target_os = "linux")]
  {
    build_linux(cmake_build_dir, icu4c_dir);
  }
}

#[cfg(target_os = "macos")]
fn build_osx(cmake_build_dir: PathBuf) {
  build_js_core(
    cmake_build_dir,
    JSCoreBuildConfig {
      use_pthread_permission_api: true,
      set_system_cc: false,
      ..Default::default()
    },
  )
}

#[cfg(target_os = "linux")]
fn build_linux(cmake_build_dir: PathBuf, icu4c_dir: PathBuf) {
  let mut icu4c_config = Command::new("sh");
  icu4c_config
    .arg("-c")
    .arg("./runConfigureICU Linux --enable-static --disable-shared --with-data-packaging=static")
    .env("CC", "clang")
    .env("CXX", "clang++")
    .env(
      "CXXFLAGS",
      "-std=c++20 -stdlib=libc++ -I/usr/lib/llvm-14/include/c++/v1",
    )
    .env("LDFLAGS", "-L/usr/lib/llvm-14/lib")
    .current_dir(icu4c_dir.clone())
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit());
  assert_command_success(icu4c_config, "config icu4c failed");
  let cpus = num_cpus::get();
  let mut make_icu4c = Command::new("make");
  make_icu4c
    .arg("-j")
    .arg(&format!("{}", cpus))
    .current_dir(icu4c_dir.clone())
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit());
  assert_command_success(make_icu4c, "build icu4c failed");
  build_js_core(
    cmake_build_dir,
    JSCoreBuildConfig {
      use_pthread_permission_api: false,
      set_system_cc: true,
      self_build_icu: true,
      extra_cxx_flag: format!(
        "-fuse-ld=lld -stdlib=libc++ -I{} -I{} -I/usr/lib/llvm-14/include/c++/v1 -L/usr/lib/llvm-14/lib -L{}",
        icu4c_dir.join("common").to_str().unwrap(),
        icu4c_dir.join("i18n").to_str().unwrap(),
        icu4c_dir.join("lib").to_str().unwrap(),
      ),
      ..Default::default()
    },
  )
}

#[derive(Debug, Default)]
struct JSCoreBuildConfig {
  use_pthread_permission_api: bool,
  set_system_cc: bool,
  self_build_icu: bool,
  extra_c_flag: String,
  extra_cxx_flag: String,
}

fn build_js_core(cmake_build_dir: PathBuf, config: JSCoreBuildConfig) {
  let use_pthread_permission_api_flag = if config.use_pthread_permission_api {
    "-DUSE_PTHREAD_JIT_PERMISSIONS_API=1"
  } else {
    ""
  };
  let macos_deploy_target_flag = if cfg!(target_os = "macos") {
    "-DCMAKE_OSX_DEPLOYMENT_TARGET=10.15"
  } else {
    ""
  };
  let mut cmake_config = Command::new("sh");
  let extra_c_flag = &config.extra_c_flag;
  let extra_cxx_flag = &config.extra_cxx_flag;
  let c_flags = format!("{use_pthread_permission_api_flag} {extra_c_flag}")
    .trim()
    .to_owned();
  let cxx_flags = format!("{use_pthread_permission_api_flag} -std=c++20 {extra_cxx_flag}")
    .trim()
    .to_owned();
  let icu_flag = if config.self_build_icu {
    format!(
      "-DICU_INCLUDE_DIR={}",
      env::current_dir()
        .unwrap()
        .join("icu/icu4c/source/common")
        .to_str()
        .unwrap()
    )
  } else {
    "".to_owned()
  };
  cmake_config
    .arg("-c")
    .arg(
      format!(
        r#"cmake .. \
    -DPORT="JSCOnly" \
    -DENABLE_STATIC_JSC=ON \
    -DUSE_THIN_ARCHIVES=OFF \
    -DCMAKE_BUILD_TYPE=Release \
    -DENABLE_FTL_JIT=ON \
    -DENABLE_JIT=ON \
    -DCMAKE_C_FLAGS="{c_flags}" \
    -DCMAKE_CXX_FLAGS="{cxx_flags}" \
    -G Ninja {macos_deploy_target_flag} {icu_flag}
  "#,
      )
      .trim(),
    )
    .current_dir(cmake_build_dir.clone())
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit());
  #[cfg(target_os = "macos")]
  {
    cmake_config.env("MACOSX_DEPLOYMENT_TARGET", "10.15");
  }
  #[cfg(target_os = "linux")]
  {
    cmake_config.env(
      "CMAKE_LIBRARY_PATH",
      env::current_dir()
        .unwrap()
        .join("icu/icu4c/source")
        .to_str()
        .unwrap(),
    );
  }
  if config.set_system_cc {
    cmake_config.env("CC", "clang");
    cmake_config.env("CXX", "clang++");
  }
  assert_command_success(cmake_config, "cmake config failed");
  let mut cmake_build = Command::new("cmake");
  cmake_build
    .args(&["--build", ".", "--config", "Release", "--target", "jsc"])
    .current_dir(cmake_build_dir.clone())
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit());
  if config.set_system_cc {
    cmake_build.env("CC", "clang");
    cmake_build.env("CXX", "clang++");
  }
  assert_command_success(cmake_build, "Build JavaScriptCore failed");
}

fn assert_command_success(mut command: Command, msg: &str) {
  assert!(command.output().expect(msg).status.success(), "{}", msg);
}
