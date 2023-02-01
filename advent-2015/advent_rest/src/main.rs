use std::env;

use q12::run_code as q12_run_code;
use q13::run_code as q13_run_code;
use q14::run_code as q14_run_code;
use q15::run_code as q15_run_code;

pub mod q12;
pub mod q13;
pub mod q14;
pub mod q15;

fn main() {
    let q = env::args().nth(1).and_then(|arg| arg.parse::<u32>().ok());
    println!("{:?}", q);

    match q {
        Some(12) => q12_run_code(),
        Some(13) => q13_run_code(),
        Some(14) => q14_run_code(),
        Some(15) => q15_run_code(),
        _ => panic!("Error"),
    }
    // q12_run_code();
    // q13_run_code();
}
