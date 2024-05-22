use fp::*;

#[test]
fn min_max_correct() {
    assert!(FpI64::<0, 10>::MIN.raw() == 0);
    assert!(FpI64::<0, 10>::MAX.raw() == 0);
    assert!(FpI64::<0, 10>::SIGNED);

    assert!(FpU64::<0, 0>::MIN.raw() == 0);
    assert!(FpU64::<0, 0>::MAX.raw() == 0);
    assert!(!FpU64::<0, 10>::SIGNED);

    assert!(FpI32::<8, -2>::MIN.raw() == -128);
    assert!(FpI32::<8, -2>::MAX.raw() == 127);

    assert!(FpU32::<8, 3>::MIN.raw() == 0);
    assert!(FpU32::<8, 3>::MAX.raw() == 255);

    assert!(FpI16::<16, 0>::MIN.raw() == -32768);
    assert!(FpI16::<16, 0>::MAX.raw() == 32767);

    assert!(FpU16::<16, -5>::MIN.raw() == 0);
    assert!(FpU16::<16, -5>::MAX.raw() == 65535);
}

#[test]
#[should_panic]
fn min_error_signed() {
    let a = FpI32::<8, 7>::MIN;
    let _ = FpI32::<8, 7>::new(a.raw() - 1).unwrap();
}

#[test]
#[should_panic]
fn max_error_signed() {
    let a = FpI32::<8, 0>::MAX;
    let _ = FpI32::<8, 0>::new(a.raw() + 1).unwrap();
}

#[test]
#[should_panic]
fn max_error_unsigned() {
    let a = FpU64::<8, 4>::MAX;
    let _ = FpU64::<8, 4>::new(a.raw() + 1).unwrap();
}
