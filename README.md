# caesar-rs
This is a library that provides encryption and decryption for the caesar cipher.

# WARNING: OBVIOUSLY NOT CRYPTOGRAPHICALLY SECURE

# Usage
```
use caesar::{Message, Kind};

fn main() {
    let plain_text = Message::new("Hello world!".to_string(), Kind::Plain);
    let cipher_text = plain_text.translate(2);
    assert_eq!("Jgnnq yqtnf!", cipher_text);
}
```
