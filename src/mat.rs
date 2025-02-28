use num_bigint::BigUint;
use num_traits::{One, Zero};

pub fn fib(n: usize) -> BigUint {
    let f: (&BigUint, &BigUint, &BigUint) = (&Zero::zero(), &One::one(), &One::one());
    let a = pow(f, n);
    return a.1;
}

fn mult(
    a: (&BigUint, &BigUint, &BigUint),
    b: (&BigUint, &BigUint, &BigUint),
) -> (BigUint, BigUint, BigUint) {
    let tmp: BigUint = a.1 * b.1;
    return (a.0 * b.0 + &tmp, a.0 * b.1 + a.1 * b.2, tmp + a.2 * b.2);
}

fn square(
    x: (&BigUint, &BigUint, &BigUint),
 ) -> (BigUint, BigUint, BigUint) {
    let m1 = x.0*x.0;
    let m2 = x.1*x.1;
    let m3 = x.2*x.2;
    let m4 = (x.0 + x.2) * x.1;
   
    return (&m1 + &m2, m4, &m2 + &m3);
}

fn pow(base: (&BigUint, &BigUint, &BigUint), exp: usize) -> (BigUint, BigUint, BigUint) {
    if exp == 0 {
        return (One::one(), Zero::zero(), One::one());
    } else if exp == 1 {
        return (base.0.clone(), base.1.clone(), base.2.clone());
    } else {
        let unit: (BigUint, BigUint, BigUint) = pow(base, exp % 2);
        let acc: (BigUint, BigUint, BigUint) = pow(base, exp / 2);
        let oth: (BigUint, BigUint, BigUint) =
            square((&acc.0, &acc.1, &acc.2));

        return mult((&unit.0, &unit.1, &unit.2), (&oth.0, &oth.1, &oth.2));
    }
}

/*
fn pow(base: (&BigUint, &BigUint, &BigUint), exp: usize) -> (BigUint, BigUint, BigUint) {
    let mut ret = (One::one(), Zero::zero(), One::one());
    let mut acc = (base.0.clone(), base.1.clone(), base.2.clone());
    let mut exp = exp;

    while exp > 0 {
        if exp & 1 == 1 {
            ret = mult((&acc.0, &acc.1, &acc.2), (&ret.0, &ret.1, &ret.2));
        }
        acc = square((&acc.0, &acc.1, &acc.2));
        exp >>= 1;
    }
    ret
}
*/
