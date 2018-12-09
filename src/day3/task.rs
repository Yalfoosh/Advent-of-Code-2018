use std::fs;
use std::collections::HashMap;

use super::super::global as Global;
use super::first;
use super::second;

pub fn main()
{
    let input = get_input_vector(Global::DAY3_INPUT_PATH);

    let map = run_first(&input);
    run_second(&input, &map);

    println!("\n");
}

fn get_input_vector(path: &str) -> Vec<(usize, usize, usize, usize)>
{
    let input = match fs::read_to_string(path)
    {
        Ok(value) => value,
        Err(_) =>
        {
            match fs::read_to_string(path.replace("src/", "./"))
            {
                Ok(value) => value,
                Err(_) => panic!(Global::FILE_NOT_FOUND_PANIC_MESSAGE)
            }
        }
    };

    // Every line in our input is going to be a new String we'll be looking at.
    let mut input_vector: Vec<(usize, usize, usize, usize)> = Vec::new();

    for line in input.lines()
    {
        let first_separation = line.split(Global::DAY3_FIRST_SEPARATOR).collect::<Vec<&str>>();
        let coords_and_size = first_separation[1].split(Global::DAY3_COORDINATE_SIZE_SEPARATOR).collect::<Vec<&str>>();
        let coords = coords_and_size[0].split(Global::DAY3_COORDINATE_SEPARATOR).collect::<Vec<&str>>();
        let size = coords_and_size[1].split(Global::DAY3_SIZE_SEPARATOR).collect::<Vec<&str>>();

        input_vector.push((coords[0].parse::<usize>().unwrap(), coords[1].parse::<usize>().unwrap(),
                             size[0].parse::<usize>().unwrap(), size[1].parse::<usize>().unwrap()));
    }

    return input_vector;
}

fn run_first(input: &Vec<(usize, usize, usize, usize)>) -> HashMap<usize, HashMap<usize, usize>>
{
    let result = first::run(input);

    println!("{} {}", result.0, Global::DAY3_SQUARE_INCHES_MESSAGE);

    return result.1;
}

fn run_second(input: &Vec<(usize, usize, usize, usize)>, map: &HashMap<usize, HashMap<usize, usize>>)
{
    match second::run(input, map)
    {
        Ok(value) => println!("{} {}.", Global::DAY3_ID_NOT_TAKEN_MESSAGE, value),
        Err(e) => println!("{}", e)
    }
}