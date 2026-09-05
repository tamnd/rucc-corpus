//! SHA-256, from FIPS 180-4.
//!
//! Every generated program carries the digest of its own source, and every report carries the
//! digest of the case list it was produced from. That is what makes a report checkable: you
//! can tell whether two runs saw the same corpus without shipping the corpus alongside them.

/// The round constants, the first thirty-two bits of the fractional parts of the cube roots
/// of the first sixty-four primes.
const K: [u32; 64] = [
    0x428a_2f98,
    0x7137_4491,
    0xb5c0_fbcf,
    0xe9b5_dba5,
    0x3956_c25b,
    0x59f1_11f1,
    0x923f_82a4,
    0xab1c_5ed5,
    0xd807_aa98,
    0x1283_5b01,
    0x2431_85be,
    0x550c_7dc3,
    0x72be_5d74,
    0x80de_b1fe,
    0x9bdc_06a7,
    0xc19b_f174,
    0xe49b_69c1,
    0xefbe_4786,
    0x0fc1_9dc6,
    0x240c_a1cc,
    0x2de9_2c6f,
    0x4a74_84aa,
    0x5cb0_a9dc,
    0x76f9_88da,
    0x983e_5152,
    0xa831_c66d,
    0xb003_27c8,
    0xbf59_7fc7,
    0xc6e0_0bf3,
    0xd5a7_9147,
    0x06ca_6351,
    0x1429_2967,
    0x27b7_0a85,
    0x2e1b_2138,
    0x4d2c_6dfc,
    0x5338_0d13,
    0x650a_7354,
    0x766a_0abb,
    0x81c2_c92e,
    0x9272_2c85,
    0xa2bf_e8a1,
    0xa81a_664b,
    0xc24b_8b70,
    0xc76c_51a3,
    0xd192_e819,
    0xd699_0624,
    0xf40e_3585,
    0x106a_a070,
    0x19a4_c116,
    0x1e37_6c08,
    0x2748_774c,
    0x34b0_bcb5,
    0x391c_0cb3,
    0x4ed8_aa4a,
    0x5b9c_ca4f,
    0x682e_6ff3,
    0x748f_82ee,
    0x78a5_636f,
    0x84c8_7814,
    0x8cc7_0208,
    0x90be_fffa,
    0xa450_6ceb,
    0xbef9_a3f7,
    0xc671_78f2,
];

/// The initial state, the first thirty-two bits of the fractional parts of the square roots
/// of the first eight primes.
const H0: [u32; 8] = [
    0x6a09_e667,
    0xbb67_ae85,
    0x3c6e_f372,
    0xa54f_f53a,
    0x510e_527f,
    0x9b05_688c,
    0x1f83_d9ab,
    0x5be0_cd19,
];

/// An in-progress digest.
#[derive(Debug, Clone)]
pub struct Sha256 {
    state: [u32; 8],
    buffer: [u8; 64],
    filled: usize,
    length: u64,
}

impl Default for Sha256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha256 {
    /// A fresh digest.
    #[must_use]
    pub const fn new() -> Self {
        Self { state: H0, buffer: [0; 64], filled: 0, length: 0 }
    }

    /// Adds bytes.
    pub fn update(&mut self, mut bytes: &[u8]) {
        self.length = self.length.wrapping_add(bytes.len() as u64);
        if self.filled > 0 {
            let want = 64 - self.filled;
            let take = want.min(bytes.len());
            self.buffer[self.filled..self.filled + take].copy_from_slice(&bytes[..take]);
            self.filled += take;
            bytes = &bytes[take..];
            if self.filled == 64 {
                let block = self.buffer;
                self.compress(&block);
                self.filled = 0;
            }
        }
        while bytes.len() >= 64 {
            let (block, rest) = bytes.split_at(64);
            let mut fixed = [0u8; 64];
            fixed.copy_from_slice(block);
            self.compress(&fixed);
            bytes = rest;
        }
        if !bytes.is_empty() {
            self.buffer[..bytes.len()].copy_from_slice(bytes);
            self.filled = bytes.len();
        }
    }

