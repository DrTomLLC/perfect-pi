#[cfg(feature = "complex")]
use num_complex::{Complex32, Complex64};
#[cfg(any(feature = "complex", feature = "decimal", feature = "fixed-point"))]
use perfect_pi::Pi;

#[cfg(feature = "complex")]
#[test]
fn complex_adapter_preserves_existing_float_semantics() {
    let c32: Complex32 = match Pi::<6>::round_nearest_even().try_to_complex32_preserving_places() {
        Ok(value) => value,
        Err(error) => panic!(
            "unexpected complex32 precision loss: requested={}, guaranteed={}",
            error.requested_decimal_places(),
            error.guaranteed_decimal_places()
        ),
    };
    assert_eq!(
        c32.re.to_bits(),
        Pi::<6>::round_nearest_even().to_f32_lossy().to_bits()
    );
    assert_eq!(c32.im.to_bits(), 0.0_f32.to_bits());

    let c64: Complex64 = Pi::<40>::round_nearest_even().to_complex64_lossy();
    assert_eq!(
        c64.re.to_bits(),
        Pi::<40>::round_nearest_even().to_f64_lossy().to_bits()
    );
    assert_eq!(c64.im.to_bits(), 0.0_f64.to_bits());

    assert!(
        Pi::<7>::round_nearest_even()
            .try_to_complex32_preserving_places()
            .is_err()
    );
    assert!(
        Pi::<16>::round_nearest_even()
            .try_to_complex64_preserving_places()
            .is_err()
    );
}

#[cfg(feature = "fixed-point")]
#[test]
fn fixed_adapter_uses_nearest_even_without_float_detour() {
    use fixed::types::{I2F6, I16F16, I32F32};

    let source = Pi::<40>::round_nearest_even();
    let i16f16: I16F16 = match source.to_fixed_nearest_even() {
        Ok(value) => value,
        Err(error) => panic!("unexpected I16F16 conversion error: {error}"),
    };
    let i32f32: I32F32 = match source.to_fixed_nearest_even() {
        Ok(value) => value,
        Err(error) => panic!("unexpected I32F32 conversion error: {error}"),
    };

    assert_eq!(i16f16.to_bits(), 205_887);
    assert_eq!(i32f32.to_bits(), 13_493_037_705);

    let too_small: Result<I2F6, _> = source.to_fixed_nearest_even();
    assert!(too_small.is_err());
}

#[cfg(feature = "decimal")]
#[test]
fn rust_decimal_adapter_is_exact_through_28_places_and_explicit_above() {
    let exact = match Pi::<28>::truncated().try_to_rust_decimal_exact() {
        Ok(value) => value,
        Err(error) => panic!("unexpected exact rust_decimal conversion error: {error}"),
    };
    assert_eq!(exact.to_string(), "3.1415926535897932384626433832");
    assert_eq!(exact.scale(), 28);

    let rounded = match Pi::<40>::round_nearest_even().to_rust_decimal_nearest_even() {
        Ok(value) => value,
        Err(error) => panic!("unexpected rounded rust_decimal conversion error: {error}"),
    };
    assert_eq!(rounded.to_string(), "3.1415926535897932384626433833");
    assert_eq!(rounded.scale(), 28);

    let too_wide = Pi::<29>::truncated().try_to_rust_decimal_exact();
    match too_wide {
        Err(perfect_pi::RustDecimalInteropError::TooManyDecimalPlaces { requested, maximum }) => {
            assert_eq!(requested, 29);
            assert_eq!(maximum, perfect_pi::RUST_DECIMAL_MAX_PLACES);
        }
        Err(error) => panic!("unexpected rust_decimal error variant: {error}"),
        Ok(value) => panic!("unexpected exact rust_decimal conversion: {value}"),
    }
}
