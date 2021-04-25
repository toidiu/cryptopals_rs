use bytes::Bytes;

// scoring currently based on if is alphanumeric and not whitespace
pub fn score_plaintext_alpha(b: &Bytes) -> f64 {
    let mut len: f64 = 0.0;
    let mut num_alpha = 0_f64;

    for byte in b.iter() {
        let c = *byte as char;
        if c.is_alphanumeric() {
            num_alpha += 1.0;
        }
        if c != ' ' {
            len += 1.0;
        }
    }

    let score: f64 = num_alpha / len;
    score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn score_plaintext_test() {
        let t = Bytes::from("I hope you and your wife have a nice trip");
        let score = score_plaintext_alpha(&t);
        assert!(score > 0.9);
    }

    #[test]
    fn score_plaintext_fail_test() {
        let t = Bytes::from("b!o31<, a!kj  @%4&^as# akj# iom");
        let score = score_plaintext_alpha(&t);
        assert!(score < 0.9);
    }
}
