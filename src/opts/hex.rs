#![allow(unused)]
use bytes::{Buf, BufMut, Bytes, BytesMut};

pub fn hex_to_bytes(hex: &Bytes) -> Bytes {
    let bytes = hex::decode(hex).unwrap();
    Bytes::from(bytes)
}

pub fn bytes_to_hex(b: Bytes) -> Bytes {
    Bytes::from(hex::encode(b))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hex_to_bytes_test() {
        // decimal value
        let mut x = Bytes::from("00");
        let mut b = hex_to_bytes(&x);
        assert_eq!(b.as_ref(), [0]);

        x = Bytes::from("ff");
        b = hex_to_bytes(&x);
        assert_eq!(b.as_ref(), [255]);

        // char value
        x = Bytes::from("30");
        b = hex_to_bytes(&x);
        assert_eq!(b, "0");

        x = Bytes::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        b = hex_to_bytes(&x);
        assert_eq!(b, "I'm killing your brain like a poisonous mushroom");
    }

    #[test]
    fn hex_to_bytes_and_back() {
        let b1 = Bytes::from("the kid don't play");
        let x = bytes_to_hex(b1.clone());
        let b2 = hex_to_bytes(&x);
        assert_eq!(b1, b2);
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
}
