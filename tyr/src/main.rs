#![allow(non_upper_case_globals)]

use std::fs;

use clap::{self, Parser};
use jsc_safe::{ClassAttribute, ClassDefinition, Context, PropertyAttributes};

#[cfg(all(not(all(target_os = "linux", target_env = "musl", target_arch = "aarch64")),))]
#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

mod console;

#[derive(Parser)]
#[clap(author, version, about = "tyr", long_about = "Tyr JavaScript runtime")]
#[clap(propagate_version = true)]
struct Tyr {
  /// JavaScript file to run
  entry: String,
}

fn main() -> Result<(), anyhow::Error> {
  let global_class = ClassDefinition::default()
    .with_name("globalThis")?
    .with_attribute(ClassAttribute::NoAutomaticPrototype)
    .into_class()?;
  let ctx = Context::with_global_class(global_class)?;
  let mut global = ctx.global();
  let mut console = ClassDefinition::default()
    .with_name("Console")?
    .with_attribute(ClassAttribute::NoAutomaticPrototype)
    .into_class()?
    .make_object(&ctx);
  let log = ctx.create_function("log", Some(console::console_log))?;
  let info = ctx.create_function("info", Some(console::console_log))?;
  let warn = ctx.create_function("warn", Some(console::console_log))?;
  let error = ctx.create_function("error", Some(console::console_log))?;
  console
    .set_property("log", &log, PropertyAttributes::None)
    .unwrap();
  console
    .set_property("info", &info, PropertyAttributes::None)
    .unwrap();
  console
    .set_property("warn", &warn, PropertyAttributes::None)
    .unwrap();
  console
    .set_property("error", &error, PropertyAttributes::None)
    .unwrap();
  global
    .set_property("console", &console, PropertyAttributes::DontDelete)
    .unwrap();
  let tyr = Tyr::parse();
  let script = fs::read_to_string(tyr.entry)?;
  ctx.eval(script)?;
  Ok(())
}
