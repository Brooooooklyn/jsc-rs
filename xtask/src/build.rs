#![allow(unused)]

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
  #[cfg(target_os = "windows")]
  {
    build_windows(cmake_build_dir, icu4c_dir);
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
  build_icu(icu4c_dir.clone());
  build_js_core(
    cmake_build_dir,
    JSCoreBuildConfig {
      use_pthread_permission_api: false,
      set_system_cc: true,
      self_build_icu: true,
      extra_cxx_flag: format!(
        "-fuse-ld=lld -stdlib=libc++ -I{} -I/usr/lib/llvm-14/include/c++/v1 -L/usr/lib/llvm-14/lib",
        icu4c_dir
          .parent()
          .unwrap()
          .join("include")
          .to_str()
          .unwrap(),
      ),
      ..Default::default()
    },
  )
}

#[cfg(target_os = "windows")]
fn build_windows(cmake_build_dir: PathBuf, icu4c_dir: PathBuf) {
  build_icu(icu4c_dir.clone());
  let include_flags = format!(
    "-I{}",
    icu4c_dir
      .parent()
      .unwrap()
      .join("include")
      .to_str()
      .unwrap()
      .replace(r#"\"#, "/"),
  );
  build_js_core(
    cmake_build_dir,
    JSCoreBuildConfig {
      use_pthread_permission_api: false,
      set_system_cc: true,
      self_build_icu: true,
      extra_cxx_flag: include_flags.clone(),
      extra_c_flag: include_flags.clone(),
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

fn build_icu(icu4c_dir: PathBuf) {
  let sh_bin = if cfg!(target_os = "windows") {
    env::var("GNU_SH_PATH").unwrap_or_else(|_| "C:/msys64/usr/bin/sh.exe".to_string())
  } else {
    "sh".to_owned()
  };
  let mut icu4c_config = Command::new(&sh_bin);
  #[cfg(target_os = "windows")]
  {
    icu4c_config.arg("--noprofile").arg("--norc");
  }
  icu4c_config.arg("-c");
  icu4c_config.arg(&format!(
    "./runConfigureICU {} --enable-static=yes --enable-shared=no --with-data-packaging=static --prefix={} {}",
    if cfg!(target_os = "linux") {
      "Linux"
    } else if cfg!(target_os = "windows") {
      "MSYS/MSVC"
    } else {
      panic!("Unsupported OS")
    },
    icu4c_dir.parent().unwrap().to_str().unwrap().replace(r#"\"#, "/"),
    if cfg!(target_os = "windows") {
      format!("--enable-extras=no --enable-tests=no --enable-tools=no --enable-samples=no --build=x86_64-msvc-mingw64 --host=x86_64-msvc-mingw64")
    } else {
      String::new()
    }
  ));

  if !cfg!(target_os = "windows") {
    icu4c_config.env("CC", "clang").env("CXX", "clang++");
  }
  icu4c_config
    .env(
      "CFLAGS",
      if cfg!(target_os = "windows") {
        "-Gy -MD"
      } else {
        ""
      },
    )
    .env(
      "CXXFLAGS",
      if cfg!(target_os = "windows") {
        "/std:c++20 -Gy -MD"
      } else {
        "-std=c++20 -stdlib=libc++ -I/usr/lib/llvm-14/include/c++/v1"
      },
    )
    .env(
      "LDFLAGS",
      if cfg!(target_os = "windows") {
        ""
      } else {
        "-L/usr/lib/llvm-14/lib"
      },
    )
    .current_dir(icu4c_dir.clone())
    .stderr(Stdio::inherit())
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit());
  assert_command_success(icu4c_config, "config icu4c failed");
  let cpus = num_cpus::get();
  let make_program = if cfg!(target_os = "windows") {
    env::var("GNU_MAKE_PATH").unwrap_or("C:/msys64/usr/bin/make.exe".to_string())
  } else {
    "make".to_owned()
  };
  let mut make_icu4c = Command::new(&make_program);
  make_icu4c
    .arg("-j")
    .arg(&format!("{}", cpus))
    .current_dir(icu4c_dir.clone())
    .stderr(Stdio::inherit())
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit());
  assert_command_success(make_icu4c, "build icu4c failed");
  let mut install_icu4c_command = Command::new(&make_program);
  install_icu4c_command
    .arg("install")
    .current_dir(icu4c_dir.clone())
    .stderr(Stdio::inherit())
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit());
  assert_command_success(install_icu4c_command, "install icu4c failed");
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
  let cxx20_flag = if cfg!(target_os = "windows") {
    ""
  } else {
    "-std=c++20"
  };
  let cxx_flags = format!("{use_pthread_permission_api_flag} {cxx20_flag} {extra_cxx_flag}")
    .trim()
    .to_owned();
  let icu_flag = if config.self_build_icu {
    format!(
      "-DICU_INCLUDE_DIR={} -DCMAKE_LIBRARY_PATH={}",
      env::current_dir()
        .unwrap()
        .join("icu/icu4c/include")
        .to_str()
        .unwrap()
        .replace(r#"\"#, "/"),
      env::current_dir()
        .unwrap()
        .join("icu/icu4c/lib")
        .to_str()
        .unwrap()
        .replace(r#"\"#, "/")
    )
  } else {
    "".to_owned()
  };
  let icu_uc_in_flag = if cfg!(target_os = "windows") {
    format!(
      "-DICU_UC_LIBRARY_RELEASE={} -DICU_I18N_LIBRARY_RELEASE={}",
      env::current_dir()
        .unwrap()
        .join("icu/icu4c/lib/sicuuc.lib")
        .to_str()
        .unwrap()
        .replace(r#"\"#, "/"),
      env::current_dir()
        .unwrap()
        .join("icu/icu4c/lib/sicuin.lib")
        .to_str()
        .unwrap()
        .replace(r#"\"#, "/")
    )
  } else {
    String::new()
  };
  let enable_ftl_jit = if cfg!(target_os = "windows") {
    "OFF"
  } else {
    "ON"
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
    -DENABLE_FTL_JIT={enable_ftl_jit} \
    -DENABLE_JIT=ON \
    -DCMAKE_C_FLAGS="{c_flags}" \
    -DCMAKE_CXX_FLAGS="{cxx_flags}" \
    -G Ninja {icu_uc_in_flag} {macos_deploy_target_flag} {icu_flag}
  "#,
      )
      .trim()
      .replace("\\\n", ""),
    )
    .current_dir(cmake_build_dir.clone())
    .stderr(Stdio::inherit())
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit());
  #[allow(unused)]
  let icu4c_source = env::current_dir().unwrap().join("icu/icu4c/source");
  #[cfg(target_os = "macos")]
  {
    cmake_config.env("MACOSX_DEPLOYMENT_TARGET", "10.15");
  }
  cmake_config.env("CMAKE_LIBRARY_PATH", icu4c_source.to_str().unwrap());
  if config.set_system_cc {
    if !cfg!(target_os = "windows") {
      cmake_config.env("CC", "clang").env("CXX", "clang++");
    }
  }
  println!("{:?}", &cmake_config);
  assert_command_success(cmake_config, "cmake config failed");
  let mut cmake_build = Command::new("cmake");
  cmake_build
    .args(&["--build", ".", "--config", "Release", "--", "jsc"])
    .current_dir(cmake_build_dir.clone())
    .stderr(Stdio::inherit())
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit());
  if config.set_system_cc {
    if !cfg!(target_os = "windows") {
      cmake_build.env("CC", "clang");
      cmake_build.env("CXX", "clang++");
    }
  }
  assert_command_success(cmake_build, "Build JavaScriptCore failed");
}

fn assert_command_success(mut command: Command, msg: &str) {
  assert!(command.output().expect(msg).status.success(), "{}", msg);
}
