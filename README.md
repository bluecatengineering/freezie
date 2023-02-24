# Freezie

A wrapper type with no `DerefMut` impl, disallowing mutation

```rust
use freezie::Freeze;
let mut v = Freeze::new(vec![1, 2, 3]);
v.push(1); // ERROR - cannot borrow data in dereference of ...
```
