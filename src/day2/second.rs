/*
Confident that your list of box IDs is complete, you're ready to find the boxes full of prototype fabric.

The boxes will have IDs which differ by exactly one character at the same position in both strings.
For example, given the following box IDs:

abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz

The IDs abcde and axcye are close, but they differ by two characters (the second and fourth).
However, the IDs fghij and fguij differ by exactly one character, the third (h and u).
Those must be the correct boxes.

What letters are common between the two correct box IDs?
(In the example above, this is found by removing the differing character from either ID, producing fgij.)
*/

use std::collections::HashSet;

use super::super::global as Global;

pub fn run(input: &Vec<String>) -> String
{
    let identical_entries = get_identical(input);
    let mut identical_boxes_without_difference = String::new();

    for entry in identical_entries
        {
            identical_boxes_without_difference.push_str(&(fix_first_difference(&entry.0, &entry.1) + "\n"));
        }

    return identical_boxes_without_difference[0..identical_boxes_without_difference.len() - 1].to_string();
}

/// Determines whether or not the first string has at most 1 different character than the second string
///
/// # Examples
///
/// ```
/// let first = "abcde";
/// let second = "absde";
/// let third = "abcba";
///
/// assert_eq!(second::are_identical(first, first), true);
/// assert_eq!(second::are_identical(first, second), true);
/// assert_eq!(second::are_identical(first, third), false);
/// ```
fn are_identical(first: &str, second: &str) -> bool
{
    let mut found_difference = false;

    for (index, character) in first.chars().enumerate()
        {
            let characters_are_different = match second.chars().nth(index)
                {
                    Some(value) => character != value,
                    None => panic!(Global::CANT_ARE_IDENTICAL_MESSAGE)
                };

            if found_difference && characters_are_different { return false; }
            else if characters_are_different { found_difference = true; }
        }

    return true;
}

/// Returns a HashSet of String tuples containing identical pairs (determined by are_identical())
fn get_identical(codes: &Vec<String>) -> HashSet<(String, String)>
{
    let mut matches: HashSet<(String, String)> = HashSet::new();
    let mut strings_to_check: Vec<String> = Vec::with_capacity(codes.len());

    for code in codes
        {
            for currently_checking in &strings_to_check
                {
                    if are_identical(code, currently_checking)
                        {
                            matches.insert((code.clone(), currently_checking.clone()));
                        }
                }

            strings_to_check.push(code.clone());
        }

    return matches;
}

/// Returns a new string which remove the first difference occurrence between first and second from first.
///
/// # Examples
///
/// ```
/// let first = "abcde";
/// let second = "absde";
/// let third = "abcba";
///
/// assert_eq!(second::fix_first_difference(first, first), first);
/// assert_eq!(second::fix_first_difference(first, second), "abde");
/// assert_eq!(second::fix_first_difference(first, third), "abde");
/// ```
fn fix_first_difference(first: &str, second: &str) -> String
{
    for (index, character) in first.chars().enumerate()
        {
            let characters_are_different = match second.chars().nth(index)
                {
                    Some(value) => character != value,
                    None => panic!(Global::CANT_FIX_FIRST_DIFFERENCE_MESSAGE)
                };

            if characters_are_different
                {
                    return (&first[0..index]).to_string() + (&first[index + 1..]);
                }
        }

    return first.to_string();
}