#![no_main]

use libfuzzer_sys::fuzz_target;
use perfect_pi::{PI_F32, PI_F64, Pi};

const SENTINEL: u8 = 0xA5;

macro_rules! exercise {
    ($d:literal, $data:expr) => {{
        let rounded = $data.get(1).copied().unwrap_or(0) & 1 != 0;
        let value = if rounded {
            Pi::<$d>::round_nearest_even()
        } else {
            Pi::<$d>::truncated()
        };

        assert_eq!(value.integer_part(), 3);
        assert_eq!(value.decimal_places(), $d);
        assert!(value.fractional_digits().iter().all(|digit| *digit <= 9));

        let capacity = usize::from($data.get(2).copied().unwrap_or(0)) % 48;
        let mut buffer = [SENTINEL; 48];
        let before = buffer;
        let result = value.write_ascii(&mut buffer[..capacity]);
        if capacity < value.ascii_len() {
            let error = result.expect_err("undersized buffer unexpectedly succeeded");
            assert_eq!(error.required(), value.ascii_len());
            assert_eq!(error.provided(), capacity);
            assert_eq!(buffer, before);
        } else {
            let written = result.expect("sufficient buffer unexpectedly failed");
            assert_eq!(written, value.ascii_len());
            assert!(buffer[written..].iter().all(|byte| *byte == SENTINEL));
        }

        let f32_value = value.to_f32_lossy();
        let f64_value = value.to_f64_lossy();
        assert!(f32_value.is_finite() && (3.0..4.0).contains(&f32_value));
        assert!(f64_value.is_finite() && (3.0..4.0).contains(&f64_value));
        assert_eq!(value.try_to_f32_preserving_places().is_ok(), $d <= 6);
        assert_eq!(value.try_to_f64_preserving_places().is_ok(), $d <= 15);
        if $d >= 8 {
            assert_eq!(f32_value.to_bits(), PI_F32.to_bits());
        }
        if $d >= 15 {
            assert_eq!(f64_value.to_bits(), PI_F64.to_bits());
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
