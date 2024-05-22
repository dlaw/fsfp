#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use fp::*;

use core::ops::{Add, Neg, Sub};

fn validate<A: Fp, B: Fp, C: Fp>()
where
    A: Add<A, Output = B> + Sub<A, Output = C> + Neg<Output = C>,
    <A as Fp>::Raw: Add<<A as Fp>::Raw, Output = <B as Fp>::Raw>,
    <A as Fp>::Raw: TryInto<<C as Fp>::Raw>,
    <C as Fp>::Raw: Sub<<C as Fp>::Raw, Output = <C as Fp>::Raw>,
    <C as Fp>::Raw: Neg<Output = <C as Fp>::Raw>,
{
    for a0 in [A::MIN, A::MAX] {
        for a1 in [A::MIN, A::MAX] {
            assert!(a0 + a1 >= B::MIN);
            assert!(a0 + a1 <= B::MAX);
            assert!(a0.raw() + a1.raw() == (a0 + a1).raw());
            assert!(a0 - a1 >= C::MIN);
            assert!(a0 - a1 <= C::MAX);
            assert!(
                a0.raw().try_into().ok().unwrap() - a1.raw().try_into().ok().unwrap()
                    == (a0 - a1).raw()
            )
        }
        assert!(-a0 >= C::MIN);
        assert!(-a0 <= C::MAX);
        assert!((-a0).raw() == -(a0.raw().try_into().ok().unwrap()));
    }
}

#[test]
fn add_sub_limits() {
    validate::<FpI8<7, -3>, FpI8<8, -3>, FpI8<8, -3>>();
    validate::<FpI32<4, 0>, FpI32<5, 0>, FpI32<5, 0>>();
    validate::<FpUsize<12, 0>, FpUsize<13, 0>, FpIsize<13, 0>>();
    validate::<FpU128<127, 41>, FpU128<128, 41>, FpI128<128, 41>>();
}
