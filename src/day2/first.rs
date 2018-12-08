/*
You stop falling through time, catch your breath, and check the screen on the device.
"Destination reached. Current Year: 1518. Current Location: North Pole Utility Closet 83N10."
You made it! Now, to find those anomalies.

Outside the utility closet, you hear footsteps and a voice. "...I'm not sure either.
But now that so many people have chimneys, maybe he could sneak in that way?"
Another voice responds, "Actually, we've been working on a new kind of suit that would let him fit
through tight spaces like that. But, I heard that a few days ago, they lost the prototype fabric,
the design plans, everything! Nobody on the team can even seem to remember important details of the project!"

"Wouldn't they have had enough fabric to fill several boxes in the warehouse?
They'd be stored together, so the box IDs should be similar. Too bad it would take forever
to search the warehouse for two similar box IDs..." They walk too far away to hear any more.

Late at night, you sneak to the warehouse - who knows what kinds of paradoxes you could cause
if you were discovered - and use your fancy wrist device to quickly scan every box and produce
a list of the likely candidates (your puzzle input).

To make sure you didn't miss any, you scan the likely candidate boxes again, counting the number
that have an ID containing exactly two of any letter and then separately counting those
with exactly three of any letter. You can multiply those two counts together to get
a rudimentary checksum and compare it to what your device predicts.

For example, if you see the following box IDs:

`abcdef` contains no letters that appear exactly two or three times.
`bababc` contains two a and three b, so it counts for both.
`abbcde` contains two b, but no letter appears exactly three times.
`abcccd` contains three c, but no letter appears exactly two times.
`aabcdd` contains two a and two d, but it only counts once.
`abcdee` contains two e.
`ababab` contains three a and three b, but it only counts once.

Of these box IDs, four of them contain a letter which appears exactly twice,
and three of them contain a letter which appears exactly three times.
Multiplying these together produces a checksum of 4 * 3 = 12.

What is the checksum for your list of box IDs?
*/

use std::collections::HashMap;

pub fn run(input: &Vec<String>) -> usize
{
    // First we initialize two counters; one for words that have 2 repeating characters,
    // one for words that have 3 repeating characters.
    let mut two_count: usize = 0;
    let mut three_count: usize = 0;

    for word in input
        {
            // char_to_count will map each character to the number of its occurences in the word.
            let mut char_to_count: HashMap<char, usize> = HashMap::with_capacity(word.len());

            // We iterate through each letter of the word we're looking at through chars().
            for letter in word.chars()
                {
                    // We initialize a reference to the entry letter, or if it doesn't exist,
                    // we first insert 0 at its place, then fetch it.
                    let letter_count = char_to_count.entry(letter).or_insert(0);

                    // We increase the letter count by one. Since we did it through a reference,
                    // it's like we updated the value.
                    *letter_count += 1;
                }

            // Since we only want to count if at least 1 character appeared twice or thrice,
            // we'll use booleans to flag whether or not it happened.
            let mut flag2 = false;
            let mut flag3 = false;

            // We'll then go through all the values, and if the conditions meet, we turn the flags.
            for entry in char_to_count.values()
                {
                    if *entry == 2usize    { flag2 = true; }
                    else if *entry == 3usize    { flag3 = true; }
                }

            if flag2    { two_count += 1; }
            if flag3    { three_count += 1; }
        }

    // Finally, our checksum is the sum of words with at least 2 repeating letters and 3 repeating letters.
    // So, we return the sum of the variables which counter how many such words exist.
    return two_count * three_count;
}