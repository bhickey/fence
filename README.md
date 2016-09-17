fence
=====

A simple rate limiter.

## Examples

```rust
let mut f = Fence::from_secs(1);

loop {
  // Something expensive
  f.sleep();
}
```
