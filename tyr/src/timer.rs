use std::{
  collections::HashMap,
  ptr, slice,
  sync::{
    atomic::{AtomicU32, Ordering},
    Mutex,
  },
  time::Duration,
};

use jsc_safe::sys::*;
use once_cell::sync::Lazy;
use tokio::{task::JoinHandle, time::sleep};

use crate::async_callback::AsyncCallback;

static TIMER_ID: AtomicU32 = AtomicU32::new(0);
static TIMER_MAP: Lazy<Mutex<HashMap<u32, JoinHandle<()>>>> = Lazy::new(|| Default::default());

pub unsafe extern "C" fn set_timeout(
  ctx: JSContextRef,
  _function: JSObjectRef,
  _this: JSObjectRef,
  argument_count: usize,
  arguments: *const JSValueRef,
  exception: *mut JSValueRef,
) -> JSValueRef {
  crate::queue_async_task();
  let args = slice::from_raw_parts(arguments, argument_count);
  let callback = args[0];
  let duration_time = if let Some(dur) = args.get(1) {
    JSValueToNumber(ctx, *dur, exception) as i32
  } else {
    4
  };
  let duration = if duration_time < 0 {
    Duration::from_millis(0)
  } else {
    Duration::from_millis(duration_time as u64)
  };
  let timer_id = TIMER_ID.fetch_add(1, Ordering::Relaxed);
  let callback = AsyncCallback::new(
    callback,
    JSValueMakeUndefined(ctx),
    if argument_count == 1 {
      ptr::null()
    } else {
      args[1..].as_ptr()
    },
    if argument_count == 1 {
      0
    } else {
      argument_count - 2
    },
    exception,
  );
  let jh = crate::GLOBAL_RUNTIME.spawn(async move {
    sleep(duration).await;
    let s = crate::GLOBAL_SENDER.get_unchecked();
    if !s.is_closed() {
      if let Err(err) = s.send(callback) {
        eprintln!("{err}");
      }
    }
  });
  let mut timer_map_lock_guard = TIMER_MAP.lock().unwrap();
  timer_map_lock_guard.insert(timer_id, jh);
  JSValueMakeNumber(ctx, timer_id as f64)
}
