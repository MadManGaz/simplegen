# simplegen ⚙️

simplegen is a simple code generator library.

It is a text buffer that preserves indentation level on calls to `println()`.
The contents of the buffer can be retrieved with `to_string()`.

## Example

```rust
fn main() {
    let mut w = simplegen::IndentedPrinter::new(4);
    w.println("long factorial(long n) {");
    w.println_right("long acc = n;");
    w.println("for (int i = 1; i < n; i++) {");
    w.println_right("acc *= i;");
    w.println_left("}");
    w.println("return acc;");
    w.println_left("});
    println("{}", w.to_string());
}
```

### Output

```C
long factorial(long n) {
    long acc = n;
    for (int i = 1; i < n; i++) {
        acc *= i;
    }
    return acc;
}
```
