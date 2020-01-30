use num::cast::AsPrimitive;
use std::ops::Deref;

#[derive(Clone, Copy)]
pub struct Caesar {
    shift: u8,
}

impl Caesar {
    pub fn new<U: AsPrimitive<u8>>(shift: U) -> Self {
        // Shift size must be bigger than 0 and smaller than or equal to 26
        match shift.as_() {
            0..=26 => Caesar { shift: shift.as_() },
            _ => panic!("Shift size must be between 0 and 26!"),
        }
    }

    pub fn encrypt<S: Deref<Target = str>>(&self, buf: S) -> String {
        let chars = buf.as_bytes();

        let vec: Vec<u8> = chars
            .iter()
            .map(|c| match c {
                65..=90 => {
                    let pos = c % 65;
                    65 + ((pos + self.shift) % 26)
                }
                97..=122 => {
                    let pos = c % 97;
                    97 + ((pos + self.shift) % 26)
                }
                _ => *c,
            })
            .collect();

        // this is safe because non-utf8 bytes will never be pushed to vec
        unsafe { String::from_utf8_unchecked(vec) }
    }

    pub fn decrypt<S: Deref<Target = str>>(&self, buf: S) -> String {
        let chars = buf.as_bytes();

        let vec: Vec<u8> = chars
            .iter()
            .map(|c| match c {
                65..=90 => {
                    let pos = c % 65;
                    90 - (((25 - pos) + self.shift) % 26)
                }
                97..=122 => {
                    let pos = c % 97;
                    122 - (((25 - pos) + self.shift) % 26)
                }
                _ => *c,
            })
            .collect();

        // this is safe because non-utf8 bytes will never be pushed to vec
        unsafe { String::from_utf8_unchecked(vec) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_basic() {
        let key: u8 = 10;
        let caesar = Caesar::new(key);

        let input = String::from("Drsc sc k coxdoxmo");
        let output = String::from("This is a sentence");

        assert_eq!(caesar.decrypt(input), output);
    }

    #[test]
    fn test_encrypt_basic() {
        let key: u8 = 20;
        let caesar = Caesar::new(key);

        let input = String::from("Tests are important");
        let output = String::from("Nymnm uly cgjilnuhn");

        assert_eq!(caesar.encrypt(input), output);
    }

    #[test]
    fn test_emoji_passthrough_decrypt() {
        let key: u8 = 15;
        let caesar = Caesar::new(key);

        let input = String::from("😀 😁 😂 🤣 😃 😄 😅 😆 😉 😊 😋 😎 😍");
        let output = input.clone();

        assert_eq!(caesar.decrypt(input), output);
    }

    #[test]
    fn test_emoji_passthrough_encrypt() {
        let key: u8 = 15;
        let caesar = Caesar::new(key);

        let input = String::from("😀 😁 😂 🤣 😃 😄 😅 😆 😉 😊 😋 😎 😍");
        let output = input.clone();

        assert_eq!(caesar.encrypt(input), output);
    }

    #[test]
    fn str_test() {
        let key: u8 = 2;
        let caesar = Caesar::new(key);

        let input = "Hello world!";
        let output = "Jgnnq yqtnf!".to_string();

        assert_eq!(caesar.encrypt(input), output);
    }

    #[test]
    fn slice_test() {
        let key: u8 = 2;
        let caesar = Caesar::new(key);

        let input = "Top secret message!";
        let output = "Vqr ugetgv".to_string();

        assert_eq!(caesar.encrypt(&input[0..10]), output);
    }
}
