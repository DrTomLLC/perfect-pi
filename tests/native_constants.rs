use perfect_pi::{
    FRAC_PI_2_F32, FRAC_PI_2_F64, FRAC_PI_3_F32, FRAC_PI_3_F64, FRAC_PI_4_F32, FRAC_PI_4_F64,
    FRAC_PI_6_F32, FRAC_PI_6_F64, FRAC_PI_8_F32, FRAC_PI_8_F64, INV_PI_F32, INV_PI_F64, PI_F32,
    PI_F64, TAU_F32, TAU_F64, TWO_INV_PI_F32, TWO_INV_PI_F64, TWO_INV_SQRT_PI_F32,
    TWO_INV_SQRT_PI_F64,
};

#[test]
fn pi_native_bit_patterns_are_exactly_known() {
    assert_eq!(PI_F32.to_bits(), 0x4049_0fdb);
    assert_eq!(PI_F64.to_bits(), 0x4009_21fb_5444_2d18);
}

#[test]
fn native_exports_match_rust_core_constants() {
    assert_eq!(PI_F32.to_bits(), core::f32::consts::PI.to_bits());
    assert_eq!(TAU_F32.to_bits(), core::f32::consts::TAU.to_bits());
    assert_eq!(
        FRAC_PI_2_F32.to_bits(),
        core::f32::consts::FRAC_PI_2.to_bits()
    );
    assert_eq!(
        FRAC_PI_3_F32.to_bits(),
        core::f32::consts::FRAC_PI_3.to_bits()
    );
    assert_eq!(
        FRAC_PI_4_F32.to_bits(),
        core::f32::consts::FRAC_PI_4.to_bits()
    );
    assert_eq!(
        FRAC_PI_6_F32.to_bits(),
        core::f32::consts::FRAC_PI_6.to_bits()
    );
    assert_eq!(
        FRAC_PI_8_F32.to_bits(),
        core::f32::consts::FRAC_PI_8.to_bits()
    );
    assert_eq!(INV_PI_F32.to_bits(), core::f32::consts::FRAC_1_PI.to_bits());
    assert_eq!(
        TWO_INV_PI_F32.to_bits(),
        core::f32::consts::FRAC_2_PI.to_bits()
    );
    assert_eq!(
        TWO_INV_SQRT_PI_F32.to_bits(),
        core::f32::consts::FRAC_2_SQRT_PI.to_bits()
    );

    assert_eq!(PI_F64.to_bits(), core::f64::consts::PI.to_bits());
    assert_eq!(TAU_F64.to_bits(), core::f64::consts::TAU.to_bits());
    assert_eq!(
        FRAC_PI_2_F64.to_bits(),
        core::f64::consts::FRAC_PI_2.to_bits()
    );
    assert_eq!(
        FRAC_PI_3_F64.to_bits(),
        core::f64::consts::FRAC_PI_3.to_bits()
    );
    assert_eq!(
        FRAC_PI_4_F64.to_bits(),
        core::f64::consts::FRAC_PI_4.to_bits()
    );
    assert_eq!(
        FRAC_PI_6_F64.to_bits(),
        core::f64::consts::FRAC_PI_6.to_bits()
    );
    assert_eq!(
        FRAC_PI_8_F64.to_bits(),
        core::f64::consts::FRAC_PI_8.to_bits()
    );
    assert_eq!(INV_PI_F64.to_bits(), core::f64::consts::FRAC_1_PI.to_bits());
    assert_eq!(
        TWO_INV_PI_F64.to_bits(),
        core::f64::consts::FRAC_2_PI.to_bits()
    );
    assert_eq!(
        TWO_INV_SQRT_PI_F64.to_bits(),
        core::f64::consts::FRAC_2_SQRT_PI.to_bits()
    );
}
