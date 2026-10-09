use std::fs::File;
use rand::RngExt;

use std::io;
use std::io::Write;

fn main() {
    // the range including zero and five
    // 0 ..< 5 the range from 0 excluding 5

    // loop is an infinite loop
    let result = loop {
        let rand = rand::rng().random_range(0..=5);
        println!("Enter a number");

        // In Rust variables are immutable by default
        // To make a variable mutable we add mut before it
        let mut input = String::new(); // provides a new String

        // StdIn a type that is a handle to the standard input
        // & indicates that the argument is a reference
        // As variables, references are immutable by default

        // read_line also returns a Result value. Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.
        // The purpose of Result is to encode error-handling info
        // Ok variant indicates the operation was successful, and it contains the successfully generated value. The Err variant means the operation failed, and it contains information about how or why the operation failed.
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut number: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break true,
        };
        if number == 0 {
            break false;
        }
        number += rand;
        println!("Added by random [0,5] = {number}");

        let power_array = powers(number);
        let conjecture_array = does_conjecture_hold(power_array);
        let mut file = File::create("xyz.txt")
            .expect("Failed to create file");
        let bytes : [u8; 10] = conjecture_array.map(|b| b as u8);
        file.write_all(&bytes)
            .expect("Failed to write to file");
    };

    match result {
        false => println!("The loop ended on user's request"),
        true => println!("There was an error"),
    }

    let (start, _, _)  = first_climb_above(1000);
    println!("First number to climb above {start} in Collatz Conjecture"); // {} can keep only variables
    //println!("First number to climb above {start} in Collatz Conjecture", start = result.0); alternative if we nee dan expression
}

fn first_climb_above(threshold: u64) -> (u64, u32, u64) {
    let mut result = (0, 0, 0);

    'outer: for start in 1u64.. {
        let mut num = start;
        let mut step: u32 = 0;

        while num != 1 {
            num = if num.is_multiple_of(2) { num / 2 } else { 3 * num + 1 };
            step += 1;

            if num > threshold {
                result = (start, step, num);
                break 'outer;
            }
        }
    }
    result
}

fn powers(number: u64) -> [u64; 10] {
    let powers: [u64; 10] = std::array::from_fn(|i| number ^ i as u64);
    powers
}

fn does_conjecture_hold(arr: [u64; 10]) -> [bool; 10] {
    // let result: [bool; 10] = arr.map(|mut num| {
    //     for _ in 0..=100 {
    //         if num == 1 {
    //             return true;
    //         }
    //         if num.is_multiple_of(2) {
    //             num /= 2;
    //         } else {
    //             num = 3 * num + 1;
    //         }
    //     }
    //     false
    // });
    // result
    let mut result: [bool; 10] = [true; 10];
    for (index, &num) in arr.iter().enumerate() {
        let mut num = num;
        let flag = 'check: {
            for _ in 0..=100 {
                if num == 1 {
                    break 'check true;
                }
                if num.is_multiple_of(2) {
                    num /= 2;
                } else {
                    num = 3 * num + 1;
                }
            }
            false
        };
        result[index] = flag;
    }
    result
}
