#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use fp::*;

use core::ops::{Mul, Div};

fn validate_mul<A: Fp, B: Fp, C: Fp>() where A: Mul<B, Output=C> {
    for a in [A::MIN, A::MAX] {
        for b in [B::MIN, B::MAX] {
            assert!(a * b >= C::MIN);
            assert!(a * b <= C::MAX);
        }
    }
}

#[test]
fn mul_limits() {
    validate_mul::<FpI32<4, 0>, FpI32<5, 0>, FpI32<9, 0>>();
    validate_mul::<FpI32<4, 0>, FpU32<5, 0>, FpI32<9, 0>>();
    validate_mul::<FpU32<4, 0>, FpI32<5, 0>, FpI32<9, 0>>();
    validate_mul::<FpU32<4, 0>, FpU32<5, 0>, FpU32<9, 0>>();
}

#[test]
fn mul_const() {
    let a = FpI32::<4, 0>::new(4).unwrap();
    let b: FpI32<6, 0> = a.mul_const::<4>();
    assert!(b.raw() == 4 * 4);
    let c: FpI32<7, 0> = a.mul_const::<5>();
    assert!(c.raw() == 4 * 5);
}
