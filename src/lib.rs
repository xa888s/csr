use num::cast::AsPrimitive;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy)]
pub struct Caesar {
    shift: u8,
}

impl Caesar {
    pub fn new<U: AsPrimitive<u8>>(shift: U) -> Self {
        // shift size must be bigger than 0 and smaller than or equal to 26
        match shift.as_() {
            0..=26 => Caesar { shift: shift.as_() },
            _ => panic!("Shift size must be between 0 and 26!"),
        }
    }

    pub fn encrypt<S: Deref<Target = str> + DerefMut>(self, buf: &mut S) {
        let chars = unsafe { buf.as_bytes_mut() };

        for c in chars {
            *c = match c {
                65..=90 => {
                    let pos = *c % 65;
                    65 + ((pos + self.shift) % 26)
                }
                97..=122 => {
                    let pos = *c % 97;
                    97 + ((pos + self.shift) % 26)
                }
                _ => *c,
            }
        }
    }

    pub fn decrypt<S: Deref<Target = str> + DerefMut>(self, buf: &mut S) {
        let chars = unsafe { buf.as_bytes_mut() };

        for c in chars {
            *c = match c {
                65..=90 => {
                    let pos = *c % 65;
                    90 - (((25 - pos) + self.shift) % 26)
                }
                97..=122 => {
                    let pos = *c % 97;
                    122 - (((25 - pos) + self.shift) % 26)
                }
                _ => *c,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_basic() {
        let key: u8 = 10;
        let caesar = Caesar::new(key);

        let mut input = String::from("Drsc sc k coxdoxmo");
        caesar.decrypt(&mut input);
        let output = String::from("This is a sentence");

        assert_eq!(input, output);
    }

    #[test]
    fn test_encrypt_basic() {
        let key: u8 = 20;
        let caesar = Caesar::new(key);

        let mut input = String::from("Tests are important");
        caesar.encrypt(&mut input);
        let output = String::from("Nymnm uly cgjilnuhn");

        assert_eq!(input, output);
    }

    #[test]
    fn test_emoji_passthrough_decrypt() {
        let key: u8 = 15;
        let caesar = Caesar::new(key);

        let mut input = String::from("😀 😁 😂 🤣 😃 😄 😅 😆 😉 😊 😋 😎 😍");
        caesar.decrypt(&mut input);
        let output = input.clone();

        assert_eq!(input, output);
    }

    #[test]
    fn test_emoji_passthrough_encrypt() {
        let key: u8 = 15;
        let caesar = Caesar::new(key);

        let mut input = String::from("😀 😁 😂 🤣 😃 😄 😅 😆 😉 😊 😋 😎 😍");
        caesar.encrypt(&mut input);
        let output = input.clone();

        assert_eq!(input, output);
    }
}
