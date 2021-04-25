pub fn u8_to_binary(num: &u8, bin: &mut [char; 8]) {
    let mut n: u8 = *num;
    let mut i = 0;
    while n != 0_u8 {
        if n % 2 == 1 {
            bin[i] = '1';
        } else {
            bin[i] = '0';
        }
        n /= 2;
        i += 1;
    }

    // fill in remaining with 0
    while i < 8 {
        bin[i] = '0';
        i += 1;
    }

    bin.reverse()
}

#[cfg(test)]
mod test {
    use super::*;

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
}
