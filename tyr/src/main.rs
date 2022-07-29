#![allow(non_upper_case_globals)]

use std::fs;
use std::os::raw::c_char;
use std::process;
use std::ptr;

use clap::{self, Parser};
use jsc_safe::{ClassAttribute, ClassDefinition, Context, JscError, PropertyAttributes};

#[cfg(all(not(all(target_os = "linux", target_env = "musl", target_arch = "aarch64")),))]
#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

mod console;
mod web;
mod wellknown_property_name;

#[derive(Parser)]
#[clap(author, version, about = "tyr", long_about = "Tyr JavaScript runtime")]
#[clap(propagate_version = true)]
struct Tyr {
  /// JavaScript file to run
  entry: String,
}

fn main() {
  let global_class = ClassDefinition::default()
    .with_c_name("globalThis\0".as_ptr() as *const c_char)
    .with_attribute(ClassAttribute::NoAutomaticPrototype)
    .into_class();
  let ctx = Context::with_global_class(global_class);
  match create_runtime(&ctx) {
    Err(JscError::JSCException(exception)) => {
      let js_error = unsafe {
        console::js_value_to_console(
          ctx.raw(),
          exception.exception,
          &mut Default::default(),
          ptr::null_mut(),
        )
      };
      eprintln!("{js_error}");
      process::exit(1);
    }
    Err(e) => {
      eprintln!("{e}");
      process::exit(1);
    }
    _ => {}
  }
}

fn create_runtime(ctx: &Context) -> Result<(), JscError> {
  let mut global = ctx.global();
  let mut console = ClassDefinition::default()
    .with_c_name(c_str("Console\0"))
    .with_attribute(ClassAttribute::NoAutomaticPrototype)
    .into_class()
    .make_object(&ctx);
  let log = ctx.create_function("log", Some(console::log))?;
  let info = ctx.create_function("info", Some(console::log))?;
  let warn = ctx.create_function("warn", Some(console::log))?;
  let error = ctx.create_function("error", Some(console::log))?;
  let btoa = ctx.create_function("btoa", Some(web::btoa::btoa))?;
  console.set_property("log", &log, PropertyAttributes::None)?;
  console.set_property("info", &info, PropertyAttributes::None)?;
  console.set_property("warn", &warn, PropertyAttributes::None)?;
  console.set_property("error", &error, PropertyAttributes::None)?;
  global.set_property("console", &console, PropertyAttributes::DontDelete)?;
  global.set_property("btoa", &btoa, PropertyAttributes::DontDelete)?;
  let tyr = Tyr::parse();
  let script = fs::read_to_string(tyr.entry)?;
  ctx.eval(script)?;
  Ok(())
}

fn c_str(s: &str) -> *const c_char {
  s.as_ptr() as *const c_char
}
