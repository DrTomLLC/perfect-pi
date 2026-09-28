use perfect_pi::Pi;

#[test]
fn core_error_display_contracts_are_stable_and_nonempty() {
    let value = Pi::<40>::truncated();
    let mut buffer = [0_u8; 1];
    let buffer_error = match value.write_ascii(&mut buffer) {
        Err(error) => error,
        Ok(written) => panic!("unexpected {written}-byte write"),
    };
    assert_eq!(buffer_error.to_string(), "output buffer is too small");

    let precision_error = match Pi::<7>::round_nearest_even().try_to_f32_preserving_places() {
        Err(error) => error,
        Ok(value) => panic!("unexpected precision-preserving f32 conversion: {value}"),
    };
    assert_eq!(
        precision_error.to_string(),
        "target float cannot guarantee all requested decimal places"
    );
}

#[cfg(feature = "fixed-point")]
#[test]
fn fixed_interop_error_display_is_nonempty() {
    use fixed::types::I2F6;

    let error = match Pi::<40>::round_nearest_even().to_fixed_nearest_even::<I2F6>() {
        Err(error) => error,
        Ok(value) => panic!("unexpected I2F6 conversion: {value}"),
    };
    assert!(!error.to_string().is_empty());
}

#[cfg(feature = "decimal")]
#[test]
fn rust_decimal_error_display_contract_is_stable() {
    let error = match Pi::<29>::truncated().try_to_rust_decimal_exact() {
        Err(error) => error,
        Ok(value) => panic!("unexpected exact Decimal conversion: {value}"),
    };
    assert_eq!(
        error.to_string(),
        "rust_decimal cannot exactly retain all requested decimal places"
    );
}

#[cfg(feature = "fixed-point")]
#[test]
fn fixed_buffer_error_display_contract_is_stable() {
    let value = Pi::<40>::truncated();
    let mut tiny = [0_u8; 1];
    let buffer_error = match value.write_ascii(&mut tiny) {
        Err(error) => error,
        Ok(written) => panic!("unexpected {written}-byte write"),
    };
    let error = perfect_pi::FixedInteropError::Buffer(buffer_error);
    assert_eq!(error.to_string(), "output buffer is too small");
}

#[cfg(feature = "decimal")]
#[test]
fn rust_decimal_defensive_error_displays_are_nonempty() {
    let coefficient = perfect_pi::RustDecimalInteropError::CoefficientOverflow;
    assert_eq!(coefficient.to_string(), "decimal coefficient overflow");

    let destination_error = match rust_decimal::Decimal::try_from_i128_with_scale(0, 29) {
        Err(error) => error,
        Ok(value) => panic!("unexpected Decimal with invalid scale: {value}"),
    };
    let destination = perfect_pi::RustDecimalInteropError::Destination(destination_error);
    assert!(!destination.to_string().is_empty());
}
