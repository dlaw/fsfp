#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use fp::*;

use core::ops::{Add, Neg, Sub};

fn validate<A: Num, B: Num, C: Num>()
where
    A: Add<A, Output = B> + Sub<A, Output = C> + Neg<Output = C>,
    <A as Num>::Raw: Add<<A as Num>::Raw, Output = <B as Num>::Raw>,
    <A as Num>::Raw: TryInto<<C as Num>::Raw>,
    <C as Num>::Raw: Sub<<C as Num>::Raw, Output = <C as Num>::Raw>,
    <C as Num>::Raw: Neg<Output = <C as Num>::Raw>,
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
    validate::<I8<7, -3>, I8<8, -3>, I8<8, -3>>();
    validate::<I32<4, 0>, I32<5, 0>, I32<5, 0>>();
    validate::<Usize<12, 0>, Usize<13, 0>, Isize<13, 0>>();
    validate::<U128<127, 41>, U128<128, 41>, I128<128, 41>>();
}
