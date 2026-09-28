//! Bounded decimal π representation.

use core::fmt::{self, Write};

/// Maximum public bounded precision, measured in places after the decimal point.
pub const MAX_DECIMAL_PLACES: usize = 40;

const PI_INTEGER_PART: u8 = 3;

// Forty public fractional digits plus the one digit needed to round D=40.
const CANONICAL_FRACTIONAL_DIGITS: [u8; 41] = [
    1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4, 3, 3, 8, 3, 2, 7, 9, 5, 0,
    2, 8, 8, 4, 1, 9, 7, 1, 6,
];

/// Internal compile-time allow-list for bounded decimal precision.
#[doc(hidden)]
pub trait SupportedPrecision<const D: usize> {}

macro_rules! impl_supported_precision {
    ($($d:expr),* $(,)?) => {
        $(impl SupportedPrecision<$d> for () {})*
    };
}
impl_supported_precision!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
);

/// Compile-time marker for requesting π at `D` decimal places.
///
/// The marker has zero runtime size. Operations only exist for the supported
/// bounded precision range `0..=40`.
///
/// ```compile_fail
/// let _ = perfect_pi::Pi::<41>::truncated();
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pi<const D: usize>(());

impl<const D: usize> Pi<D>
where
    (): SupportedPrecision<D>,
{
    /// Returns the number of requested places after the decimal point.
    pub const fn decimal_places() -> usize {
        D
    }

    /// Returns π truncated after exactly `D` decimal places.
    #[must_use]
    pub fn truncated() -> DecimalPi<D> {
        let mut fractional = [0_u8; D];

        let mut index = 0;
        while index < D {
            let destination = match fractional.get_mut(index) {
                Some(destination) => destination,
                None => break,
            };
            let source = match CANONICAL_FRACTIONAL_DIGITS.get(index) {
                Some(source) => *source,
                None => break,
            };
            *destination = source;
            index += 1;
        }

        DecimalPi {
            integer: PI_INTEGER_PART,
            fractional,
        }
    }

    /// Returns π rounded to `D` decimal places using nearest, ties-to-even.
    ///
    /// π is irrational, so an exact finite decimal halfway tie cannot occur.
    /// For π specifically, a first discarded digit of 5 necessarily has a
    /// later nonzero digit, making `next >= 5` equivalent to the declared
    /// nearest, ties-to-even result.
    #[must_use]
    pub fn round_nearest_even() -> DecimalPi<D> {
        let mut value = Self::truncated();
        let next = match CANONICAL_FRACTIONAL_DIGITS.get(D) {
            Some(digit) => *digit,
            None => return value,
        };

        // An exact halfway tie would require π to terminate after a 5,
        // which is impossible because π is irrational. Therefore a leading
        // discarded 5 always has a later nonzero digit and rounds upward.
        let round_up = next >= 5;

        if round_up {
            value.increment_last_place();
        }

        value
    }
}

/// A fixed, allocation-free decimal representation of π.
///
/// The integer portion is stored separately from exactly `D` fractional
/// decimal digits. Values are created by [`Pi::truncated`] or
/// [`Pi::round_nearest_even`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecimalPi<const D: usize> {
    integer: u8,
    fractional: [u8; D],
}

impl<const D: usize> DecimalPi<D> {
    /// Returns the integer portion of the represented value.
    #[must_use]
    pub const fn integer_part(&self) -> u8 {
        self.integer
    }

    /// Returns the exact stored fractional decimal digits.
    #[must_use]
    pub const fn fractional_digits(&self) -> &[u8; D] {
        &self.fractional
    }

    /// Returns the number of fractional decimal digits.
    #[must_use]
    pub const fn decimal_places(&self) -> usize {
        D
    }

    /// Returns the number of ASCII bytes required by [`Self::write_ascii`].
    #[must_use]
    pub const fn ascii_len(&self) -> usize {
        if D == 0 { 1 } else { D.saturating_add(2) }
    }

    /// Writes the decimal representation into a caller-provided ASCII buffer.
    ///
    /// No terminator is written. Bytes after the returned length are untouched.
    pub fn write_ascii(&self, output: &mut [u8]) -> Result<usize, BufferTooSmall> {
        let required = self.ascii_len();

        if output.len() < required {
            return Err(BufferTooSmall::new(required, output.len()));
        }

        let provided = output.len();
        let mut slots = output.iter_mut();

        let integer_slot = match slots.next() {
            Some(slot) => slot,
            None => return Err(BufferTooSmall::new(required, provided)),
        };
        *integer_slot = b'0'.saturating_add(self.integer);

        if D != 0 {
            let decimal_slot = match slots.next() {
                Some(slot) => slot,
                None => return Err(BufferTooSmall::new(required, provided)),
            };
            *decimal_slot = b'.';

            for (slot, digit) in slots.zip(self.fractional.iter().copied()) {
                *slot = b'0'.saturating_add(digit);
            }
        }

        Ok(required)
    }

    fn increment_last_place(&mut self) {
        let mut index = D;

        while index != 0 {
            index -= 1;
            let digit = match self.fractional.get_mut(index) {
                Some(digit) => digit,
                None => return,
            };

            if *digit == 9 {
                *digit = 0;
            } else {
                *digit = digit.saturating_add(1);
                return;
            }
        }

        self.integer = self.integer.saturating_add(1);
    }
}

impl<const D: usize> fmt::Display for DecimalPi<D> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_char(char::from(b'0'.saturating_add(self.integer)))?;

        if D != 0 {
            formatter.write_char('.')?;

            for digit in self.fractional.iter().copied() {
                formatter.write_char(char::from(b'0'.saturating_add(digit)))?;
            }
        }

        Ok(())
    }
}

/// Error returned when a caller-provided output buffer is too small.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BufferTooSmall {
    required: usize,
    provided: usize,
}

impl BufferTooSmall {
    const fn new(required: usize, provided: usize) -> Self {
        Self { required, provided }
    }

    /// Returns the number of bytes required for the requested operation.
    #[must_use]
    pub const fn required(&self) -> usize {
        self.required
    }

    /// Returns the number of bytes provided by the caller.
    #[must_use]
    pub const fn provided(&self) -> usize {
        self.provided
    }
}

impl fmt::Display for BufferTooSmall {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("output buffer is too small")
    }
}
