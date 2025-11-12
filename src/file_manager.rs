use crate::buffer::Buffer;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct FileManager {
  pub files: HashMap<String, Buffer>,
}

impl FileManager {
  pub fn new() -> Self {
    Self {
      files: HashMap::new(),
    }
  }

  pub fn open_file(&mut self, name: String) {
    self.files.entry(name).or_insert_with(Buffer::new);
  }

  pub fn get_buffer(&self, name: &str) -> Option<&Buffer> {
    self.files.get(name)
  }

  pub fn close_file(&mut self, name: &str) {
    self.files.remove(name);
  }

  pub fn list_files(&self) -> Vec<String> {
    self.files.keys().cloned().collect()
  }
}
