use num_bigint::{BigUint};
use num_traits::{Zero, One};

pub fn fib(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();

    for _ in 0..n {
        let t = f0 + &f1;
        f0 = f1;
        f1 = t;
    }
    f0
}
