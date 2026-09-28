use perfect_pi::{FRAC_PI_2_F64, PI_F64, TAU_F64};

fn main() {
    let radius_m = 0.125_f64;
    let circumference_m = TAU_F64 * radius_m;
    let area_m2 = PI_F64 * radius_m * radius_m;
    let quarter_turn_rad = FRAC_PI_2_F64;

    println!("circumference_m={circumference_m:.12}");
    println!("area_m2={area_m2:.12}");
    println!("quarter_turn_rad={quarter_turn_rad:.12}");
}