    /// Finishes and returns the thirty-two byte digest.
    #[must_use]
    pub fn finish(mut self) -> [u8; 32] {
        let bits = self.length.wrapping_mul(8);
        self.update_raw(&[0x80]);
        while self.filled != 56 {
            self.update_raw(&[0]);
        }
        self.buffer[56..].copy_from_slice(&bits.to_be_bytes());
        let block = self.buffer;
        self.compress(&block);
        let mut out = [0u8; 32];
        for (chunk, word) in out.chunks_exact_mut(4).zip(self.state) {
            chunk.copy_from_slice(&word.to_be_bytes());
        }
        out
    }

    /// Adds padding bytes without counting them toward the message length.
    fn update_raw(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.buffer[self.filled] = byte;
            self.filled += 1;
            if self.filled == 64 {
                let block = self.buffer;
                self.compress(&block);
                self.filled = 0;
            }
        }
    }

    fn compress(&mut self, block: &[u8; 64]) {
        let mut w = [0u32; 64];
        for (at, chunk) in block.chunks_exact(4).enumerate() {
            w[at] = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }
        for at in 16..64 {
            let s0 = w[at - 15].rotate_right(7) ^ w[at - 15].rotate_right(18) ^ (w[at - 15] >> 3);
            let s1 = w[at - 2].rotate_right(17) ^ w[at - 2].rotate_right(19) ^ (w[at - 2] >> 10);
            w[at] = w[at - 16].wrapping_add(s0).wrapping_add(w[at - 7]).wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.state;
        for at in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choose = (e & f) ^ (!e & g);
            let t1 =
                h.wrapping_add(s1).wrapping_add(choose).wrapping_add(K[at]).wrapping_add(w[at]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(majority);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (slot, value) in self.state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *slot = slot.wrapping_add(value);
        }
    }
}

/// The digest of some bytes, as lowercase hex.
#[must_use]
pub fn hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finish();
    let mut out = String::with_capacity(64);
    for byte in digest {
        out.push(char::from_digit(u32::from(byte >> 4), 16).unwrap_or('0'));
        out.push(char::from_digit(u32::from(byte & 0xf), 16).unwrap_or('0'));
    }
    out
}

/// The first sixteen hex characters of the digest, which is what a case id uses.
///
/// Sixty-four bits of a good hash over a corpus that will never hold more than a few hundred
/// thousand programs leaves the chance of a collision far below the chance of a disk error.
#[must_use]
pub fn short(bytes: &[u8]) -> String {
    let mut full = hex(bytes);
    full.truncate(16);
    full
}

#[cfg(test)]
mod tests {
    use super::{Sha256, hex, short};

    #[test]
    fn the_three_vectors_from_the_standard_come_out_right() {
        assert_eq!(hex(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert_eq!(
            hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
    }

    #[test]
    fn a_million_letter_a_comes_out_right_which_exercises_the_length_counter() {
        let mut hasher = Sha256::new();
        for _ in 0..1000 {
            hasher.update(&[b'a'; 1000]);
        }
        let digest = hasher.finish();
        let mut text = String::new();
        for byte in digest {
            text.push_str(&format!("{byte:02x}"));
        }
        assert_eq!(text, "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0");
    }

    #[test]
    fn feeding_the_same_bytes_in_different_sized_pieces_gives_the_same_digest() {
        let message: Vec<u8> = (0u8..=255).cycle().take(1000).collect();
        let whole = {
            let mut hasher = Sha256::new();
            hasher.update(&message);
            hasher.finish()
        };
        for piece in [1usize, 7, 63, 64, 65, 127, 128] {
            let mut hasher = Sha256::new();
            for chunk in message.chunks(piece) {
                hasher.update(chunk);
            }
            assert_eq!(hasher.finish(), whole, "piece size {piece}");
        }
    }

    #[test]
    fn a_short_digest_is_the_front_of_the_long_one() {
        let full = hex(b"case source");
        assert_eq!(short(b"case source"), full[..16]);
    }
}
