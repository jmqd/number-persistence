extern crate clap;
extern crate num;

use clap::{App, Arg};
use num::bigint::*;
use num::FromPrimitive;
use std::str::FromStr;

fn main() {
    let matches = App::new("number-persistence")
        .version("0.1.0")
        .author("Jordan McQueen <jordan@whoami.sh>")
        .about("Multiplicative persistence checker.")
        .arg(
            Arg::with_name("number")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("number to check"),
        )
        .get_matches();
    let number: BigUint = FromStr::from_str(matches.value_of("number").unwrap()).unwrap();
    println!("{}", calculate_multiplicative_persistence(number));
}

/// Calculates the multiplicative persistence of a number.
///
/// i.e. The count of times we can progressively multiply the set of integers
///      represented by the base-10 digits of the number while the product is a
///      different number from the previous number.
///
/// e.g. For the number 256, we convert this to the following set: {2, 5, 6}.
///      `2 * 5 * 6` => 60 => {6, 0} => 6 * 0 => 0 => {0, 1} => 0 * 1 => 0`
///      Iterating this process, we see that it takes 2 steps to reach a point
///      where further steps will always result in the same number. (In the
///      case of a single digit, we add the mutliplicative identity (1) as a
///      simple way to show the property.)
///
/// 277777788888899 is the number with the current highest-known multiplicative
/// persistence count of 11.
/// ```rust
/// let record_num: BigUint = FromStr::from_str("277777788888899")?
/// assert!(calculate_multiplicative_persistence(record_num) == 11)
/// ```
fn calculate_multiplicative_persistence(number: BigUint) -> u32 {
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
