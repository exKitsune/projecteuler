/*

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?

*/

// use prime factors tree
fn find_primes(mut n: u64) -> Vec<u64> {
    let mut primes = Vec::<u64>::new();

    let mut divisor: u64 = 1;
    while divisor <= n {
        divisor += 1;

        if n % divisor == 0 {
            primes.push(divisor);
            n = n / divisor;
            divisor = 1;
        }     
    }

    primes
}



pub fn solve(n: u64) -> u64 {
    let primes = find_primes(n);
    let max_value = primes.iter().max();
    match max_value {
        Some(max) => *max,
        None      => panic!("Huh?"),
    }
}

pub fn answer() -> u64 {
    solve(600851475143)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_3_answer() {
        let expected: u64 = 29;
        let result = solve(13195);
    
        // println!("Expected {} got {}", EXPECTED, result);
        assert!(expected == result);
    }

    #[test]
    fn problem_3_find_primes() {
        let expected: Vec::<u64> = vec![5, 7, 13, 29];
        assert!(expected == find_primes(13195));
    }
}
