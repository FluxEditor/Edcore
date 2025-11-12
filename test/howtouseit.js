// You can use Edcore like that:
const { Editor } = require("../edcore.node");

// Make a new variable using the Editor constructor
const editor = new Editor();
// Insert some text at the beginning of the editor
editor.insert_text(0, "Hello from a program using Edcore!");
// Print the result
console.log(editor.get_line(0));
