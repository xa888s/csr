# caesar-rs
This is a library that provides encryption and decryption for the caesar cipher.

# WARNING: OBVIOUSLY NOT CRYPTOGRAPHICALLY SECURE

# Usage
```
use caesar::Caesar;

fn main() {
    // the key or "shift"
    let key: u8 = 2;
    let caesar = Caesar::new(2);
    let mut input = String::from("Hello world!");
    caesar.encrypt(&mut input)

    assert_eq!(input, "Jgnnq yqtnf!".to_string());
}
```
