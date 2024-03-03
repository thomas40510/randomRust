pub struct Caesar {
    shift: u8,
}

impl Caesar {
    pub fn new(shift: u8) -> Caesar {
        Caesar { shift }
    }

    pub fn encrypt(&self, plaintext: &str) -> String {
        let mut ciphertext = String::new();
        let plaintext = plaintext.to_ascii_uppercase();
        for c in plaintext.bytes() {
            if c.is_ascii_alphabetic() {
                let shifted = (c - 'A' as u8 + self.shift) % 26 + 'A' as u8;
                ciphertext.push(shifted as char);
            } else {
                ciphertext.push(c as char);
            }
        }
        ciphertext
    }

    pub fn decrypt(&self, ciphertext: &str) -> String {
        let mut plaintext = String::new();
        let ciphertext = ciphertext.to_ascii_uppercase();
        for c in ciphertext.bytes() {
            if c.is_ascii_alphabetic() {
                let shifted = (c - 'A' as u8 + 26 - self.shift) % 26 + 'A' as u8;
                plaintext.push(shifted as char);
            } else {
                plaintext.push(c as char);
            }
        }
        plaintext
    }
}