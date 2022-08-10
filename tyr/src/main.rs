#![allow(non_upper_case_globals)]

use std::{
  fs,
  os::raw::c_char,
  process, ptr,
  sync::atomic::{AtomicU32, Ordering},
};

use async_callback::AsyncCallback;
use clap::{self, Parser};
use jsc_safe::{ClassAttribute, ClassDefinition, Context, JscError, PropertyAttributes};
use once_cell::sync::{Lazy, OnceCell};
use tokio::{
  runtime::{Builder, Runtime},
  sync::mpsc::{unbounded_channel, UnboundedSender},
};

#[cfg(all(not(all(target_os = "linux", target_env = "musl", target_arch = "aarch64")),))]
#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

pub(crate) static GLOBAL_SENDER: OnceCell<UnboundedSender<AsyncCallback>> = OnceCell::new();
pub(crate) static GLOBAL_RUNTIME: Lazy<Runtime> =
  Lazy::new(|| Builder::new_multi_thread().enable_all().build().unwrap());
static ASYNC_TASK_QUEUE_SIZE: AtomicU32 = AtomicU32::new(0);

mod async_callback;
mod console;
mod timer;
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
  let _ = GLOBAL_RUNTIME.enter();
  let (sender, mut receiver) = unbounded_channel::<AsyncCallback>();
  GLOBAL_SENDER.get_or_init(move || sender);
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
  GLOBAL_RUNTIME.block_on(async move {
    while let Some(cb) = receiver.recv().await {
      cb.call(ctx.raw());
      if ASYNC_TASK_QUEUE_SIZE.fetch_sub(1, Ordering::Relaxed) == 1 {
        break;
      };
    }
  });
}

fn create_runtime(ctx: &Context) -> Result<(), JscError> {
  let mut global = ctx.global();
  let console = console::create(&ctx)?;
  let btoa = ctx.create_function("btoa", Some(web::btoa::btoa))?;
  let set_timeout = ctx.create_function("setTimeout", Some(timer::set_timeout))?;
  global.set_property("console", &console, PropertyAttributes::DontDelete)?;
  global.set_property("btoa", &btoa, PropertyAttributes::DontDelete)?;
  global.set_property("setTimeout", &set_timeout, PropertyAttributes::DontDelete)?;
  let tyr = Tyr::parse();
  let script = fs::read_to_string(tyr.entry)?;
  ctx.eval(script)?;
  Ok(())
}

#[inline(always)]
pub(crate) fn c_str(s: &str) -> *const c_char {
  s.as_ptr() as *const c_char
}

#[inline]
pub(crate) fn queue_async_task() {
  ASYNC_TASK_QUEUE_SIZE.fetch_add(1, Ordering::Relaxed);
}
