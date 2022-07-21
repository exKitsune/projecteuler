/*

The sum of the squares of the first ten natural numbers is, 1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is, (1 + 2 + ... + 10)^2 = 3025

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

*/

use super::utils;

fn sum_of_squares(numbers: Vec<u64>) -> u64 {
    numbers.iter().fold(0, |acc, x| acc + x.pow(2))
}

fn square_of_sums(numbers: Vec<u64>) -> u64 {
    numbers.iter().fold(0, |acc, x| acc + x).pow(2)
}

pub fn solve(lower_bound: u64, upper_bound: u64) -> u64 {
    let numbers = utils::vec_from_bounds(lower_bound, upper_bound);
    ((square_of_sums(numbers.to_vec()) - sum_of_squares(numbers.to_vec())) as i64).abs() as u64
}

pub fn answer() -> u64 {
    solve(1, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_6_answer() {
        let expected: u64 = 2640;
        let result = solve(1, 10);
    
        assert!(expected == result);
    }
}
