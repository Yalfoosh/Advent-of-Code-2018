/*
The Elves managed to locate the chimney-squeeze prototype fabric for Santa's suit
(thanks to someone who helpfully wrote its box IDs on the wall of the warehouse in the middle of the night).
Unfortunately, anomalies are still affecting them - nobody can even agree on how to cut the fabric.

The whole piece of fabric they're working on is a very large square - at least 1000 inches on each side.

Each Elf has made a claim about which area of fabric would be ideal for Santa's suit.
All claims have an ID and consist of a single rectangle with edges parallel to the edges of the fabric.
Each claim's rectangle is defined as follows:

The number of inches between the left edge of the fabric and the left edge of the rectangle.
The number of inches between the top edge of the fabric and the top edge of the rectangle.
The width of the rectangle in inches.
The height of the rectangle in inches.
A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3 inches from the left edge,
2 inches from the top edge, 5 inches wide, and 4 inches tall. Visually, it claims the square inches
of fabric represented by # (and ignores the square inches of fabric represented by .) in the diagram below:

...........
...........
...#####...
...#####...
...#####...
...#####...
...........
...........
...........

The problem is that many of the claims overlap, causing two or more claims to cover part of the same areas.
For example, consider the following claims:

#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2

Visually, these claim the following areas:

........
...2222.
...2222.
.11XX22.
.11XX22.
.111133.
.111133.
........

The four square inches marked with X are claimed by both 1 and 2.
(Claim 3, while adjacent to the others, does not overlap either of them.)

If the Elves all proceed with their own plans, none of them will have enough fabric.
How many square inches of fabric are within two or more claims?
*/

use std::collections::HashMap;

pub fn run(input: &Vec<(usize, usize, usize, usize)>) -> (usize, HashMap<usize, HashMap<usize, usize>>)
{
    let mut map: HashMap<usize, HashMap<usize, usize>> = HashMap::new();

    for entry in input { map = claim_fabric(map, entry); }

    return (count_multiple_claims(&map), map);
}

/// Returns a modified version of the map given as an argument with a placed order (x, y, width, height)
fn claim_fabric(map: HashMap<usize, HashMap<usize, usize>>, order: &(usize, usize, usize, usize)) -> HashMap<usize, HashMap<usize, usize>>
{
    // First we claim ownership of the map that we'll modify.
    let mut modifying = map;

    // For every square inch of width...
    for x in 0..order.2
    {
        // We find the entry for that x coordinate + the current portion of width we're at,
        // or if it doesn't exist, we insert a new HashMap for it.
        let x_coordinated = modifying.entry(order.0 + x).or_insert(HashMap::new());

        // For every square inch of height...
        for y in 0..order.3
        {
            // We find the entry for that y coordinate + the current portion of height we're at,
            // or if it doesn't exist, we insert a new usize in it, with value 0.
            let fully_coordinated = (*x_coordinated).entry(order.1 + y).or_insert(0);

            // Adding 1 to the HashMap entry is like claiming the square inch.
            *fully_coordinated += 1;
        }
    }

    // Finally, we return the modified HashMap and give ownership to the caller.
    return modifying;
}

/// Returns the number of square inches in the map given in arguments where fabric was claimed
/// more than once.
fn count_multiple_claims(map: &HashMap<usize, HashMap<usize, usize>>) -> usize
{
    let mut count = 0usize;

    for x_coordinated in map.values()
    {
        for fully_coordinated in x_coordinated.values()
        {
            if *fully_coordinated != 1usize { count += 1; }
        }
    }

    return count;
}