/*

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?

*/

use super::utils;

pub fn solve(nth_prime: u64) -> u64 {
    let mut n: u64 = 128;
    loop {
        let primes = utils::atkin_sieve(n);
        if primes.len() > nth_prime as usize {
            return primes[nth_prime as usize - 1];
        }
        n *= 2;
    }
}

pub fn answer() -> u64 {
    solve(10001)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_7_answer() {
        let expected: u64 = 13;
        let result = solve(6);
    
        assert!(expected == result);
    }
}
