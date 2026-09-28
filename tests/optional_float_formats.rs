#[cfg(feature = "binary16")]
use perfect_pi::{
    Binary16, FRAC_PI_2_BINARY16, FRAC_PI_3_BINARY16, FRAC_PI_4_BINARY16, FRAC_PI_6_BINARY16,
    FRAC_PI_8_BINARY16, INV_PI_BINARY16, PI_BINARY16, TAU_BINARY16, TWO_INV_PI_BINARY16,
    TWO_INV_SQRT_PI_BINARY16,
};
#[cfg(feature = "binary128")]
use perfect_pi::{
    Binary128, FRAC_PI_2_BINARY128, FRAC_PI_3_BINARY128, FRAC_PI_4_BINARY128, FRAC_PI_6_BINARY128,
    FRAC_PI_8_BINARY128, INV_PI_BINARY128, PI_BINARY128, TAU_BINARY128, TWO_INV_PI_BINARY128,
    TWO_INV_SQRT_PI_BINARY128,
};

#[cfg(feature = "binary16")]
#[test]
fn binary16_constants_match_verified_ieee_bits() {
    assert_eq!(core::mem::size_of::<Binary16>(), 2);
    assert_eq!(PI_BINARY16.to_bits(), 0x4248);
    assert_eq!(TAU_BINARY16.to_bits(), 0x4648);
    assert_eq!(FRAC_PI_2_BINARY16.to_bits(), 0x3e48);
    assert_eq!(FRAC_PI_3_BINARY16.to_bits(), 0x3c30);
    assert_eq!(FRAC_PI_4_BINARY16.to_bits(), 0x3a48);
    assert_eq!(FRAC_PI_6_BINARY16.to_bits(), 0x3830);
    assert_eq!(FRAC_PI_8_BINARY16.to_bits(), 0x3648);
    assert_eq!(INV_PI_BINARY16.to_bits(), 0x3518);
    assert_eq!(TWO_INV_PI_BINARY16.to_bits(), 0x3918);
    assert_eq!(TWO_INV_SQRT_PI_BINARY16.to_bits(), 0x3c83);
    assert_eq!(PI_BINARY16.to_be_bytes(), [0x42, 0x48]);
    assert_eq!(PI_BINARY16.to_le_bytes(), [0x48, 0x42]);
    assert_eq!(Binary16::from_bits(0x4248), PI_BINARY16);
}

#[cfg(feature = "binary128")]
#[test]
fn binary128_constants_match_verified_ieee_bits() {
    assert_eq!(core::mem::size_of::<Binary128>(), 16);
    assert_eq!(
        PI_BINARY128.to_bits(),
        0x4000_921f_b544_42d1_8469_898c_c517_01b8
    );
    assert_eq!(
        TAU_BINARY128.to_bits(),
        0x4001_921f_b544_42d1_8469_898c_c517_01b8
    );
    assert_eq!(
        FRAC_PI_2_BINARY128.to_bits(),
        0x3fff_921f_b544_42d1_8469_898c_c517_01b8
    );
    assert_eq!(
        FRAC_PI_3_BINARY128.to_bits(),
        0x3fff_0c15_2382_d736_5846_5bb3_2e0f_567b
    );
    assert_eq!(
        FRAC_PI_4_BINARY128.to_bits(),
        0x3ffe_921f_b544_42d1_8469_898c_c517_01b8
    );
    assert_eq!(
        FRAC_PI_6_BINARY128.to_bits(),
        0x3ffe_0c15_2382_d736_5846_5bb3_2e0f_567b
    );
    assert_eq!(
        FRAC_PI_8_BINARY128.to_bits(),
        0x3ffd_921f_b544_42d1_8469_898c_c517_01b8
    );
    assert_eq!(
        INV_PI_BINARY128.to_bits(),
        0x3ffd_45f3_06dc_9c88_2a53_f84e_afa3_ea6a
    );
    assert_eq!(
        TWO_INV_PI_BINARY128.to_bits(),
        0x3ffe_45f3_06dc_9c88_2a53_f84e_afa3_ea6a
    );
    assert_eq!(
        TWO_INV_SQRT_PI_BINARY128.to_bits(),
        0x3fff_20dd_7504_29b6_d11a_e3a9_14fe_d7fe
    );
    assert_eq!(
        PI_BINARY128.to_be_bytes(),
        [
            0x40, 0x00, 0x92, 0x1f, 0xb5, 0x44, 0x42, 0xd1, 0x84, 0x69, 0x89, 0x8c, 0xc5, 0x17,
            0x01, 0xb8
        ]
    );
    assert_eq!(
        PI_BINARY128.to_le_bytes(),
        [
            0xb8, 0x01, 0x17, 0xc5, 0x8c, 0x89, 0x69, 0x84, 0xd1, 0x42, 0x44, 0xb5, 0x1f, 0x92,
            0x00, 0x40
        ]
    );
    assert_eq!(Binary128::from_bits(PI_BINARY128.to_bits()), PI_BINARY128);
}
