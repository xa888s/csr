# csr
This is a library that provides encryption and decryption for the caesar cipher.

Docs: [![docs.rs](https://docs.rs/csr/badge.svg)](https://docs.rs/csr)

### WARNING: OBVIOUSLY NOT CRYPTOGRAPHICALLY SECURE

# Usage
```rust
use csr::Caesar;

fn main() {
    // the key or "shift"
    let key: u8 = 2;
    let caesar = Caesar::new(2);

    let input = "Hello world!";

    assert_eq!(caesar.encrypt(input), "Jgnnq yqtnf!");
}
```
