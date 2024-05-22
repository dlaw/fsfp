use core::ops::{Add, Neg, Sub};

use crate::Fp;

/// Needed for const-generic support, because the standard
/// ways to compute maximum of two values are not const.
pub const fn max(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}

macro_rules! fp_impl {
    ($Name:ident, $Iname:ident) => {
        use crate::$Name;
        /// Two fixed-point integers with the same raw type and the same shift may be
        /// added together.  The result has the same raw type and the same shift.  The result
        /// has 1 more bit than the number of bits in the wider of the two inputs.
        impl<const B0: u32, const B1: u32, const S: i32> Add<$Name<B1, S>> for $Name<B0, S>
        where
            [(); (max(B0, B1) + 1) as usize]:,
        {
            type Output = $Name<{ max(B0, B1) + 1 }, S>;
            fn add(self: $Name<B0, S>, other: $Name<B1, S>) -> Self::Output {
                unsafe {
                    Self::Output::new_unchecked(
                        // use wrapping_sub to ensure we don't do overflow checks
                        // (overflow safety is guaranteed by the type system)
                        self.raw().wrapping_add(other.raw()),
                    )
                }
            }
        }
        /// Two fixed-point integers with the same raw type and the same shift may be
        /// subtracted.  The result is always signed, even if the inputs were unsigned.
        /// The result has the same shift as the inputs, and 1 more bit than the number
        /// of bits in the wider of the two inputs.
        impl<const B0: u32, const B1: u32, const S: i32> Sub<$Name<B1, S>> for $Name<B0, S>
        where
            [(); (max(B0, B1) + 1) as usize]:,
        {
            // Subtraction output is always signed, even for unsigned inputs.
            type Output = $Iname<{ max(B0, B1) + 1 }, S>;
            fn sub(self: $Name<B0, S>, other: $Name<B1, S>) -> Self::Output {
                unsafe {
                    Self::Output::new_unchecked(
                        // use wrapping_sub to ensure we don't do overflow checks
                        // (overflow safety is guaranteed by the type system)
                        self.raw().wrapping_sub(other.raw()) as <Self::Output as Fp>::Raw,
                    )
                }
            }
        }
        impl<const B: u32, const S: i32> Neg for $Name<B, S>
        where
            [(); (B + 1) as usize]:,
        {
            // Negation output is always signed, even for unsigned inputs.
            // Negation adds a bit: unsigned values gain a sign bit;
            // signed values can overflow from MIN to -MIN = MAX + 1.
            type Output = $Iname<{ B + 1 }, S>;
            fn neg(self: $Name<B, S>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(-(self.raw() as <Self::Output as Fp>::Raw)) }
            }
        }
    };
}

fp_impl!(FpU8, FpI8);
fp_impl!(FpI8, FpI8);
fp_impl!(FpU16, FpI16);
fp_impl!(FpI16, FpI16);
fp_impl!(FpU32, FpI32);
fp_impl!(FpI32, FpI32);
fp_impl!(FpU64, FpI64);
fp_impl!(FpI64, FpI64);
fp_impl!(FpU128, FpI128);
fp_impl!(FpI128, FpI128);
fp_impl!(FpUsize, FpIsize);
fp_impl!(FpIsize, FpIsize);
