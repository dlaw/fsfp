use fp::*;

#[test]
fn min_max_correct() {
    assert!(I64::<0, 10>::MIN.raw() == 0);
    assert!(I64::<0, 10>::MAX.raw() == 0);
    assert!(I64::<0, 10>::SIGNED);

    assert!(U64::<0, 0>::MIN.raw() == 0);
    assert!(U64::<0, 0>::MAX.raw() == 0);
    assert!(!U64::<0, 10>::SIGNED);

    assert!(I32::<8, -2>::MIN.raw() == -128);
    assert!(I32::<8, -2>::MAX.raw() == 127);

    assert!(U32::<8, 3>::MIN.raw() == 0);
    assert!(U32::<8, 3>::MAX.raw() == 255);

    assert!(I16::<16, 0>::MIN.raw() == -32768);
    assert!(I16::<16, 0>::MAX.raw() == 32767);

    assert!(U16::<16, -5>::MIN.raw() == 0);
    assert!(U16::<16, -5>::MAX.raw() == 65535);
}

#[test]
#[should_panic]
fn min_error_signed() {
    let a = I32::<8, 7>::MIN;
    let _ = I32::<8, 7>::new(a.raw() - 1).unwrap();
}

#[test]
#[should_panic]
fn max_error_signed() {
    let a = I32::<8, 0>::MAX;
    let _ = I32::<8, 0>::new(a.raw() + 1).unwrap();
}

#[test]
#[should_panic]
fn max_error_unsigned() {
    let a = U64::<8, 4>::MAX;
    let _ = U64::<8, 4>::new(a.raw() + 1).unwrap();
}
