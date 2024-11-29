use num_bigint::{BigUint};
use num_traits::{Zero, One};

pub fn fib(n: usize) -> BigUint {
    fib_sq(n).0
}

fn fib_sq(n: usize) -> (BigUint, BigUint) {
    if n == 0 {
        (Zero::zero(), One::one())
    } else {
        let (a,b) = fib_sq(n >> 1);
        let c = &a*((&b << 1) - &a);
        let d = &a*&a + &b*&b;
        if n&1 == 1 {
            let e = c + &d;
            (d, e)
        } else {
            (c, d)
        }
    }
}
