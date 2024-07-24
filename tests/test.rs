#[cfg(test)]
mod tests {
    use itoa;

    // Constants for expected string values
    const ZERO: &str = "0";
    const U32_MAX: &str = "4294967295";
    const U64_MAX: &str = "18446744073709551615";
    const I64_MIN: &str = "-9223372036854775808";
    const I16_MIN: &str = "-32768";
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
}
