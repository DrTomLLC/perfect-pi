use perfect_pi::Pi;

#[test]
fn write_ascii_uses_caller_buffer_without_allocation() {
    let value = Pi::<3>::round_nearest_even();
    let mut output = [0xAA_u8; 8];

    let written = match value.write_ascii(&mut output) {
        Ok(written) => written,
        Err(error) => panic!(
            "unexpected buffer error: required={}, provided={}",
            error.required(),
            error.provided()
        ),
    };

    assert_eq!(written, 5);
    assert_eq!(&output[..written], b"3.142");
    assert_eq!(&output[written..], &[0xAA, 0xAA, 0xAA]);
}

#[test]
fn write_ascii_reports_required_capacity() {
    let value = Pi::<40>::round_nearest_even();
    let mut output = [0_u8; 41];

    let error = match value.write_ascii(&mut output) {
        Err(error) => error,
        Ok(written) => panic!("unexpected successful write of {written} bytes"),
    };

    assert_eq!(error.required(), 42);
    assert_eq!(error.provided(), 41);
}
