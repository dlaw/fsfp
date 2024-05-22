use fp::*;

#[test]
fn good_conversions() {
    let x: FpI16<7, 0> = FpI32::<6, 0>::new(5).unwrap().into_fp();
    let x: FpI32<8, 0> = 125i8.into_fp();
    let x: u16 = FpU32::<16, 0>::new(5).unwrap().into_fp();
}
