#![feature(test)]
#![allow(non_snake_case)]
#![allow(clippy::cast_lossless)]
extern crate test;

use std::io::Write;
use test::{black_box, Bencher};

mod bench_itoa {
    use super::*;

    const I32_NUMS: [i32; 12] = [
        0,
        1,
        9,
        99,
        999,
        9999,
        99999,
        999999,
        9999999,
        99999999,
        i32::MIN,
        i32::MAX,
    ];

    #[bench]
    fn u64_0(b: &mut Bencher) {
        let mut buffer = itoa::Buffer::new();
        b.iter(|| {
            let printed = buffer.format(black_box(0u64));
            black_box(printed);
        });
    }

    #[bench]
    fn u64_half(b: &mut Bencher) {
        let mut buffer = itoa::Buffer::new();
        b.iter(|| {
            let printed = buffer.format(black_box(u32::MAX as u64));
            black_box(printed);
        });
    }

    #[bench]
    fn u64_max(b: &mut Bencher) {
        let mut buffer = itoa::Buffer::new();
        b.iter(|| {
            let printed = buffer.format(black_box(u64::MAX));
            black_box(printed);
        });
    }

    #[bench]
    fn i16_0(b: &mut Bencher) {
        let mut buffer = itoa::Buffer::new();
        b.iter(|| {
            let printed = buffer.format(black_box(0i16));
            black_box(printed);
        });
    }

    #[bench]
    fn i16_min(b: &mut Bencher) {
        let mut buffer = itoa::Buffer::new();
        b.iter(|| {
            let printed = buffer.format(black_box(i16::MIN));
            black_box(printed);
        });
    }

    #[bench]
    fn u128_0(b: &mut Bencher) {
        let mut buffer = itoa::Buffer::new();
        b.iter(|| {
            let printed = buffer.format(black_box(0u128));
            black_box(printed);
        });
    }

    #[bench]
    fn u128_max(b: &mut Bencher) {
        let mut buffer = itoa::Buffer::new();
        b.iter(|| {
            let printed = buffer.format(black_box(u128::MAX));
            black_box(printed);
        });
    }

    #[bench]
    fn i32_range(b: &mut Bencher) {
        let mut buffer = itoa::Buffer::new();

        b.iter(|| {
            for &i in I32_NUMS.iter() {
                let printed = buffer.format(black_box(i));
                black_box(printed);
            }
        });
    }

    #[bench]
    fn i32_small_range(b: &mut Bencher) {
        let mut buffer = itoa::Buffer::new();

        b.iter(|| {
            for i in 0..99999 {
                let printed = buffer.format(black_box(i));
                black_box(printed);
            }
        });
    }

    #[bench]
    fn i8_range(b: &mut Bencher) {
        let mut buffer = itoa::Buffer::new();

        b.iter(|| {
            for _ in 0..100 {
                for i in i8::MIN..i8::MAX {
                    let printed = buffer.format(black_box(i));
                    black_box(printed);
                }
            }
        });
    }
}

mod bench_std_fmt {
    use super::*;

    #[bench]
    fn u64_0(b: &mut Bencher) {
        let mut buf = Vec::with_capacity(40);
        b.iter(|| {
            buf.clear();
            write!(&mut buf, "{}", black_box(0u64)).unwrap();
            black_box(&buf);
        });
    }

    #[bench]
    fn u64_half(b: &mut Bencher) {
        let mut buf = Vec::with_capacity(40);
        b.iter(|| {
            buf.clear();
            write!(&mut buf, "{}", black_box(u32::MAX as u64)).unwrap();
            black_box(&buf);
        });
    }

    #[bench]
    fn u64_max(b: &mut Bencher) {
        let mut buf = Vec::with_capacity(40);
        b.iter(|| {
            buf.clear();
            write!(&mut buf, "{}", black_box(u64::MAX)).unwrap();
            black_box(&buf);
        });
    }

    #[bench]
    fn i16_0(b: &mut Bencher) {
        let mut buf = Vec::with_capacity(40);
        b.iter(|| {
            buf.clear();
            write!(&mut buf, "{}", black_box(0i16)).unwrap();
            black_box(&buf);
        });
    }

