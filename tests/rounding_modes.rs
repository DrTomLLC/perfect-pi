use perfect_pi::{Pi, RoundingMode};

#[test]
fn bounded_modes_have_explicit_positive_pi_semantics() {
    assert_eq!(Pi::<0>::with_rounding(RoundingMode::TowardZero).to_string(), "3");
    assert_eq!(
        Pi::<0>::with_rounding(RoundingMode::TowardPositiveInfinity).to_string(),
        "4"
    );
    assert_eq!(
        Pi::<0>::with_rounding(RoundingMode::NearestTiesToEven).to_string(),
        "3"
    );
    assert_eq!(Pi::<2>::with_rounding(RoundingMode::TowardZero).to_string(), "3.14");
    assert_eq!(
        Pi::<2>::with_rounding(RoundingMode::TowardNegativeInfinity).to_string(),
        "3.14"
    );
    assert_eq!(Pi::<2>::with_rounding(RoundingMode::AwayFromZero).to_string(), "3.15");
    assert_eq!(
        Pi::<2>::with_rounding(RoundingMode::TowardPositiveInfinity).to_string(),
        "3.15"
    );
    assert_eq!(
        Pi::<2>::with_rounding(RoundingMode::NearestTiesToEven).to_string(),
        "3.14"
    );
    assert_eq!(
        Pi::<2>::with_rounding(RoundingMode::NearestTiesAwayFromZero).to_string(),
        "3.14"
    );
    assert_eq!(
        Pi::<3>::with_rounding(RoundingMode::NearestTiesToEven).to_string(),
        "3.142"
    );
}

macro_rules! verify_precision {
    ($($d:expr),* $(,)?) => {
        $(
            assert_eq!(
                Pi::<$d>::with_rounding(RoundingMode::TowardZero),
                Pi::<$d>::truncated()
            );
            assert_eq!(
                Pi::<$d>::with_rounding(RoundingMode::TowardNegativeInfinity),
                Pi::<$d>::truncated()
            );
            assert_eq!(
                Pi::<$d>::with_rounding(RoundingMode::NearestTiesToEven),
                Pi::<$d>::round_nearest_even()
            );
            assert_eq!(
                Pi::<$d>::with_rounding(RoundingMode::NearestTiesAwayFromZero),
                Pi::<$d>::round_nearest_even()
            );
            assert_eq!(
                Pi::<$d>::with_rounding(RoundingMode::AwayFromZero),
                Pi::<$d>::with_rounding(RoundingMode::TowardPositiveInfinity)
            );
        )*
    };
}

#[test]
fn every_bounded_precision_obeys_mode_aliases() {
    verify_precision!(
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
        40,
    );
}
