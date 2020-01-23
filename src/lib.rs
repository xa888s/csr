use std::ops::Deref;

#[derive(Clone)]
pub enum Caesar<T> {
    Plain(T),
    Cipher(T),
}

impl<T: Deref<Target = str>> Caesar<T> {
    pub fn translate(&self, key: u8) -> String {
        match self {
            Self::Plain(buf) => Self::encrypt(buf.as_bytes(), key),
            Self::Cipher(buf) => Self::decrypt(buf.as_bytes(), key),
        }
    }

    fn encrypt(buf: &[u8], key: u8) -> String {
        let mut vec: Vec<u8> = Vec::with_capacity(buf.len());

        for c in buf {
            vec.push(match c {
                65..=90 => {
                    let pos = c % 65;
                    65 + ((pos + key) % 26)
                }
                97..=122 => {
                    let pos = c % 97;
                    97 + ((pos + key) % 26)
                }
                _ => *c,
            });
        }
        // this is safe because non-utf8 bytes will never be pushed to vec
        unsafe { String::from_utf8_unchecked(vec) }
    }

    fn decrypt(buf: &[u8], key: u8) -> String {
        let mut vec: Vec<u8> = Vec::with_capacity(buf.len());

        for c in buf {
            vec.push(match c {
                65..=90 => {
                    let pos = c % 65;
                    90 - (((25 - pos) + key) % 26)
                }
                97..=122 => {
                    let pos = c % 97;
                    122 - (((25 - pos) + key) % 26)
                }
                _ => *c,
            });
        }
        // this is safe because non-utf8 bytes will never be pushed to vec
        unsafe { String::from_utf8_unchecked(vec) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_basic() {
        let input = String::from("Drsc sc k coxdoxmo");
        let output = String::from("This is a sentence");

        let message = Caesar::Cipher(input);
        let key: u8 = 10;

        assert_eq!(message.translate(key), output);
    }

    #[test]
    fn test_encrypt_basic() {
        let input = String::from("Tests are important");
        let output = String::from("Nymnm uly cgjilnuhn");

        let message = Caesar::Plain(input);
        let key: u8 = 20;

        assert_eq!(message.translate(key), output);
    }

    #[test]
    fn test_emoji_passthrough_decrypt() {
        let input = String::from("😀 😁 😂 🤣 😃 😄 😅 😆 😉 😊 😋 😎 😍");

        let output = input.clone();
        let message = Caesar::Cipher(input);
        let key: u8 = 15;

        assert_eq!(message.translate(key), output);
    }

    #[test]
    fn test_emoji_passthrough_encrypt() {
        let input = String::from("😀 😁 😂 🤣 😃 😄 😅 😆 😉 😊 😋 😎 😍");

        let output = input.clone();
        let message = Caesar::Plain(input);
        let key: u8 = 15;

        assert_eq!(message.translate(key), output);
    }

    #[test]
    fn slice_test() {
        let plain_text = Caesar::Plain("Hello world!");
        let cipher_text = plain_text.translate(2);
        assert_eq!("Jgnnq yqtnf!", cipher_text);
    }
}
