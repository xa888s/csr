#[derive(Copy, Clone)]
pub enum Kind {
    Plain,
    Cipher,
}

pub struct Message {
    pub text: String,
    pub kind: Kind,
}

impl Message {
    pub fn new(text: String, kind: Kind) -> Message {
        Message { text, kind }
    }
    pub fn translate(self, key: u8) -> String {
        match self.kind {
            Kind::Plain => self.encrypt(key),
            Kind::Cipher => self.decrypt(key),
        }
    }

    fn encrypt(self, key: u8) -> String {
        let mut vec: Vec<u8> = Vec::with_capacity(self.text.len());

        for char in self.text.bytes() {
            vec.push(match char {
                65..=90 => {
                    let pos = char % 65;
                    65 + ((pos + key) % 26)
                }
                97..=122 => {
                    let pos = char % 97;
                    97 + ((pos + key) % 26)
                }
                _ => char,
            });
        }
        // this is safe because non-utf8 bytes will never be pushed to vec
        unsafe { String::from_utf8_unchecked(vec) }
    }

    fn decrypt(self, key: u8) -> String {
        let mut vec: Vec<u8> = Vec::with_capacity(self.text.len());

        for char in self.text.bytes() {
            vec.push(match char {
                65..=90 => {
                    let pos = char % 65;
                    90 - (((25 - pos) + key) % 26)
                }
                97..=122 => {
                    let pos = char % 97;
                    122 - (((25 - pos) + key) % 26)
                }
                _ => char,
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

        let message = Message::new(input, Kind::Cipher);
        let key: u8 = 10;

        assert_eq!(message.translate(key), output);
    }

    #[test]
    fn test_encrypt_basic() {
        let input = String::from("Tests are important");
        let output = String::from("Nymnm uly cgjilnuhn");

        let message = Message::new(input, Kind::Plain);
        let key: u8 = 20;

        assert_eq!(message.translate(key), output);
    }

    #[test]
    fn test_emoji_passthrough_decrypt() {
        let input = String::from("😀 😁 😂 🤣 😃 😄 😅 😆 😉 😊 😋 😎 😍");

        let output = input.clone();
        let message = Message::new(input, Kind::Cipher);
        let key: u8 = 15;

        assert_eq!(message.translate(key), output);
    }

    #[test]
    fn test_emoji_passthrough_encrypt() {
        let input = String::from("😀 😁 😂 🤣 😃 😄 😅 😆 😉 😊 😋 😎 😍");

        let output = input.clone();
        let message = Message::new(input, Kind::Plain);
        let key: u8 = 15;

        assert_eq!(message.translate(key), output);
    }

    #[test]
    fn clone_test() {
        let plain_text = Message::new("Hello world!".to_string(), Kind::Plain);
        let cipher_text = plain_text.translate(2);
        assert_eq!("Jgnnq yqtnf!", cipher_text);
    }
}
