extern crate clap;
extern crate hashbrown;
extern crate num;

use clap::{App, Arg, SubCommand};
use hashbrown::HashMap;
use num::bigint::*;
use num::One;
use num::ToPrimitive;
use std::fs::OpenOptions;
use std::io::Write;
use std::str::FromStr;

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
        .subcommand(SubCommand::with_name("programmed-long-search"))
        .get_matches();

    // TODO(mcqueenjordan): This is verbose and ugly, but I don't immediately see a better way.
    match cli.subcommand_name().unwrap() {
        "programmed-long-search" => {
            let mut lower_bound: BigUint = FromStr::from_str("10").unwrap();

            for _ in 0..20587u32 {
                lower_bound = lower_bound * 10u32;
            }
            let upper_bound: BigUint = &lower_bound * 10u32;

            search_for_new_record_multiplicative_persistence(&lower_bound, &upper_bound);
        }
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
                FromStr::from_str(
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
fn calculate_multiplicative_persistence(mut number: BigUint) -> u8 {
    let mut persistence = 0;
    match number.to_u8() {
        // For small numbers, don't bother with the loop unrolling.
        Some(_small_num) => (),

        // Unroll the loop twice before beginning.
        // This allows slightly denser instruction packing.
        None => {
            persistence = 2;
            number = number
                .to_string()
                .chars()
                .fold(num::one(), |acc: BigUint, digit| {
                    acc * char::to_digit(digit, 10).unwrap()
                })
                .to_string()
                .chars()
                .fold(num::one(), |acc, digit| {
                    acc * char::to_digit(digit, 10).unwrap()
                });
        }
    }
    // TODO(mcqueenjordan): better optimize this while condition
    while number >= FromStr::from_str("10").unwrap() {
        persistence += 1;

        // TODO(mcqueenjordan): better optimize this logic.
        // perhaps utilizing `to_radix_digits_le` and clever vec code
        number = number.to_string().chars().fold(num::one(), |acc, digit| {
            acc * char::to_digit(digit, 10).unwrap()
        });
    }
    return persistence;
}

fn calculate_multiplicative_persistence_dp(
    mut number: BigUint,
    cache: &mut HashMap<String, BigUint>,
) -> u8 {
    let mut persistence = 0;
    // TODO(mcqueenjordan): better optimize this while condition
    while number >= FromStr::from_str("10").unwrap() {
        number = divide_and_conquer_digit_multiplication(number.to_string(), cache);
        persistence += 1;
    }
    return persistence;
}

// A recursive dynamic-programming approach to the product of a number's digits.
fn divide_and_conquer_digit_multiplication(
    digits: String,
    cache: &mut HashMap<String, BigUint>,
) -> BigUint {
    match digits.len() {
        // Base cases. For 0 and 1, we return those numbers.
        // For small lengths, we perform the calculation directly, as it is faster than
        // waiting for a register LOAD instruction.
        0 => return BigUint::one(),
        1 => return FromStr::from_str(&digits).unwrap(),

        // Between digit lengths of 2 and 8 inclusive, perform the calculation inline.
        2...8 => {
            return digits.chars().fold(num::one(), |acc, digit| {
                acc * char::to_digit(digit, 10).unwrap()
            });
        }

        // For digits with longer lengths, recursively split the digits in half
        // and merge the results by multiplication, caching intermediate results
        // and the final result in our DP cache.
        _ => match cache.get(&digits) {
            Some(big_num) => return big_num.clone(),
            None => {
                let (lhs, rhs) = digits.split_at(digits.len() / 2);
                let lhs_num = divide_and_conquer_digit_multiplication(lhs.to_string(), cache);
                let rhs_num = divide_and_conquer_digit_multiplication(rhs.to_string(), cache);
                cache.insert(lhs.to_string(), lhs_num);
                cache.insert(rhs.to_string(), rhs_num);
                cache.insert(
                    digits.clone(),
                    cache.get(lhs).unwrap() * cache.get(rhs).unwrap(),
                );
                return cache.get(&digits).unwrap().clone();
            }
        },
    }
}

// TODO(mcqueenjordan): Be smarter about skipping obviously bad digits.
fn search_for_maximum_multiplicative_persistence(start: &BigUint, end: &BigUint) {
    let mut working_num: BigUint = start.clone();
    let mut max_seen: u8 = 0;

    while working_num < *end {
        // An optimization to skip past numbers with 0s in them. We replace the
        // 0s with 1s, always making the number greater. This is a safe
        // search-space reduction because any number with any 0-digits will
        // immediately end persistence.
        // TODO(mcqueenjordan): Do we need this intermediate String result?
        let digits: String = working_num
            .to_str_radix(10)
            .chars()
            .map(|c| match c {
                '0' => '1',
                _ => c,
            })
            .collect();
        working_num = FromStr::from_str(&digits).unwrap();

        let persistence = calculate_multiplicative_persistence(working_num.clone());
        if persistence > max_seen {
            max_seen = persistence;
            println!(
                "Found a new record: {} has a persistence of {}",
                working_num, persistence
            );
        }
        working_num = working_num + BigUint::one();
    }
}

// Heurstic search, skipping unlikely digits.
// Reduces search space by ~70%.
fn search_for_new_record_multiplicative_persistence(start: &BigUint, end: &BigUint) {
    let mut working_num: BigUint = start.clone();
    let mut max_seen: u8 = 0;
    let mut cache: HashMap<String, BigUint> = HashMap::with_capacity(102048);

    while working_num < *end {
        // An optimization to skip past a lot of candidates. This is an unproven
        // conjecture, so it is not guaranteed that the search space was exhaustively
        // searched. But we're going for the gold here, so we're playing a numbers game.
        let digits: String = working_num
            .to_str_radix(10)
            .chars()
            .map(|c| match c {
                '0' => '7',
                '1' => '2',
                '3' => '7',
                '4' => '7',
                '5' => '7',
                '6' => '7',
                '8' => '9',
                _ => c,
            })
            .collect();
        working_num = FromStr::from_str(&digits).unwrap();

        let persistence = calculate_multiplicative_persistence_dp(working_num.clone(), &mut cache);
        if persistence > max_seen {
            max_seen = persistence;
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("/tmp/multiplicative-persistence-data.csv")
                .unwrap();
            if let Err(e) = writeln!(file, "{},{}", working_num, persistence) {
                eprintln!("Couldn't write to the file: {}", e);
            }
        }
        working_num = working_num + BigUint::one() + BigUint::one();
    }
}
