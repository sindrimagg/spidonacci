mod mat;
mod mat_strassen;
mod naive;
mod sq5;
mod square;
mod time;
mod write;
use num_bigint::BigUint;

fn main() {
    let funs: [(&str, fn(usize) -> BigUint); 5] = [
        ("naive", naive::fib),
        ("matrix", mat::fib),
        ("matrix_strassen", mat_strassen::fib),
        ("golden", sq5::fib),
        ("square", square::fib),
    ];

    for (fun_name, fun) in funs {
        println!("Starting: {}", fun_name);
        let _ = write::write_file(fun_name, fun);
    }
}
