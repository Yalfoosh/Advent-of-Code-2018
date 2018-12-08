use std::io;
use std::process;

mod global;
mod day1;
mod day2;

use day1::task as D1task;
use day2::task as D2task;


fn main()
{
    loop
    {
        let mut input = String::new();

        println!("{}", global::MAIN_DAY_MESSAGE);

        match io::stdin().read_line(&mut input)
            {
                Ok(_) => (),
                Err(_) => println!("{}", global::MAIN_INVALID_INPUT_MESSAGE)
            }

        let input = input.trim();

        match input
            {
                "exit" => { process::exit(0); }
                _ => ()
            }

        match input.parse::<usize>()
            {
                Ok(day) => match day
                    {
                        0 => println!("{}", global::MAIN_NO_DAY_1_MESSAGE),
                        1 => { D1task::main() },
                        2 => { D2task::main() },
                        x => println!("{} ({})\n\n", global::MAIN_DAY_NOT_IMPLEMENTED_MESSAGE, x)
                    }
                Err(_) => println!("{}", global::MAIN_POSITIVE_NUMBER_NEEDED_MESSAGE)
            }
    }
}