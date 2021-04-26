use std::fs::File;
use std::io::Write;

/// An `IndentedWriter` is a simple tool that's useful for basic code generation.
///
/// It's a text buffer that maintains indentation level when writing new lines.
/// You retrieve the correctly indented contents of the buffer by calling
/// `to_string()` on the `IndentedWriter`.
pub struct IndentedPrinter {
    buffer: Vec<String>,
    indent: u32,
    level: u32,
}

impl Default for IndentedPrinter {
    /// Create a default implementation of the `IndentedWriter` with an
    /// indentation level of 4 spaces.
    fn default() -> Self {
        let buffer: Vec<String> = Vec::new();
        let indent = 4;
        let level = 0;
        IndentedPrinter {
            buffer,
            indent,
            level,
        }
    }
}

impl ToString for IndentedPrinter {
    /// Retrieve a string of the internal state of the printer. This will be a
    /// string that has been formatted with correct indentation levels
    fn to_string(&self) -> String {
        self.buffer.join("\n")
    }
}

impl IndentedPrinter {
    /// Create a new `IndentedWriter`, specifying the number of spaces to
    /// indent at.
    pub fn new(indent: u32) -> Self {
        let buffer: Vec<String> = Vec::new();
        let level = 0;
        IndentedPrinter {
            buffer,
            indent,
            level,
        }
    }

    /// Write a line to the internal buffer at the current indentation level.
    pub fn println(&mut self, str: &str) {
        let indent_size = self.indent * self.level;
        let indent_str = " ".repeat(indent_size as usize);
        self.buffer.push(format!("{}{}", indent_str, str));
    }

    /// Indent the internal buffer left.
    pub fn indent_right(&mut self) {
        self.level += 1;
    }

    /// Indent the internal buffer right.
    pub fn indent_left(&mut self) {
        if self.level > 0 {
            self.level -= 1;
        }
    }

    /// Indent right then print a string to the internal buffer.
    pub fn println_right(&mut self, str: &str) {
        self.indent_right();
        self.println(str);
    }

    /// Indent left then print a string to the internal buffer.
    pub fn println_left(&mut self, str: &str) {
        self.indent_left();
        self.println(str);
    }

    /// Flush the buffer to a specified file.
    pub fn flush_to_file(&self, file: &mut File) -> std::io::Result<usize> {
        // Defer error handling to the caller.
        let output = self.to_string();
        file.write(output.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use crate::printer::IndentedPrinter;

    #[test]
    fn should_write_to_buffer() {
        let mut printer = IndentedPrinter::new(4);
        printer.println("testing");
        let actual = printer.to_string();

        assert_eq!("testing", actual);
    }

    #[test]
    fn default_should_return_valid_printer() {
        let mut printer = IndentedPrinter::default();
        printer.println("default");
        let actual = printer.to_string();

        assert_eq!("default", actual);
    }

    #[test]
    fn should_indent_on_indent_right() {
        // Don't use default as we are relying on the indentation being a
        // particular length.
        let mut printer = IndentedPrinter::new(4);
        printer.indent_right();
        printer.println("testing");
        let actual = printer.to_string();

        assert_eq!("    testing", actual);
    }

    #[test]
    fn should_indent_on_indent_left() {
        let mut printer = IndentedPrinter::new(4);
        printer.indent_right();
        printer.indent_right();
        printer.indent_left();
        printer.println("testing");

        let actual = printer.to_string();

        assert_eq!("    testing", actual);
    }

    #[test]
    fn printer_should_not_unindent_beyond_level_zero() {
        let mut printer = IndentedPrinter::default();
        printer.indent_left();
        printer.println("testing");
        let actual = printer.to_string();

        assert_eq!("testing", actual);
    }

    #[test]
    fn printer_should_handle_indent_then_unindent() {
        let mut printer = IndentedPrinter::new(4);

        let expected = "fn do_something() {\n    println!(\"Hello, World!\");\n}";

        printer.println("fn do_something() {");
        printer.indent_right();
        printer.println("println!(\"Hello, World!\");");
        printer.indent_left();
        printer.println("}");
        let actual = printer.to_string();

        assert_eq!(expected, actual);
    }

    #[test]
    fn printer_should_maintain_indent_across_multiple_println() {
        let mut printer = IndentedPrinter::new(4);

        let expected = "        testing1\n        testing2";

        printer.indent_right();
        printer.indent_right();
        printer.println("testing1");
        printer.println("testing2");
        let actual = printer.to_string();

        assert_eq!(expected, actual);
    }

    #[test]
    fn println_left_should_indent_left_then_print() {
        let mut printer = IndentedPrinter::new(4);
        printer.indent_right();
        printer.indent_right();
        printer.println_left("testing");
        let actual = printer.to_string();

        assert_eq!("    testing", actual);
    }

    #[test]
    fn println_right_should_indent_right_then_print() {
        let mut printer = IndentedPrinter::new(4);
        printer.println_right("testing");
        let actual = printer.to_string();

        assert_eq!("    testing", actual);
    }
}
