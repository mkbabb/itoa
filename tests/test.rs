#[cfg(test)]
mod tests {
    use itoa;

    // Constants for expected string values
    const ZERO: &str = "0";
    const NINE: &str = "9";
    const TEN: &str = "10";
    const HUNDRED: &str = "100";
    const NINES: &str = "999";
    const THOUSAND: &str = "1000";

    const I16_MIN: &str = "-32768";

    const U32_MAX: &str = "4294967295";

    const U64_MAX: &str = "18446744073709551615";
    const I64_MIN: &str = "-9223372036854775808";

    const U128_MAX: &str = "340282366920938463463374607431768211455";

    const I128_MIN: &str = "-170141183460469231731687303715884105728";
    const I128_MAX: &str = "170141183460469231731687303715884105727";

    #[test]
    fn test_u64() {
        let mut buffer = itoa::Buffer::new();
        assert_eq!(buffer.format(0u64), ZERO);
        assert_eq!(buffer.format(u32::MAX as u64), U32_MAX);
        assert_eq!(buffer.format(u64::MAX), U64_MAX);
    }

    #[test]
    fn test_i64() {
        let mut buffer = itoa::Buffer::new();
        assert_eq!(buffer.format(i64::MIN), I64_MIN);
    }

    #[test]
    fn test_i16() {
        let mut buffer = itoa::Buffer::new();
        assert_eq!(buffer.format(0i16), ZERO);
        assert_eq!(buffer.format(9i16), NINE);
        assert_eq!(buffer.format(10i16), TEN);
        assert_eq!(buffer.format(100i16), HUNDRED);
        assert_eq!(buffer.format(999i16), NINES);
        assert_eq!(buffer.format(1000i16), THOUSAND);

        assert_eq!(buffer.format(i16::MIN), I16_MIN);
    }

    #[test]
    fn test_u128() {
        let mut buffer = itoa::Buffer::new();
        assert_eq!(buffer.format(0u128), ZERO);
        assert_eq!(buffer.format(u128::MAX), U128_MAX);
    }

    #[test]
    fn test_i128() {
        let mut buffer = itoa::Buffer::new();
        assert_eq!(buffer.format(i128::MIN), I128_MIN);
        assert_eq!(buffer.format(i128::MAX), I128_MAX);
    }

    #[test]
    fn test_raw_format() {
        unsafe {
            // Create a buffer:
            let mut buffer = [0u8; itoa::raw::I128_MAX_LEN];

            let value = u128::MAX;

            // Format the value into the buffer:
            let len = itoa::raw::format(value, buffer.as_mut_ptr());

            // Convert the buffer to a string:
            let s = std::str::from_utf8_unchecked(&buffer[..len]);

            // Check the string:
            assert_eq!(s, U128_MAX);
        }
    }

    #[test]
    fn test_raw_format_offset() {
        unsafe {
            const OFFSET: usize = 10;

            // Create a buffer:
            let mut buffer = [0u8; itoa::raw::I128_MAX_LEN + OFFSET];

            let value = u128::MAX;

            // Format the value into the buffer:
            let len = itoa::raw::format(value, buffer.as_mut_ptr().add(OFFSET));

            // Convert the buffer to a string:
            let s = std::str::from_utf8_unchecked(&buffer[OFFSET..OFFSET + len]);

            // Check the string:
            assert_eq!(s, U128_MAX);
        }
    }
}

mod range_tests {
    use itoa;
    use rand::{distributions::uniform::SampleUniform, Rng};

    // Generic test function for a range of values with stochastic sampling
    fn test_range<T>(start: T, end: T, n_samples: usize)
    where
        T: itoa::Integer + std::fmt::Display + std::cmp::PartialOrd + SampleUniform,
        rand::distributions::Standard: rand::distributions::Distribution<T>,
    {
        let mut buffer = itoa::Buffer::new();
        let mut rng = rand::thread_rng();

        // Always test the start and end of the range
        assert_eq!(buffer.format(start), start.to_string());
        assert_eq!(buffer.format(end), end.to_string());

        // Stochastically sample the range
        for _ in 0..n_samples {
            let sample: T = rng.gen_range(start..=end);
            assert_eq!(buffer.format(sample), sample.to_string());
        }
    }

    // Range tests for various integer types
    #[test]
    fn test_i8_range() {
        test_range(i8::MIN, i8::MAX, 100);
    }

    #[test]
    fn test_u8_range() {
        test_range(u8::MIN, u8::MAX, 100);
    }

    #[test]
    fn test_i16_range() {
        test_range(i16::MIN, i16::MAX, 1000);
    }

    #[test]
    fn test_u16_range() {
        test_range(u16::MIN, u16::MAX, 1000);
    }

    #[test]
    fn test_i32_range() {
        test_range(i32::MIN, i32::MAX, 10000);
    }

    #[test]
    fn test_u32_range() {
        test_range(u32::MIN, u32::MAX, 10000);
    }

    #[test]
    fn test_i64_range() {
        test_range(i64::MIN, i64::MAX, 100000);
    }

    #[test]
    fn test_u64_range() {
        test_range(u64::MIN, u64::MAX, 100000);
    }

    #[test]
    fn test_i128_range() {
        test_range(i128::MIN, i128::MAX, 1000000);
    }

    #[test]
    fn test_u128_range() {
        test_range(u128::MIN, u128::MAX, 1000000);
    }
}
