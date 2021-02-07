#![allow(unused)]
use bytes::{Bytes, BytesMut};

fn main() {}

fn hex_to_bytes(hex: Bytes) -> Bytes {
    let bytes = hex::decode(hex).unwrap();
    Bytes::from(bytes)
}

fn bytes_to_base64(b: Bytes) -> String {
    base64::encode_config(b, base64::STANDARD)
}

fn hex_to_base64(hex: Bytes) -> String {
    bytes_to_base64(hex_to_bytes(hex))
}

fn xor(a: Bytes, b: Bytes, out: &mut [u8]) {
    for (i, item) in a.iter().enumerate() {
        // println!("====== {:?}", item);
        // println!("====== {:?}", b[i]);
        let uu: u8 = item ^ b[i];
        // println!("----- {}", uu);
        out[i] = item ^ b[i];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn xor_test() {
        //     let x = Bytes::from("1c0111001f010100061a024b53535009181c");
        //     let y = Bytes::from("686974207468652062756c6c277320657965");
        //     let mut out = [0u8; 36];
        //     println!("====== {:?}", x.as_ref());

        //     xor(x, y, &mut out);
        //     let s = str::from_utf8(&out).unwrap();
        //     assert_eq!(s, "746865206b696420646f6e277420706c6179");
    }

    #[test]
    fn hex_to_bytes_test() {
        let mut x = Bytes::from("00");
        let mut b = hex_to_bytes(x);
        assert_eq!(b.as_ref(), [0]);

        x = Bytes::from("01");
        b = hex_to_bytes(x);
        assert_eq!(b.as_ref(), [1]);

        x = Bytes::from("ff");
        b = hex_to_bytes(x);
        assert_eq!(b.as_ref(), [255]);

        x = Bytes::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        b = hex_to_bytes(x);
        assert_eq!(b, "I'm killing your brain like a poisonous mushroom");
    }

    #[test]
    fn bytes_to_base64_test() {
        let b = Bytes::from("I'm killing your brain like a poisonous mushroom");
        assert_eq!(
            bytes_to_base64(b),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }

    #[test]
    fn hex_to_base64_test() {
        let hex = Bytes::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");

        assert_eq!(
            hex_to_base64(hex),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
