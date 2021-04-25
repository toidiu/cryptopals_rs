#![allow(unused)]
use bytes::{Buf, BufMut, Bytes, BytesMut};
use super::binary;

pub fn hamming_distance(a: &Bytes, b: &Bytes) -> u32 {
    debug_assert_eq!(a.len(), b.len(), "bytes must have the same length");
    let mut distance = 0;

    println!("{:?}", a.as_ref());
    println!("{:?}", b.as_ref());
    let mut bin_a = ['0'; 8];
    let mut bin_b = ['0'; 8];

    // for each byte get binary for a and b
    for (idx, item) in a.iter().enumerate() {
        binary::u8_to_binary(&item, &mut bin_a);
        binary::u8_to_binary(&b[idx], &mut bin_b);

        println!("byte {}, {:?} {:?}", idx, &bin_a, &bin_b);
        // TODO: extract this into its own function
        // for each binary compare the binary char
        for (one, two) in bin_a.iter().zip(bin_b.iter()) {
            let different_bit = one != two; // char will be either 1 or 0
            println!("{:?} {:?} {:?}", &one, &two, different_bit);
            if different_bit {
                distance = distance + 1;
            }
        }
    }
    distance
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
}
