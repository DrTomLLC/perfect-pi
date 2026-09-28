#![no_main]

use fixed::types::{I2F6, I16F16, I32F32};
use libfuzzer_sys::fuzz_target;
use perfect_pi::Pi;

macro_rules! exercise {
    ($d:literal, $data:expr) => {{
        let rounded = $data.get(1).copied().unwrap_or(0) & 1 != 0;
        let value = if rounded {
            Pi::<$d>::round_nearest_even()
        } else {
            Pi::<$d>::truncated()
        };

        let complex32 = value.to_complex32_lossy();
        let complex64 = value.to_complex64_lossy();
        assert_eq!(complex32.im.to_bits(), 0.0_f32.to_bits());
        assert_eq!(complex64.im.to_bits(), 0.0_f64.to_bits());
        assert_eq!(complex32.re.to_bits(), value.to_f32_lossy().to_bits());
        assert_eq!(complex64.re.to_bits(), value.to_f64_lossy().to_bits());
        assert_eq!(value.try_to_complex32_preserving_places().is_ok(), $d <= 6);
        assert_eq!(value.try_to_complex64_preserving_places().is_ok(), $d <= 15);

        assert_eq!(value.try_to_rust_decimal_exact().is_ok(), $d <= 28);
        assert!(value.to_rust_decimal_nearest_even().is_ok());

        let fixed_kind = $data.get(2).copied().unwrap_or(0) % 3;
        match fixed_kind {
            0 => {
                let result: Result<I2F6, _> = value.to_fixed_nearest_even();
                assert!(result.is_err());
            }
            1 => {
                let result: Result<I16F16, _> = value.to_fixed_nearest_even();
                assert!(result.is_ok());
            }
            _ => {
                let result: Result<I32F32, _> = value.to_fixed_nearest_even();
                assert!(result.is_ok());
            }
        }
    }};
}

fuzz_target!(|data: &[u8]| {
    let precision = usize::from(data.first().copied().unwrap_or(0)) % 41;
    match precision {
        0 => exercise!(0, data),
        1 => exercise!(1, data),
        2 => exercise!(2, data),
        3 => exercise!(3, data),
        4 => exercise!(4, data),
        5 => exercise!(5, data),
        6 => exercise!(6, data),
        7 => exercise!(7, data),
        8 => exercise!(8, data),
        9 => exercise!(9, data),
        10 => exercise!(10, data),
        11 => exercise!(11, data),
        12 => exercise!(12, data),
        13 => exercise!(13, data),
        14 => exercise!(14, data),
        15 => exercise!(15, data),
        16 => exercise!(16, data),
        17 => exercise!(17, data),
        18 => exercise!(18, data),
        19 => exercise!(19, data),
        20 => exercise!(20, data),
        21 => exercise!(21, data),
        22 => exercise!(22, data),
        23 => exercise!(23, data),
        24 => exercise!(24, data),
        25 => exercise!(25, data),
        26 => exercise!(26, data),
        27 => exercise!(27, data),
        28 => exercise!(28, data),
        29 => exercise!(29, data),
        30 => exercise!(30, data),
        31 => exercise!(31, data),
        32 => exercise!(32, data),
        33 => exercise!(33, data),
        34 => exercise!(34, data),
        35 => exercise!(35, data),
        36 => exercise!(36, data),
        37 => exercise!(37, data),
        38 => exercise!(38, data),
        39 => exercise!(39, data),
        40 => exercise!(40, data),
        _ => unreachable!(),
    }
});
