use fraction::Fraction;

#[cfg(feature = "my_testing_flag")]
fn test() {
    println!("test() called!");
}

fn main() {
    let mut frac1 = Fraction::init(14, 27).unwrap();
    let mut frac2 = Fraction::init(12, 13).unwrap();
    frac1.multiply(&mut frac2);
    println!("{}/{} * {}/{} = {}/{}", 14, 27, 12, 13, frac1.numerator, frac1.denominator);

    #[cfg(feature = "my_testing_flag")]
    test();
}
