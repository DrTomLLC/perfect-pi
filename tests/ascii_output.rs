use perfect_pi::Pi;

#[test]
fn writes_zero_decimal_places_without_allocation() {
    let value = Pi::<0>::truncated();
    let mut buffer = [0_u8; 1];
    let written = value.write_ascii(&mut buffer);
    assert_eq!(written, Ok(1));
    assert_eq!(&buffer, b"3");
}

#[test]
fn writes_forty_decimal_places_without_allocation() {
    let value = Pi::<40>::round_nearest_even();
    let mut buffer = [0_u8; 42];
    let written = value.write_ascii(&mut buffer);
    assert_eq!(written, Ok(42));
    assert_eq!(&buffer, b"3.1415926535897932384626433832795028841972");
}

#[test]
fn reports_required_buffer_size_without_panicking() {
    let value = Pi::<40>::truncated();
    let mut buffer = [0_u8; 41];
    let error = value.write_ascii(&mut buffer);

    match error {
        Err(error) => {
            assert_eq!(error.required(), 42);
            assert_eq!(error.provided(), 41);
        }
        Ok(written) => panic!("unexpected write of {written} bytes"),
    }
}
