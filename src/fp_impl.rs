use crate::*;

// Because Rust does not provide const traits, nor even any suitable
// non-const traits over integer types, we must use a macro for the impls.
macro_rules! fp_impl {
    ($Name:ident, $T:ty) => {
        // Every integer type is trivially a fixed-point number of that type:
        impl Fp for $T {
            type Raw = $T;
            type Output<const B: u32, const S: i32> = $Name<B, S>;
            const BITS: u32 = <$T>::BITS;
            const SHIFT: i32 = 0;
            const MIN: $T = <$T>::MIN;
            const MAX: $T = <$T>::MAX;
            #[allow(unused_comparisons)]
            const SIGNED: bool = <$T>::MIN < 0;
            unsafe fn new_unchecked(val: $T) -> Self {
                val
            }
            unsafe fn from_f32_unchecked(val: f32) -> Self {
                val as $T
            }
            unsafe fn from_f64_unchecked(val: f64) -> Self {
                val as $T
            }
            fn raw(self) -> $T {
                self
            }
            fn into_f32(self) -> f32 {
                self as f32
            }
            fn into_f64(self) -> f64 {
                self as f64
            }
        }

        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
        #[doc = concat!("`#[repr(transparent)]` struct containing `", stringify!($T), "`, interpreted as a fixed-point number.")]
        ///
        /// Impelements the trait `Fp` for fixed-point manipulation.
        pub struct $Name<const BITS: u32, const SHIFT: i32>($T);

        impl<const BITS: u32, const SHIFT: i32> Fp for $Name<BITS, SHIFT> {
            type Raw = $T;
            type Output<const B: u32, const S: i32> = $Name<B, S>;
            const BITS: u32 = {
                assert!(BITS <= <$T>::BITS, concat!("too many bits for ", stringify!($T)));
                BITS
            };
            const SHIFT: i32 = SHIFT;
            const MIN: Self = Self({
                if Self::BITS == 0 {
                    0
                } else {
                    // n.b. shifting by >= T::BITS is undefined for integer types!
                    <$T>::MIN >> (<$T>::BITS - Self::BITS)
                }
            });
            const MAX: Self = Self({
                if Self::BITS == 0 {
                    0
                } else {
                    // n.b. shifting by >= T::BITS is undefined for integer types!
                    <$T>::MAX >> (<$T>::BITS - Self::BITS)
                }
            });
            #[allow(unused_comparisons)]
            const SIGNED: bool = <$T>::MIN < 0;
            unsafe fn new_unchecked(val: $T) -> Self {
                let _ = Self::BITS;  // force the compile-time check that T is wide enough for BITS
                Self(val)
            }
            /// May cause a divide by zero error if `SHIFT` is extremely small.
            unsafe fn from_f32_unchecked(val: f32) -> Self {
                unsafe { Self::new_unchecked((val * (2_f32).powi(SHIFT)) as $T) }
            }
            /// May cause a divide by zero error if `SHIFT` is extremely small.
            unsafe fn from_f64_unchecked(val: f64) -> Self {
                unsafe { Self::new_unchecked((val * (2_f64).powi(SHIFT)) as $T) }
            }
            fn raw(self) -> $T {
                self.0
            }
            /// Panics when the logical value could exceed `f32::MAX`.
            fn into_f32(self) -> f32 {
                assert!(
                    BITS as i32 - SHIFT - Self::SIGNED as i32 <= f32::MAX_EXP as i32,
                    "number could overflow f32"
                );
                self.0 as f32 / 2_f32.powi(SHIFT)
            }
            /// Panics when the logical value could exceed `f64::MAX`.
            fn into_f64(self) -> f64 {
                assert!(
                    BITS as i32 - SHIFT - Self::SIGNED as i32 <= f64::MAX_EXP as i32,
                    "number could overflow f64"
                );
                self.0 as f64 / 2_f64.powi(SHIFT)
            }
        }

        /// A fixed-point number with no shift and maximum bit width is equivalent
        /// to its internal ("raw") representation.
        impl From<$T> for $Name<{ <$T>::BITS }, 0> {
            fn from(val: $T) -> Self {
                unsafe { Self::new_unchecked(val) }
            }
        }

        /// A fixed-point number with no shift and maximum bit width is equivalent
        /// to its internal ("raw") representation.
        impl From<$Name<{ <$T>::BITS }, 0>> for $T {
            fn from(val: $Name<{ <$T>::BITS }, 0>) -> Self {
                val.raw()
            }
        }
    };
}

fp_impl!(FpI8, i8);
fp_impl!(FpU8, u8);
fp_impl!(FpI16, i16);
fp_impl!(FpU16, u16);
fp_impl!(FpI32, i32);
fp_impl!(FpU32, u32);
fp_impl!(FpI64, i64);
fp_impl!(FpU64, u64);
fp_impl!(FpI128, i128);
fp_impl!(FpU128, u128);
fp_impl!(FpIsize, isize);
fp_impl!(FpUsize, usize);

macro_rules! fp_signed_unsigned_impl {
    ($Uname:ident, $Iname:ident) => {
        impl<const B: u32, const S: i32> $Uname<B, S> {
            pub fn into_signed(self) -> $Iname<{ B + 1 }, S>
            where
                [(); (B + 1) as usize]:,
            {
                unsafe { $Iname::new_unchecked(self.raw() as <$Iname<{ B + 1 }, S> as Fp>::Raw) }
            }
        }
        impl<const B: u32, const S: i32> $Iname<B, S> {
            pub unsafe fn into_unsigned_unchecked(self) -> $Uname<{ B - 1 }, S>
            where
                [(); (B - 1) as usize]:,
            {
                unsafe { $Uname::new_unchecked(self.raw() as <$Uname<B, S> as Fp>::Raw) }
            }
            pub fn into_unsigned(self) -> Option<$Uname<{ B - 1 }, S>>
            where
                [(); (B - 1) as usize]:,
            {
                if self.raw() >= 0 {
                    Some(unsafe{ self.into_unsigned_unchecked() })
                } else {
                    None
                }
            }
        }
    };
}

fp_signed_unsigned_impl!(FpU8, FpI8);
fp_signed_unsigned_impl!(FpU16, FpI16);
fp_signed_unsigned_impl!(FpU32, FpI32);
fp_signed_unsigned_impl!(FpU64, FpI64);
fp_signed_unsigned_impl!(FpU128, FpI128);
fp_signed_unsigned_impl!(FpUsize, FpIsize);
