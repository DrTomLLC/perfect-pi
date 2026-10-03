#![no_main]

use libfuzzer_sys::fuzz_target;
use perfect_pi::{RoundingMode, RuntimePiError, generate_pi_ascii_with_limit, runtime_pi_ascii_len};

const SENTINEL: u8 = 0xA5;
const ARITHMETIC_PLACES: [usize; 4] = [10_001, 36_000, 36_700, 36_808];

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
    let decimal_places = ARITHMETIC_PLACES[usize::from(data.first().copied().unwrap_or(0)) % ARITHMETIC_PLACES.len()];
    let mode = rounding_mode(data.get(1).copied().unwrap_or(0));
    let scenario = data.get(2).copied().unwrap_or(0) % 4;
    let required = runtime_pi_ascii_len(decimal_places).expect("fixed deep fuzz precision must fit");

    let reject_by_limit = scenario == 2;
    let short_buffer = scenario == 1;
    let limit = if reject_by_limit { decimal_places - 1 } else { decimal_places };
    let capacity = if short_buffer { required - 1 } else { required };
    let mut output = vec![SENTINEL; required + 8];
    let before = output.clone();
    let result = generate_pi_ascii_with_limit(decimal_places, limit, mode, &mut output[..capacity]);

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

    if short_buffer {
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

    let written = result.expect("sufficient deep runtime buffer unexpectedly failed");
    assert_eq!(written, required);
    assert_eq!(output[0], b'3');
    assert_eq!(output[1], b'.');
    assert!(output[2..written].iter().all(u8::is_ascii_digit));
    assert!(output[written..].iter().all(|byte| *byte == SENTINEL));
});
