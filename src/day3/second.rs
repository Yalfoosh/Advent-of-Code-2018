/*
Amidst the chaos, you notice that exactly one claim doesn't overlap by even
a single square inch of fabric with any other claim. If you can somehow draw attention to it,
maybe the Elves will be able to make Santa's suit after all!

For example, in the claims above, only claim 3 is intact after all claims are made.
What is the ID of the only claim that doesn't overlap?
*/

use std::collections::HashMap;

use super::super::global as Global;

pub fn run(input: &Vec<(usize, usize, usize, usize)>, map: &HashMap<usize, HashMap<usize, usize>>) -> Result<usize, String>
{
    // We will trust the task saying that only one order is placed on square inches only it affects;
    // This looks for its first occurence.
    // If it wasn't like that, we'd be returning Result<Vec<usize>>, String>, adjusted accordingly.
    for (i, order) in input.iter().enumerate()
        {
            if is_order_taken_once(order, map) { return Ok(i + 1); }
        }

    return Err(Global::DAY3_ORDER_NOT_FOUND_ERROR.to_string());
}

/// Returns true if all the square inches of the current order have been placed only once, false otherwise.
fn is_order_taken_once(order: &(usize, usize, usize, usize), map: &HashMap<usize, HashMap<usize, usize>>) -> bool
{
    for x in 0..order.2
        {
            match map.get(&(order.0 + x))
                {
                    Some(value) =>
                        {
                            for y in 0..order.3
                                {
                                    match value.get(&(order.1 + y))
                                    {
                                        Some(value2) => if *value2 != 1usize { return false; }
                                        None => { continue; }
                                    }
                                }
                        }
                    None => { continue; }
                }
        }

    return true;
}