use std::io;
use std::io::Write;
use std::process;

mod global;
mod day1;
mod day2;
mod day3;

use day1::task as D1task;
use day2::task as D2task;
use day3::task as D3task;


fn main()
{
    loop
    {
        // Assign a variable for our input; it's a simply string.
        let mut input = String::new();

        print!("{}: ", global::MAIN_DAY_MESSAGE);

        match io::stdout().flush()
        {
            Ok(_) => (),
            Err(_) => ()
        }

        match io::stdin().read_line(&mut input)
        {
            Ok(_) => (),
            Err(_) => println!("{}", global::MAIN_INVALID_INPUT_MESSAGE)
        }

        //Trimming input because whitespaces are a nuisance.
        let input = input.trim();

        match input
        {
            //exit will simply exit the process with code 0.
            "exit" => { process::exit(0); }
            _ => ()
        }

        println!("");

        //If our input wasn't matched above, assume the user is entering the day number they
        //want to load. We're parsing it to usize.
        match input.parse::<usize>()
        {
            Ok(day) => match day
            {
                0 => println!("{}", global::MAIN_NO_DAY_1_MESSAGE),
                1 => { D1task::main() },
                2 => { D2task::main() },
                3 => { D3task::main() },
                x => println!("{} ({})\n\n", global::MAIN_DAY_NOT_IMPLEMENTED_MESSAGE, x)
            }

            //This triggers if the user entered something that wasn't a N+ number.
            Err(_) => println!("{}", global::MAIN_POSITIVE_NUMBER_NEEDED_MESSAGE)
        }
    }
}