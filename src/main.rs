fn main() {}

fn hex_to_base64(hex: &[u8]) -> String {
    let bytes = hex::decode(hex).unwrap();
    base64::encode_config(bytes, base64::STANDARD)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unsafe_test() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        assert_eq!(
            hex_to_base64(input.as_bytes()),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
