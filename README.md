# simplegen ⚙️

simplegen is a simple code generator library.

It is a text buffer that preserves indentation level on calls to `println()`.
The contents of the buffer can be retrieved with `to_string()`.

## Example

```rust
use simplegen::CodeBuffer;

fn main() {
    let mut buffer = CodeBuffer::new(2);

    buffer.println("[");
    buffer.indent_right();

    for number in 1..=5 {
        buffer.println("{");
        buffer.println_right(format!("\"number\": {}", number).as_str());
        
        if number != 5 {
            buffer.println_left("},");
        } else {
            buffer.println_left("}");
        }
    }
    
    buffer.println_left("]");

    // CodeBuffer::to_string() joins all the lines in the buffer into
    // a single string.
    println!("{}", buffer.to_string());
}
```

### Output

```json
[
  {
    "number": 1
  },
  {
    "number": 2
  },
  {
    "number": 3
  },
  {
    "number": 4
  },
  {
    "number": 5
  }
]
```
