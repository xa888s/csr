# caesar-rs
This is a library that provides encryption and decryption for the caesar cipher.

# WARNING: OBVIOUSLY NOT CRYPTOGRAPHICALLY SECURE

# Usage
```
use caesar::Caesar;

fn main() {
    let plain_text = Caesar::Plain("Hello world!".to_string());
    let cipher_text = plain_text.translate(2);
    assert_eq!("Jgnnq yqtnf!", cipher_text);
}
```
