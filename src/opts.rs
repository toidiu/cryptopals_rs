#![allow(unused)]
use bytes::{Buf, BufMut, Bytes, BytesMut};

pub fn hex_to_bytes(hex: &Bytes) -> Bytes {
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
    bytes_to_base64(hex_to_bytes(&hex))
}

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

pub fn u8_to_binary(num: &u8, bin: &mut [char; 8]) {
    let mut n: u8 = *num;
    let mut i = 0;
    while (n != 0_u8) {
        if (n % 2 == 1) {
            bin[i] = '1';
        } else {
            bin[i] = '0';
        }
        n = n / 2;
        i = i + 1;
    }

    // fill in remaining with 0
    while (i < 8) {
        bin[i] = '0';
        i = i + 1;
    }

    bin.reverse()
}

pub fn hamming_distance(a: &Bytes, b: &Bytes) -> u32 {
    let mut dis = 0;

    println!("{:?}", a.as_ref());
    println!("{:?}", b.as_ref());
    let mut bin_a = ['0'; 8];
    let mut bin_b = ['0'; 8];

    for (i, item) in a.iter().enumerate() {
        u8_to_binary(&item, &mut bin_a);
        u8_to_binary(&b[i], &mut bin_b);

        let chars_a = bin_a.iter();
        let mut chars_b = bin_b.iter();

        println!("byte {}, {:?} {:?}", i, &bin_a, &bin_b);
        for (one, two) in chars_a.zip(chars_b) {
            let diff = one != two;
            println!("{:?} {:?} {:?}", &one, &two, diff);
            if diff {
                dis = dis + 1;
            }
        }
    }
    dis
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hamming_distance_test() {
        let a = Bytes::from("this is a test");
        let b = Bytes::from("wokka wokka!!!");

        let x = hamming_distance(&a, &b);
        assert_eq!(x, 37);
    }

    #[test]
    fn u8_to_binary_test() {
        let mut bin = ['0'; 8];
        u8_to_binary(&255, &mut bin);
        assert_eq!(bin, ['1', '1', '1', '1', '1', '1', '1', '1']);

        u8_to_binary(&0, &mut bin);
        assert_eq!(bin, ['0', '0', '0', '0', '0', '0', '0', '0']);

        u8_to_binary(&255, &mut bin);
        assert_eq!(bin, ['1', '1', '1', '1', '1', '1', '1', '1']);

        u8_to_binary(&1, &mut bin);
        assert_eq!(bin, ['0', '0', '0', '0', '0', '0', '0', '1']);

        u8_to_binary(&2, &mut bin);
        assert_eq!(bin, ['0', '0', '0', '0', '0', '0', '1', '0']);

        u8_to_binary(&42, &mut bin);
        assert_eq!(bin, ['0', '0', '1', '0', '1', '0', '1', '0']);

        u8_to_binary(&170, &mut bin);
        assert_eq!(bin, ['1', '0', '1', '0', '1', '0', '1', '0']);

        u8_to_binary(&85, &mut bin);
        assert_eq!(bin, ['0', '1', '0', '1', '0', '1', '0', '1']);
    }

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
