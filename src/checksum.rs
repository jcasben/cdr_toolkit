#![allow(dead_code)]

use std::{io::{self, Write}, num::ParseIntError};
use colored::Colorize;

/// Calculates the value of the checksum.
///
/// # Arguments
/// * `values` - a vector that holds the values for the
/// checksum in hexadecimal.
/// * `num_of_bits` - num of bits that will be represented.
///
/// # Returns
/// * The result of the checksum.
fn calculate_checksum(values: Vec<String>, num_of_bits: usize) -> String {
    let mut result: i64 = 0;

    for i in values.iter() {

        let x: i64 = match i64::from_str_radix(i.trim(), 2) {
            Ok(num) => {
                let mut num = num;
                num = ones_complement(num);
                // num = num << (64 - num_of_bits);
                // num = num >> (64 - num_of_bits);
                num
            },
            Err(err) => {
                eprintln!("\n{}{}", "ERROR: ".red(), err.to_string().red());
                0
            }
        };
        result += x;
    }

    result = !result;
    let hex_string = format!("{:b}", result);
    let mut truncated_hex_string = hex_string.chars().rev().take(num_of_bits).collect::<String>();
    truncated_hex_string = truncated_hex_string.chars().rev().collect::<String>();

    truncated_hex_string
}

fn ones_complement(mut num: i64) -> i64 {
    let mut cnt = 7;
    let mut tmp: i64;
    let mut flg = 0;
    
    while cnt >= 0 
    {
        tmp = num & (1 << cnt);
        if tmp > 0
        {
            flg=1;
            num &= !(1<<cnt);
        }
        else
        {
            if flg==1
            {
                num |= 1<<cnt;
            }
        }
        cnt=cnt-1;
    }

    num
}

/// Takes the binary user input for all the values of the checksum
/// and prints the result.
pub fn input_calculate_checksum() {
    let mut user_input = String::new();
    print!("{}", "Enter the number of values to be entered: ".blue());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("ERROR: Failed to read user input.");

    let num_of_elements = user_input.trim().parse::<i64>();

    let mut user_input = String::new();
    print!("{}", "Enter the number of bits per word: ".blue());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("ERROR: Failed to read user input.");

    let num_of_bits: Result<usize, ParseIntError> = user_input.trim().parse::<usize>();

    if num_of_elements.is_err() || num_of_bits.is_err() {
        eprintln!("\n{}{}", "ERROR: ".red(), "Couldn't parse the user input to a number".red());
        return;
    }

    let num_of_elements = num_of_elements.unwrap();
    let num_of_bits = num_of_bits.unwrap();

    let mut values: Vec<String> = Vec::with_capacity(num_of_elements as usize);
    let mut i = 0;
    while i < num_of_elements {
        let mut user_input = String::new();
        print!("\n{}{}: ", "Enter the binary value ".blue(), (i).to_string().blue());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).expect("ERROR: Failed to read user input.");

        values.push(user_input);
        i += 1;
    }

    println!("\n{}{}", "Checksum value (binary) = ".green(), calculate_checksum(values, num_of_bits));
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn calculate_checksum_test() {
    //     let values = vec!["0001".to_owned(), "F203".to_owned(), "F4F5".to_owned(), "F6F7".to_owned()];
    //     assert_eq!("220F".to_owned(), super::calculate_checksum(values, 16));
    // }
}