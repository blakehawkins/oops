Lightweight error-handling.

Provides `Option<T>::oops(self, &str) -> Result<T>`.

```rust
use std::io::Result;

use oops::Opps;

fn third_element(slice: &[usize]) -> Result<&usize> {
    slice.iter().nth(3).oops("No third element")
}

fn main() -> Result<()> {
    third_element(&[1, 2, 3])
}
```
