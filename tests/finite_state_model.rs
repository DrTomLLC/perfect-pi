use perfect_pi::{DecimalPi, Pi};

const CANONICAL: &[u8; 41] = b"14159265358979323846264338327950288419716";
const SENTINEL: u8 = 0xA5;

fn expected_ascii(decimal_places: usize, rounded: bool) -> Vec<u8> {
    if decimal_places == 0 {
        return vec![b'3'];
    }

    let mut output = Vec::with_capacity(decimal_places + 2);
    output.push(b'3');
    output.push(b'.');
    output.extend_from_slice(&CANONICAL[..decimal_places]);

    if rounded && CANONICAL[decimal_places] >= b'5' {
        let mut index = output.len();
        while index > 2 {
            index -= 1;
            if output[index] == b'9' {
                output[index] = b'0';
            } else {
                output[index] += 1;
                return output;
            }
        }
        output[0] += 1;
    }

    output
}

fn verify_value<const D: usize>(value: DecimalPi<D>, expected: &[u8]) {
    assert_eq!(value.integer_part(), 3);
    assert_eq!(value.decimal_places(), D);
    assert_eq!(value.fractional_digits().len(), D);
    assert!(value.fractional_digits().iter().all(|digit| *digit <= 9));
    assert_eq!(value.ascii_len(), expected.len());
    assert_eq!(format!("{value}"), String::from_utf8_lossy(expected));

    for capacity in 0..=44 {
        let mut backing = [SENTINEL; 48];
        let before = backing;
        let result = value.write_ascii(&mut backing[..capacity]);

        if capacity < expected.len() {
            let error = match result {
                Err(error) => error,
                Ok(written) => {
                    panic!("D={D}, capacity={capacity}: unexpected {written}-byte write")
                }
            };
            assert_eq!(error.required(), expected.len());
            assert_eq!(error.provided(), capacity);
            assert_eq!(
                backing, before,
                "D={D}, capacity={capacity}: failed write modified memory"
            );
        } else {
            let written = match result {
                Ok(written) => written,
                Err(error) => panic!(
                    "D={D}, capacity={capacity}: unexpected short-buffer error required={} provided={}",
                    error.required(),
                    error.provided()
                ),
            };
            assert_eq!(written, expected.len());
            assert_eq!(&backing[..written], expected);
            assert!(
                backing[written..].iter().all(|byte| *byte == SENTINEL),
                "D={D}, capacity={capacity}: successful write modified bytes beyond result length"
            );
        }
    }
}

macro_rules! verify_precision {
    ($d:literal) => {{
        let truncated = Pi::<$d>::truncated();
        let rounded = Pi::<$d>::round_nearest_even();
        let expected_truncated = expected_ascii($d, false);
        let expected_rounded = expected_ascii($d, true);

        verify_value(truncated, &expected_truncated);
        verify_value(rounded, &expected_rounded);
    }};
}

#[test]
fn exhaustively_model_checks_every_bounded_ascii_state() {
    verify_precision!(0);
    verify_precision!(1);
    verify_precision!(2);
    verify_precision!(3);
    verify_precision!(4);
    verify_precision!(5);
    verify_precision!(6);
    verify_precision!(7);
    verify_precision!(8);
    verify_precision!(9);
    verify_precision!(10);
    verify_precision!(11);
    verify_precision!(12);
    verify_precision!(13);
    verify_precision!(14);
    verify_precision!(15);
    verify_precision!(16);
    verify_precision!(17);
    verify_precision!(18);
    verify_precision!(19);
    verify_precision!(20);
    verify_precision!(21);
    verify_precision!(22);
    verify_precision!(23);
    verify_precision!(24);
    verify_precision!(25);
    verify_precision!(26);
    verify_precision!(27);
    verify_precision!(28);
    verify_precision!(29);
    verify_precision!(30);
    verify_precision!(31);
    verify_precision!(32);
    verify_precision!(33);
    verify_precision!(34);
    verify_precision!(35);
    verify_precision!(36);
    verify_precision!(37);
    verify_precision!(38);
    verify_precision!(39);
    verify_precision!(40);
}

#[test]
fn precision_markers_cover_exactly_the_documented_bounded_domain() {
    assert_eq!(perfect_pi::MAX_DECIMAL_PLACES, 40);
    assert_eq!(Pi::<0>::decimal_places(), 0);
    assert_eq!(Pi::<40>::decimal_places(), 40);
}
