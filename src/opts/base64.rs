#![allow(unused)]
use bytes::{Buf, BufMut, Bytes, BytesMut};
use super::hex::hex_to_bytes;

pub fn bytes_to_base64(b: Bytes) -> String {
    base64::encode_config(b, base64::STANDARD)
}

pub fn hex_to_base64(hex: Bytes) -> String {
    bytes_to_base64(hex_to_bytes(&hex))
}

#[cfg(test)]
mod test {
    use super::*;

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
