#![allow(unused)]
use crate::opts::{hex::hex_to_bytes, xor::xor_char};
use bytes::{Buf, BufMut, Bytes, BytesMut};

// we can use freq to ensure that top letters occur a certain percent of times
pub fn score_plaintext_freq(b: &Bytes) -> f64 {
    let mut len: f64 = 0.0;
    let mut top_freq = 0_f64;

    for byte in b.iter() {
        let c = *byte as char;
        if c != ' ' {
            len += 1.0;
        }
        match c {
            'e' | 'E' | // e 12.02
            't' | 'T' | // t 9.10
            'a' | 'A' | // a 8.12
            'o' | 'O' | // o 7.68
            'i' | 'I' | // i 7.31
            'n' | 'N' | // n 6.95
            's' | 'S' | // s 6.28
            'r' | 'R' | // r 6.02
            'h' | 'H' | // h 5.92
            'd' | 'D'   // d 4.32
            => top_freq += 1.0,
            _ => (),
        }
    }

    let score: f64 = top_freq / len;
    score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn score_plaintext_freq_test() {
        let t = Bytes::from("I hope you and your wife have a nice trip");
        // let t = Bytes::from("b!o31<, a!kj  @%4&^as# akj# iom");
        let score = score_plaintext_freq(&t);
        assert!(score > 0.5);
    }

    #[test]
    fn score_plaintext_freq_fail_test() {
        let t = Bytes::from("]qquwpy>S]9m>rwu{>\x7f>nqkpz>qx>|\x7f}qp");
        let score = score_plaintext_freq(&t);
        println!("{}", score);
        assert!(score < 0.5);
    }
}
