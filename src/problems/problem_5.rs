/*

2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

*/

use super::utils;

pub fn solve(lower_bound: u64, upper_bound: u64) -> u64 {
    // create array of numbers from lower to upper
    let values: Vec<u64> = (lower_bound..=upper_bound).collect();

    values.iter().fold(1, |acc, x| utils::lcm(acc, *x))
}

pub fn answer() -> u64 {
    solve(1, 20)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_5_answer() {
        let expected: u64 = 2520;
        let result = solve(1, 10);
    
        assert!(expected == result);
    }
}
