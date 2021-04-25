#![allow(unused)]
use bytes::{Buf, BufMut, Bytes, BytesMut};

pub fn xor_bytes(a: &Bytes, b: &Bytes) -> Bytes {
    let mut out = BytesMut::with_capacity(a.len());
    for (i, item) in a.iter().enumerate() {
        out.put_u8(item ^ b[i]);
    }
    out.freeze()
}

// [deprecated] use `xor_with_key`..
// a good exercise will be to do bench testing and fuzzing between the two
pub fn xor_char(a: &Bytes, c: &char) -> Bytes {
    let mut out = BytesMut::with_capacity(a.len());
    for item in a.iter() {
        let u = *c as u8;
        out.put_u8(item ^ u);
    }
    out.freeze()
}

pub fn xor_with_key(a: &Bytes, key: &Bytes) -> Bytes {
    let mut key_bytes = BytesMut::with_capacity(a.len());
    // generate buffer bytes by repeating the bytes `key`
    let mut key_len = key.len();

    // generate key_bytes
    while key_bytes.len() < a.len() {
        key_bytes.extend_from_slice(key.as_ref());
    }

    // resize key_bytes to length of a and convert to Bytes
    key_bytes.resize(a.len(), 0_u8);
    let key_bytes_frozen = key_bytes.freeze();

    xor_bytes(a, &key_bytes_frozen)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::opts::hex::*;

    #[test]
    fn xor_with_key_ice_test() {
        let a = Bytes::from(
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
        );
        let key = Bytes::from("ICE");

        let x = xor_with_key(&a, &key);
        let x_byte = bytes_to_hex(x);
        assert_eq!(
            x_byte,
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );
    }

    #[test]
    fn xor_bytes_test() {
        let x_hex = Bytes::from("1c0111001f010100061a024b53535009181c");
        let y_hex = Bytes::from("686974207468652062756c6c277320657965");

        let x = xor_bytes(&hex_to_bytes(&x_hex), &hex_to_bytes(&y_hex));
        let s = bytes_to_hex(x);
        assert_eq!(s, "746865206b696420646f6e277420706c6179");
    }

    #[test]
    fn xor_char_test() {
        let x_hex = Bytes::from("1c011100");
        let mut x = xor_char(&hex_to_bytes(&x_hex), &'e');
        assert_eq!(x, "ydte");

        x = xor_char(&hex_to_bytes(&x_hex), &'X');
        assert_eq!(x, "DYIX");

        x = xor_char(&hex_to_bytes(&x_hex), &'x');
        assert_eq!(x, "dyix");
    }

    #[test]
    fn xor_with_key_test() {
        let x_hex = Bytes::from("1c01110033");
        let mut x = xor_with_key(&hex_to_bytes(&x_hex), &Bytes::from("e"));
        assert_eq!(x, "ydteV");

        x = xor_with_key(&hex_to_bytes(&x_hex), &Bytes::from("X"));
        assert_eq!(x, "DYIXk");

        x = xor_with_key(&hex_to_bytes(&x_hex), &Bytes::from("x"));
        assert_eq!(x, "dyixK");

        x = xor_with_key(&hex_to_bytes(&x_hex), &Bytes::from("xX"));
        assert_eq!(x, "dYiXK");

        x = xor_with_key(&hex_to_bytes(&x_hex), &Bytes::from("xXc"));
        assert_eq!(x, "dYrxk");
    }
}
