#![allow(unused)]
use bytes::{Buf, BufMut, Bytes, BytesMut};

pub fn hex_to_bytes(hex: Bytes) -> Bytes {
    let bytes = hex::decode(hex).unwrap();
    Bytes::from(bytes)
}

pub fn bytes_to_hex(b: Bytes) -> Bytes {
    Bytes::from(hex::encode(b))
}

pub fn bytes_to_base64(b: Bytes) -> String {
    base64::encode_config(b, base64::STANDARD)
}

pub fn hex_to_base64(hex: Bytes) -> String {
    bytes_to_base64(hex_to_bytes(hex))
}

pub fn xor_bytes(a: &Bytes, b: &Bytes) -> Bytes {
    let mut out = BytesMut::with_capacity(a.len());
    for (i, item) in a.iter().enumerate() {
        out.put_u8(item ^ b[i]);
    }
    out.freeze()
}

pub fn xor_char(a: &Bytes, c: &char) -> Bytes {
    let mut out = BytesMut::with_capacity(a.len());
    for item in a.iter() {
        let u = *c as u8;
        out.put_u8(item ^ u);
    }
    out.freeze()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn xor_test() {
        let x_hex = Bytes::from("1c0111001f010100061a024b53535009181c");
        let y_hex = Bytes::from("686974207468652062756c6c277320657965");

        let x = xor_bytes(&hex_to_bytes(x_hex), &hex_to_bytes(y_hex));
        let s = bytes_to_hex(x);
        assert_eq!(s, "746865206b696420646f6e277420706c6179");
    }

    #[test]
    fn bytes_to_hex_test() {
        let mut b = Bytes::from("0");
        let mut x = bytes_to_hex(b);
        assert_eq!(x, "30");

        b = Bytes::from(":");
        x = bytes_to_hex(b);
        assert_eq!(x, "3a");

        b = Bytes::from("the kid don't play");
        x = bytes_to_hex(b);
        assert_eq!(x, "746865206b696420646f6e277420706c6179");
    }

    #[test]
    fn hex_to_bytes_test() {
        // decimal value
        let mut x = Bytes::from("00");
        let mut b = hex_to_bytes(x);
        assert_eq!(b.as_ref(), [0]);

        x = Bytes::from("ff");
        b = hex_to_bytes(x);
        assert_eq!(b.as_ref(), [255]);

        // char value
        x = Bytes::from("30");
        b = hex_to_bytes(x);
        assert_eq!(b, "0");

        x = Bytes::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        b = hex_to_bytes(x);
        assert_eq!(b, "I'm killing your brain like a poisonous mushroom");
    }

    #[test]
    fn hex_to_bytes_and_back() {
        let b1 = Bytes::from("the kid don't play");
        let x = bytes_to_hex(b1.clone());
        let b2 = hex_to_bytes(x);
        assert_eq!(b1, b2);
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
