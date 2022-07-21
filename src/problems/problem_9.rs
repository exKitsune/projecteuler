/*

A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
a^2 + b^2 = c^2

For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.

*/

// use super::utils;

fn pyth_triplet_from_sum(sum: u64) -> Option<[u64; 3]> {
    let mut a: u64 = 1;
    let mut b: u64 = 2;
    let mut c: u64 = sum - b - a;

    // ceiling operation
    'ml: loop {
        while b > a {
            a = sum - c - b;
            // println!("{} {} {}", a, b, c);
            
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return Some([a,b,c]);
            }
            if c == b + 1 && b == a + 1 { break 'ml }
            b -= 1;
        }

        c -= 1;
        b = sum - c - 1;
    }

    None
}

pub fn solve(sum: u64) -> u64 {
    let triplet = pyth_triplet_from_sum(sum);
    match triplet {
        Some(t) => t[0] * t[1] * t[2],
        _ => 0,
    }
}

pub fn answer() -> u64 {
    solve(1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_9_triplet() {
        let expected: [u64; 3] = [3, 4, 5];
        let result = pyth_triplet_from_sum(12);
    
        assert!(expected == result.unwrap());
    }
}
