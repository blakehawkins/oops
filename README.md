Lightweight error-handling for transforming values into std::io::Result. Provides:

- `Option<T>::oops(self, &str) -> std::io::Result<T>`
- `Result<T, E>::oops(self, &str) -> std::io::Result<T>`
- `Option<T>::lazy_oops(self, FnOnce() -> String) -> std::io::Result<T>`
- `Result<T, E>::lazy_oops(self, FnOnce() -> String) -> std::io::Result<T>`

## Examples

```rust
use std::io::Result;

fn third_element(slice: &[usize]) -> Result<&usize> {
    // Using oops to add context to a None
    slice.iter().nth(3).oops("No third element")
}

fn parse_batch(slice: &[&str]) -> Result<Vec<usize>> {
    slice
        .iter()
        .map(|v| {
            v
                .parse::<usize>()

                // Using lazy_oops to add debug messages
                .lazy_oops(|| format!("Failed to parse {} from {:?}", v, slice))
        })
        .collect()
}

assert_eq!(
    // No third element
    third_element(&[1, 2, 3]).err().unwrap().kind(),
    std::io::ErrorKind::Other
);

assert_eq!(
    // Failed to parse lo from ["2", "3", "7", "lo", "11"]
    parse_batch(&["2", "3", "7", "lo", "11"]).err().unwrap().kind(),
    std::io::ErrorKind::Other
);
```
