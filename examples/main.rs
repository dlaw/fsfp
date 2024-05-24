#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use fp::Num;

fn main() {
    // There are many ways to create a new fixed-point number
    let options: [fp::I32<10, 5>; 4] = [
        fp::I32::from_f32(3.125).unwrap(),
        fp::I32::from_f64(3.125).unwrap(),
        fp::I32::new(100).unwrap(),
        100i32.logical_shr::<5>().set_bits().unwrap(),
    ];
    assert!(options.iter().min() == options.iter().max()); // all equal

    // Arithmetic works pretty much seamlessly
    let a = 12i32.set_bits::<5>().unwrap();
    let b = (-1i32).set_bits::<1>().unwrap();
    let c = a + b; // type of C is I32<6, 0>

    // Addition is associative in value, but not in type
    let d: fp::I32<6, 0> = a + (b + b);
    let e: fp::I32<7, 0> = (a + b) + b;

    let x = fp::I32::<21, 20>::from_f32(0.497).unwrap();
    let y = x.div_const::<12>();
    let z = x + (-y);
    println!("{} {}", x.raw(), x.into_f64());
    println!("{} {}", z.raw(), z.into_f64());
}
