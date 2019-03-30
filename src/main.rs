extern crate num;

use num::bigint::*;
use num::FromPrimitive;
use std::str::FromStr;

fn main() {
    println!(
        "{}",
        calculate_persistence(
            FromStr::from_str("277777788888899").expect("From string on constant failed.")
        )
    );
}

fn calculate_persistence(number: BigUint) -> u32 {
    let mut persistence: u32 = 0;
    let mut working_num: BigUint = number;

    while working_num >= FromStr::from_str("10").unwrap() {
        persistence += 1;
        working_num = working_num
            .to_string()
            .chars()
            .fold(num::one(), |acc, digit| {
                acc * BigUint::from_u32(char::to_digit(digit, 10).unwrap()).unwrap()
            });
    }
    return persistence;
}
