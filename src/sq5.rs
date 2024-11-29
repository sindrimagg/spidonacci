use num_bigint::{BigUint};
use num_traits::{Zero, One};

pub fn fib(n: usize) -> BigUint {
    pow((&One::one(), &One::one()), n).1
}

fn mult(a: (&BigUint, &BigUint), b: (&BigUint, &BigUint)) -> (BigUint, BigUint) {
    ((a.0*b.0+a.1*b.1*BigUint::from(5usize)) >> 1, (a.0*b.1+a.1*b.0) >> 1)
}

fn square(a: (&BigUint, &BigUint)) -> (BigUint, BigUint) {
    ((a.0*a.0+a.1*a.1*BigUint::from(5usize)) >> 1, a.0*a.1)
}

/*
fn pow(base: (&BigUint, &BigUint), exp: usize) -> (BigUint, BigUint) {
    let mut ret = (BigUint::from(2usize), Zero::zero());
    let mut acc = (base.0.clone(), base.1.clone());
    let mut exp = exp;

    while exp > 0 {
        if exp & 1 == 1 {
            ret = mul((&ret.0, &ret.1), (&acc.0, &acc.1));
        }
        acc = mul((&acc.0, &acc.1), (&acc.0, &acc.1));
        exp >>= 1;
    }

    ret
}
*/

fn pow(base: (&BigUint, &BigUint), exp: usize) -> (BigUint, BigUint) {
    if exp == 0 {
        return (BigUint::from(2usize), Zero::zero());
    } else if exp == 1 {
        return (base.0.clone(), base.1.clone());
    } else {
        let unit: (BigUint, BigUint) = pow(base, exp % 2);
        let acc: (BigUint, BigUint) = pow(base, exp / 2);
        let oth: (BigUint, BigUint) =
            square((&acc.0, &acc.1));

        return mult((&unit.0, &unit.1), (&oth.0, &oth.1));
    }
}


