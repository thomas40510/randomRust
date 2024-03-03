pub struct RSA {
    n: u64,
    e: u64,
    d: u64,
}

impl RSA {
    pub(crate) fn new(n: u64, e: u64, d: u64) -> RSA {
        RSA { n, e, d }
    }

    pub(crate) fn encrypt(&self, m: u64) -> u64 {
        mod_exp(m, self.e, self.n)
    }

    pub(crate) fn decrypt(&self, c: u64) -> u64 {
        mod_exp(c, self.d, self.n)
    }

    pub fn get_public_key(&self) -> (u64, u64) {
        (self.n, self.e)
    }

    pub fn get_private_key(&self) -> (u64, u64) {
        (self.n, self.d)
    }
    
    pub fn encrypt_string(&self, message: &str) -> Vec<u64> {
        message.chars().map(|c| self.encrypt(c as u64)).collect()
    }

    pub fn decrypt_string(&self, message: &Vec<u64>) -> String {
        message.iter().map(|c| self.decrypt(*c) as u8 as char).collect()
    }
}

fn mod_exp(p0: u64, p1: u64, p2: u64) -> u64 {
    let mut result = 1;
    let mut p0 = p0;
    let mut p1 = p1;
    let p2 = p2;
    while p1 > 0 {
        if p1 % 2 == 1 {
            result = (result * p0) % p2;
        }
        p1 >>= 1;
        p0 = (p0 * p0) % p2;
    }
    result
}