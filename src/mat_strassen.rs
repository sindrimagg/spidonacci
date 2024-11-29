use num_bigint::BigUint;
use num_traits::{One, Zero};

pub fn fib(n: usize) -> BigUint {
/*    if n == 0 {
        return Zero::zero();
    }
*/
    let f: (&BigUint, &BigUint, &BigUint) = (&Zero::zero(), &One::one(), &One::one());
    let a = pow(f, n);
    return a.1;
}

fn mult(
    a: (&BigUint, &BigUint, &BigUint),
    b: (&BigUint, &BigUint, &BigUint),
) -> (BigUint, BigUint, BigUint) {
    let m1 = (a.0 + a.1) * b.0;
    let m2 = a.1 * (b.1 - b.0);
    let m3 = (a.1 + a.2) * b.1;
    let m4 = a.2 * (b.2 - b.1);

    return (&m1 + &m2, &m3 - &m2, &m3 + &m4);
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
        let acc: (BigUint, BigUint, BigUint) = pow(base, exp >> 1);
        let oth: (BigUint, BigUint, BigUint) =
            square((&acc.0, &acc.1, &acc.2));
        if exp & 1 == 1 {
            mult((&base.0, &base.1, &base.2), (&oth.0, &oth.1, &oth.2))
        } else {
            (oth.0, oth.1, oth.2)
        }
        //let unit: (BigUint, BigUint, BigUint) = pow(base, exp & 1);

        //return mult((&unit.0, &unit.1, &unit.2), (&oth.0, &oth.1, &oth.2));
    }
}

/*
   fn pow(base: (&BigUint, &BigUint, &BigUint), exp: usize) -> (BigUint, BigUint, BigUint) {
   if exp == 0 {
   return (One::one(), Zero::zero(), One::one());
   }
   let mut ret = (base.0.clone(), base.1.clone(), base.2.clone());
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
