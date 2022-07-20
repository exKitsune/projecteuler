/*

A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.

*/

fn check_palindrome(original: u64) -> bool {
    let mut temp = original;
    let mut reverse: u64 = 0;

    while temp > 0 {
        reverse *= 10;
        reverse += temp % 10;
        temp /= 10;
    }

    original == reverse
}

pub fn solve(lower_bound: u64, upper_bound: u64) -> u64 {
    let mut largest = 0;

    for i in lower_bound..upper_bound {
        for j in lower_bound..upper_bound {
            let product = i * j;
            if check_palindrome(product) { 
                if product > largest { largest = product }
            }
        }
    }

    largest
}

pub fn answer() -> u64 {
    solve(100, 1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_4_answer() {
        let expected: u64 = 9009;
        let result = solve(10, 100);
    
        assert!(expected == result);
    }

    #[test]
    fn problem_4_palindrome() {
        assert!(check_palindrome(11111));
        assert!(check_palindrome(9009));
        assert!(check_palindrome(12344321));
    }
}
