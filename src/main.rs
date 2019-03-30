extern crate clap;
extern crate num;

use clap::{App, Arg, SubCommand};
use num::bigint::*;
use num::FromPrimitive;
use num::One;
use std::str::FromStr;

// TODO(mcqueenjordan): Add optimized searches to find maximums within integer ranges.
// TODO(mcqueenjordan): Add a command for iteratively checking ranges etc.

fn main() {
    let cli = App::new("number-persistence")
        .version("0.1.0")
        .author("Jordan McQueen <jordan@whoami.sh>")
        .about("Multiplicative persistence checker.")
        .subcommand(
            SubCommand::with_name("check-multiplicative").arg(
                Arg::with_name("number")
                    .required(true)
                    .takes_value(true)
                    .index(1)
                    .help("number to check"),
            ),
        )
        .subcommand(
            SubCommand::with_name("search")
                .arg(
                    Arg::with_name("begin")
                        .required(true)
                        .takes_value(true)
                        .index(1)
                        .help("Number at which to begin searching."),
                )
                .arg(
                    Arg::with_name("end")
                        .required(true)
                        .takes_value(true)
                        .index(2)
                        .help("Upper bound. When to stop searching."),
                ),
        )
        .get_matches();

    // TODO(mcqueenjordan): This is verbose and ugly, but I don't immediately see a better way.
    match cli.subcommand_name().unwrap() {
        "search" => search_for_maximum_multiplicative_persistence(
            &FromStr::from_str(
                cli.subcommand_matches("search")
                    .unwrap()
                    .value_of("begin")
                    .unwrap(),
            )
            .unwrap(),
            &FromStr::from_str(
                cli.subcommand_matches("search")
                    .unwrap()
                    .value_of("end")
                    .unwrap(),
            )
            .unwrap(),
        ),
        "check-multiplicative" => println!(
            "{}",
            calculate_multiplicative_persistence(
                &FromStr::from_str(
                    cli.subcommand_matches("check-multiplicative")
                        .expect("Subcommand unwrap failed")
                        .value_of("number")
                        .unwrap()
                )
                .unwrap()
            )
        ),
        _ => panic!("Unknown subcommand..."),
    }
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
fn calculate_multiplicative_persistence(number: &BigUint) -> u32 {
    let mut persistence: u32 = 0;
    let mut working_num: BigUint = number.clone();

    // TODO(mcqueenjordan): better optimize this while condition
    // TODO(mcqueenjordan): DP utilizing least-recently-used eviction
    while working_num >= FromStr::from_str("10").unwrap() {
        persistence += 1;

        // TODO(mcqueenjordan): better optimize this logic.
        // perhaps utilizing `to_radix_digits_le` and clever vec code
        working_num = working_num
            .to_string()
            .chars()
            .fold(num::one(), |acc, digit| {
                acc * BigUint::from_u32(char::to_digit(digit, 10).unwrap()).unwrap()
            });
    }
    return persistence;
}

// TODO(mcqueenjordan): Be smarter about skipping obviously bad digits.
fn search_for_maximum_multiplicative_persistence(start: &BigUint, end: &BigUint) {
    let mut working_num: BigUint = start.clone();
    let mut record: BigUint = BigUint::one();
    let mut max_seen: u32 = 0;

    while working_num < *end {
        // An optimization to skip past numbers with 0s in them.
        let digits: String = working_num
            .to_str_radix(10)
            .chars()
            .map(|c| match c {
                '0' => '1',
                _ => c,
            })
            .collect();
        working_num = FromStr::from_str(&digits).unwrap();

        let persistence = calculate_multiplicative_persistence(&working_num);
        if persistence > max_seen {
            max_seen = persistence;
            record = working_num.to_owned();
            println!("Found a new record: {}", record);
        }
        working_num = working_num + BigUint::one();
    }
    println!("Overall record: {}", record);
}
