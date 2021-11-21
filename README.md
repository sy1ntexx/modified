# Modified
Keep track of changes in your variables

# Installation
```toml
[dependencies]
modified = "0.1.0"
```

# Example
```rs
use modified::Modified;

let mut var = Modified::new(15);
*var = 20;
// Or
// var.set(20);
assert!(var.is_modified());
```
