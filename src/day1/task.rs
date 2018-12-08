use std::fs;

use super::super::global as Global;
use super::first;
use super::second;

pub fn main()
{
    let input = get_input_vector();

    run_first(Global::DAY1_STARTING_FREQUENCY, &input);
    run_second(Global::DAY1_STARTING_FREQUENCY, &input);

    println!("\n");
}

fn get_input_vector() -> Vec<i32>
{
    let read_lines = match fs::read_to_string(Global::DAY1_INPUT_PATH)
        {
            Ok(value) => value,
            Err(_) =>
                {
                    match fs::read_to_string(Global::DAY1_INPUT_PATH.replace("src/", "./"))
                        {
                            Ok(value) => value,
                            Err(_) => panic!(Global::FILE_NOT_FOUND_PANIC_MESSAGE)
                        }
                }
        };

    let mut input_vector: Vec<i32> = Vec::new();

    for line in read_lines.lines()
        {
            match line.parse::<i32>()
                {
                    Ok(value) => input_vector.push(value),
                    Err(_) => { println!("{}", Global::SOME_NUMBER_IS_INVALID_MESSAGE); }
                }
        }

    return input_vector;
}

fn run_first(starting_frequency: i32, input: &Vec<i32>)
{
    println!("{} {}.", Global::DAY_1_SUM_TEXT, first::run(starting_frequency, input));
}

fn run_second(starting_frequency: i32, input: &Vec<i32>)
{
    println!("{} {}.", Global::DAY_1_SECOND_FREQUENCY_TEXT, second::run(starting_frequency, input))
}