use napi::bindgen_prelude::*;
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use std::sync::{Arc, Mutex};

pub type ChangeCallback = Arc<Mutex<Option<ThreadsafeFunction<(), ErrorStrategy::Fatal>>>>;

pub fn call_change(callback: &ChangeCallback) {
  if let Some(tsfn) = &*callback.lock().unwrap() {
    tsfn.call((), ThreadsafeFunctionCallMode::NonBlocking);
  }
}

pub fn set_callback(cb: &ChangeCallback, js_fn: JsFunction) -> Result<()> {
  let tsfn: ThreadsafeFunction<(), ErrorStrategy::Fatal> =
    js_fn.create_threadsafe_function(0, |ctx| Ok(vec![ctx.env.get_undefined()?]))?;

  let mut lock = cb.lock().unwrap();
  *lock = Some(tsfn);

  Ok(())
}
