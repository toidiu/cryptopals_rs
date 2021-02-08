#![allow(unused)]
use crate::opts::{hex_to_bytes, xor_char};
use bytes::{Buf, BufMut, Bytes, BytesMut};

// scoring currently based on if is alphanumeric and not whitespace
fn score_plaintext(b: &Bytes) -> f64 {
    let mut len: f64 = 0.0;
    // let mut num_e = 0_f64;
    // let mut num_t = 0_f64;
    // let mut num_a = 0_f64;
    // let mut num_o = 0_f64;
    // let mut num_i = 0_f64;

    let mut num_alpha = 0_f64;

    for byte in b.iter() {
        let c = *byte as char;

        if c.is_alphanumeric() {
            num_alpha = num_alpha + 1.0;
        }
        if c != ' ' {
            len = len + 1.0;
        }

        //     match c {
        //         'e' | 'E' => num_e = num_e + 1.0,
        //         't' | 'T' => num_t = num_t + 1.0,
        //         'a' | 'A' => num_a = num_a + 1.0,
        //         'o' | 'O' => num_o = num_o + 1.0,
        //         'i' | 'I' => num_i = num_i + 1.0,
        //         _ => (),
        //     }
        // }
    }

    // let score: f64 = (num_e + num_t + num_a + num_o + num_i) / len;
    let score: f64 = num_alpha / len;
    score
}

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
        if (score > highest_score) {
            highest_score = score;
            win_char = *c;
        }

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
