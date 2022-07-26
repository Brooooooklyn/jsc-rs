use std::env;
use std::fs;

use octorust::{auth::Credentials, types::ReposCreateReleaseRequest, Client};
use serde::Deserialize;
use tar::{Archive, Builder};

static ICU_DIR_TO_TAR: &[&str] = &[
  "icu/icu4c/include",
  "icu/icu4c/lib",
  "icu-linux-aarch64/lib",
];

static WEBKIT_DIR_TO_TAR: &[&str] = &[
  #[cfg(not(target_os = "windows"))]
  "WebKit/WebKitBuild/bmalloc",
  "WebKit/WebKitBuild/JavaScriptCore",
  #[cfg(target_os = "macos")]
  "WebKit/WebKitBuild/ICU/Headers/unicode",
  #[cfg(not(target_os = "windows"))]
  "WebKit/WebKitBuild/lib",
  #[cfg(target_os = "windows")]
  "WebKit/WebKitBuild/lib64",
  "WebKit/WebKitBuild/WTF",
];

static REPO_OWNER: &str = "Brooooooklyn";
static REPO_NAME: &str = "jsc-rs";

#[derive(Debug, Deserialize)]
struct RootConfig {
  workspace: Workspace,
}

#[derive(Debug, Deserialize)]
struct Workspace {
  metadata: ExternalDependencies,
}

#[derive(Debug, Deserialize)]
struct ExternalDependencies {
  icu: ExternalVersion,
  #[serde(rename = "WebKit")]
  webkit: ExternalVersion,
}

#[derive(Debug, Deserialize)]
struct ExternalVersion {
  tag: String,
}

pub async fn release(target: &str) {
  let current_path = env::current_dir().expect("Get current dir failed");
  let mut webkit_tar = Builder::new(Vec::new());
  let mut icu_tar = Builder::new(Vec::new());
  if !cfg!(target_os = "macos") {
    for dir in ICU_DIR_TO_TAR {
      let p = current_path.join(dir);
      if p.exists() {
        icu_tar
          .append_dir_all(dir, p)
          .expect(&format!("Append {} failed", dir));
      }
    }
  }
  for dir in WEBKIT_DIR_TO_TAR {
    let src_dir = current_path.join(dir);
    if src_dir.exists() {
      webkit_tar
        .append_dir_all(dir, src_dir)
        .expect(&format!("Append {} failed", dir));
    }
  }
  let client = create_client();
  let root_config = read_root_config();
  if !cfg!(target_os = "macos") {
    upload_release(
      &client,
      &root_config.workspace.metadata.icu.tag,
      target,
      icu_tar.into_inner().expect("Get icu tar failed"),
    )
    .await;
  }
  upload_release(
    &client,
    &root_config.workspace.metadata.webkit.tag,
    target,
    webkit_tar.into_inner().expect("Get icu tar failed"),
  )
  .await;
}

async fn upload_release(client: &Client, tag: &str, target: &str, tar: Vec<u8>) {
  let release = if let Ok(release) = client
    .repos()
    .get_release_by_tag(REPO_OWNER, REPO_NAME, tag)
    .await
  {
    release
  } else {
    client
      .repos()
      .create_release(
        REPO_OWNER,
        REPO_NAME,
        &ReposCreateReleaseRequest {
          name: tag.to_owned(),
          body: format!("Static linked lib and c headers for **{}**", tag),
          draft: None,
          discussion_category_name: String::new(),
          prerelease: None,
          tag_name: tag.to_owned(),
          target_commitish: String::new(),
        },
      )
      .await
      .expect("Create release failed")
  };
  let assets = client
    .repos()
    .list_release_assets(REPO_OWNER, REPO_NAME, release.id, 20, 1)
    .await
    .expect("Get assets list from release failed");
  for asset in assets.iter() {
    if asset.name == format!("{}.tar", target) {
      client
        .repos()
        .delete_release_asset(REPO_OWNER, REPO_NAME, asset.id)
        .await
        .expect("Delete release asset failed");
    }
  }
  let mut ttl = 0;
  while ttl < 5 {
    if let Ok(_) = reqwest::Client::new()
      .post(&format!(
        "https://uploads.github.com/repos/{}/{}/releases/{}/assets?name={}",
        REPO_OWNER,
        REPO_NAME,
        release.id,
        format!("{}.tar", target)
      ))
      .body(tar.clone())
      .header("Content-Type", "application/x-tar")
      .header(
        "Authorization",
        &format!(
          "token {}",
          env::var("GITHUB_TOKEN").expect("No GITHUB_TOKEN provided")
        ),
      )
      .send()
      .await
    {
      break;
    } else {
      ttl += 1;
    }
  }
  assert_ne!(ttl, 5, "Upload release failed");
}

pub async fn download(target: &str) {
  let root_config = read_root_config();
  let client = create_client();
  if !cfg!(target_os = "macos") {
    download_and_unpack_asset(&client, &root_config.workspace.metadata.icu.tag, target).await;
  }
  download_and_unpack_asset(&client, &root_config.workspace.metadata.webkit.tag, target).await;
}

fn create_client() -> Client {
  Client::new(
    "jsc-rs Release bot",
    Credentials::Token(env::var("GITHUB_TOKEN").expect("No GITHUB_TOKEN provided")),
  )
  .expect("Initialize octorust Client failed")
}

fn read_root_config() -> RootConfig {
  toml::from_slice(
    fs::read("Cargo.toml")
      .expect("Read Cargo.toml failed")
      .as_slice(),
  )
  .expect("Parse root Cargo.toml failed")
}

async fn download_and_unpack_asset(client: &Client, tag: &str, target: &str) {
  let release = client
    .repos()
    .get_release_by_tag(REPO_OWNER, REPO_NAME, &tag)
    .await
    .expect("Get release failed");
  let assets = client
    .repos()
    .list_release_assets(REPO_OWNER, REPO_NAME, release.id, 20, 1)
    .await
    .expect("List release assets failed");
  let asset = assets
    .iter()
    .find(|asset| asset.name == format!("{}.tar", target))
    .expect("Can't find asset");
  let mut ttl = 0;
  let mut tar_content = None;
  while ttl < 5 {
    if let Ok(body) = reqwest::ClientBuilder::new()
      .redirect(reqwest::redirect::Policy::limited(10))
      .build()
      .expect("Build reqwest client failed")
      .get(&asset.browser_download_url)
      .send()
      .await
      .expect("Get asset failed")
      .bytes()
      .await
    {
      tar_content = Some(body);
      break;
    } else {
      ttl += 1;
    }
  }
  let mut tar = Archive::new(tar_content.as_ref().unwrap().as_ref());
  tar
    .unpack(env::current_dir().expect("Get current dir failed"))
    .expect("Unpack tar failed");
}
