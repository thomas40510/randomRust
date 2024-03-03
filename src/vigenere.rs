pub struct Vigenere {
    key: Vec<u8>,
}

impl Vigenere {
    pub fn new(key: &str) -> Vigenere {
        Vigenere {
            key: key.bytes().collect(),
        }
    }

    pub fn encrypt(&self, plaintext: &str) -> String {
        let mut ciphertext = String::new();
        let plaintext = plaintext.to_ascii_uppercase();
        let mut key_index = 0;
        for c in plaintext.bytes() {
            if c.is_ascii_alphabetic() {
                let shift = self.key[key_index] as char as u8 - 'a' as u8;
                let shifted = (c - 'A' as u8 + shift) % 26 + 'A' as u8;
                ciphertext.push(shifted as char);
            } else {
                ciphertext.push(c as char);
            }
            key_index = (key_index + 1) % self.key.len();
        }
        ciphertext
    }

    pub fn decrypt(&self, ciphertext: &str) -> String {
        let mut plaintext = String::new();
        let ciphertext = ciphertext.to_ascii_uppercase();
        let mut key_index = 0;
        for c in ciphertext.bytes() {
            if c.is_ascii_alphabetic() {
                let shift = self.key[key_index] as char as u8 - 'a' as u8;
                let shifted = (c - 'A' as u8 + 26 - shift) % 26 + 'A' as u8;
                plaintext.push(shifted as char);
            } else {
                plaintext.push(c as char);
            }
            key_index = (key_index + 1) % self.key.len();
        }
        plaintext
    }
}