#![no_main]

use libfuzzer_sys::fuzz_target;
use perfect_pi::{
    RoundingMode, RuntimePiError, generate_pi_ascii_with_limit, generate_pi_ascii_with_rounding,
    runtime_pi_ascii_len,
};

const SENTINEL: u8 = 0xA5;
const MAX_FUZZ_PLACES: usize = 512;

fn rounding_mode(selector: u8) -> RoundingMode {
    match selector % 6 {
        0 => RoundingMode::TowardZero,
        1 => RoundingMode::AwayFromZero,
        2 => RoundingMode::TowardNegativeInfinity,
        3 => RoundingMode::TowardPositiveInfinity,
        4 => RoundingMode::NearestTiesToEven,
        _ => RoundingMode::NearestTiesAwayFromZero,
    }
}

fuzz_target!(|data: &[u8]| {
    let low = usize::from(data.first().copied().unwrap_or(0));
    let high = usize::from(data.get(1).copied().unwrap_or(0));
    let decimal_places = ((high << 8) | low) % (MAX_FUZZ_PLACES + 1);
    let mode = rounding_mode(data.get(3).copied().unwrap_or(0));
    let reject_by_limit = decimal_places != 0 && data.get(4).copied().unwrap_or(0) & 1 != 0;
    let limit = if reject_by_limit {
        decimal_places - 1
    } else {
        decimal_places
    };
    let required = runtime_pi_ascii_len(decimal_places).expect("bounded fuzz precision must fit");

    let short_by = usize::from(data.get(2).copied().unwrap_or(0)) % required.max(1);
    let capacity = required.saturating_sub(short_by);
    let mut output = vec![SENTINEL; required + 8];
    let before = output.clone();
    let result =
        generate_pi_ascii_with_limit(decimal_places, limit, mode, &mut output[..capacity]);

    if reject_by_limit {
        assert_eq!(
            result,
            Err(RuntimePiError::PrecisionLimitExceeded {
                requested: decimal_places,
                limit,
            })
        );
        assert_eq!(output, before);
        return;
    }

    if capacity < required {
        assert_eq!(
            result,
            Err(RuntimePiError::BufferTooSmall {
                required,
                provided: capacity,
            })
        );
        assert_eq!(output, before);
        return;
    }

    let written = result.expect("sufficient runtime buffer unexpectedly failed");
    assert_eq!(written, required);
    assert!(matches!(output[0], b'3' | b'4'));
    if decimal_places == 0 {
        assert_eq!(written, 1);
    } else {
        assert_eq!(output[0], b'3');
        assert_eq!(output[1], b'.');
        assert!(output[2..written].iter().all(u8::is_ascii_digit));
    }
    assert!(output[written..].iter().all(|byte| *byte == SENTINEL));

    let mut second = vec![0_u8; required];
    assert_eq!(
        generate_pi_ascii_with_rounding(decimal_places, mode, &mut second),
        Ok(required)
    );
    assert_eq!(&output[..written], second.as_slice());
});
