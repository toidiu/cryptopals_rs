#![allow(unused)]
use crate::opts::{hex_to_bytes, xor_char};
use bytes::{Buf, BufMut, Bytes, BytesMut};

// scoring currently based on if is alphanumeric and not whitespace
fn score_plaintext(b: &Bytes) -> f64 {
    let mut len: f64 = 0.0;
    let mut num_alpha = 0_f64;

    for byte in b.iter() {
        let c = *byte as char;
        if c.is_alphanumeric() {
            num_alpha = num_alpha + 1.0;
        }
        if c != ' ' {
            len = len + 1.0;
        }
    }

    let score: f64 = num_alpha / len;
    score
}

// we can use freq to ensure that top letters occur a certain percent of times
fn score_plaintext_freq(b: &Bytes) -> f64 {
    let mut len: f64 = 0.0;
    let mut top_freq = 0_f64;

    for byte in b.iter() {
        let c = *byte as char;
        if c != ' ' {
            len = len + 1.0;
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
            => top_freq = top_freq + 1.0,
            _ => (),
        }
    }

    let score: f64 = top_freq / len;
    score
}

// we can split by space and ensure that each word had a vowel
// we can look at double letters and the percent that it is likely to happen
// fn score_plaintext_freq(b: &Bytes) -> f64 {

fn break_xor_1char(b: Bytes) -> Vec<(f64, char, Bytes)> {
    let mut ans = Vec::new();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    let mut win_char = 'a';
    let mut highest_score = 0.0;

    // for char in alpha
    for c in alphabet.iter() {
        // xor with s
        let x = xor_char(&b, c);
        let score = score_plaintext(&x);
        let score_a = score_plaintext_freq(&x);
        if (score > highest_score) {
            highest_score = score;
            win_char = *c;
        }

        // println!("{}, {}, {}, {:?}", c, score, score_a, x);
        if score > 0.9 {
            ans.push((score, *c, x));
        }
    }
    ans.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn score_plaintext_test() {
        let t = Bytes::from("I hope you and your wife have a nice trip");
        let score = score_plaintext(&t);
        assert!(score > 0.9);
    }

    #[test]
    fn score_plaintext_fail_test() {
        let t = Bytes::from("b!o31<, a!kj  @%4&^as# akj# iom");
        let score = score_plaintext(&t);
        assert!(score < 0.9);
    }

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

    #[test]
    fn break_xor_1char_test() {
        let hex =
            Bytes::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let b = hex_to_bytes(&hex);
        let mut ans = break_xor_1char(b);
        for a in &ans {
            println!("{:?}---", a);
        }

        assert_eq!(ans.first().unwrap().1, 'X');
    }
}
