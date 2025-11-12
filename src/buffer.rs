use ropey::Rope;
use std::sync::Mutex;

pub struct Buffer {
  pub rope: Mutex<Rope>,
}

impl Buffer {
  pub fn new() -> Self {
    Self {
      rope: Mutex::new(Rope::new()),
    }
  }

  pub fn insert_text(&self, pos: usize, text: &str) {
    let mut rope = self.rope.lock().unwrap();
    rope.insert(pos, text);
  }

  pub fn get_line(&self, line: usize) -> String {
    let rope = self.rope.lock().unwrap();
    if line >= rope.len_lines() {
      return "".to_string();
    }
    rope.line(line).to_string()
  }

  pub fn line_count(&self) -> usize {
    let rope = self.rope.lock().unwrap();
    rope.len_lines()
  }

  pub fn to_string(&self) -> String {
    self.rope.lock().unwrap().to_string()
  }

  pub fn clear(&self) {
    self
      .rope
      .lock()
      .unwrap()
      .remove(0..self.rope.lock().unwrap().len_chars());
  }
}
