/*

The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.

*/

use super::utils;

pub fn solve(n: u64) -> u64 {
    let primes = utils::atkin_sieve(n);
    primes.iter().fold(0, |acc, x| acc + x)
}

pub fn answer() -> u64 {
    solve(2000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_10_answer() {
        let expected: u64 = 17;
        let result = solve(10);
    
        assert!(expected == result);
    }
}
