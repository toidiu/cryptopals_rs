use std::str;

fn main() {}

fn hex_to_base64_copy(hex: &[u8]) -> String {
    let bytes = hex::decode(hex).unwrap();
    base64::encode_config(bytes, base64::STANDARD)
}

fn hex_to_base64(hex: &[u8], out: &mut [u8; 64], buf: &mut [u8; 48]) {
    hex::decode_to_slice(hex, buf).unwrap();
    base64::encode_config_slice(buf, base64::STANDARD, out);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unsafe_test_slice() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let mut out = [0u8; 64];
        let mut buf = [0u8; 48];
        hex_to_base64(input.as_bytes(), &mut out, &mut buf);
        let s = str::from_utf8(&out).unwrap();

        assert_eq!(
            s,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }

    #[test]
    fn unsafe_test() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        assert_eq!(
            hex_to_base64_copy(input.as_bytes()),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
