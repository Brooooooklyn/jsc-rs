use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::process::Stdio;

pub fn build() {
  let current_dir = env::current_dir().expect("get current_dir failed");
  let cmake_build_dir = current_dir.join("WebKit/WebKitBuild");
  fs::create_dir_all(&cmake_build_dir).expect("Create WebKitBuild dir failed");
  #[cfg(target_os = "macos")]
  {
    build_osx(cmake_build_dir);
  }
}

fn build_osx(cmake_build_dir: PathBuf) {
  let mut cmake_config = process::Command::new("sh");
  cmake_config
    .arg("-c")
    .arg(
      r#"cmake .. \
    -DPORT="JSCOnly" \
    -DENABLE_STATIC_JSC=ON \
    -DUSE_THIN_ARCHIVES=OFF \
    -DCMAKE_BUILD_TYPE=Release \
    -DENABLE_FTL_JIT=ON \
    -DENABLE_JIT=ON \
    -DCMAKE_OSX_DEPLOYMENT_TARGET=10.15 \
    -DCMAKE_C_FLAGS="-DUSE_PTHREAD_JIT_PERMISSIONS_API=1" \
    -DCMAKE_CXX_FLAGS="-DUSE_PTHREAD_JIT_PERMISSIONS_API=1 -std=c++20" \
    -G Ninja
  "#,
    )
    .current_dir(cmake_build_dir.clone())
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit())
    .env("MACOSX_DEPLOYMENT_TARGET", "10.15");
  let cmake_config_status = cmake_config
    .output()
    .expect("cmake config failed")
    .status
    .success();
  assert!(cmake_config_status, "cmake config failed");
  let cmake_build_status = process::Command::new("cmake")
    .args(&["--build", ".", "--config", "Release", "--target", "jsc"])
    .current_dir(cmake_build_dir)
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit())
    .output()
    .expect("failed to run cmake build")
    .status
    .success();
  assert!(cmake_build_status, "cmake build failed");
}
