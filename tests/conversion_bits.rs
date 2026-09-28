use perfect_pi::{F32_GUARANTEED_DECIMAL_PLACES, F64_GUARANTEED_DECIMAL_PLACES, Pi};

macro_rules! assert_bits {
    ($d:literal, $tf32:literal, $tf64:literal, $rf32:literal, $rf64:literal) => {{
        let truncated = Pi::<$d>::truncated();
        let rounded = Pi::<$d>::round_nearest_even();
        assert_eq!(truncated.to_f32_lossy().to_bits(), $tf32, "D={} truncated f32", $d);
        assert_eq!(truncated.to_f64_lossy().to_bits(), $tf64, "D={} truncated f64", $d);
        assert_eq!(rounded.to_f32_lossy().to_bits(), $rf32, "D={} rounded f32", $d);
        assert_eq!(rounded.to_f64_lossy().to_bits(), $rf64, "D={} rounded f64", $d);
    }};
}

#[test]
fn all_bounded_values_match_independent_ieee_reference_bits() {
    assert_bits!(0, 0x40400000, 0x4008000000000000, 0x40400000, 0x4008000000000000);
    assert_bits!(1, 0x40466666, 0x4008cccccccccccd, 0x40466666, 0x4008cccccccccccd);
    assert_bits!(2, 0x4048f5c3, 0x40091eb851eb851f, 0x4048f5c3, 0x40091eb851eb851f);
    assert_bits!(3, 0x40490625, 0x400920c49ba5e354, 0x40491687, 0x400922d0e5604189);
    assert_bits!(4, 0x40490e56, 0x400921cac083126f, 0x40490ff9, 0x400921ff2e48e8a7);
    assert_bits!(5, 0x40490fd0, 0x400921f9f01b866e, 0x40490fd0, 0x400921f9f01b866e);
    assert_bits!(6, 0x40490fd8, 0x400921fafc8b007a, 0x40490fdc, 0x400921fb82c2bd7f);
    assert_bits!(7, 0x40490fda, 0x400921fb4d12d84a, 0x40490fdb, 0x400921fb5a7ed197);
    assert_bits!(8, 0x40490fdb, 0x400921fb53c8d4f1, 0x40490fdb, 0x400921fb53c8d4f1);
    assert_bits!(9, 0x40490fdb, 0x400921fb542fe938, 0x40490fdb, 0x400921fb54524550);
    assert_bits!(10, 0x40490fdb, 0x400921fb54411744, 0x40490fdb, 0x400921fb544486e0);
    assert_bits!(11, 0x40490fdb, 0x400921fb5443d6f4, 0x40490fdb, 0x400921fb54442eea);
    assert_bits!(12, 0x40490fdb, 0x400921fb5444261e, 0x40490fdb, 0x400921fb54442eea);
    assert_bits!(13, 0x40490fdb, 0x400921fb54442c46, 0x40490fdb, 0x400921fb54442d28);
    assert_bits!(14, 0x40490fdb, 0x400921fb54442d11, 0x40490fdb, 0x400921fb54442d11);
    assert_bits!(15, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(16, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(17, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(18, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(19, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(20, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(21, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(22, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(23, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(24, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(25, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(26, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(27, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(28, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(29, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(30, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(31, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(32, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(33, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(34, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(35, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(36, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(37, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(38, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(39, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
    assert_bits!(40, 0x40490fdb, 0x400921fb54442d18, 0x40490fdb, 0x400921fb54442d18);
}

#[test]
fn checked_conversion_boundaries_are_explicit() {
    assert_eq!(F32_GUARANTEED_DECIMAL_PLACES, 6);
    assert_eq!(F64_GUARANTEED_DECIMAL_PLACES, 15);

    assert!(Pi::<6>::round_nearest_even().try_to_f32_preserving_places().is_ok());
    let f32_error = Pi::<7>::round_nearest_even().try_to_f32_preserving_places();
    match f32_error {
        Err(error) => {
            assert_eq!(error.requested_decimal_places(), 7);
            assert_eq!(error.guaranteed_decimal_places(), F32_GUARANTEED_DECIMAL_PLACES);
        }
        Ok(value) => panic!("unexpected checked f32 conversion: {value}"),
    }

    assert!(Pi::<15>::round_nearest_even().try_to_f64_preserving_places().is_ok());
    let f64_error = Pi::<16>::round_nearest_even().try_to_f64_preserving_places();
    match f64_error {
        Err(error) => {
            assert_eq!(error.requested_decimal_places(), 16);
            assert_eq!(error.guaranteed_decimal_places(), F64_GUARANTEED_DECIMAL_PLACES);
        }
        Ok(value) => panic!("unexpected checked f64 conversion: {value}"),
    }
}

macro_rules! assert_preserves_f32 {
    ($d:literal) => {{
        for source in [Pi::<$d>::truncated(), Pi::<$d>::round_nearest_even()] {
            let converted = match source.try_to_f32_preserving_places() {
                Ok(value) => value,
                Err(error) => panic!("unexpected f32 precision loss at D={}: requested={}, guaranteed={}", $d, error.requested_decimal_places(), error.guaranteed_decimal_places()),
            };
            assert_eq!(format!("{:.*}", $d, converted), source.to_string());
        }
    }};
}

macro_rules! assert_preserves_f64 {
    ($d:literal) => {{
        for source in [Pi::<$d>::truncated(), Pi::<$d>::round_nearest_even()] {
            let converted = match source.try_to_f64_preserving_places() {
                Ok(value) => value,
                Err(error) => panic!("unexpected f64 precision loss at D={}: requested={}, guaranteed={}", $d, error.requested_decimal_places(), error.guaranteed_decimal_places()),
            };
            assert_eq!(format!("{:.*}", $d, converted), source.to_string());
        }
    }};
}

#[test]
fn checked_conversions_preserve_every_guaranteed_decimal_place() {
    assert_preserves_f32!(0);
    assert_preserves_f32!(1);
    assert_preserves_f32!(2);
    assert_preserves_f32!(3);
    assert_preserves_f32!(4);
    assert_preserves_f32!(5);
    assert_preserves_f32!(6);
    assert_preserves_f64!(0);
    assert_preserves_f64!(1);
    assert_preserves_f64!(2);
    assert_preserves_f64!(3);
    assert_preserves_f64!(4);
    assert_preserves_f64!(5);
    assert_preserves_f64!(6);
    assert_preserves_f64!(7);
    assert_preserves_f64!(8);
    assert_preserves_f64!(9);
    assert_preserves_f64!(10);
    assert_preserves_f64!(11);
    assert_preserves_f64!(12);
    assert_preserves_f64!(13);
    assert_preserves_f64!(14);
    assert_preserves_f64!(15);
}
