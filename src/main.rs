extern crate clap;
extern crate num;

use clap::{App, Arg, SubCommand};
use num::bigint::*;
use num::One;
use num::Zero;
use std::fs::OpenOptions;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let cli = App::new("number-persistence")
        .version("0.1.0")
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

    // TODO: This is verbose and ugly, but I don't immediately see a better way.
    match cli.subcommand_name().unwrap() {
        "programmed-long-search" => {
            let mut lower_bound: BigUint = FromStr::from_str("10").unwrap();

            // The currently known theoretical lower bound is ~2.67 * 10^30000.
            for _ in 0..33300u32 {
                lower_bound = lower_bound * 10u32;
            }

            let coeff: u32 = 3;

            let upper_bound: BigUint = &lower_bound * 10u32 * coeff;
            println!("Finished calculating the lower bound; starting search");
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
    // Standard 2018 way to get a BigUint 10
    let ten = BigUint::from(10u32);

    // Loop while number is >= 10
    while number >= ten {
        persistence += 1;
        let digits = number.to_radix_le(10);

        // PERF: If any digit is 0, the product is immediately 0.
        // This short-circuits the multiplying massive numbers.
        if digits.contains(&0) {
            number = BigUint::zero();
        } else {
            number = digits
                .iter()
                .fold(BigUint::one(), |acc, &digit| acc * digit as u32);
        }
    }

    persistence
}

// TODO: Be smarter about skipping obviously bad digits.
fn search_for_maximum_multiplicative_persistence(start: &BigUint, end: &BigUint) {
    let mut working_num: BigUint = start.clone();
    let mut max_seen: u8 = 0;

    while working_num < *end {
        // An optimization to skip past numbers with 0s in them. We replace the
        // 0s with 1s, always making the number greater. This is a safe
        // search-space reduction because any number with any 0-digits will
        // immediately end persistence.
        // PERF: String allocation still present here, possible to avoid.
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

        working_num = working_num + BigUint::one() + BigUint::one();
    }
}

// Heurstic search, skipping unlikely digits.
// Reduces search space by ~70%.
fn search_for_new_record_multiplicative_persistence(start: &BigUint, end: &BigUint) {
    let mut working_num: BigUint = start.clone();
    let mut max_seen: u8 = 0;

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

        let persistence = calculate_multiplicative_persistence(working_num.clone());
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
        println!("Checked one; going next");
    }
}
