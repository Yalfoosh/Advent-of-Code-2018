use std::fs;

use super::super::global as Global;
use super::first;
use super::second;

pub fn main()
{
    let input = get_input_vector(Global::DAY2_INPUT_PATH);

    run_first(&input);
    run_second(&input);

    println!("\n");
}

fn get_input_vector(path: &str) -> Vec<String>
{
    let input = match fs::read_to_string(path)
        {
            Ok(value) => value,
            Err(_) =>
                {
                    match fs::read_to_string(Global::DAY2_INPUT_PATH.replace("src/", "./"))
                        {
                            Ok(value) => value,
                            Err(_) => panic!(Global::FILE_NOT_FOUND_PANIC_MESSAGE)
                        }
                }
        };

    // Every line in our input is going to be a new String we'll be looking at.
    let mut input_vector: Vec<String> = Vec::new();

    for line in input.lines() { input_vector.push(line.to_string()); }

    return input_vector
}

fn run_first(input: &Vec<String>)
{
    println!("{} {}.", Global::DAY2_CHECKSUM_TEXT, first::run(input));
}

fn run_second(input: &Vec<String>)
{
    println!("{}:\n{}.", Global::DAY2_COMMON_LETTERS_TEXT, second::run(input));
}