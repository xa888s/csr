use num::cast::AsPrimitive;
use std::ops::{Deref, Rem};

/// The main type of this crate. Holds a key (u8), and provides the methods
/// to encrypt and decrypt Strings, slices, and more!
#[derive(Clone, Copy)]
pub struct Caesar {
    shift: u8,
}

impl Caesar {
    /// Constructs a new Caesar with the provided shift. If the shift
    /// isn't valid, this function will get the remainder and shift by
    /// that instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use csr::Caesar;
    ///
    /// // value is in between 0 and 26 so it is ok!
    /// let c = Caesar::new(2);
    /// ```
    ///
    /// ```
    /// use csr::Caesar;
    ///
    /// // gets remainder, returning 22
    /// let c = Caesar::new(100);
    /// ```
    pub fn new<U: AsPrimitive<u8> + Rem>(shift: U) -> Self {
        // shift size should be bigger than 0 and smaller than or equal to 26
        Caesar {
            shift: match shift.as_() {
                0..=26 => shift.as_(),
                _ => shift.as_() % 26,
            },
        }
    }

    /// Encrypts a buffer and consumes the Caesar.
    ///
    /// # Example
    ///
    /// ```
    /// use csr::Caesar;
    ///
    /// let c = Caesar::new(2);
    /// let input = "Attack at dawn!";
    /// assert_eq!(c.encrypt(input), "Cvvcem cv fcyp!")
    /// ```
    pub fn encrypt<S: Deref<Target = str>>(self, buf: S) -> String {
        let chars = buf.as_bytes();

        let vec: Vec<u8> = chars
            .iter()
            .map(|c| match c {
                // this is first because most letters will be lowercase
                // a-z lowercase
                97..=122 => {
                    let pos = c % 97;
                    97 + ((pos + self.shift) % 26)
                }
                // A-Z uppercase
                65..=90 => {
                    let pos = c % 65;
                    65 + ((pos + self.shift) % 26)
                }
                _ => *c,
            })
            .collect();

        // this is safe because non-utf8 bytes will never be passed
        // thanks to the trait bound.
        unsafe { String::from_utf8_unchecked(vec) }
    }

    /// This function takes a mutable slice of bytes and encrypts them in place.
    ///
    /// # Safety
    ///
    /// This function is safe because it only guarantees valid UTF-8 bytes
    /// if the input is also valid.
    ///
    /// # Example
    ///
    /// ```
    /// use csr::Caesar;
    ///
    /// let c = Caesar::new(2);
    /// // "bruh"
    /// let mut bytes = [98, 114, 117, 104];
    /// // "dtwj"
    /// let output = [100, 116, 119, 106];
    /// c.encrypt_bytes(&mut bytes);
    /// assert_eq!(bytes, output);
    /// ```
    pub fn encrypt_bytes(self, chars: &mut [u8]) {
        for c in chars {
            *c = match *c {
                // this is first because most letters will be lowercase
                // a-z lowercase
                97..=122 => {
                    let pos = *c % 97;
                    97 + ((pos + self.shift) % 26)
                }
                // A-Z uppercase
                65..=90 => {
                    let pos = *c % 65;
                    65 + ((pos + self.shift) % 26)
                }
                _ => *c,
            }
        }
    }

    /// Decrypts a buffer and consumes the Caesar.
    ///
    /// # Example
    ///
    /// ```
    /// use csr::Caesar;
    ///
    /// let c = Caesar::new(2);
    /// let input = "They are coming from the north!";
    /// assert_eq!(c.encrypt(input), "Vjga ctg eqokpi htqo vjg pqtvj!")
    /// ```
    pub fn decrypt<S: Deref<Target = str>>(self, buf: S) -> String {
        let chars = buf.as_bytes();

        let vec: Vec<u8> = chars
            .iter()
            .map(|c| match c {
                // this is first because most letters will be lowercase
                // a-z lowercase
                97..=122 => {
                    let pos = c % 97;
                    122 - (((25 - pos) + self.shift) % 26)
                }
                // A-Z uppercase
                65..=90 => {
                    let pos = c % 65;
                    90 - (((25 - pos) + self.shift) % 26)
                }
                _ => *c,
            })
            .collect();

        // this is safe because non-utf8 bytes will never be passed
        // thanks to the trait bound.
        unsafe { String::from_utf8_unchecked(vec) }
    }

    /// This function takes a mutable slice of bytes and decrypts them in place.
    ///
    /// # Safety
    ///
    /// This function is safe because it only guarantees valid UTF-8 bytes
    /// if the input is also valid.
    ///
    /// # Example
    ///
    /// ```
    /// use csr::Caesar;
    ///
    /// let c = Caesar::new(2);
    /// // "skrrt"
    /// let mut bytes = [115, 107, 114, 114, 116];
    /// // "qippr"
    /// let output = [113, 105, 112, 112, 114];
    /// c.decrypt_bytes(&mut bytes);
    /// assert_eq!(bytes, output);
    /// ```
    pub fn decrypt_bytes(self, chars: &mut [u8]) {
        for c in chars {
            *c = match *c {
                // this is first because most letters will be lowercase
                // a-z lowercase
                97..=122 => {
                    let pos = *c % 97;
                    122 - (((25 - pos) + self.shift) % 26)
                }
                // A-Z uppercase
                65..=90 => {
                    let pos = *c % 65;
                    90 - (((25 - pos) + self.shift) % 26)
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
