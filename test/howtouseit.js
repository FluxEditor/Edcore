// howtouseit.js

// Import the native module built by napi-rs
// When published, users would just: import { Editor } from "@edcore/main"
const edcore = require("./index.js");

// Create an instance of the editor
const editor = new edcore.Editor();

// Insert some text
editor.insert_text(0, "Hello, Flex!\nThis is Edcore in action.");

// Print the first line
console.log("Line 0:", editor.get_line(0));

// Show how many lines exist
console.log("Total lines:", editor.line_count());

// Optional: test your on_change handler (if implemented)
if (editor.on_change) {
  editor.on_change((text) => {
    console.log("Buffer changed:", text);
  });

  editor.insert_text(11, "\nThis triggered the on_change callback!");
}
