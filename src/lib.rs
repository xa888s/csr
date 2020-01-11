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
        Message {
            text: text,
            kind: kind,
        }
    }

    pub fn encrypt(self, key: u8) -> String {
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

    pub fn decrypt(self, key: u8) -> String {
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

        let message = Message::new(input);
        let key: u8 = 10;

        assert_eq!(message.decrypt(key), output);
    }

    #[test]
    fn test_encrypt_basic() {
        let input = String::from("Tests are important");
        let output = String::from("Nymnm uly cgjilnuhn");

        let message = Message::new(input);
        let key: u8 = 20;

        assert_eq!(message.encrypt(key), output);
    }

    #[test]
    fn test_emoji_passthrough_decrypt() {
        let input = String::from("😀 😁 😂 🤣 😃 😄 😅 😆 😉 😊 😋 😎 😍");

        let output = input.clone();
        let message = Message::new(input);
        let key: u8 = 15;

        assert_eq!(message.decrypt(key), output);
    }

    #[test]
    fn test_emoji_passthrough_encrypt() {
        let input = String::from("😀 😁 😂 🤣 😃 😄 😅 😆 😉 😊 😋 😎 😍");

        let output = input.clone();
        let message = Message::new(input);
        let key: u8 = 15;

        assert_eq!(message.encrypt(key), output);
    }
}
