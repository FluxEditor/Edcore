use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::{Arc, Mutex};

use crate::buffer::Buffer;
use crate::events::{ChangeCallback, call_change, set_callback};
use crate::file_manager::FileManager;

#[napi]
pub struct Editor {
  files: Arc<Mutex<FileManager>>,
  on_change_cb: ChangeCallback,
}

#[napi]
impl Editor {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      files: Arc::new(Mutex::new(FileManager::new())),
      on_change_cb: Arc::new(Mutex::new(None)),
    }
  }

  #[napi]
  pub fn on_change(&self, callback: JsFunction) -> Result<()> {
    set_callback(&self.on_change_cb, callback)
  }

  #[napi]
  pub fn open_file(&self, name: String) {
    self.files.lock().unwrap().open_file(name);
  }

  #[napi]
  pub fn close_file(&self, name: String) {
    self.files.lock().unwrap().close_file(&name);
  }

  #[napi]
  pub fn list_files(&self) -> Vec<String> {
    self.files.lock().unwrap().list_files()
  }

  #[napi]
  pub fn insert_text(&self, file_name: String, pos: u32, text: String) -> Result<()> {
    if let Some(buf) = self.files.lock().unwrap().get_buffer(&file_name) {
      buf.insert_text(pos as usize, &text);
      call_change(&self.on_change_cb);
      Ok(())
    } else {
      Err(Error::from_reason("File not found"))
    }
  }

  #[napi]
  pub fn get_line(&self, file_name: String, line: u32) -> Result<String> {
    if let Some(buf) = self.files.lock().unwrap().get_buffer(&file_name) {
      Ok(buf.get_line(line as usize))
    } else {
      Err(Error::from_reason("File not found"))
    }
  }

  #[napi]
  pub fn line_count(&self, file_name: String) -> Result<u32> {
    if let Some(buf) = self.files.lock().unwrap().get_buffer(&file_name) {
      Ok(buf.line_count() as u32)
    } else {
      Err(Error::from_reason("File not found"))
    }
  }

  #[napi]
  pub fn clear_file(&self, file_name: String) -> Result<()> {
    if let Some(buf) = self.files.lock().unwrap().get_buffer(&file_name) {
      buf.clear();
      call_change(&self.on_change_cb);
      Ok(())
    } else {
      Err(Error::from_reason("File not found"))
    }
  }
}
