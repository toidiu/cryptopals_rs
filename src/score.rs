pub mod alpha;
pub mod freq;

use crate::opts::xor::xor_char;
use alpha::score_plaintext_alpha;
use bytes::Bytes;
// use freq::score_plaintext_freq;

pub fn break_xor_1char(b: Bytes) -> Vec<(f64, char, Bytes)> {
    let mut ans = Vec::new();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    // let mut win_char = 'a';
    let mut highest_score = 0.0;

    // for char in alpha
    for c in alphabet.iter() {
        // xor with s
        let x = xor_char(&b, c);
        let score = score_plaintext_alpha(&x);
        // let score_a = score_plaintext_freq(&x);
        if score > highest_score {
            highest_score = score;
            // win_char = *c;
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
    use crate::opts::hex;

    #[test]
    fn break_xor_1char_test() {
        let hex =
            Bytes::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let b = hex::hex_to_bytes(&hex);
        let ans = break_xor_1char(b);
        for a in &ans {
            println!("{:?}---", a);
        }

        assert_eq!(ans.first().unwrap().1, 'X');
    }
}
