#![cfg(feature = "runtime-generation")]

use perfect_pi::{
    RoundingMode, RuntimePiError, generate_pi_ascii_with_limit, generate_pi_ascii_with_rounding,
    generate_pi_string,
};

fn generated(decimal_places: usize, mode: RoundingMode) -> String {
    let required = if decimal_places == 0 {
        1
    } else {
        decimal_places + 2
    };
    let mut output = vec![0_u8; required];
    let written = match generate_pi_ascii_with_rounding(decimal_places, mode, &mut output) {
        Ok(written) => written,
        Err(error) => panic!("unexpected runtime generation error: {error}"),
    };
    assert_eq!(written, required);
    match String::from_utf8(output) {
        Ok(value) => value,
        Err(error) => panic!("runtime output was not UTF-8: {error}"),
    }
}

#[test]
fn runtime_supports_all_rounding_directions() {
    assert_eq!(generated(0, RoundingMode::TowardZero), "3");
    assert_eq!(generated(0, RoundingMode::TowardNegativeInfinity), "3");
    assert_eq!(generated(0, RoundingMode::AwayFromZero), "4");
    assert_eq!(generated(0, RoundingMode::TowardPositiveInfinity), "4");
    assert_eq!(generated(0, RoundingMode::NearestTiesToEven), "3");
    assert_eq!(generated(0, RoundingMode::NearestTiesAwayFromZero), "3");
    assert_eq!(generated(2, RoundingMode::TowardZero), "3.14");
    assert_eq!(generated(2, RoundingMode::TowardPositiveInfinity), "3.15");
    assert_eq!(generated(2, RoundingMode::NearestTiesToEven), "3.14");
    assert_eq!(generated(3, RoundingMode::NearestTiesToEven), "3.142");
    assert_eq!(generated(3, RoundingMode::NearestTiesAwayFromZero), "3.142");
}

#[test]
fn precision_limit_rejects_before_output_mutation() {
    let mut output = [0xA5_u8; 16];
    let result = generate_pi_ascii_with_limit(11, 10, RoundingMode::NearestTiesToEven, &mut output);
    assert_eq!(
        result,
        Err(RuntimePiError::PrecisionLimitExceeded {
            requested: 11,
            limit: 10,
        })
    );
    assert_eq!(output, [0xA5_u8; 16]);
}

#[test]
fn owned_string_api_enforces_limit_and_matches_buffer_api() {
    assert_eq!(
        generate_pi_string(6, 6, RoundingMode::NearestTiesToEven),
        Ok(String::from("3.141593"))
    );
    assert_eq!(
        generate_pi_string(7, 6, RoundingMode::TowardZero),
        Err(RuntimePiError::PrecisionLimitExceeded {
            requested: 7,
            limit: 6,
        })
    );
}
