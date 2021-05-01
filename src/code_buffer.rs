/// A [CodeBuffer] is a simple tool that's useful for basic code generation.
///
/// It's a text buffer that maintains indentation level when writing new lines.
/// You retrieve the correctly indented contents of the buffer by calling
/// [CodeBuffer::to_string()].
///
/// # Example
/// ```
/// use simplegen::CodeBuffer;
/// // The CodeBuffer must be mutable as the buffer maintains state in a
/// // struct.
/// let mut code_buffer = CodeBuffer::new(4);
///
/// code_buffer.println("fn add_one(x: u64) -> u64) {");
/// code_buffer.println_right("x + 1");
/// code_buffer.println_left("}");
///
/// let code_string = code_buffer.to_string();
///
/// println!("{}", code_string);
/// // Prints:
/// // fn add_one(x: u64) -> u64 {
/// //     x + 1
/// // }
/// ```
pub struct CodeBuffer {
    /// Vector of lines of code. The whitespace at the start of each line is
    /// preserved here.
    buffer: Vec<String>,
    /// Number of spaces to indent code by.
    indent: i32,
    /// Level of indentation of the current line of code.
    level: i32,
}

impl Default for CodeBuffer {
    /// Create a default implementation of the `CodeBuffer` with an
    /// indentation level of 4 spaces.
    fn default() -> Self {
        let buffer: Vec<String> = Vec::new();
        let indent = 4;
        let level = 0;
        CodeBuffer {
            buffer,
            indent,
            level,
        }
    }
}

impl ToString for CodeBuffer {
    /// Retrieve a string of the internal state of the printer. This will be a
    /// string that has been formatted with correct indentation levels
    ///
    /// # Examples
    ///
    /// ```
    /// use simplegen::CodeBuffer;
    ///
    /// let mut buffer = CodeBuffer::default();
    ///
    /// buffer.println("[");
    /// buffer.indent_right();
    ///
    /// for number in 1..10 {
    ///     buffer.println("{");
    ///     buffer.println_right(format!("\"number\": {}", number).as_str());
    ///     buffer.println_left("},");
    /// }
    ///
    /// buffer.println_left("]");
    ///
    /// // CodeBuffer::to_string() joins all the lines in the buffer into
    /// // a single string.
    /// println!("{}", buffer.to_string());
    /// ```
    fn to_string(&self) -> String {
        self.buffer.join("\n")
    }
}

impl CodeBuffer {
    /// Create a new `IndentedWriter`.
    ///
    /// # Arguments
    ///
    /// * `indent` - Number of spaces to indent by.
    ///
    /// # Examples
    ///
    /// ```
    /// use simplegen::CodeBuffer;
    ///
    /// let mut buffer = CodeBuffer::new(4);
    /// ```
    pub fn new(indent: i32) -> Self {
        let buffer: Vec<String> = Vec::new();
        let level = 0;
        CodeBuffer {
            buffer,
            indent,
            level,
        }
    }

    /// Write a line to the internal buffer at the current indentation level.
    ///
    /// # Arguments
    ///
    /// * `str` - String to append to the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use simplegen::CodeBuffer;
    ///
    /// let mut buffer = CodeBuffer::default();
    ///
    /// // Calling this will append "Hello, World!" to the buffer at the current
    /// // indentation level, in this case an indentation level of 0.
    /// buffer.println("Hello, World!");
    /// ```
    pub fn println(&mut self, str: &str) {
        let indent_size = self.indent * self.level;
        let indent_str = " ".repeat(indent_size as usize);
        self.buffer.push(format!("{}{}", indent_str, str));
    }

    /// Indent the internal buffer right.
    ///
    /// # Examples
    ///
    /// ```
    /// use simplegen::CodeBuffer;
    /// let mut buffer = CodeBuffer::default();
    ///
    /// // Increases indentation level by one.
    /// buffer.indent_right();
    ///
    /// buffer.println("Hello, World!");
    /// // The string "    Hello, World!" gets appended to the buffer, as we
    /// // have increased indentation level by one, and default indentation
    /// // level is 4 spaces.
    /// ```
    pub fn indent_right(&mut self) {
        self.level += 1;
    }