    #[bench]
    fn i16_min(b: &mut Bencher) {
        let mut buf = Vec::with_capacity(40);
        b.iter(|| {
            buf.clear();
            write!(&mut buf, "{}", black_box(i16::MIN)).unwrap();
            black_box(&buf);
        });
    }

    #[bench]
    fn u128_0(b: &mut Bencher) {
        let mut buf = Vec::with_capacity(40);
        b.iter(|| {
            buf.clear();
            write!(&mut buf, "{}", black_box(0u128)).unwrap();
            black_box(&buf);
        });
    }

    #[bench]
    fn u128_max(b: &mut Bencher) {
        let mut buf = Vec::with_capacity(40);
        b.iter(|| {
            buf.clear();
            write!(&mut buf, "{}", black_box(u128::MAX)).unwrap();
            black_box(&buf);
        });
    }
}

mod bench_repeated {

    use super::*;

    #[bench]
    fn itoa_new_buffer(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..100 {
                let mut buffer = itoa::Buffer::new();
                let printed = buffer.format(black_box(u128::MAX));
                black_box(printed.as_bytes());
            }
        });
    }

    #[bench]
    fn itoa_reusable_buffer(b: &mut Bencher) {
        b.iter(|| unsafe {
            let mut buffer = [0u8; itoa::raw::I128_MAX_LEN];
            let buffer = buffer.as_mut_ptr();

            for i in 0..100 {
                let len = itoa::raw::format(black_box(u128::MAX), buffer);
                black_box(len);
            }
        });
    }

    #[bench]
    fn itoa_new_buffer_extend(b: &mut Bencher) {
        b.iter(|| {
            let mut to_extend = Vec::new();

            for i in 0..100 {
                let mut buffer = itoa::Buffer::new();
                let printed = buffer.format(black_box(u128::MAX));

                to_extend.extend_from_slice(printed.as_bytes());
            }
        });
    }

    #[bench]
    fn itoa_reusable_buffer_extend(b: &mut Bencher) {
        b.iter(|| unsafe {
            let mut to_extend = Vec::new();

            for i in 0..100 {
                to_extend.extend_from_slice(&[0u8; itoa::raw::I128_MAX_LEN]);

                let len = itoa::raw::format(
                    black_box(u128::MAX),
                    to_extend
                        .as_mut_ptr()
                        .add(to_extend.len() - itoa::raw::I128_MAX_LEN),
                );

                to_extend.truncate(to_extend.len() - itoa::raw::I128_MAX_LEN + len);

                black_box(len);
            }
        });
    }

    #[bench]
    fn std_fmt_new_buffer(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..100 {
                let mut buf = Vec::with_capacity(40);
                write!(&mut buf, "{}", black_box(u128::MAX)).unwrap();
                black_box(&buf);
            }
        });
    }

    #[bench]
    fn std_fmt_reusable_buffer(b: &mut Bencher) {
        b.iter(|| {
            let mut buf = Vec::with_capacity(40);

            for i in 0..100 {
                buf.clear();
                write!(&mut buf, "{}", black_box(u128::MAX)).unwrap();
                black_box(&buf);
            }
        });
    }

    #[bench]
    fn std_fmt_new_buffer_extend(b: &mut Bencher) {
        b.iter(|| {
            let mut to_extend = Vec::new();

            for i in 0..100 {
                let mut buf = Vec::with_capacity(40);
                write!(&mut buf, "{}", black_box(u128::MAX)).unwrap();

                to_extend.extend_from_slice(&buf);
            }
        });
    }

    #[bench]
    fn std_fmt_reusable_buffer_extend(b: &mut Bencher) {
        b.iter(|| {
            let mut to_extend = Vec::new();
            let mut buf = Vec::with_capacity(40);

            for i in 0..100 {
                buf.clear();
                write!(&mut buf, "{}", black_box(u128::MAX)).unwrap();

                to_extend.extend_from_slice(&buf);
            }
        });
    }
}
