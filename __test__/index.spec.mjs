import test from "ava";
import { Editor } from "../index.js"; // this points to your napi-built module

test("Editor can be created", (t) => {
  const editor = new Editor();
  t.truthy(editor);
});

test("Editor inserts and reads text", (t) => {
  const editor = new Editor();
  editor.insert_text(0, "Hello, Edcore!");
  const line = editor.get_line(0);
  t.is(line, "Hello, Edcore!");
});

test("Editor line count works", (t) => {
  const editor = new Editor();
  editor.insert_text(0, "Line 1\nLine 2\nLine 3");
  t.is(editor.line_count(), 3);
});

test("Editor returns empty string for out-of-bounds line", (t) => {
  const editor = new Editor();
  t.is(editor.get_line(42), "");
});