    /// Indent the internal buffer left.
    ///
    /// # Examples
    ///
    /// ```
    /// use simplegen::CodeBuffer;
    /// let mut buffer = CodeBuffer::default();
    ///
    /// // We set the indentation level to 1 for demonstration purposes.
    /// buffer.indent_right();
    ///
    /// buffer.indent_left();
    /// // The indentation level has returned to 0.
    ///
    /// buffer.println("Hello, World!");
    /// // The string "Hello, World!" is appended to the buffer, as we set the
    /// // indentation level from 1, to 0.
    /// ```
    pub fn indent_left(&mut self) {
        if self.level > 0 {
            self.level -= 1;
        }
    }

    /// Indent right then print a string to the internal buffer.
    ///
    /// # Arguments
    ///
    /// * `str` - String to append to the buffer.
    ///
    /// # Examples
    /// ```
    /// use simplegen::CodeBuffer;
    ///
    /// let mut buffer = CodeBuffer::default();
    /// buffer.println_right("Hello, World!");
    /// // The string "    Hello, World!" is appended to the buffer, as the
    /// // CodeBuffer::println_right() function will first increment the
    /// // indentation level, then append the indented string to the buffer.
    /// ```
    pub fn println_right(&mut self, str: &str) {
        self.indent_right();
        self.println(str);
    }

    /// Indent left then print a string to the internal buffer.
    ///
    /// # Arguments
    ///
    /// * `str` - String to append to the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use simplegen::CodeBuffer;
    ///
    /// let mut buffer = CodeBuffer::default();
    /// buffer.indent_right();
    /// // Set the indentation level to 1.
    ///
    /// buffer.println_left("Hello, World!");
    /// // Indentation level will be returned to 0, so "Hello, World!" will be
    /// // appended to the buffer.
    /// ```
    pub fn println_left(&mut self, str: &str) {
        self.indent_left();
        self.println(str);
    }
}

#[cfg(test)]
mod tests {
    use crate::code_buffer::CodeBuffer;

    #[test]
    fn should_write_to_buffer() {
        let mut printer = CodeBuffer::new(4);
        printer.println("testing");
        let actual = printer.to_string();

        assert_eq!("testing", actual);
    }

    #[test]
    fn default_should_return_valid_printer() {
        let mut printer = CodeBuffer::default();
        printer.println("default");
        let actual = printer.to_string();

        assert_eq!("default", actual);
    }

    #[test]
    fn should_indent_on_indent_right() {
        // Don't use default as we are relying on the indentation being a
        // particular length.
        let mut printer = CodeBuffer::new(4);
        printer.indent_right();
        printer.println("testing");
        let actual = printer.to_string();

        assert_eq!("    testing", actual);
    }

    #[test]
    fn should_indent_on_indent_left() {
        let mut printer = CodeBuffer::new(4);
        printer.indent_right();
        printer.indent_right();
        printer.indent_left();
        printer.println("testing");

        let actual = printer.to_string();

        assert_eq!("    testing", actual);
    }

    #[test]
    fn printer_should_not_unindent_beyond_level_zero() {
        let mut printer = CodeBuffer::default();
        printer.indent_left();
        printer.println("testing");
        let actual = printer.to_string();

        assert_eq!("testing", actual);
    }

    #[test]
    fn printer_should_handle_indent_then_unindent() {
        let mut printer = CodeBuffer::new(4);

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
        let mut printer = CodeBuffer::new(4);

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
        let mut printer = CodeBuffer::new(4);
        printer.indent_right();
        printer.indent_right();
        printer.println_left("testing");
        let actual = printer.to_string();

        assert_eq!("    testing", actual);
    }

    #[test]
    fn println_right_should_indent_right_then_print() {
        let mut printer = CodeBuffer::new(4);
        printer.println_right("testing");
        let actual = printer.to_string();

        assert_eq!("    testing", actual);
    }
}
