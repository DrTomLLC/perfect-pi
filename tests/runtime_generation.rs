#![cfg(feature = "runtime-generation")]

use perfect_pi::{
    RuntimePiError, generate_pi_ascii, generate_pi_ascii_round_nearest_even, runtime_pi_ascii_len,
};

fn generate(decimal_places: usize) -> Vec<u8> {
    let required = runtime_pi_ascii_len(decimal_places).expect("test precision must fit");
    let mut output = vec![0_u8; required];
    let written = generate_pi_ascii(decimal_places, &mut output).expect("generation must succeed");
    assert_eq!(written, required);
    output
}

fn generate_rounded(decimal_places: usize) -> Vec<u8> {
    let required = runtime_pi_ascii_len(decimal_places).expect("test precision must fit");
    let mut output = vec![0_u8; required];
    let written = generate_pi_ascii_round_nearest_even(decimal_places, &mut output)
        .expect("rounded generation must succeed");
    assert_eq!(written, required);
    output
}

#[test]
fn generates_zero_and_bounded_precision_exactly() {
    assert_eq!(generate(0), b"3");
    assert_eq!(generate(40), b"3.1415926535897932384626433832795028841971");
}

#[test]
fn rounded_runtime_generation_matches_bounded_semantics() {
    assert_eq!(generate_rounded(0), b"3");
    assert_eq!(generate_rounded(1), b"3.1");
    assert_eq!(generate_rounded(2), b"3.14");
    assert_eq!(generate_rounded(3), b"3.142");
    assert_eq!(generate_rounded(12), b"3.141592653590");
    assert_eq!(
        generate_rounded(40),
        b"3.1415926535897932384626433832795028841972"
    );
}

#[test]
fn generates_one_hundred_fractional_digits() {
    let expected = b"3.14159265358979323846264338327950288419716939937510\
58209749445923078164062862089986280348253421170679";
    assert_eq!(generate(100), expected);
}

#[test]
fn reports_runtime_error_contracts() {
    assert_eq!(
        RuntimePiError::PrecisionTooLarge.to_string(),
        "requested precision is too large"
    );
    assert_eq!(
        RuntimePiError::InternalInvariant.to_string(),
        "runtime pi generation invariant failed"
    );
    assert_eq!(
        RuntimePiError::BufferTooSmall {
            required: 12,
            provided: 10,
        }
        .to_string(),
        "output buffer is too small"
    );
    assert_eq!(
        runtime_pi_ascii_len(usize::MAX),
        Err(RuntimePiError::PrecisionTooLarge)
    );
}

#[test]
fn rejects_short_output_without_modifying_it() {
    let mut output = [0xA5_u8; 10];
    let before = output;
    let error = generate_pi_ascii(10, &mut output).expect_err("buffer must be rejected");
    assert_eq!(
        error,
        RuntimePiError::BufferTooSmall {
            required: 12,
            provided: 10,
        }
    );
    assert_eq!(output, before);

    let rounded_error = generate_pi_ascii_round_nearest_even(10, &mut output)
        .expect_err("rounded buffer must be rejected");
    assert_eq!(rounded_error, error);
    assert_eq!(output, before);
}
